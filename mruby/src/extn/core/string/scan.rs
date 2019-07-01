use std::cmp;
use std::mem;

use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{syntax, Encoding, Options, Regexp};
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::{Value, ValueLike};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    WrongType,
}

#[derive(Debug)]
pub struct Args {
    pub regexp: Option<Regexp>,
    pub block: Option<Value>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o&\0";

    pub unsafe fn extract(interp: &Mrb) -> Result<Self, Error> {
        let pattern = mem::uninitialized::<sys::mrb_value>();
        let block = mem::uninitialized::<sys::mrb_value>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &pattern,
            &block,
        );
        let regexp = if let Ok(regexp) = Regexp::try_from_ruby(interp, &Value::new(interp, pattern))
        {
            Some(regexp.borrow().clone())
        } else if let Some(ref pattern) = Value::new(interp, pattern)
            .funcall::<Option<String>, _, _>("to_str", &[])
            .map_err(|_| Error::WrongType)?
        {
            Regexp::new(
                syntax::escape(pattern),
                syntax::escape(pattern),
                Options::default(),
                Options::default(),
                Encoding::default(),
            )
        } else {
            None
        };
        let block = if sys::mrb_sys_value_is_nil(block) {
            None
        } else {
            Some(Value::new(interp, block))
        };
        Ok(Self { regexp, block })
    }
}

pub fn method(interp: &Mrb, args: Args, value: Value) -> Result<Value, Error> {
    let mrb = interp.borrow().mrb;
    let regexp = args.regexp.ok_or(Error::WrongType)?;
    let s = value.to_s();
    let mut collected = vec![];
    for pos in regexp.regex.find_iter(s.as_str()) {
        let scanned = &s[pos.0..pos.1];
        if let Some(captures) = regexp.regex.captures(scanned) {
            let num_regexp_globals_to_set = {
                let num_previously_set_globals = interp.borrow().num_set_regexp_capture_globals;
                cmp::max(num_previously_set_globals, captures.len())
            };
            for group in 1..=num_regexp_globals_to_set {
                let value = Value::from_mrb(&interp, captures.at(group));
                unsafe {
                    sys::mrb_gv_set(
                        mrb,
                        interp.borrow_mut().sym_intern(&format!("${}", group)),
                        value.inner(),
                    );
                }
            }
            interp.borrow_mut().num_set_regexp_capture_globals = captures.len();

            let matched = if captures.len() > 1 {
                let mut collected_groups = vec![];
                for index in 1..captures.len() {
                    collected_groups.push(Value::from_mrb(&interp, captures.at(index)));
                }
                Value::from_mrb(&interp, collected_groups)
            } else {
                Value::from_mrb(&interp, captures.at(0))
            };
            let data = MatchData::new(s.as_str(), regexp.clone(), pos.0, pos.1);
            let data = unsafe { data.try_into_ruby(&interp, None) }.map_err(|_| Error::Fatal)?;
            unsafe {
                sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), data.inner());
            }
            if let Some(ref block) = args.block {
                unsafe {
                    sys::mrb_yield(mrb, block.inner(), matched.inner());
                    sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), data.inner());
                }
            }
            collected.push(matched);
        }
    }
    if collected.is_empty() {
        unsafe {
            sys::mrb_gv_set(
                mrb,
                interp.borrow_mut().sym_intern("$~"),
                interp.nil().inner(),
            );
        }
    }
    if args.block.is_some() {
        Ok(value)
    } else {
        Ok(Value::from_mrb(&interp, collected))
    }
}
