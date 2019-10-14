use artichoke_core::value::Value as ValueLike;
use std::convert::TryFrom;
use std::iter::Iterator;
use std::str::FromStr;

use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Error<'a> {
    ContainsNullByte,
    InvalidValue(&'a str),
    InvalidRadix(Int),
    NoImplicitConversionToString(&'a str),
}

#[derive(Copy, Clone, Debug)]
pub struct Args<'a> {
    pub arg: &'a str,
    pub radix: Option<Int>,
}

impl<'a> Args<'a> {
    pub unsafe fn extract(
        interp: &Artichoke,
        arg: sys::mrb_value,
        base: Option<sys::mrb_value>,
    ) -> Result<Self, Error> {
        let radix = base
            .map(|base| Value::new(interp, base))
            .and_then(|base| base.try_into::<Int>().ok());
        let value = Value::new(interp, arg);
        let value_type = value.pretty_name();
        if let Ok(arg) = interp.try_convert(value) {
            Ok(Args { arg, radix })
        } else {
            Err(Error::NoImplicitConversionToString(value_type))
        }
    }
}

pub fn method<'a>(interp: &'a Artichoke, args: &Args<'a>) -> Result<Value, Error<'a>> {
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
    let arg = args.arg;
    let radix = match args.radix.map(u32::try_from) {
        Some(Ok(radix)) => Some(radix),
        Some(Err(_)) => return Err(Error::InvalidRadix(args.radix.unwrap_or_default())),
        None => None,
    };

    let mut state = ParseState::Initial;
    let mut chars = arg.chars().skip_while(|c| c.is_whitespace()).peekable();
    let mut prev = None::<char>;
    while chars.peek().is_some() {
        let current = chars.next();

        if let Some('\0') = current {
            return Err(Error::ContainsNullByte);
        }

        // Ignore an embedded underscore (`_`).
        if let Some('_') = current {
            let valid_prev = prev
                .map(|prev| prev.is_numeric() || prev.is_alphabetic())
                .unwrap_or_default();
            let next = chars.peek();
            let valid_next = next
                .map(|next| next.is_numeric() || next.is_alphabetic())
                .unwrap_or_default();
            if valid_prev && valid_next {
                prev = current;
                continue;
            }
        }
        if current.map(char::is_whitespace).unwrap_or_default() {
            if let Some('+') | Some('-') = prev {
                return Err(Error::InvalidValue(arg));
            } else {
                prev = current;
                continue;
            }
        }

        state = match current {
            Some('+') => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Error::InvalidValue(arg));
                }
                ParseState::Sign(Sign::Pos)
            }
            Some('-') => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Error::InvalidValue(arg));
                }
                ParseState::Sign(Sign::Neg)
            }
            Some(digit) => match state {
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
            None => return Err(Error::InvalidValue(arg)),
        };
        prev = current;
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
                        return Err(Error::InvalidValue(arg));
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
        return Err(Error::InvalidValue(arg));
    };

    match (radix, parsed_radix) {
        (Some(radix), Some(parsed_radix)) if radix == parsed_radix => {
            if let Ok(integer) = Int::from_str_radix(candidate.as_str(), radix) {
                Ok(interp.convert(integer))
            } else {
                Err(Error::InvalidValue(arg))
            }
        }
        (Some(radix), None) | (None, Some(radix)) if radix >= 2 && radix <= 36 => {
            if let Ok(integer) = Int::from_str_radix(candidate.as_str(), radix) {
                Ok(interp.convert(integer))
            } else {
                Err(Error::InvalidValue(arg))
            }
        }
        (None, None) => {
            if let Ok(integer) = Int::from_str(candidate.as_str()) {
                Ok(interp.convert(integer))
            } else {
                Err(Error::InvalidValue(arg))
            }
        }
        (Some(_), Some(_)) => Err(Error::InvalidValue(arg)),
        (Some(radix), None) | (None, Some(radix)) => Err(Error::InvalidRadix(Int::from(radix))),
    }
}
