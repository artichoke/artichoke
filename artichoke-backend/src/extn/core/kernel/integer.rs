use std::convert::TryInto;
use std::str::FromStr;

use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Error {
    ContainsNullByte,
    InvalidValue(String),
    InvalidRadix(i64),
    NoImplicitConversionToString(String),
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
        let radix: Option<Int> = if let Some(base) = base {
            match interp.try_convert(Value::new(interp, base)) {
                Ok(base) => base,
                _ => None,
            }
        } else {
            None
        };

        let value = Value::new(interp, arg);
        let value_type = value.pretty_name();
        if let Ok(arg) = interp.try_convert(value) {
            Ok(Args { arg, radix })
        } else {
            Err(Error::NoImplicitConversionToString(value_type.to_owned()))
        }
    }
}

pub fn method<'a>(interp: &'a Artichoke, args: &Args<'a>) -> Result<Value, Error> {
    let arg = args.arg;
    let radix = args.radix;

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
                    err = Some(Error::InvalidValue(arg.to_string()));
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
            err = Some(Error::ContainsNullByte);
            break;
        }

        if c == '+' || c == '-' {
            // handle >1 consecutive sign
            let next_c = arg.chars().nth(i + 1);
            if next_c.is_none() || next_c == Some('+') || next_c == Some('-') {
                err = Some(Error::InvalidValue(arg.to_string()));
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
                return Err(Error::InvalidValue(digits));
            }
            if let Ok(v) = Int::from_str_radix(&digits, radix.try_into().unwrap()) {
                let result = interp.convert(v);
                Ok(result)
            } else {
                Err(Error::InvalidValue(digits))
            }
        }
        (Some(radix), None) | (None, Some(radix)) => {
            if radix < 2 || radix > 36 {
                return Err(Error::InvalidRadix(radix));
            }
            if let Ok(v) = Int::from_str_radix(&digits, radix.try_into().unwrap()) {
                let result = interp.convert(v);
                Ok(result)
            } else {
                Err(Error::InvalidValue(digits))
            }
        }
        (None, None) => {
            if let Ok(v) = i64::from_str(&digits) {
                let result = interp.convert(v);
                Ok(result)
            } else {
                Err(Error::InvalidValue(digits))
            }
        }
    }
}
