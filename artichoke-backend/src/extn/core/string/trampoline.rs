use bstr::ByteSlice;

use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{self, Regexp, Scan};
use crate::extn::prelude::*;

pub fn ord(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let string = value.try_into::<&[u8]>()?;

    let ord = if let Some((start, end, ch)) = string.char_indices().next() {
        if ch == '\u{FFFD}' {
            let slice = string.get(start..end);
            match slice {
                Some(&[]) => 0,
                Some(&[a]) => u32::from_le_bytes([a, 0, 0, 0]),
                Some(&[a, b]) => u32::from_le_bytes([a, b, 0, 0]),
                Some(&[a, b, c]) => u32::from_le_bytes([a, b, c, 0]),
                Some(&[a, b, c, d]) => u32::from_le_bytes([a, b, c, d]),
                _ => {
                    return Err(Exception::from(ArgumentError::new(
                        interp,
                        "Unicode out of range",
                    )))
                }
            }
        } else {
            // All `char`s are valid `u32`s
            // https://github.com/rust-lang/rust/blob/1.41.0/src/libcore/char/convert.rs#L12-L20
            ch as u32
        }
    } else {
        return Err(Exception::from(ArgumentError::new(interp, "empty string")));
    };
    Ok(interp.convert(ord))
}

pub fn scan(
    interp: &mut Artichoke,
    value: Value,
    pattern: Value,
    block: Option<Block>,
) -> Result<Value, Exception> {
    if let Ruby::Symbol = pattern.ruby_type() {
        let mut message = String::from("wrong argument type ");
        message.push_str(pattern.pretty_name(interp));
        message.push_str(" (expected Regexp)");
        Err(Exception::from(TypeError::new(interp, message)))
    } else if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
        let haystack = value.clone().try_into::<&[u8]>()?;
        match regexp.borrow().inner().scan(interp, haystack, block)? {
            Scan::Haystack => Ok(value),
            Scan::Collected(collected) => Ok(interp.convert_mut(collected)),
            Scan::Patterns(patterns) => Ok(interp.convert_mut(patterns)),
        }
    } else if let Ok(pattern_bytes) = pattern.implicitly_convert_to_string(interp) {
        let string = value.clone().try_into::<&[u8]>()?;
        if let Some(ref block) = block {
            let regex = Regexp::lazy(pattern_bytes.to_vec());
            let matchdata = MatchData::new(string.to_vec(), regex, ..);
            let patlen = pattern_bytes.len();
            if let Some(pos) = string.find(pattern_bytes) {
                let mut data = matchdata.clone();
                data.set_region(pos..pos + patlen);
                let data = data.try_into_ruby(interp, None)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let block_arg = interp.convert_mut(pattern_bytes);
                let _ = block.yield_arg::<Value>(interp, &block_arg)?;

                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let offset = pos + patlen;
                let string = string.get(offset..).unwrap_or_default();
                for pos in string.find_iter(pattern_bytes) {
                    let mut data = matchdata.clone();
                    data.set_region(offset + pos..offset + pos + patlen);
                    let data = data.try_into_ruby(interp, None)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                    let block_arg = interp.convert_mut(pattern_bytes);
                    let _ = block.yield_arg::<Value>(interp, &block_arg)?;

                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;
                }
            } else {
                interp.unset_global_variable(regexp::LAST_MATCH)?;
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
                let regex = Regexp::lazy(pattern_bytes.to_vec());
                let matchdata = MatchData::new(
                    string.to_vec(),
                    regex,
                    last_pos..last_pos + pattern_bytes.len(),
                );
                let data = matchdata.try_into_ruby(interp, None)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;
            } else {
                interp.unset_global_variable(regexp::LAST_MATCH)?;
            }
            Ok(interp.convert_mut(result))
        }
    } else {
        let mut message = String::from("wrong argument type ");
        message.push_str(pattern.pretty_name(interp));
        message.push_str(" (expected Regexp)");
        Err(Exception::from(TypeError::new(interp, message)))
    }
}
