use std::convert::TryFrom;

use crate::extn::core::regexp::Regexp;
use crate::extn::prelude::*;

pub fn initialize(
    interp: &mut Artichoke,
    pattern: Value,
    options: Option<Value>,
    encoding: Option<Value>,
    into: Option<Value>,
) -> Result<Value, Exception> {
    let (options, encoding) = interp.try_convert_mut((options, encoding))?;
    let regexp = Regexp::initialize(interp, pattern, options, encoding)?;
    regexp.try_into_ruby(interp, into.as_ref().map(Value::inner))
}

pub fn escape(interp: &mut Artichoke, pattern: Value) -> Result<Value, Exception> {
    let pattern = Regexp::escape(interp, pattern)?;
    Ok(interp.convert_mut(pattern))
}

pub fn union(interp: &mut Artichoke, patterns: Vec<Value>) -> Result<Value, Exception> {
    let regexp = Regexp::union(interp, patterns)?;
    regexp.try_into_ruby(interp, None)
}

pub fn is_match(
    interp: &mut Artichoke,
    regexp: Value,
    pattern: Value,
    pos: Option<Value>,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let pattern = pattern.implicitly_convert_to_nilable_string()?;
    let pos = if let Some(pos) = pos {
        Some(pos.implicitly_convert_to_int()?)
    } else {
        None
    };
    let is_match = borrow.is_match(interp, pattern, pos)?;
    Ok(interp.convert(is_match))
}

pub fn match_(
    interp: &mut Artichoke,
    regexp: Value,
    pattern: Value,
    pos: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let pattern = pattern.implicitly_convert_to_nilable_string()?;
    let pos = if let Some(pos) = pos {
        Some(pos.implicitly_convert_to_int()?)
    } else {
        None
    };
    borrow.match_(interp, pattern, pos, block)
}

pub fn eql(interp: &mut Artichoke, regexp: Value, other: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let cmp = borrow.eql(interp, other);
    Ok(interp.convert(cmp))
}

pub fn case_compare(
    interp: &mut Artichoke,
    regexp: Value,
    other: Value,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let cmp = borrow.case_compare(interp, other)?;
    Ok(interp.convert(cmp))
}

pub fn match_operator(
    interp: &mut Artichoke,
    regexp: Value,
    pattern: Value,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let pattern = pattern.implicitly_convert_to_nilable_string()?;
    let pos = borrow.match_operator(interp, pattern)?;
    match pos.map(Int::try_from) {
        Some(Ok(pos)) => Ok(interp.convert(pos)),
        Some(Err(_)) => Err(Exception::from(ArgumentError::new(
            interp,
            "string too long",
        ))),
        None => Ok(interp.convert(None::<Value>)),
    }
}

pub fn is_casefold(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let is_casefold = borrow.is_casefold(interp);
    Ok(interp.convert(is_casefold))
}

pub fn is_fixed_encoding(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let is_fixed_encoding = borrow.is_fixed_encoding(interp);
    Ok(interp.convert(is_fixed_encoding))
}

pub fn hash(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let hash = borrow.hash(interp);
    Ok(interp.convert(hash as Int))
}

pub fn inspect(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let inspect = borrow.inspect(interp);
    Ok(interp.convert_mut(inspect))
}

pub fn named_captures(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let named_captures = borrow.named_captures(interp)?;
    Ok(interp.convert_mut(named_captures))
}

pub fn names(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let names = borrow.names(interp);
    Ok(interp.convert_mut(names))
}

pub fn options(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let opts = borrow.options(interp);
    Ok(interp.convert(opts))
}

pub fn source(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let source = borrow.source(interp);
    Ok(interp.convert_mut(source))
}

pub fn to_s(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }?;
    let borrow = regexp.borrow();
    let s = borrow.string(interp);
    Ok(interp.convert_mut(s))
}
