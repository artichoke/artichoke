//! [`Regexp::union`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-union)

use std::mem;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::regexp::{syntax, Regexp};
use crate::sys;
use crate::types::Ruby;
use crate::value::{Value, ValueLike};
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    NoImplicitConversionToString,
}

#[derive(Debug)]
pub struct Args {
    pub rest: Vec<Value>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"*\0";

    pub unsafe fn extract(interp: &Artichoke) -> Self {
        let mut args = <mem::MaybeUninit<*const sys::mrb_value>>::uninit();
        let mut count = <mem::MaybeUninit<usize>>::uninit();
        sys::mrb_get_args(
            interp.0.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            args.as_mut_ptr(),
            count.as_mut_ptr(),
        );
        let args = std::slice::from_raw_parts(args.assume_init(), count.assume_init());
        let args = args
            .iter()
            .map(|value| Value::new(&interp, *value))
            .collect::<Vec<_>>();
        Self { rest: args }
    }
}

pub fn method(interp: &Artichoke, args: Args, slf: sys::mrb_value) -> Result<Value, Error> {
    let mrb = interp.0.borrow().mrb;
    let pattern = if args.rest.is_empty() {
        "(?!)".to_owned()
    } else if args.rest.len() == 1 {
        let arg = args.rest.into_iter().nth(0).unwrap();
        if arg.ruby_type() == Ruby::Array {
            let mut patterns = vec![];
            for pattern in arg
                .itself::<Vec<Value>>()
                .map_err(|_| Error::NoImplicitConversionToString)?
            {
                if let Ok(regexp) = unsafe { Regexp::try_from_ruby(&interp, &pattern) } {
                    patterns.push(regexp.borrow().pattern.clone());
                } else if let Ok(pattern) = pattern.funcall::<&str>("to_str", &[], None) {
                    patterns.push(syntax::escape(pattern));
                } else {
                    return Err(Error::NoImplicitConversionToString);
                }
            }
            patterns.join("|")
        } else {
            let pattern = arg;
            if let Ok(regexp) = unsafe { Regexp::try_from_ruby(&interp, &pattern) } {
                regexp.borrow().pattern.clone()
            } else if let Ok(pattern) = pattern.funcall::<&str>("to_str", &[], None) {
                syntax::escape(pattern)
            } else {
                return Err(Error::NoImplicitConversionToString);
            }
        }
    } else {
        let mut patterns = vec![];
        for pattern in args.rest {
            if let Ok(regexp) = unsafe { Regexp::try_from_ruby(&interp, &pattern) } {
                patterns.push(regexp.borrow().pattern.clone());
            } else if let Ok(pattern) = pattern.funcall::<&str>("to_str", &[], None) {
                patterns.push(syntax::escape(pattern));
            } else {
                return Err(Error::NoImplicitConversionToString);
            }
        }
        patterns.join("|")
    };

    let value = unsafe {
        sys::mrb_obj_new(
            mrb,
            sys::mrb_sys_class_ptr(slf),
            1,
            [interp.convert(pattern).inner()].as_ptr() as *const sys::mrb_value,
        )
    };
    Ok(Value::new(interp, value))
}
