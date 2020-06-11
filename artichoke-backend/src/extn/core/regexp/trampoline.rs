use std::convert::TryFrom;

use crate::extn::core::regexp::Regexp;
use crate::extn::prelude::*;

pub fn initialize(
    interp: &mut Artichoke,
    pattern: Value,
    options: Option<Value>,
    encoding: Option<Value>,
    into: Value,
) -> Result<Value, Exception> {
    let (options, encoding) = interp.try_convert_mut((options, encoding))?;
    let regexp = Regexp::initialize(interp, pattern, options, encoding)?;
    Regexp::box_into_value(regexp, into, interp)
}

pub fn escape(interp: &mut Artichoke, pattern: Value) -> Result<Value, Exception> {
    let pattern = pattern.implicitly_convert_to_string(interp)?;
    let pattern = Regexp::escape(interp, pattern)?;
    Ok(interp.convert_mut(pattern))
}

pub fn union<T>(interp: &mut Artichoke, patterns: T) -> Result<Value, Exception>
where
    T: IntoIterator<Item = Value>,
{
    let regexp = Regexp::union(interp, patterns)?;
    Regexp::alloc_value(regexp, interp)
}

pub fn is_match(
    interp: &mut Artichoke,
    mut regexp: Value,
    pattern: Value,
    pos: Option<Value>,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let pattern = pattern.implicitly_convert_to_nilable_string(interp)?;
    let pos = if let Some(pos) = pos {
        Some(pos.implicitly_convert_to_int(interp)?)
    } else {
        None
    };
    let is_match = regexp.is_match(interp, pattern, pos)?;
    Ok(interp.convert(is_match))
}

pub fn match_(
    interp: &mut Artichoke,
    mut regexp: Value,
    pattern: Value,
    pos: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let pattern = pattern.implicitly_convert_to_nilable_string(interp)?;
    let pos = if let Some(pos) = pos {
        Some(pos.implicitly_convert_to_int(interp)?)
    } else {
        None
    };
    regexp.match_(interp, pattern, pos, block)
}

pub fn eql(interp: &mut Artichoke, mut regexp: Value, other: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let cmp = regexp.eql(interp, other);
    Ok(interp.convert(cmp))
}

pub fn case_compare(
    interp: &mut Artichoke,
    mut regexp: Value,
    other: Value,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let cmp = regexp.case_compare(interp, other)?;
    Ok(interp.convert(cmp))
}

pub fn match_operator(
    interp: &mut Artichoke,
    mut regexp: Value,
    pattern: Value,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let pattern = pattern.implicitly_convert_to_nilable_string(interp)?;
    let pos = regexp.match_operator(interp, pattern)?;
    match pos.map(Int::try_from) {
        Some(Ok(pos)) => Ok(interp.convert(pos)),
        Some(Err(_)) => Err(ArgumentError::from("string too long").into()),
        None => Ok(Value::nil()),
    }
}

pub fn is_casefold(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let is_casefold = regexp.is_casefold(interp);
    Ok(interp.convert(is_casefold))
}

pub fn is_fixed_encoding(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let is_fixed_encoding = regexp.is_fixed_encoding(interp);
    Ok(interp.convert(is_fixed_encoding))
}

pub fn hash(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let hash = regexp.hash(interp);
    #[allow(clippy::cast_possible_wrap)]
    Ok(interp.convert(hash as Int))
}

pub fn inspect(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let inspect = regexp.inspect(interp);
    Ok(interp.convert_mut(inspect))
}

pub fn named_captures(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let named_captures = regexp.named_captures(interp)?;
    interp.try_convert_mut(named_captures)
}

pub fn names(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let names = regexp.names(interp);
    interp.try_convert_mut(names)
}

pub fn options(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let opts = regexp.options(interp);
    Ok(interp.convert(opts))
}

pub fn source(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let source = regexp.source(interp);
    Ok(interp.convert_mut(source))
}

pub fn to_s(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let s = regexp.string(interp);
    Ok(interp.convert_mut(s))
}
