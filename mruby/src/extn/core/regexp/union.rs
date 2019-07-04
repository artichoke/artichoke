use std::mem;

use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::regexp::{syntax, Regexp};
use crate::sys;
use crate::value::types::Ruby;
use crate::value::{Value, ValueLike};
use crate::Mrb;

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

    pub unsafe fn extract(interp: &Mrb) -> Self {
        let args = mem::uninitialized::<*const sys::mrb_value>();
        let count = mem::uninitialized::<usize>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &args,
            &count,
        );
        let args = std::slice::from_raw_parts(args, count);
        let args = args
            .iter()
            .map(|value| Value::new(&interp, *value))
            .collect::<Vec<_>>();
        Self { rest: args }
    }
}

pub fn method(interp: &Mrb, args: Args, slf: sys::mrb_value) -> Result<Value, Error> {
    let mrb = interp.borrow().mrb;
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
                } else if let Ok(pattern) = pattern.funcall::<String, _, _>("to_str", &[]) {
                    patterns.push(syntax::escape(pattern.as_str()));
                } else {
                    return Err(Error::NoImplicitConversionToString);
                }
            }
            patterns.join("|")
        } else {
            arg.try_into::<String>()
                .map_err(|_| Error::NoImplicitConversionToString)?
        }
    } else {
        let mut patterns = vec![];
        for pattern in args.rest {
            if let Ok(regexp) = unsafe { Regexp::try_from_ruby(&interp, &pattern) } {
                patterns.push(regexp.borrow().pattern.clone());
            } else if let Ok(pattern) = pattern.funcall::<String, _, _>("to_str", &[]) {
                patterns.push(syntax::escape(pattern.as_str()));
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
            [Value::from_mrb(interp, pattern).inner()].as_ptr() as *const sys::mrb_value,
        )
    };
    Ok(Value::new(interp, value))
}
