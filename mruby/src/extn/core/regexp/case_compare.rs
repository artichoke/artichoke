use std::cmp;
use std::mem;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Regexp;
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
    pub string: Option<String>,
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
        if let Ok(string) = <Option<String>>::try_from_mrb(interp, Value::new(interp, string)) {
            Ok(Self { string })
        } else {
            Err(Error::BadType)
        }
    }
}

pub fn method(interp: &Mrb, args: Args, value: &Value) -> Result<Value, Error> {
    let mrb = interp.borrow().mrb;
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let string = match args.string {
        Some(string) => string,
        None => unsafe {
            sys::mrb_gv_set(
                mrb,
                interp.borrow_mut().sym_intern("$~"),
                sys::mrb_sys_nil_value(),
            );
            return Ok(Value::from_mrb(interp, false));
        },
    };
    let matchdata = if let Some(captures) = data.borrow().regex.captures(string.as_str()) {
        let num_regexp_globals_to_set = {
            let num_previously_set_globals = interp.borrow().num_set_regexp_capture_globals;
            cmp::max(num_previously_set_globals, captures.len())
        };
        for group in 0..num_regexp_globals_to_set {
            let sym = if group == 0 {
                interp.borrow_mut().sym_intern("$&")
            } else {
                interp.borrow_mut().sym_intern(&format!("${}", group))
            };

            let value = Value::from_mrb(&interp, captures.at(group));
            unsafe {
                sys::mrb_gv_set(mrb, sym, value.inner());
            }
        }
        interp.borrow_mut().num_set_regexp_capture_globals = captures.len();

        let matchdata = MatchData::new(string.as_str(), data.borrow().clone(), 0, string.len());
        unsafe { matchdata.try_into_ruby(&interp, None) }.map_err(|_| Error::Fatal)?
    } else {
        Value::from_mrb(interp, None::<Value>)
    };
    unsafe {
        sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), matchdata.inner());
    }
    Ok(Value::from_mrb(&interp, !unsafe {
        sys::mrb_sys_value_is_nil(matchdata.inner())
    }))
}
