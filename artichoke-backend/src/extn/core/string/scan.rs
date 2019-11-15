use bstr::ByteSlice;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Regexp;
use crate::sys;
use crate::types::Ruby;
use crate::value::{Block, Value, ValueLike};
use crate::Artichoke;

#[allow(clippy::cognitive_complexity)]
pub fn method(
    interp: &Artichoke,
    value: Value,
    pattern: Value,
    block: Option<Block>,
) -> Result<Value, Box<dyn RubyException>> {
    let string = value.clone().try_into::<&[u8]>().map_err(|_| {
        Fatal::new(
            interp,
            "Unable to convert Ruby String Receiver to Rust byte slice",
        )
    })?;
    if let Ruby::Symbol = pattern.ruby_type() {
        Err(Box::new(TypeError::new(
            interp,
            format!(
                "wrong argument type {} (expected Regexp)",
                pattern.pretty_name()
            ),
        )))
    } else if let Ok(pattern_bytes) = pattern.clone().try_into::<&[u8]>() {
        if let Some(ref block) = block {
            let mrb = interp.0.borrow().mrb;
            let regex = Regexp::lazy(pattern_bytes);
            let last_match_sym = interp.0.borrow_mut().sym_intern("$~".as_bytes());
            let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
            let patlen = pattern_bytes.len();
            let mut restore_nil = true;
            for pos in string.find_iter(pattern_bytes) {
                restore_nil = false;
                matchdata.set_region(pos, pos + patlen);
                let data = unsafe { matchdata.clone().try_into_ruby(interp, None) }
                    .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
                // TODO: Propagate exceptions from yield.
                let _ = block.yield_arg(interp, &interp.convert(pattern_bytes));
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
            }
            if restore_nil {
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, sys::mrb_sys_nil_value());
                }
            }
            Ok(value)
        } else {
            let (matches, last_pos) = string
                .find_iter(pattern_bytes)
                .enumerate()
                .last()
                .map(|(m, p)| (m + 1, p))
                .unwrap_or_default();
            let mut result = Vec::with_capacity(matches);
            for _ in 0..matches {
                result.push(interp.convert(pattern_bytes));
            }
            if matches > 0 {
                let regex = Regexp::lazy(pattern_bytes);
                let mrb = interp.0.borrow().mrb;
                let last_match_sym = interp.0.borrow_mut().sym_intern("$~".as_bytes());
                let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
                matchdata.set_region(last_pos, last_pos + pattern_bytes.len());
                let data = unsafe { matchdata.clone().try_into_ruby(interp, None) }
                    .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
            } else {
                let mrb = interp.0.borrow().mrb;
                let last_match_sym = interp.0.borrow_mut().sym_intern("$~".as_bytes());
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, sys::mrb_sys_nil_value());
                }
            }
            Ok(interp.convert(result))
        }
    } else if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
        regexp.borrow().inner().scan(interp, value, block)
    } else {
        let pattern_type_name = pattern.pretty_name();
        let pattern_bytes = pattern.funcall::<&[u8]>("to_str", &[], None);
        if let Ok(pattern_bytes) = pattern_bytes {
            if let Some(ref block) = block {
                let regex = Regexp::lazy(pattern_bytes);
                let mrb = interp.0.borrow().mrb;
                let last_match_sym = interp.0.borrow_mut().sym_intern("$~".as_bytes());
                let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
                let patlen = pattern_bytes.len();
                let mut restore_nil = true;
                for pos in string.find_iter(pattern_bytes) {
                    restore_nil = false;
                    matchdata.set_region(pos, pos + patlen);
                    let data =
                        unsafe { matchdata.clone().try_into_ruby(interp, None) }.map_err(|_| {
                            Fatal::new(interp, "Failed to convert MatchData to Ruby Value")
                        })?;
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                    }
                    // TODO: Propagate exceptions from yield.
                    let _ = block.yield_arg(interp, &interp.convert(pattern_bytes));
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                    }
                }
                if restore_nil {
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, sys::mrb_sys_nil_value());
                    }
                }
                Ok(value)
            } else {
                let (matches, last_pos) = string
                    .find_iter(pattern_bytes)
                    .enumerate()
                    .last()
                    .map(|(m, p)| (m + 1, p))
                    .unwrap_or_default();
                let mut result = Vec::with_capacity(matches);
                for _ in 0..matches {
                    result.push(interp.convert(pattern_bytes));
                }
                if matches > 0 {
                    let regex = Regexp::lazy(pattern_bytes);
                    let mrb = interp.0.borrow().mrb;
                    let last_match_sym = interp.0.borrow_mut().sym_intern("$~".as_bytes());
                    let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
                    matchdata.set_region(last_pos, last_pos + pattern_bytes.len());
                    let data =
                        unsafe { matchdata.clone().try_into_ruby(interp, None) }.map_err(|_| {
                            Fatal::new(interp, "Failed to convert MatchData to Ruby Value")
                        })?;
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                    }
                } else {
                    let mrb = interp.0.borrow().mrb;
                    let last_match_sym = interp.0.borrow_mut().sym_intern("$~".as_bytes());
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, sys::mrb_sys_nil_value());
                    }
                }
                Ok(interp.convert(result))
            }
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
