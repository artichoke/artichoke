use std::cmp;
use std::convert::TryFrom;
use std::mem;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Regexp;
use crate::sys;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    PosType,
    StringType,
}

#[derive(Debug)]
pub struct Args {
    pub string: Option<String>,
    pub pos: Option<i64>,
    pub block: Option<Value>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o&|o?\0";

    pub unsafe fn extract(interp: &Mrb) -> Result<Self, Error> {
        let string = mem::uninitialized::<sys::mrb_value>();
        let pos = mem::uninitialized::<sys::mrb_value>();
        let has_pos = mem::uninitialized::<sys::mrb_bool>();
        let block = mem::uninitialized::<sys::mrb_value>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &string,
            &block,
            &pos,
            &has_pos,
        );
        let string = if let Ok(string) =
            <Option<String>>::try_from_mrb(&interp, Value::new(interp, string))
        {
            string
        } else {
            return Err(Error::StringType);
        };
        let pos = if has_pos == 0 {
            None
        } else {
            let pos =
                i64::try_from_mrb(&interp, Value::new(&interp, pos)).map_err(|_| Error::PosType)?;
            Some(pos)
        };
        let block = if sys::mrb_sys_value_is_nil(block) {
            None
        } else {
            Some(Value::new(interp, block))
        };
        Ok(Self { string, pos, block })
    }
}
pub fn method(interp: &Mrb, args: Args, value: &Value) -> Result<Value, Error> {
    let mrb = interp.borrow().mrb;
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let string = match args.string {
        Some(string) => string,
        None => unsafe {
            let matchdata = Value::from_mrb(interp, None::<Value>);
            sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), matchdata.inner());
            return Ok(matchdata);
        },
    };
    let pos = args.pos.unwrap_or_default();
    let pos = if pos < 0 {
        let strlen = i64::try_from(string.len()).unwrap_or_default();
        let pos = strlen + pos;
        if pos < 0 {
            return Ok(Value::from_mrb(interp, None::<Value>));
        }
        usize::try_from(pos).map_err(|_| Error::Fatal)?
    } else {
        usize::try_from(pos).map_err(|_| Error::Fatal)?
    };
    // onig will panic if pos is beyond the end of string
    if pos > string.len() {
        return Ok(Value::from_mrb(interp, None::<Value>));
    }

    let borrow = data.borrow();
    if let Some(captures) = borrow.regex.captures(&string[pos..]) {
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

        let matchdata = MatchData::new(
            &string[pos..],
            data.borrow().clone(),
            0,
            string[pos..].len(),
        );
        let data = unsafe { matchdata.try_into_ruby(interp, None) }.map_err(|_| Error::Fatal)?;
        unsafe {
            sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), data.inner());
        }
        if let Some(block) = args.block {
            Ok(Value::new(interp, unsafe {
                sys::mrb_yield(mrb, block.inner(), data.inner())
            }))
        } else {
            Ok(data)
        }
    } else {
        unsafe {
            sys::mrb_gv_set(
                mrb,
                interp.borrow_mut().sym_intern("$~"),
                Value::from_mrb(interp, None::<Value>).inner(),
            );
        }
        Ok(Value::from_mrb(interp, None::<Value>))
    }
}
