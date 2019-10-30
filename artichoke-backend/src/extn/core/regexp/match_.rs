//! [`Regexp#match`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-match)

use artichoke_core::value::Value as ValueLike;
use std::cmp;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{Backend, Regexp};
use crate::sys;
use crate::types::Int;
use crate::value::{Block, Value};
use crate::Artichoke;

#[derive(Clone)]
pub struct Args<'a> {
    pub pattern: Option<&'a str>,
    pub pos: Option<Int>,
    pub block: Option<Block>,
}

impl<'a> Args<'a> {
    pub fn extract(
        interp: &Artichoke,
        pattern: Value,
        pos: Option<Value>,
        block: Option<Block>,
    ) -> Result<Self, Box<dyn RubyException>> {
        let pattern = if let Ok(pattern) = pattern.clone().try_into::<Option<&str>>() {
            pattern
        } else if let Ok(pattern) = pattern.funcall::<Option<&str>>("to_str", &[], None) {
            pattern
        } else {
            return Err(Box::new(TypeError::new(
                interp,
                "No implicit conversion into String",
            )));
        };
        let pos = if let Some(pos) = pos {
            if let Ok(pos) = pos.clone().try_into::<Int>() {
                Some(pos)
            } else if let Ok(pos) = pos.funcall::<Int>("to_int", &[], None) {
                Some(pos)
            } else {
                return Err(Box::new(TypeError::new(
                    interp,
                    "no implicit conversion into Integer",
                )));
            }
        } else {
            None
        };
        Ok(Self {
            pattern,
            pos,
            block,
        })
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
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let pattern = if let Some(pattern) = args.pattern {
        pattern
    } else {
        let sym = interp.0.borrow_mut().sym_intern("$~");
        let matchdata = interp.convert(None::<Value>);
        unsafe {
            sys::mrb_gv_set(mrb, sym, matchdata.inner());
        }
        return Ok(matchdata);
    };
    let pattern_char_len = pattern.chars().count();
    let pos = args.pos.unwrap_or_default();
    let pos = if pos < 0 {
        let strlen = Int::try_from(pattern_char_len)
            .map_err(|_| Fatal::new(interp, "Pattern length greater than Integer max"))?;
        let pos = strlen + pos; // this can never wrap since char len is at most `usize`.
        if pos < 0 {
            return Ok(interp.convert(None::<Value>));
        }
        usize::try_from(pos)
            .map_err(|_| Fatal::new(interp, "Expected positive position to convert to usize"))?
    } else {
        usize::try_from(pos)
            .map_err(|_| Fatal::new(interp, "Expected positive position to convert to usize"))?
    };
    // onig will panic if pos is beyond the end of string
    if pos > pattern_char_len {
        return Ok(interp.convert(None::<Value>));
    }
    let byte_offset = pattern.chars().take(pos).collect::<String>().len();

    let match_target = &pattern[byte_offset..];
    let borrow = data.borrow();
    let regex = (*borrow.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "Uninitialized Regexp"))?;
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

                    let value = interp.convert(captures.at(group));
                    unsafe {
                        sys::mrb_gv_set(mrb, sym, value.inner());
                    }
                }
                interp.0.borrow_mut().num_set_regexp_capture_globals = captures.len();

                let mut matchdata = MatchData::new(pattern, borrow.clone(), 0, pattern.len());
                if let Some(match_pos) = captures.pos(0) {
                    let pre_match = &match_target[..match_pos.0];
                    let post_match = &match_target[match_pos.1..];
                    let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                    let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                    unsafe {
                        sys::mrb_gv_set(mrb, pre_match_sym, interp.convert(pre_match).inner());
                        sys::mrb_gv_set(mrb, post_match_sym, interp.convert(post_match).inner());
                    }
                    matchdata.set_region(byte_offset + match_pos.0, byte_offset + match_pos.1);
                }
                let data = unsafe { matchdata.try_into_ruby(interp, None) }.map_err(|_| {
                    Fatal::new(
                        interp,
                        "Failed to initialize Ruby MatchData Value with Rust MatchData",
                    )
                })?;
                let sym = interp.0.borrow_mut().sym_intern("$~");
                unsafe {
                    sys::mrb_gv_set(mrb, sym, data.inner());
                }
                if let Some(block) = args.block {
                    let result = block.yield_arg(interp, &data).map_err(|_| {
                        Fatal::new(
                            interp,
                            "Failed to initialize Ruby MatchData Value with Rust MatchData",
                        )
                    })?;
                    Ok(result)
                } else {
                    Ok(data)
                }
            } else {
                let last_match_sym = interp.0.borrow_mut().sym_intern("$~");
                let pre_match_sym = interp.0.borrow_mut().sym_intern("$`");
                let post_match_sym = interp.0.borrow_mut().sym_intern("$'");
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, interp.convert(None::<Value>).inner());
                    sys::mrb_gv_set(mrb, pre_match_sym, interp.convert(None::<Value>).inner());
                    sys::mrb_gv_set(mrb, post_match_sym, interp.convert(None::<Value>).inner());
                }
                Ok(interp.convert(None::<Value>))
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
