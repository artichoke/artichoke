use bstr::ByteSlice;

use crate::convert::implicitly_convert_to_string;
#[cfg(feature = "core-regexp")]
use crate::extn::core::matchdata::MatchData;
#[cfg(feature = "core-regexp")]
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::prelude::*;

pub fn ord(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    let string = value.try_convert_into_mut::<&[u8]>(interp)?;
    // NOTE: This implementation assumes all `String`s have encoding =
    // `Encoding::UTF_8`. Artichoke does not implement the `Encoding` APIs and
    // `String`s are assumed to be UTF-8 encoded.
    let (ch, size) = bstr::decode_utf8(string);
    let ord = match ch {
        // All `char`s are valid `u32`s
        // https://github.com/rust-lang/rust/blob/1.48.0/library/core/src/char/convert.rs#L12-L20
        Some(ch) => u32::from(ch),
        None if size == 0 => return Err(ArgumentError::with_message("empty string").into()),
        None => return Err(ArgumentError::with_message("invalid byte sequence in UTF-8").into()),
    };
    Ok(interp.convert(ord))
}

pub fn scan(interp: &mut Artichoke, value: Value, mut pattern: Value, block: Option<Block>) -> Result<Value, Error> {
    if let Ruby::Symbol = pattern.ruby_type() {
        let mut message = String::from("wrong argument type ");
        message.push_str(interp.inspect_type_name_for_value(pattern));
        message.push_str(" (expected Regexp)");
        return Err(TypeError::from(message).into());
    }
    #[cfg(feature = "core-regexp")]
    if let Ok(regexp) = unsafe { Regexp::unbox_from_value(&mut pattern, interp) } {
        let haystack = value.try_convert_into_mut::<&[u8]>(interp)?;
        let scan = regexp.inner().scan(interp, haystack, block)?;
        return Ok(interp.try_convert_mut(scan)?.unwrap_or(value));
    }
    #[cfg(feature = "core-regexp")]
    // Safety:
    //
    // Convert `pattern_bytes` to an owned byte vec to ensure the underlying
    // `RString` is not garbage collected when yielding matches.
    if let Ok(pattern_bytes) = unsafe { implicitly_convert_to_string(interp, &mut pattern) } {
        let pattern_bytes = pattern_bytes.to_vec();

        let string = value.try_convert_into_mut::<&[u8]>(interp)?;
        if let Some(ref block) = block {
            let regex = Regexp::lazy(pattern_bytes.clone());
            let matchdata = MatchData::new(string.to_vec(), regex, ..);
            let patlen = pattern_bytes.len();
            if let Some(pos) = string.find(&pattern_bytes) {
                let mut data = matchdata.clone();
                data.set_region(pos..pos + patlen);
                let data = MatchData::alloc_value(data, interp)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                block.yield_arg(interp, &block_arg)?;

                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let offset = pos + patlen;
                let string = string.get(offset..).unwrap_or_default();
                for pos in string.find_iter(&pattern_bytes) {
                    let mut data = matchdata.clone();
                    data.set_region(offset + pos..offset + pos + patlen);
                    let data = MatchData::alloc_value(data, interp)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                    let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                    block.yield_arg(interp, &block_arg)?;

                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;
                }
            } else {
                interp.unset_global_variable(regexp::LAST_MATCH)?;
            }
            return Ok(value);
        }
        let (matches, last_pos) = string
            .find_iter(&pattern_bytes)
            .enumerate()
            .last()
            .map(|(m, p)| (m + 1, p))
            .unwrap_or_default();
        let mut result = Vec::with_capacity(matches);
        for _ in 0..matches {
            result.push(interp.try_convert_mut(pattern_bytes.as_slice())?);
        }
        if matches > 0 {
            let regex = Regexp::lazy(pattern_bytes.clone());
            let matchdata = MatchData::new(string.to_vec(), regex, last_pos..last_pos + pattern_bytes.len());
            let data = MatchData::alloc_value(matchdata, interp)?;
            interp.set_global_variable(regexp::LAST_MATCH, &data)?;
        } else {
            interp.unset_global_variable(regexp::LAST_MATCH)?;
        }
        return interp.try_convert_mut(result);
    }
    #[cfg(not(feature = "core-regexp"))]
    // Safety:
    //
    // Convert `pattern_bytes` to an owned byte vec to ensure the underlying
    // `RString` is not garbage collected when yielding matches.
    if let Ok(pattern_bytes) = unsafe { implicitly_convert_to_string(interp, &mut pattern) } {
        let pattern_bytes = pattern_bytes.to_vec();

        let string = value.try_convert_into_mut::<&[u8]>(interp)?;
        if let Some(ref block) = block {
            let patlen = pattern_bytes.len();
            if let Some(pos) = string.find(&pattern_bytes) {
                let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                block.yield_arg(interp, &block_arg)?;

                let offset = pos + patlen;
                let string = string.get(offset..).unwrap_or_default();
                for _ in string.find_iter(&pattern_bytes) {
                    let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                    block.yield_arg(interp, &block_arg)?;
                }
            }
            return Ok(value);
        }
        let matches = string
            .find_iter(&pattern_bytes)
            .enumerate()
            .last()
            .map(|(m, _)| m + 1)
            .unwrap_or_default();
        let mut result = Vec::with_capacity(matches);
        for _ in 0..matches {
            result.push(interp.try_convert_mut(pattern_bytes.as_slice())?);
        }
        return interp.try_convert_mut(result);
    }
    let mut message = String::from("wrong argument type ");
    message.push_str(interp.inspect_type_name_for_value(pattern));
    message.push_str(" (expected Regexp)");
    Err(TypeError::from(message).into())
}
