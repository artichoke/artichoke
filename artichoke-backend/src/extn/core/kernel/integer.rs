use bstr::ByteSlice;
use std::convert::TryFrom;
use std::iter::Iterator;
use std::str::{self, FromStr};

use crate::extn::prelude::*;

pub fn method(
    interp: &mut Artichoke,
    arg: Value,
    radix: Option<Value>,
) -> Result<Value, Exception> {
    #[derive(Debug, Clone, Copy)]
    enum Sign {
        Pos,
        Neg,
    }
    #[derive(Debug, Clone)]
    enum ParseState {
        Initial,
        Sign(Sign),
        Accumulate(Sign, String),
    }
    let radix = if let Some(radix) = radix {
        radix.implicitly_convert_to_int(interp).ok()
    } else {
        None
    };
    let radix = match radix.map(u32::try_from) {
        Some(Ok(radix)) if radix >= 2 && radix <= 36 => Some(radix),
        Some(Ok(radix)) => return Err(Exception::from(invalid_radix_error(interp, radix)?)),
        Some(Err(_)) => return Err(Exception::from(ArgumentError::new(interp, "invalid radix"))),
        None => None,
    };
    let arg = arg.implicitly_convert_to_string(interp).map_err(|_| {
        let mut message = String::from("can't convert ");
        message.push_str(arg.pretty_name(interp));
        message.push_str(" into Integer");
        TypeError::new(interp, message)
    })?;
    if arg.find_byte(b'\0').is_some() {
        return Err(Exception::from(invalid_value_err(interp, arg)?));
    }
    let arg = if let Ok(arg) = str::from_utf8(arg) {
        arg
    } else {
        return Err(Exception::from(invalid_value_err(interp, arg)?));
    };

    let mut state = ParseState::Initial;
    let mut chars = arg.chars().skip_while(|c| c.is_whitespace()).peekable();
    let mut prev = None::<char>;

    while let Some(current) = chars.next() {
        // Ignore an embedded underscore (`_`).
        if current == '_' {
            let valid_prev = prev
                .map(|prev| prev.is_numeric() || prev.is_alphabetic())
                .unwrap_or_default();
            let next = chars.peek();
            let valid_next = next
                .map(|next| next.is_numeric() || next.is_alphabetic())
                .unwrap_or_default();
            if valid_prev && valid_next {
                prev = Some(current);
                continue;
            }
        }
        if current.is_whitespace() {
            if let Some('+') | Some('-') = prev {
                return Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?));
            } else {
                prev = Some(current);

                continue;
            }
        }

        state = match current {
            '+' => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?));
                }
                ParseState::Sign(Sign::Pos)
            }
            '-' => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?));
                }
                ParseState::Sign(Sign::Neg)
            }
            digit => match state {
                ParseState::Initial => {
                    let mut digits = String::new();
                    digits.push(digit);
                    ParseState::Accumulate(Sign::Pos, digits)
                }
                ParseState::Sign(sign) => {
                    let mut digits = String::new();
                    digits.push(digit);
                    ParseState::Accumulate(sign, digits)
                }
                ParseState::Accumulate(sign, mut digits) => {
                    digits.push(digit);
                    ParseState::Accumulate(sign, digits)
                }
            },
        };
        prev = Some(current);
    }

    let (candidate, parsed_radix) = if let ParseState::Accumulate(sign, mut digits) = state {
        let parsed_radix = match digits.chars().take(2).collect::<String>().as_str() {
            "0b" | "0B" => {
                digits.drain(..2);
                Some(2)
            }
            "0o" | "0O" => {
                digits.drain(..2);
                Some(8)
            }
            "0d" | "0D" => {
                digits.drain(..2);
                Some(10)
            }
            "0x" | "0X" => {
                digits.drain(..2);
                Some(16)
            }
            prefix => {
                let mut chars = prefix.chars();
                let first = chars.next();
                let next = chars.next();
                if let Some(next) = next {
                    if !next.is_numeric() && !next.is_alphabetic() {
                        return Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?));
                    } else if let Some('0') = first {
                        digits.drain(..1);
                        Some(8)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };

        if let Sign::Neg = sign {
            digits.insert(0, '-');
        }
        (digits, parsed_radix)
    } else {
        return Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?));
    };

    match (radix, parsed_radix) {
        (Some(radix), Some(parsed_radix)) if radix == parsed_radix => {
            if let Ok(integer) = Int::from_str_radix(candidate.as_str(), radix) {
                Ok(interp.convert(integer))
            } else {
                Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?))
            }
        }
        (Some(radix), None) | (None, Some(radix)) if radix >= 2 && radix <= 36 => {
            if let Ok(integer) = Int::from_str_radix(candidate.as_str(), radix) {
                Ok(interp.convert(integer))
            } else {
                Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?))
            }
        }
        (None, None) => {
            if let Ok(integer) = Int::from_str(candidate.as_str()) {
                Ok(interp.convert(integer))
            } else {
                Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?))
            }
        }
        (Some(_), Some(_)) => Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?)),
        (Some(radix), None) | (None, Some(radix)) => {
            Err(Exception::from(invalid_radix_error(interp, radix)?))
        }
    }
}

fn invalid_value_err(interp: &Artichoke, arg: &[u8]) -> Result<ArgumentError, Exception> {
    let mut message = String::from(r#"invalid value for Integer(): ""#);
    string::format_unicode_debug_into(&mut message, arg)?;
    message.push('"');
    Ok(ArgumentError::new(interp, message))
}

fn invalid_radix_error(
    interp: &Artichoke,
    radix: u32,
) -> Result<ArgumentError, string::WriteError> {
    let mut message = String::from("invalid radix ");
    string::format_int_into(&mut message, radix)?;
    Ok(ArgumentError::new(interp, message))
}
