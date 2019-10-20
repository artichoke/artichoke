//! [`Regexp#=~`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-7E)

use artichoke_core::value::Value as ValueLike;
use std::cmp;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{Backend, Regexp};
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub struct Args<'a> {
    pub pattern: Option<&'a str>,
}

impl<'a> Args<'a> {
    pub fn extract(interp: &Artichoke, pattern: Value) -> Result<Self, Box<dyn RubyException>> {
        if let Ok(pattern) = pattern.clone().try_into::<Option<&str>>() {
            Ok(Self { pattern })
        } else if let Ok(pattern) = pattern.funcall::<Option<&str>>("to_str", &[], None) {
            Ok(Self { pattern })
        } else {
            Err(Box::new(TypeError::new(
                interp,
                "no implicit conversion into String",
            )))
        }
    }
}

// TODO: extract named captures and assign to local variables, see GH-156.
//
// See: https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-7E
pub fn method(
    interp: &Artichoke,
    args: Args,
    value: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    let mrb = interp.0.borrow().mrb;
    let value = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let pattern = if let Some(pattern) = args.pattern {
        pattern
    } else {
        return Ok(interp.convert(None::<Value>));
    };
    let borrow = value.borrow();
    let regex = (*borrow.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "Uninitialized Regexp"))?;
    let (matchdata, pos) = match regex {
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

                let matchdata = MatchData::new(pattern, borrow.clone(), 0, pattern.len());
                let matchdata = unsafe { matchdata.try_into_ruby(interp, None) }.map_err(|_| {
                    Fatal::new(
                        interp,
                        "Failed to initialize Ruby MatchData Value with Rust MatchData",
                    )
                })?;
                if let Some(match_pos) = captures.pos(0) {
                    let pre_match = &pattern[..match_pos.0];
                    let post_match = &pattern[match_pos.1..];
                    let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                    let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                    unsafe {
                        sys::mrb_gv_set(mrb, pre_match_sym, interp.convert(pre_match).inner());
                        sys::mrb_gv_set(mrb, post_match_sym, interp.convert(post_match).inner());
                    }
                    (matchdata, interp.convert(Int::try_from(match_pos.0).ok()))
                } else {
                    (matchdata, interp.convert(None::<Value>))
                }
            } else {
                let last_match_sym = interp.0.borrow_mut().sym_intern("$~");
                let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                let nil = interp.convert(None::<Value>).inner();
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, nil);
                    sys::mrb_gv_set(mrb, pre_match_sym, nil);
                    sys::mrb_gv_set(mrb, post_match_sym, nil);
                }
                (Value::new(interp, nil), Value::new(interp, nil))
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    };
    let sym = interp.0.borrow_mut().sym_intern("$~");
    unsafe {
        sys::mrb_gv_set(mrb, sym, matchdata.inner());
    }
    Ok(pos)
}
