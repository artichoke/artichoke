use bstr::ByteSlice;

use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::prelude::*;

#[allow(clippy::cognitive_complexity)]
pub fn method(
    interp: &mut Artichoke,
    value: Value,
    pattern: Value,
    block: Option<Block>,
) -> Result<Value, Exception> {
    let string = value.clone().try_into::<&[u8]>().map_err(|_| {
        Fatal::new(
            interp,
            "Unable to convert Ruby String Receiver to Rust byte slice",
        )
    })?;
    if let Ruby::Symbol = pattern.ruby_type() {
        let mut message = String::from("wrong argument type ");
        message.push_str(pattern.pretty_name());
        message.push_str(" (expected Regexp)");
        Err(Exception::from(TypeError::new(interp, message)))
    } else if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
        regexp.borrow().inner().scan(interp, value, block)
    } else if let Ok(pattern_bytes) = pattern.implicitly_convert_to_string() {
        if let Some(ref block) = block {
            let regex = Regexp::lazy(pattern_bytes);
            let mrb = interp.0.borrow().mrb;
            let last_match_sym = interp.intern_symbol(regexp::LAST_MATCH);
            let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
            let patlen = pattern_bytes.len();
            let mut restore_nil = true;
            for pos in string.find_iter(pattern_bytes) {
                restore_nil = false;
                matchdata.set_region(pos, pos + patlen);
                let data = matchdata.clone().try_into_ruby(interp, None)?;
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
                let block_arg = interp.convert_mut(pattern_bytes);
                let _ = block.yield_arg::<Value>(interp, &block_arg)?;
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
                result.push(interp.convert_mut(pattern_bytes));
            }
            if matches > 0 {
                let regex = Regexp::lazy(pattern_bytes);
                let mrb = interp.0.borrow().mrb;
                let last_match_sym = interp.intern_symbol(regexp::LAST_MATCH);
                let mut matchdata = MatchData::new(string.to_vec(), regex, 0, string.len());
                matchdata.set_region(last_pos, last_pos + pattern_bytes.len());
                let data = matchdata.try_into_ruby(interp, None)?;
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
            } else {
                let mrb = interp.0.borrow().mrb;
                let last_match_sym = interp.intern_symbol(regexp::LAST_MATCH);
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, sys::mrb_sys_nil_value());
                }
            }
            Ok(interp.convert_mut(result))
        }
    } else {
        let mut message = String::from("wrong argument type ");
        message.push_str(pattern.pretty_name());
        message.push_str(" (expected Regexp)");
        Err(Exception::from(TypeError::new(interp, message)))
    }
}
