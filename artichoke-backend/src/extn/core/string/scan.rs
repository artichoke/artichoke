use bstr::ByteSlice;

use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::prelude::*;

#[allow(clippy::cognitive_complexity)]
pub fn method(
    interp: &mut Artichoke,
    value: Value,
    pattern: Value,
    mut block: Option<Block>,
) -> Result<Value, Exception> {
    let string = value.try_into::<&[u8]>(interp).map_err(|_| {
        Fatal::new(
            interp,
            "Unable to convert Ruby String Receiver to Rust byte slice",
        )
    })?;
    if let Ruby::Symbol = pattern.ruby_type() {
        let message = format!(
            "wrong argument type {} (expected Regexp)",
            pattern.pretty_name(interp)
        );
        Err(Exception::from(TypeError::new(interp, message)))
    } else if let Ok(pattern_bytes) = pattern.try_into::<&[u8]>(interp) {
        if let Some(ref mut block) = block {
            let regex = Regexp::lazy(pattern_bytes);
            let last_match_sym = interp.sym_intern(regexp::LAST_MATCH);
            let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
            let patlen = pattern_bytes.len();
            let mut restore_nil = true;
            for pos in string.find_iter(pattern_bytes) {
                restore_nil = false;
                matchdata.set_region(pos, pos + patlen);
                let data = matchdata
                    .clone()
                    .try_into_ruby(interp, None)
                    .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
                unsafe {
                    sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, data.inner());
                }
                let block_arg = interp.convert(pattern_bytes);
                let _ = block.yield_arg::<Value>(interp, &block_arg)?;
                unsafe {
                    sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, data.inner());
                }
            }
            if restore_nil {
                unsafe {
                    sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, sys::mrb_sys_nil_value());
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
                let last_match_sym = interp.sym_intern(regexp::LAST_MATCH);
                let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
                matchdata.set_region(last_pos, last_pos + pattern_bytes.len());
                let data = matchdata
                    .try_into_ruby(interp, None)
                    .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
                unsafe {
                    sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, data.inner());
                }
            } else {
                let last_match_sym = interp.sym_intern(regexp::LAST_MATCH);
                let nil = interp.convert(None::<Value>).inner();
                unsafe {
                    sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, nil);
                }
            }
            Ok(interp.convert(result))
        }
    } else if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
        regexp.borrow().inner().scan(interp, value, block)
    } else {
        let pattern_type_name = pattern.pretty_name(interp);
        let pattern_bytes = pattern.funcall::<&[u8]>(interp, "to_str", &[], None);
        if let Ok(pattern_bytes) = pattern_bytes {
            if let Some(ref mut block) = block {
                let regex = Regexp::lazy(pattern_bytes);
                let last_match_sym = interp.sym_intern(regexp::LAST_MATCH);
                let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
                let patlen = pattern_bytes.len();
                let mut restore_nil = true;
                for pos in string.find_iter(pattern_bytes) {
                    restore_nil = false;
                    matchdata.set_region(pos, pos + patlen);
                    let data = matchdata.clone().try_into_ruby(interp, None).map_err(|_| {
                        Fatal::new(interp, "Failed to convert MatchData to Ruby Value")
                    })?;
                    unsafe {
                        sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, data.inner());
                    }
                    let block_arg = interp.convert(pattern_bytes);
                    let _ = block.yield_arg::<Value>(interp, &block_arg)?;
                    unsafe {
                        sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, data.inner());
                    }
                }
                if restore_nil {
                    let nil = interp.convert(None::<Value>).inner();
                    unsafe {
                        sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, nil);
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
                    let last_match_sym = interp.sym_intern(regexp::LAST_MATCH);
                    let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
                    matchdata.set_region(last_pos, last_pos + pattern_bytes.len());
                    let data = matchdata.try_into_ruby(interp, None).map_err(|_| {
                        Fatal::new(interp, "Failed to convert MatchData to Ruby Value")
                    })?;
                    unsafe {
                        sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, data.inner());
                    }
                } else {
                    let last_match_sym = interp.sym_intern(regexp::LAST_MATCH);
                    unsafe {
                        sys::mrb_gv_set(interp.mrb_mut(), last_match_sym, sys::mrb_sys_nil_value());
                    }
                }
                Ok(interp.convert(result))
            }
        } else {
            Err(Exception::from(TypeError::new(
                interp,
                format!(
                    "wrong argument type {} (expected Regexp)",
                    pattern_type_name
                ),
            )))
        }
    }
}
