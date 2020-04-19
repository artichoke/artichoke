use std::convert::TryFrom;

use crate::extn::core::matchdata::{CaptureAt, MatchData};
use crate::extn::prelude::*;
use crate::sys::protect;

pub fn begin(interp: &mut Artichoke, value: Value, at: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let capture = interp.try_convert_mut(&at)?;
    let begin = borrow.begin(interp, capture)?;
    match begin.map(Int::try_from) {
        Some(Ok(begin)) => Ok(interp.convert(begin)),
        Some(Err(_)) => Err(Exception::from(ArgumentError::new(
            interp,
            "input string too long",
        ))),
        None => Ok(interp.convert(None::<Value>)),
    }
}

pub fn captures(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    if let Some(captures) = borrow.captures(interp)? {
        Ok(interp.convert_mut(captures))
    } else {
        Ok(interp.convert(None::<Value>))
    }
}

pub fn element_reference(
    interp: &mut Artichoke,
    value: Value,
    elem: Value,
    len: Option<Value>,
) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    // TODO(GH-308): Once extracting a `Range` is safe, extract this conversion
    // to a `TryConvert<&'a Value, CaptureAt<'a>>` impl.
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
        let captures_len = borrow.regexp.inner().captures_len(interp, None)?;
        let rangelen = Int::try_from(captures_len)
            .map_err(|_| ArgumentError::new(interp, "input string too long"))?;
        if let Some(protect::Range { start, len }) = elem.is_range(interp, rangelen)? {
            CaptureAt::StartLen(start, len)
        } else {
            return Ok(interp.convert(None::<Value>));
        }
    };
    let matched = borrow.capture_at(interp, at)?;
    Ok(interp.convert_mut(matched))
}

pub fn end(interp: &mut Artichoke, value: Value, at: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let capture = interp.try_convert_mut(&at)?;
    let end = borrow.end(interp, capture)?;
    match end.map(Int::try_from) {
        Some(Ok(end)) => Ok(interp.convert(end)),
        Some(Err(_)) => Err(Exception::from(ArgumentError::new(
            interp,
            "input string too long",
        ))),
        None => Ok(interp.convert(None::<Value>)),
    }
}

pub fn length(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let len = borrow.len(interp)?;
    if let Ok(len) = Int::try_from(len) {
        Ok(interp.convert(len))
    } else {
        Err(Exception::from(ArgumentError::new(
            interp,
            "input string too long",
        )))
    }
}

pub fn named_captures(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let named_captures = borrow.named_captures(interp)?;
    Ok(interp.convert_mut(named_captures))
}

pub fn names(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let names = borrow.names(interp);
    Ok(interp.convert_mut(names))
}

pub fn offset(interp: &mut Artichoke, value: Value, at: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let capture = interp.try_convert_mut(&at)?;
    if let Some([begin, end]) = borrow.offset(interp, capture)? {
        if let (Ok(begin), Ok(end)) = (Int::try_from(begin), Int::try_from(end)) {
            // TODO: use a proper assoc 2-tuple
            Ok(interp.convert_mut(&[begin, end][..]))
        } else {
            Err(Exception::from(ArgumentError::new(
                interp,
                "input string too long",
            )))
        }
    } else {
        // TODO: use a proper assoc 2-tuple
        Ok(interp.convert_mut(&[None::<Value>, None::<Value>][..]))
    }
}

pub fn post_match(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let post = borrow.post();
    Ok(interp.convert_mut(post))
}

pub fn pre_match(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let pre = borrow.pre();
    Ok(interp.convert_mut(pre))
}

pub fn regexp(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let regexp = borrow.regexp();
    // TODO(GH-614): MatchData#regexp needs to return an identical Regexp to the
    // one used to create the match (same object ID).
    //
    // The `None` here should be replaced with the original `RBasic`.
    //
    // See: https://github.com/ruby/spec/pull/727
    let regexp = regexp.clone().try_into_ruby(interp, None)?;
    Ok(regexp)
}

pub fn string(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let mut string = interp.convert_mut(borrow.string());
    string.freeze(interp)?;
    Ok(string)
}

pub fn to_a(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    if let Some(ary) = borrow.to_a(interp)? {
        Ok(interp.convert_mut(ary))
    } else {
        Ok(interp.convert(None::<Value>))
    }
}

pub fn to_s(interp: &mut Artichoke, value: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let display = borrow.to_s(interp)?;
    Ok(interp.convert_mut(display))
}
