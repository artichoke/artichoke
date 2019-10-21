use std::cmp;
use std::str;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{Backend, Regexp};
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::Artichoke;

fn literal_scan(string: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        string.len() + 1
    } else if pattern.len() > string.len() {
        0
    } else if pattern == string {
        1
    } else {
        match pattern.len() {
            0 => unreachable!("handled above"),
            1 => {
                let byte0 = pattern[0];
                memchr::memchr_iter(byte0, string).count()
            }
            _ => {
                let mut count = 0;
                let byte0 = pattern[0];
                let rest = &pattern[1..];
                let strlen = string.len();
                let patlen = pattern.len();
                for pos in memchr::memchr_iter(byte0, string) {
                    if strlen - pos > patlen && &string[pos + 1..pos + patlen] == rest {
                        count += 1;
                    }
                }
                count
            }
        }
    }
}

pub fn method(
    interp: &Artichoke,
    value: Value,
    pattern: Value,
    block: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    let string = value.clone().try_into::<&[u8]>().map_err(|_| {
        Fatal::new(
            interp,
            "Unable to convert Ruby String Receiver to Rust byte slice",
        )
    })?;
    if let Ok(pattern_bytes) = pattern.clone().try_into::<&[u8]>() {
        let matches = literal_scan(string, pattern_bytes);
        if let Some(ref block) = block {
            let mrb = interp.0.borrow().mrb;
            for _ in 0..matches {
                unsafe {
                    sys::mrb_yield(mrb, block.inner(), interp.convert(pattern_bytes).inner());
                }
            }
            Ok(value)
        } else {
            let mut result = Vec::with_capacity(matches);
            for _ in 0..matches {
                result.push(interp.convert(pattern_bytes));
            }
            Ok(interp.convert(result))
        }
    } else {
        let pattern_type_name = pattern.pretty_name();
        let pattern_bytes = pattern.funcall::<&[u8]>("to_str", &[], None);
        if let Ok(pattern_bytes) = pattern_bytes {
            let matches = literal_scan(string, pattern_bytes);
            if let Some(ref block) = block {
                let mrb = interp.0.borrow().mrb;
                for _ in 0..matches {
                    unsafe {
                        sys::mrb_yield(mrb, block.inner(), interp.convert(pattern_bytes).inner());
                    }
                }
                Ok(value)
            } else {
                let mut result = Vec::with_capacity(matches);
                for _ in 0..matches {
                    result.push(interp.convert(pattern_bytes));
                }
                Ok(interp.convert(result))
            }
        } else if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
            // TODO: Regexp and MatchData should operate on byte slices.
            let s = str::from_utf8(string).map_err(|_| {
                Fatal::new(
                    interp,
                    "String#scan does not support Regexp scans over UTF-8 invalid data",
                )
            })?;
            let borrow = regexp.borrow();
            let mrb = interp.0.borrow().mrb;
            let last_match_sym = interp.0.borrow_mut().sym_intern("$~");
            let mut matchdata = MatchData::new(s, borrow.clone(), 0, string.len());
            let data = unsafe { matchdata.clone().try_into_ruby(interp, None) }
                .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
            unsafe {
                sys::mrb_gv_set(mrb, last_match_sym, data.inner());
            }

            let mut was_match = false;
            let mut collected = vec![];
            let regex = (*borrow.regex)
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
                            if let Some(ref block) = block {
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
                            if let Some(ref block) = block {
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
            let result = if block.is_some() {
                value
            } else {
                interp.convert(collected)
            };
            Ok(result)
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!(
                    "wrong argument type {} (expected Regexp)",
                    pattern_type_name
                ),
            )))
        }
    }
}
