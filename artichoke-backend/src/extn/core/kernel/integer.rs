use regex::Regex;
use std::str::FromStr;

use crate::convert::{Convert, TryConvert};
use crate::Artichoke;
use crate::value::Value;

use mruby_sys::mrb_vtype::MRB_TT_FIXNUM;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    BaseSpecifiedForNonString,
    ContainsNullByte,
    InvalidValue(String),
    InvalidRadix(String),
    NoImplicitConversionToString,
}

#[derive(Debug)]
pub struct Args {
    pub string: String,
    pub radix: Option<i64>,
    pub parsed_radix: Option<i64>,
    pub raise_exception: bool,
}

impl Args {
    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, Error> {
        let mrb = interp.0.borrow().mrb;
        let (arg, base, exception) = mrb_get_args!(mrb, required = 1, optional = 2);
        let mut string = String::new();

        let radix: Option<i64> = match base {
            Some(base) => {
                if let Ok(base) = interp.try_convert(Value::new(interp, base)) {
                    base
                } else {
                    None
                }
            },
            _ => None
        };

        if arg.tt == MRB_TT_FIXNUM && radix.is_some() {
            return Err(Error::BaseSpecifiedForNonString)
        }

        let arg = if let Ok(arg) = interp.try_convert(Value::new(interp, arg)) {
            arg
        } else {
            ""
            // Err(Error::NoImplicitConversionToString)
        };

        // If have consecutive embedded underscore, argument error!
        let multi_underscore_re = Regex::new(r"__+").unwrap();
        if arg.starts_with('_') || arg.ends_with('_') || multi_underscore_re.is_match(arg) {
            return Err(Error::InvalidValue(arg.into()))
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

        let raise_exception = match exception {
            Some(exception) => {
                if let Ok(exception) = interp.try_convert(Value::new(interp, exception)) {
                    exception
                } else {
                    true
                }
            },
            _ => true
        };

        // if `arg` is null byte, raise `ArgumentError`
        if arg.contains('\0') {
            return Err(Error::ContainsNullByte)
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

        Ok(Self { string, radix, parsed_radix, raise_exception })
    }
}

pub fn method(interp: &Artichoke, args: Args) -> Result<Value, Error> {
    let string = args.string;
    let radix = args.radix;
    let parsed_radix = args.parsed_radix;
    // let raise_exception = args.raise_exception;

    match (radix, parsed_radix) {
        (Some(radix), Some(parsed_radix)) => {
            if radix != parsed_radix {
                return Err(Error::InvalidValue(string));
            }
            if let Ok(v) = i64::from_str_radix(string.as_str(), radix as u32) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(string))
            }
        }
        (Some(radix), None) | (None, Some(radix)) => {
            if radix < 2 || radix > 36 {
                return Err(Error::InvalidRadix(radix.to_string()))
            }
            if let Ok(v) = i64::from_str_radix(string.as_str(), radix as u32) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(string))
            }
        }
        (None, None) => {
            if let Ok(v) = i64::from_str(string.as_str()) {
                Ok(interp.convert(v))
            } else {
                Err(Error::InvalidValue(string))
            }
        }
    }
}
