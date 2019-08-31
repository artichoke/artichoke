//! [`Regexp#match`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-match)

use std::cmp;
use std::convert::TryFrom;
use std::mem;

use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{Backend, Regexp};
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    PosType,
    StringType,
}

#[derive(Debug)]
pub struct Args {
    pub string: Option<String>,
    pub pos: Option<Int>,
    pub block: Option<Value>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o&|o?\0";

    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, Error> {
        let mut string = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut pos = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut has_pos = <mem::MaybeUninit<sys::mrb_bool>>::uninit();
        let mut block = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        sys::mrb_get_args(
            interp.0.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            string.as_mut_ptr(),
            block.as_mut_ptr(),
            pos.as_mut_ptr(),
            has_pos.as_mut_ptr(),
        );
        let string = string.assume_init();
        let block = block.assume_init();
        let has_pos = has_pos.assume_init() != 0;
        let string = if let Ok(string) =
            <Option<String>>::try_convert(&interp, Value::new(interp, string))
        {
            string
        } else {
            return Err(Error::StringType);
        };
        let pos = if has_pos {
            let pos = Int::try_convert(&interp, Value::new(&interp, pos.assume_init()))
                .map_err(|_| Error::PosType)?;
            Some(pos)
        } else {
            None
        };
        let block = if sys::mrb_sys_value_is_nil(block) {
            None
        } else {
            Some(Value::new(interp, block))
        };
        Ok(Self { string, pos, block })
    }
}

pub fn method(interp: &Artichoke, args: Args, value: &Value) -> Result<Value, Error> {
    let mrb = interp.0.borrow().mrb;
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let string = if let Some(string) = args.string {
        string
    } else {
        unsafe {
            let matchdata = Value::convert(interp, None::<Value>);
            sys::mrb_gv_set(
                mrb,
                interp.0.borrow_mut().sym_intern("$~"),
                matchdata.inner(),
            );
            return Ok(matchdata);
        }
    };
    let pos = args.pos.unwrap_or_default();
    let pos = if pos < 0 {
        let strlen = Int::try_from(string.chars().count()).unwrap_or_default();
        let pos = strlen + pos;
        if pos < 0 {
            return Ok(Value::convert(interp, None::<Value>));
        }
        usize::try_from(pos).map_err(|_| Error::Fatal)?
    } else {
        usize::try_from(pos).map_err(|_| Error::Fatal)?
    };
    // onig will panic if pos is beyond the end of string
    if pos > string.chars().count() {
        return Ok(Value::convert(interp, None::<Value>));
    }
    let byte_offset = string.chars().take(pos).collect::<String>().len();

    let match_target = &string[byte_offset..];
    let borrow = data.borrow();
    let regex = (*borrow.regex).as_ref().ok_or(Error::Fatal)?;
    match regex {
        Backend::Onig(regex) => {
            if let Some(captures) = regex.captures(match_target) {
                let num_regexp_globals_to_set = {
                    let num_previously_set_globals =
                        interp.0.borrow().num_set_regexp_capture_globals;
                    cmp::max(num_previously_set_globals, captures.len())
                };
                for group in 0..num_regexp_globals_to_set {
                    let sym = if group == 0 {
                        interp.0.borrow_mut().sym_intern("$&")
                    } else {
                        interp.0.borrow_mut().sym_intern(&format!("${}", group))
                    };

                    let value = Value::convert(&interp, captures.at(group));
                    unsafe {
                        sys::mrb_gv_set(mrb, sym, value.inner());
                    }
                }
                interp.0.borrow_mut().num_set_regexp_capture_globals = captures.len();

                let mut matchdata =
                    MatchData::new(string.as_str(), borrow.clone(), 0, string.len());
                if let Some(match_pos) = captures.pos(0) {
                    let pre_match = &match_target[..match_pos.0];
                    let post_match = &match_target[match_pos.1..];
                    unsafe {
                        let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                        sys::mrb_gv_set(
                            mrb,
                            pre_match_sym,
                            Value::convert(interp, pre_match).inner(),
                        );
                        let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                        sys::mrb_gv_set(
                            mrb,
                            post_match_sym,
                            Value::convert(interp, post_match).inner(),
                        );
                    }
                    matchdata.set_region(byte_offset + match_pos.0, byte_offset + match_pos.1);
                }
                let data =
                    unsafe { matchdata.try_into_ruby(interp, None) }.map_err(|_| Error::Fatal)?;
                unsafe {
                    sys::mrb_gv_set(mrb, interp.0.borrow_mut().sym_intern("$~"), data.inner());
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
                    let last_match_sym = interp.0.borrow_mut().sym_intern("$~");
                    sys::mrb_gv_set(
                        mrb,
                        last_match_sym,
                        Value::convert(interp, None::<Value>).inner(),
                    );
                    let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                    sys::mrb_gv_set(
                        mrb,
                        pre_match_sym,
                        Value::convert(interp, None::<Value>).inner(),
                    );
                    let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                    sys::mrb_gv_set(
                        mrb,
                        post_match_sym,
                        Value::convert(interp, None::<Value>).inner(),
                    );
                }
                Ok(Value::convert(interp, None::<Value>))
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
