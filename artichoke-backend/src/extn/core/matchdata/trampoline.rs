use std::convert::TryFrom;

use crate::extn::core::array::Array;
use crate::extn::core::matchdata::{CaptureAt, MatchData};
use crate::extn::core::regexp::Regexp;
use crate::extn::prelude::*;
use crate::sys::protect;

pub fn begin(interp: &mut Artichoke, mut value: Value, at: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let capture = interp.try_convert_mut(&at)?;
    let begin = data.begin(interp, capture)?;
    match begin.map(Int::try_from) {
        Some(Ok(begin)) => Ok(interp.convert(begin)),
        Some(Err(_)) => Err(ArgumentError::from("input string too long").into()),
        None => Ok(Value::nil()),
    }
}

pub fn captures(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    if let Some(captures) = data.captures(interp)? {
        interp.try_convert_mut(captures)
    } else {
        Ok(Value::nil())
    }
}

pub fn element_reference(
    interp: &mut Artichoke,
    mut value: Value,
    elem: Value,
    len: Option<Value>,
) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let at = if let Some(len) = len {
        let start = elem.implicitly_convert_to_int(interp)?;
        let len = len.implicitly_convert_to_int(interp)?;
        CaptureAt::StartLen(start, len)
    } else if let Ok(index) = elem.implicitly_convert_to_int(interp) {
        CaptureAt::GroupIndex(index)
    } else if let Ok(name) = elem.implicitly_convert_to_string(interp) {
        CaptureAt::GroupName(name)
    } else {
        // NOTE(lopopolo): Encapsulation is broken here by reaching into the
        // inner regexp.
        let captures_len = data.regexp.inner().captures_len(interp, None)?;
        let rangelen = Int::try_from(captures_len)
            .map_err(|_| ArgumentError::from("input string too long"))?;
        if let Some(protect::Range { start, len }) = elem.is_range(interp, rangelen)? {
            CaptureAt::StartLen(start, len)
        } else {
            return Ok(Value::nil());
        }
    };
    let matched = data.capture_at(interp, at)?;
    interp.try_convert_mut(matched)
}

pub fn end(interp: &mut Artichoke, mut value: Value, at: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let capture = interp.try_convert_mut(&at)?;
    let end = data.end(interp, capture)?;
    match end.map(Int::try_from) {
        Some(Ok(end)) => Ok(interp.convert(end)),
        Some(Err(_)) => Err(ArgumentError::from("input string too long").into()),
        None => Ok(Value::nil()),
    }
}

pub fn length(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let len = data.len(interp)?;
    if let Ok(len) = Int::try_from(len) {
        Ok(interp.convert(len))
    } else {
        Err(ArgumentError::from("input string too long").into())
    }
}

pub fn named_captures(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let named_captures = data.named_captures(interp)?;
    interp.try_convert_mut(named_captures)
}

pub fn names(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let names = data.names(interp);
    interp.try_convert_mut(names)
}

pub fn offset(interp: &mut Artichoke, mut value: Value, at: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let capture = interp.try_convert_mut(&at)?;
    if let Some([begin, end]) = data.offset(interp, capture)? {
        if let (Ok(begin), Ok(end)) = (Int::try_from(begin), Int::try_from(end)) {
            let ary = Array::assoc(interp.convert(begin), interp.convert(end));
            Array::alloc_value(ary, interp)
        } else {
            Err(ArgumentError::from("input string too long").into())
        }
    } else {
        let ary = Array::assoc(Value::nil(), Value::nil());
        Array::alloc_value(ary, interp)
    }
}

pub fn post_match(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let post = data.post();
    Ok(interp.convert_mut(post))
}

pub fn pre_match(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let pre = data.pre();
    Ok(interp.convert_mut(pre))
}

pub fn regexp(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let regexp = data.regexp();
    // TODO(GH-614): MatchData#regexp needs to return an identical Regexp to the
    // one used to create the match (same object ID).
    //
    // The `Regexp::alloc_value` here should be replaced with
    // `Regexp::box_into_value`.
    //
    // See: https://github.com/ruby/spec/pull/727
    let regexp = Regexp::alloc_value(regexp.clone(), interp)?;
    Ok(regexp)
}

pub fn string(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let mut string = interp.convert_mut(data.string());
    string.freeze(interp)?;
    Ok(string)
}

pub fn to_a(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    if let Some(ary) = data.to_a(interp)? {
        interp.try_convert_mut(ary)
    } else {
        Ok(Value::nil())
    }
}

pub fn to_s(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let display = data.to_s(interp)?;
    Ok(interp.convert_mut(display))
}
