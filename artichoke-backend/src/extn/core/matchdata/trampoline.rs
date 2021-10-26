use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::core::array::Array;
use crate::extn::core::matchdata::{Capture, CaptureAt, CaptureExtract, MatchData};
use crate::extn::core::regexp::Regexp;
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;
use crate::sys::protect;

pub fn begin(interp: &mut Artichoke, mut value: Value, mut at: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let capture = match interp.try_convert_mut(&mut at)? {
        CaptureExtract::GroupIndex(idx) => Capture::GroupIndex(idx),
        CaptureExtract::GroupName(name) => Capture::GroupName(name),
        CaptureExtract::Symbol(symbol) => Capture::GroupName(symbol.bytes(interp)),
    };
    let begin = data.begin(capture)?;
    match begin.map(i64::try_from) {
        Some(Ok(begin)) => Ok(interp.convert(begin)),
        Some(Err(_)) => Err(ArgumentError::with_message("input string too long").into()),
        None => Ok(Value::nil()),
    }
}

pub fn captures(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    if let Some(captures) = data.captures()? {
        interp.try_convert_mut(captures)
    } else {
        Ok(Value::nil())
    }
}

pub fn element_reference(
    interp: &mut Artichoke,
    mut value: Value,
    mut elem: Value,
    len: Option<Value>,
) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let at = if let Some(len) = len {
        let start = implicitly_convert_to_int(interp, elem)?;
        let len = implicitly_convert_to_int(interp, len)?;
        CaptureAt::StartLen(start, len)
    } else if let Ok(index) = implicitly_convert_to_int(interp, elem) {
        CaptureAt::GroupIndex(index)
    } else if let Ok(symbol) = unsafe { Symbol::unbox_from_value(&mut elem, interp) } {
        CaptureAt::GroupName(symbol.bytes(interp))
    } else if let Ok(name) = unsafe { implicitly_convert_to_string(interp, &mut elem) } {
        CaptureAt::GroupName(name)
    } else {
        // NOTE(lopopolo): Encapsulation is broken here by reaching into the
        // inner regexp.
        let captures_len = data.regexp.inner().captures_len(None)?;
        let rangelen =
            i64::try_from(captures_len).map_err(|_| ArgumentError::with_message("input string too long"))?;
        if let Some(protect::Range { start, len }) = elem.is_range(interp, rangelen)? {
            CaptureAt::StartLen(start, len)
        } else {
            return Ok(Value::nil());
        }
    };
    let matched = data.capture_at(at)?;
    interp.try_convert_mut(matched)
}

pub fn end(interp: &mut Artichoke, mut value: Value, mut at: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let capture = match interp.try_convert_mut(&mut at)? {
        CaptureExtract::GroupIndex(idx) => Capture::GroupIndex(idx),
        CaptureExtract::GroupName(name) => Capture::GroupName(name),
        CaptureExtract::Symbol(symbol) => Capture::GroupName(symbol.bytes(interp)),
    };
    let end = data.end(capture)?;
    match end.map(i64::try_from) {
        Some(Ok(end)) => Ok(interp.convert(end)),
        Some(Err(_)) => Err(ArgumentError::with_message("input string too long").into()),
        None => Ok(Value::nil()),
    }
}

pub fn length(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let len = data.len()?;
    if let Ok(len) = i64::try_from(len) {
        Ok(interp.convert(len))
    } else {
        Err(ArgumentError::with_message("input string too long").into())
    }
}

pub fn named_captures(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let named_captures = data.named_captures()?;
    interp.try_convert_mut(named_captures)
}

pub fn names(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let names = data.names();
    interp.try_convert_mut(names)
}

pub fn offset(interp: &mut Artichoke, mut value: Value, mut at: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let capture = match interp.try_convert_mut(&mut at)? {
        CaptureExtract::GroupIndex(idx) => Capture::GroupIndex(idx),
        CaptureExtract::GroupName(name) => Capture::GroupName(name),
        CaptureExtract::Symbol(symbol) => Capture::GroupName(symbol.bytes(interp)),
    };
    if let Some([begin, end]) = data.offset(capture)? {
        if let (Ok(begin), Ok(end)) = (i64::try_from(begin), i64::try_from(end)) {
            let ary = Array::assoc(interp.convert(begin), interp.convert(end));
            Array::alloc_value(ary, interp)
        } else {
            Err(ArgumentError::with_message("input string too long").into())
        }
    } else {
        let ary = Array::assoc(Value::nil(), Value::nil());
        Array::alloc_value(ary, interp)
    }
}

pub fn post_match(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let post = data.post();
    interp.try_convert_mut(post)
}

pub fn pre_match(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let pre = data.pre();
    interp.try_convert_mut(pre)
}

pub fn regexp(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
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

pub fn string(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let mut string = interp.try_convert_mut(data.string())?;
    string.freeze(interp)?;
    Ok(string)
}

pub fn to_a(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    if let Some(ary) = data.to_a()? {
        interp.try_convert_mut(ary)
    } else {
        Ok(Value::nil())
    }
}

pub fn to_s(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::unbox_from_value(&mut value, interp)? };
    let display = data.to_s()?;
    interp.try_convert_mut(display)
}
