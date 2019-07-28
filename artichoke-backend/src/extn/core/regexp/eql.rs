//! [`Regexp#eql?`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-eql-3F)
//! and
//! [`Regexp#==`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-3D)

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::regexp::Regexp;
use crate::sys;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

#[derive(Debug)]
pub struct Args {
    pub other: Option<Rc<RefCell<Regexp>>>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o\0";

    pub unsafe fn extract(interp: &Mrb) -> Self {
        let mut other = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            other.as_mut_ptr(),
        );
        let other = other.assume_init();
        if let Ok(other) = Regexp::try_from_ruby(interp, &Value::new(interp, other)) {
            Self { other: Some(other) }
        } else {
            Self { other: None }
        }
    }
}

#[allow(clippy::if_same_then_else)]
pub fn method(interp: &Mrb, args: Args, value: &Value) -> Result<Value, Error> {
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let slf = data.borrow();
    if let Some(other) = args.other {
        let borrow = other.borrow();
        if slf.pattern == borrow.pattern {
            Ok(Value::from_mrb(interp, slf.encoding == borrow.encoding))
        } else {
            Ok(Value::from_mrb(interp, false))
        }
    } else {
        Ok(Value::from_mrb(interp, false))
    }
}
