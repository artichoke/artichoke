//! [`Regexp#===`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-3D-3D)

use artichoke_core::value::Value as ValueLike;
use std::cmp;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{Backend, Regexp};
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub struct Args<'a> {
    pub pattern: Option<&'a str>,
}

impl<'a> Args<'a> {
    pub fn extract(pattern: Value) -> Self {
        if let Ok(pattern) = pattern.clone().try_into::<&str>() {
            Self {
                pattern: Some(pattern),
            }
        } else if let Ok(pattern) = pattern.funcall::<&str>("to_str", &[], None) {
            Self {
                pattern: Some(pattern),
            }
        } else {
            Self { pattern: None }
        }
    }
}

pub fn method(
    interp: &Artichoke,
    args: Args,
    value: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    let mrb = interp.0.borrow().mrb;
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Failed to extract Rust Regexp from Ruby Regexp Value",
        )
    })?;
    let pattern = if let Some(pattern) = args.pattern {
        pattern
    } else {
        let sym = interp.0.borrow_mut().sym_intern("$~");
        unsafe {
            sys::mrb_gv_set(mrb, sym, interp.convert(None::<Value>).inner());
        }
        return Ok(interp.convert(false));
    };
    let borrow = data.borrow();
    let regex = (*borrow.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "uninitialized Regexp"))?;
    let matchdata = match regex {
        Backend::Onig(regex) => {
            if let Some(captures) = regex.captures(pattern) {
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

                    let value = interp.convert(captures.at(group));
                    unsafe {
                        sys::mrb_gv_set(mrb, sym, value.inner());
                    }
                }
                interp.0.borrow_mut().num_set_regexp_capture_globals = captures.len();

                if let Some(match_pos) = captures.pos(0) {
                    let pre_match = &pattern[..match_pos.0];
                    let post_match = &pattern[match_pos.1..];
                    let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                    let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                    unsafe {
                        sys::mrb_gv_set(mrb, pre_match_sym, interp.convert(pre_match).inner());
                        sys::mrb_gv_set(mrb, post_match_sym, interp.convert(post_match).inner());
                    }
                }
                let matchdata = MatchData::new(pattern, borrow.clone(), 0, pattern.len());
                unsafe { matchdata.try_into_ruby(&interp, None) }.map_err(|_| {
                    Fatal::new(interp, "Could not create Ruby Value from Rust MatchData")
                })?
            } else {
                let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                unsafe {
                    sys::mrb_gv_set(mrb, pre_match_sym, interp.convert(None::<Value>).inner());
                    sys::mrb_gv_set(mrb, post_match_sym, interp.convert(None::<Value>).inner());
                }
                interp.convert(None::<Value>)
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    };
    let sym = interp.0.borrow_mut().sym_intern("$~");
    unsafe {
        sys::mrb_gv_set(mrb, sym, matchdata.inner());
    }
    Ok(interp.convert(!matchdata.is_nil()))
}
