use regex::Regex;
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
                value.ruby_type().class_name().into(),
                raise_exception,
            ))
        }
    }
}

pub fn method(interp: &Artichoke, args: &Args) -> Result<Value, Error> {
    let arg = args.arg;
    let radix = args.radix;
    let raise_exception = args.raise_exception;

    let mut string = String::new();

    // If have mutliple consecutive embedded underscores, argument error!
    let multi_underscore_re = Regex::new(r"__+").unwrap();
    if arg.starts_with('_') || arg.ends_with('_') || multi_underscore_re.is_match(arg) {
        return Err(Error::InvalidValue(arg.into(), raise_exception));
    }

    // If have multiple consecutive leading/trailing signs, argument error!
    let multi_sign_re = Regex::new(r"\+\++|\-\-+").unwrap();
    if multi_sign_re.is_match(arg) {
        return Err(Error::InvalidValue(arg.into(), raise_exception));
    }

    // Remove embedded underscore `_`
    // because they represent error in `from_str_radix` & `from_str`
    let arg = arg.replace('_', "");

    // Remove leading and trailing white space,
    // because they represent error in `from_str_radix` & `from_str`.
    let arg = arg.trim();
    let arg = if arg.starts_with('-') {
        string.push('-');
        &arg[1..]
    } else if arg.starts_with('+') {
        string.push('+');
        &arg[1..]
    } else {
        arg
    };

    // if `arg` is null byte, raise `ArgumentError`
    if arg.contains('\0') {
        return Err(Error::ContainsNullByte(raise_exception));
    }

    let mut parsed_radix = None;
    if arg.starts_with('0') && arg.len() > 2 {
        match &arg[0..2] {
            "0b" | "0B" => {
                string.push_str(&arg[2..]);
                parsed_radix = Some(2);
            }
            "0o" | "0O" => {
                string.push_str(&arg[2..]);
                parsed_radix = Some(8);
            }
            "0d" | "0D" => {
                string.push_str(&arg[2..]);
                parsed_radix = Some(10);
            }
            "0x" | "0X" => {
                string.push_str(&arg[2..]);
                parsed_radix = Some(16);
            }
            prefix if &prefix[0..1] == "0" => {
                string.push_str(&arg[1..]);
                parsed_radix = Some(8);
            }
            _ => {}
        };
    } else {
        string.push_str(arg);
    }

    match (radix, parsed_radix) {
        (Some(radix), Some(parsed_radix)) => {
            if radix != parsed_radix {
                return Err(Error::InvalidValue(string, raise_exception));
            }
            if let Ok(v) = i64::from_str_radix(string.as_str(), radix as u32) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(string, raise_exception))
            }
        }
        (Some(radix), None) | (None, Some(radix)) => {
            if radix < 2 || radix > 36 {
                return Err(Error::InvalidRadix(radix.to_string(), raise_exception));
            }
            if let Ok(v) = i64::from_str_radix(string.as_str(), radix as u32) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(string, raise_exception))
            }
        }
        (None, None) => {
            if let Ok(v) = i64::from_str(string.as_str()) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(string, raise_exception))
            }
        }
    }
}
