use core::convert::TryFrom;
use core::iter;

use bstr::ByteSlice;

use crate::convert::implicitly_convert_to_int;
use crate::convert::implicitly_convert_to_string;
use crate::extn::core::array::Array;
#[cfg(feature = "core-regexp")]
use crate::extn::core::matchdata::MatchData;
#[cfg(feature = "core-regexp")]
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::prelude::*;

pub fn cmp_rocket(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { super::String::unbox_from_value(&mut other, interp) } {
        let cmp = s.cmp(&*other);
        Ok(interp.convert(cmp as i64))
    } else {
        Ok(Value::nil())
    }
}

pub fn equals_equals(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { super::String::unbox_from_value(&mut other, interp) } {
        let equals = *s == *other;
        return Ok(interp.convert(equals));
    }
    // Safety:
    //
    // The byteslice is immediately discarded after extraction. There are no
    // intervening interpreter accesses.
    if value.respond_to(interp, "to_str")? {
        let result = other.funcall(interp, "==", &[value], None)?;
        // any falsy returned value yields `false`, otherwise `true`.
        if let Ok(result) = TryConvert::<_, Option<bool>>::try_convert(interp, result) {
            let result = result.unwrap_or_default();
            Ok(interp.convert(result))
        } else {
            Ok(interp.convert(true))
        }
    } else {
        Ok(interp.convert(false))
    }
}

pub fn add(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The borrowed byte slice is immediately memcpy'd into the `s` byte
    // buffer. There are no intervening interpreter accesses.
    let to_append = unsafe { implicitly_convert_to_string(interp, &mut other)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        string_mut.extend_from_slice(to_append);

        let inner = s.take();
        super::String::box_into_value(inner, value, interp)
    }
}

pub fn mul(interp: &mut Artichoke, mut value: Value, count: Value) -> Result<Value, Error> {
    let count = implicitly_convert_to_int(interp, count)?;
    let count = usize::try_from(count).map_err(|_| ArgumentError::with_message("negative argument"))?;

    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let repeated_s = iter::repeat(s.bytes()).take(count).flatten().collect::<super::String>();
    super::String::alloc_value(repeated_s, interp)
}

pub fn bytesize(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let bytesize = s.bytesize();
    interp.try_convert(bytesize)
}

pub fn bytes(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let bytes = s
        .clone()
        .bytes()
        .map(i64::from)
        .map(|byte| interp.convert(byte))
        .collect::<Array>();
    Array::alloc_value(bytes, interp)
}

pub fn eql(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { super::String::unbox_from_value(&mut other, interp) } {
        let eql = *s == *other;
        Ok(interp.convert(eql))
    } else {
        Ok(interp.convert(false))
    }
}

pub fn inspect(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let inspect = s.inspect().collect::<super::String>();
    super::String::alloc_value(inspect, interp)
}

pub fn length(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let length = s.char_len();
    interp.try_convert(length)
}

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
