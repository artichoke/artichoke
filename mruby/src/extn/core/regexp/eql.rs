use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::regexp::{Encoding, Regexp};
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
        let other = mem::uninitialized::<sys::mrb_value>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &other,
        );
        if let Ok(other) = Regexp::try_from_ruby(interp, &Value::new(interp, other)) {
            Self { other: Some(other) }
        } else {
            Self { other: None }
        }
    }
}

pub fn method(interp: &Mrb, args: Args, value: &Value) -> Result<Value, Error> {
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let slf = data.borrow();
    if let Some(other) = args.other {
        let borrow = other.borrow();
        if slf.pattern != borrow.pattern {
            Ok(Value::from_mrb(interp, false))
        } else if slf.encoding == Encoding::No && borrow.encoding == Encoding::None {
            Ok(Value::from_mrb(interp, true))
        } else if slf.encoding == Encoding::None && borrow.encoding == Encoding::No {
            Ok(Value::from_mrb(interp, true))
        } else if slf.encoding != borrow.encoding {
            Ok(Value::from_mrb(interp, false))
        } else {
            Ok(Value::from_mrb(interp, true))
        }
    } else {
        Ok(Value::from_mrb(interp, false))
    }
}
