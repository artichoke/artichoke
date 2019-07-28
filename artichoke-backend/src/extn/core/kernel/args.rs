use std::io::Write;
use std::mem;

use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

#[derive(Debug)]
pub struct Rest {
    pub rest: Vec<Value>,
}

impl Rest {
    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, ArtichokeError> {
        let mut args = <mem::MaybeUninit<*const sys::mrb_value>>::uninit();
        let mut count = <mem::MaybeUninit<usize>>::uninit();
        // TODO: use a constant argspec, see GH-174.
        let mut argspec = vec![];
        argspec
            .write_all(sys::specifiers::REST.as_bytes())
            .map_err(|_| ArtichokeError::ArgSpec)?;
        argspec
            .write_all(b"\0")
            .map_err(|_| ArtichokeError::ArgSpec)?;
        sys::mrb_get_args(
            interp.borrow().mrb,
            argspec.as_ptr() as *const i8,
            args.as_mut_ptr(),
            count.as_mut_ptr(),
        );
        let args = std::slice::from_raw_parts(args.assume_init(), count.assume_init());
        let args = args
            .iter()
            .map(|value| Value::new(&interp, *value))
            .collect::<Vec<_>>();
        Ok(Self { rest: args })
    }
}
