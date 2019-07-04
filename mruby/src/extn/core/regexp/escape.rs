use std::mem;

use crate::convert::{FromMrb, TryFromMrb};
use crate::extn::core::regexp::syntax;
use crate::sys;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    BadType,
    Fatal,
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
            Err(Error::BadType)
        }
    }
}

pub fn method(interp: &Mrb, args: Args) -> Result<Value, Error> {
    Ok(Value::from_mrb(
        interp,
        syntax::escape(args.pattern.as_str()),
    ))
}
