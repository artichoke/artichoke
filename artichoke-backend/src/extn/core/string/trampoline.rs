use artichoke_core::value::pretty_name;
use bstr::ByteSlice;

use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::prelude::*;

pub fn ord(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    let string = value.try_into_mut::<&[u8]>(interp)?;

    let ord = if let Some((start, end, ch)) = string.char_indices().next() {
        if ch == '\u{FFFD}' {
            let slice = &string[start..end];
            match slice {
                [] => 0,
                [a] => u32::from_le_bytes([*a, 0, 0, 0]),
                [a, b] => u32::from_le_bytes([*a, *b, 0, 0]),
                [a, b, c] => u32::from_le_bytes([*a, *b, *c, 0]),
                [a, b, c, d] => u32::from_le_bytes([*a, *b, *c, *d]),
                _ => return Err(ArgumentError::from("Unicode out of range").into()),
            }
        } else {
            // All `char`s are valid `u32`s
            // https://github.com/rust-lang/rust/blob/1.41.0/src/libcore/char/convert.rs#L12-L20
            ch as u32
        }
    } else {
        return Err(ArgumentError::from("empty string").into());
    };
    Ok(interp.convert(ord))
}

pub fn scan(
    interp: &mut Artichoke,
    value: Value,
    mut pattern: Value,
    block: Option<Block>,
) -> Result<Value, Error> {
    if let Ruby::Symbol = pattern.ruby_type() {
        let mut message = String::from("wrong argument type ");
        message.push_str(pretty_name(pattern, interp));
        message.push_str(" (expected Regexp)");
        Err(TypeError::from(message).into())
    } else if let Ok(regexp) = unsafe { Regexp::unbox_from_value(&mut pattern, interp) } {
        let haystack = value.try_into_mut::<&[u8]>(interp)?;
        let scan = regexp.inner().scan(interp, haystack, block)?;
        Ok(interp.try_convert_mut(scan)?.unwrap_or(value))
    } else if let Ok(pattern_bytes) = pattern.implicitly_convert_to_string(interp) {
        let string = value.try_into_mut::<&[u8]>(interp)?;
        if let Some(ref block) = block {
            let regex = Regexp::lazy(pattern_bytes.to_vec());
            let matchdata = MatchData::new(string.to_vec(), regex, ..);
            let patlen = pattern_bytes.len();
            if let Some(pos) = string.find(pattern_bytes) {
                let mut data = matchdata.clone();
                data.set_region(pos..pos + patlen);
                let data = MatchData::alloc_value(data, interp)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let block_arg = interp.convert_mut(pattern_bytes);
                let _ = block.yield_arg(interp, &block_arg)?;

                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let offset = pos + patlen;
                let string = string.get(offset..).unwrap_or_default();
                for pos in string.find_iter(pattern_bytes) {
                    let mut data = matchdata.clone();
                    data.set_region(offset + pos..offset + pos + patlen);
                    let data = MatchData::alloc_value(data, interp)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                    let block_arg = interp.convert_mut(pattern_bytes);
                    let _ = block.yield_arg(interp, &block_arg)?;

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
                let data = MatchData::alloc_value(matchdata, interp)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;
            } else {
                interp.unset_global_variable(regexp::LAST_MATCH)?;
            }
            interp.try_convert_mut(result)
        }
    } else {
        let mut message = String::from("wrong argument type ");
        message.push_str(pretty_name(pattern, interp));
        message.push_str(" (expected Regexp)");
        Err(TypeError::from(message).into())
    }
}
