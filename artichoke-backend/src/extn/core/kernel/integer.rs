use std::collections::HashMap;
use std::str::FromStr;

use crate::convert::{Convert, TryConvert};
use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    BaseSpecifiedForNonString(bool),
    ContainsNullByte(bool),
    InvalidValue(String, bool),
    InvalidRadix(String, bool),
    NoImplicitConversionToString(String, bool),
}

#[derive(Debug)]
pub struct Args<'a> {
    pub arg: &'a str,
    pub radix: Option<i64>,
    pub raise_exception: bool,
}

impl Args<'_> {
    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, Error> {
        let mrb = interp.0.borrow().mrb;

        // TODO: when `base` is not passed, but `exception: true` is,
        // exception argument goes into arg, which is not right,
        // we might want to get the argument manually.
        let (arg, base, exception) = mrb_get_args!(mrb, required = 1, optional = 2);

        let raise_exception: Option<HashMap<String, bool>> = match exception {
            Some(exception) => match interp.try_convert(Value::new(interp, exception)) {
                Ok(exception) => exception,
                Err(_) => None,
            },
            _ => None,
        };
        let raise_exception: bool = if let Some(raise_exception) = raise_exception {
            *raise_exception.get("exception").unwrap_or_else(|| &true)
        } else {
            true
        };
        let radix: Option<i64> = match base {
            Some(base) => {
                if let Ok(base) = interp.try_convert(Value::new(interp, base)) {
                    base
                } else {
                    None
                }
            }
            _ => None,
        };

        let value = Value::new(interp, arg);
        if value.ruby_type() == Ruby::Fixnum && radix.is_some() {
            return Err(Error::BaseSpecifiedForNonString(raise_exception));
        }

        if let Ok(arg) = interp.try_convert(Value::new(interp, arg)) {
            Ok(Self {
                arg,
                radix,
                raise_exception,
            })
        } else {
            Err(Error::NoImplicitConversionToString(
                value.pretty_name(),
                raise_exception,
            ))
        }
    }
}

pub fn method(interp: &Artichoke, args: &Args) -> Result<Value, Error> {
    let arg = args.arg;
    let radix = args.radix;
    let raise_exception = args.raise_exception;

    let mut digits = String::new();
    let mut sign = None;
    let mut err = None;
    for i in 0..arg.len() {
        let c = arg.chars().nth(i).unwrap(); // we're sure that it won't be `None`

        // handle space between sign & digit, it should error!
        if c.is_whitespace() {
            if i == 0 {
                continue; // ugly workaround for i - 1 index out of bound!
            }

            if let Some(prev_c) = arg.chars().nth(i - 1) {
                if prev_c == '+' || prev_c == '-' {
                    err = Some(Error::InvalidValue(arg.into(), raise_exception));
                    break;
                } else {
                    continue;
                }
            }
        }

        // ignore an embedded `_`
        if c == '_' && i > 0 && i < arg.len() - 1 {
            let next_c = arg.chars().nth(i + 1).unwrap();
            let prev_c = arg.chars().nth(i - 1).unwrap();

            if next_c.is_numeric() && prev_c.is_numeric() {
                continue;
            }
        }

        if c == '\0' {
            err = Some(Error::ContainsNullByte(raise_exception));
            break;
        }

        if c == '+' || c == '-' {
            // handle >1 consecutive sign
            let next_c = arg.chars().nth(i + 1);
            if next_c.is_none() || next_c == Some('+') || next_c == Some('-') {
                err = Some(Error::InvalidValue(arg.into(), raise_exception));
                break;
            }
            sign = Some(c);
        } else {
            digits.push(c);
        }
    }

    if let Some(err) = err {
        return Err(err);
    }

    let mut parsed_radix = None;
    if digits.len() >= 2 {
        match &digits[0..2] {
            "0b" | "0B" => {
                digits = digits[2..].to_string();
                parsed_radix = Some(2);
            }
            "0o" | "0O" => {
                digits = digits[2..].to_string();
                parsed_radix = Some(8);
            }
            "0d" | "0D" => {
                digits = digits[2..].to_string();
                parsed_radix = Some(10);
            }
            "0x" | "0X" => {
                digits = digits[2..].to_string();
                parsed_radix = Some(16);
            }
            prefix if &prefix[0..1] == "0" => {
                digits = digits[1..].to_string();
                parsed_radix = Some(8);
            }
            _ => {}
        };
    }

    if let Some(sign) = sign {
        digits.insert(0, sign);
    }

    match (radix, parsed_radix) {
        (Some(radix), Some(parsed_radix)) => {
            if radix != parsed_radix {
                return Err(Error::InvalidValue(digits, raise_exception));
            }
            if let Ok(v) = i64::from_str_radix(digits.as_str(), radix as u32) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(digits, raise_exception))
            }
        }
        (Some(radix), None) | (None, Some(radix)) => {
            if radix < 2 || radix > 36 {
                return Err(Error::InvalidRadix(radix.to_string(), raise_exception));
            }
            if let Ok(v) = i64::from_str_radix(digits.as_str(), radix as u32) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(digits, raise_exception))
            }
        }
        (None, None) => {
            if let Ok(v) = i64::from_str(digits.as_str()) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(digits, raise_exception))
            }
        }
    }
}
