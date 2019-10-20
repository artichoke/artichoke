use std::cmp;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::enc::Encoding;
use crate::extn::core::regexp::opts::Options;
use crate::extn::core::regexp::{syntax, Backend, Regexp};
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::Artichoke;

#[derive(Debug, Clone)]
pub struct Args {
    pub regexp: Option<Regexp>,
    pub block: Option<Value>,
}

impl Args {
    pub fn extract(
        interp: &Artichoke,
        pattern: Value,
        block: Option<Value>,
    ) -> Result<Self, Box<dyn RubyException>> {
        let regexp = if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
            Some(regexp.borrow().clone())
        } else if let Some(pattern) = pattern
            .funcall::<Option<&str>>("to_str", &[], None)
            .map_err(|_| TypeError::new(interp, "wrong argument type (expected Regexp)"))?
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
        Ok(Self { regexp, block })
    }
}

pub fn method(
    interp: &Artichoke,
    args: Args,
    value: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let mrb = interp.0.borrow().mrb;
    let regexp = args
        .regexp
        .ok_or_else(|| TypeError::new(interp, "wrong argument type (expected Regexp)"))?;
    let s = value
        .clone()
        .try_into::<&str>()
        .map_err(|_| Fatal::new(interp, "failed to convert String receiver to Rust String"))?;

    let last_match_sym = interp.0.borrow_mut().sym_intern("$~");
    let mut matchdata = MatchData::new(s, regexp.clone(), 0, s.len());
    let data = unsafe { matchdata.clone().try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
    unsafe {
        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
    }

    let mut was_match = false;
    let mut collected = vec![];
    let regex = (*regexp.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "Failed to extract Regexp"))?;
    match regex {
        Backend::Onig(regex) => {
            let len = regex.captures_len();

            if len > 0 {
                for captures in regex.captures_iter(s) {
                    was_match = true;
                    let mut groups = vec![];
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

                        let capture = captures.at(group);
                        if group > 0 {
                            groups.push(captures.at(group));
                        }
                        unsafe {
                            sys::mrb_gv_set(mrb, sym, interp.convert(capture).inner());
                        }
                    }
                    interp.0.borrow_mut().num_set_regexp_capture_globals = captures.len();

                    let matched = interp.convert(groups);
                    if let Some(pos) = captures.pos(0) {
                        matchdata.set_region(pos.0, pos.1);
                    }
                    if let Some(ref block) = args.block {
                        unsafe {
                            sys::mrb_yield(mrb, block.inner(), matched.inner());
                            sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                        }
                    } else {
                        collected.push(matched);
                    }
                }
            } else {
                for pos in regex.find_iter(s) {
                    was_match = true;
                    let scanned = &s[pos.0..pos.1];
                    let matched = interp.convert(scanned);
                    matchdata.set_region(pos.0, pos.1);
                    if let Some(ref block) = args.block {
                        unsafe {
                            sys::mrb_yield(mrb, block.inner(), matched.inner());
                            sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                        }
                    } else {
                        collected.push(matched);
                    }
                }
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    };
    if !was_match {
        unsafe {
            sys::mrb_gv_set(mrb, last_match_sym, sys::mrb_sys_nil_value());
        }
    }
    let result = if args.block.is_some() {
        value
    } else {
        interp.convert(collected)
    };
    Ok(result)
}
