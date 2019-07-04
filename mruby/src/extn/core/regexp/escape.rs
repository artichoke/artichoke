//! [`Regexp::escape`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-escape)
//! and
//! [`Regexp::quote`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-quote)

use std::mem;

use crate::convert::{FromMrb, TryFromMrb};
use crate::extn::core::regexp::syntax;
use crate::sys;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    NoImplicitConversionToString,
}

#[derive(Debug, Clone)]
pub struct Args {
    pub pattern: String,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o\0";

    pub unsafe fn extract(interp: &Mrb) -> Result<Self, Error> {
        let string = mem::uninitialized::<sys::mrb_value>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &string,
        );
        if let Ok(pattern) = String::try_from_mrb(interp, Value::new(interp, string)) {
            Ok(Self { pattern })
        } else {
            Err(Error::NoImplicitConversionToString)
        }
    }
}

pub fn method(interp: &Mrb, args: &Args) -> Result<Value, Error> {
    Ok(Value::from_mrb(
        interp,
        syntax::escape(args.pattern.as_str()),
    ))
}
