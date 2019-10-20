use artichoke_core::value::Value as _;
use bstr::BStr;
use std::convert::TryFrom;
use std::iter::Iterator;
use std::str::{self, FromStr};

use crate::convert::Convert;
use crate::extn::core::exception::{ArgumentError, RubyException, TypeError};
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

pub fn method(
    interp: &Artichoke,
    arg: Value,
    radix: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
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
        if let Ok(radix) = radix.clone().try_into::<Int>() {
            Some(radix)
        } else if let Ok(radix) = radix.funcall::<Int>("to_int", &[], None) {
            Some(radix)
        } else {
            None
        }
    } else {
        None
    };
    let radix = match radix.map(u32::try_from) {
        Some(Ok(radix)) if radix >= 2 && radix <= 36 => Some(radix),
        Some(Ok(radix)) => {
            return Err(Box::new(ArgumentError::new(
                interp,
                format!("invalid radix {}", radix),
            )))
        }
        Some(Err(_)) => {
            return Err(Box::new(ArgumentError::new(
                interp,
                format!("invalid radix {}", radix.unwrap_or_default()),
            )))
        }
        None => None,
    };
    let ruby_type = arg.pretty_name();
    let arg = if let Ok(arg) = arg.clone().try_into::<&[u8]>() {
        arg
    } else if let Ok(arg) = arg.funcall::<&[u8]>("to_str", &[], None) {
        arg
    } else {
        return Err(Box::new(TypeError::new(
            interp,
            format!("can't convert {} into Integer", ruby_type),
        )));
    };
    if memchr::memchr(b'\0', arg).is_some() {
        return Err(Box::new(ArgumentError::new(
            interp,
            format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
        )));
    }
    let arg = if let Ok(arg) = str::from_utf8(arg) {
        arg
    } else {
        return Err(Box::new(ArgumentError::new(
            interp,
            format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
        )));
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
                return Err(Box::new(ArgumentError::new(
                    interp,
                    format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
                )));
            } else {
                prev = Some(current);

                continue;
            }
        }

        state = match current {
            '+' => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Box::new(ArgumentError::new(
                        interp,
                        format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
                    )));
                }
                ParseState::Sign(Sign::Pos)
            }
            '-' => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Box::new(ArgumentError::new(
                        interp,
                        format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
                    )));
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
                        return Err(Box::new(ArgumentError::new(
                            interp,
                            format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
                        )));
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
        return Err(Box::new(ArgumentError::new(
            interp,
            format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
        )));
    };

    match (radix, parsed_radix) {
        (Some(radix), Some(parsed_radix)) if radix == parsed_radix => {
            if let Ok(integer) = Int::from_str_radix(candidate.as_str(), radix) {
                Ok(interp.convert(integer))
            } else {
                Err(Box::new(ArgumentError::new(
                    interp,
                    format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
                )))
            }
        }
        (Some(radix), None) | (None, Some(radix)) if radix >= 2 && radix <= 36 => {
            if let Ok(integer) = Int::from_str_radix(candidate.as_str(), radix) {
                Ok(interp.convert(integer))
            } else {
                Err(Box::new(ArgumentError::new(
                    interp,
                    format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
                )))
            }
        }
        (None, None) => {
            if let Ok(integer) = Int::from_str(candidate.as_str()) {
                Ok(interp.convert(integer))
            } else {
                Err(Box::new(ArgumentError::new(
                    interp,
                    format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
                )))
            }
        }
        (Some(_), Some(_)) => Err(Box::new(ArgumentError::new(
            interp,
            format!(r#"invalid value for Integer(): "{}"#, <&BStr>::from(arg)),
        ))),
        (Some(radix), None) | (None, Some(radix)) => Err(Box::new(ArgumentError::new(
            interp,
            format!("invalid radix {}", radix),
        ))),
    }
}
