//! [`Regexp::escape`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-escape)
//! and
//! [`Regexp::quote`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-quote)

use std::mem;

use crate::convert::{Convert, TryConvert};
use crate::extn::core::regexp::syntax;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

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

    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, Error> {
        let mut string = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        sys::mrb_get_args(
            interp.0.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            string.as_mut_ptr(),
        );
        let string = string.assume_init();
        if let Ok(pattern) = interp.try_convert(Value::new(interp, string)) {
            Ok(Self { pattern })
        } else {
            Err(Error::NoImplicitConversionToString)
        }
    }
}

pub fn method(interp: &Artichoke, args: &Args) -> Result<Value, Error> {
    let result: Value = interp.convert(syntax::escape(args.pattern.as_str()));
    Ok(result)
}
