use std::io::Write;
use std::mem;

use crate::convert::{RustBackedValue, TryFromMrb};
use crate::extn::core::regexp::Regexp;
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::Value;
use crate::MrbError;

pub struct Scan {
    pub regexp: Regexp,
}

impl Scan {
    pub unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
        let pattern = mem::uninitialized::<sys::mrb_value>();
        let mut argspec = vec![];
        argspec
            .write_all(sys::specifiers::OBJECT.as_bytes())
            .map_err(|_| MrbError::ArgSpec)?;
        argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
        sys::mrb_get_args(interp.borrow().mrb, argspec.as_ptr() as *const i8, &pattern);
        let pattern = Value::new(interp, pattern);
        let regexp = Regexp::try_from_ruby(&interp, &pattern)
            .map(|data| data.borrow().clone())
            .or_else(|_| {
                String::try_from_mrb(&interp, pattern)
                    .map_err(MrbError::ConvertToRust)
                    .and_then(Regexp::new)
            })?;
        Ok(Self { regexp })
    }
}
