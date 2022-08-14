use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_nilable_string, implicitly_convert_to_string};
use crate::extn::core::regexp::Regexp;
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

pub fn initialize(
    interp: &mut Artichoke,
    pattern: Value,
    options: Option<Value>,
    encoding: Option<Value>,
    mut into: Value,
) -> Result<Value, Error> {
    if let Ok(existing) = unsafe { Regexp::unbox_from_value(&mut into, interp) } {
        if existing.is_literal() {
            return Err(FrozenError::with_message("can't modify literal regexp").into());
        }
        return Err(TypeError::with_message("already initialized regexp").into());
    }
    let (options, encoding) = interp.try_convert_mut((options, encoding))?;
    let regexp = Regexp::initialize(interp, pattern, options, encoding)?;
    let mut value = Regexp::box_into_value(regexp, into, interp)?;
    if matches!(options, Some(options) if options.is_literal()) {
        value.freeze(interp)?;
    }
    Ok(value)
}

pub fn escape(interp: &mut Artichoke, mut pattern: Value) -> Result<Value, Error> {
    let pattern_vec = if let Ruby::Symbol = pattern.ruby_type() {
        let symbol = unsafe { Symbol::unbox_from_value(&mut pattern, interp)? };
        symbol.bytes(interp).to_vec()
    } else {
        // SAFETY: Convert the bytes to an owned vec to prevent the underlying
        // `RString*` backing `pattern` from being freed during a garbage
        // collection.
        unsafe { implicitly_convert_to_string(interp, &mut pattern)?.to_vec() }
    };
    let pattern = Regexp::escape(&pattern_vec)?;
    interp.try_convert_mut(pattern)
}

pub fn union<T>(interp: &mut Artichoke, patterns: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let regexp = Regexp::union(interp, patterns)?;
    Regexp::alloc_value(regexp, interp)
}

pub fn is_match(
    interp: &mut Artichoke,
    mut regexp: Value,
    mut pattern: Value,
    pos: Option<Value>,
) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let pattern = unsafe { implicitly_convert_to_nilable_string(interp, &mut pattern)? };
    let pos = if let Some(pos) = pos {
        Some(implicitly_convert_to_int(interp, pos)?)
    } else {
        None
    };
    let is_match = regexp.is_match(pattern, pos)?;
    Ok(interp.convert(is_match))
}

pub fn match_(
    interp: &mut Artichoke,
    mut regexp: Value,
    mut pattern: Value,
    pos: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let pattern_vec;
    let pattern = if let Ruby::Symbol = pattern.ruby_type() {
        let symbol = unsafe { Symbol::unbox_from_value(&mut pattern, interp)? };
        pattern_vec = symbol.bytes(interp).to_vec();
        Some(pattern_vec.as_slice())
    } else {
        unsafe { implicitly_convert_to_nilable_string(interp, &mut pattern)? }
    };
    let pos = if let Some(pos) = pos {
        Some(implicitly_convert_to_int(interp, pos)?)
    } else {
        None
    };
    regexp.match_(interp, pattern, pos, block)
}

pub fn eql(interp: &mut Artichoke, mut regexp: Value, other: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let cmp = regexp.eql(interp, other);
    Ok(interp.convert(cmp))
}

pub fn case_compare(interp: &mut Artichoke, mut regexp: Value, other: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let cmp = regexp.case_compare(interp, other)?;
    Ok(interp.convert(cmp))
}

pub fn match_operator(interp: &mut Artichoke, mut regexp: Value, mut pattern: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let pattern_vec;
    let pattern = if let Ruby::Symbol = pattern.ruby_type() {
        let symbol = unsafe { Symbol::unbox_from_value(&mut pattern, interp)? };
        pattern_vec = symbol.bytes(interp).to_vec();
        Some(pattern_vec.as_slice())
    } else {
        unsafe { implicitly_convert_to_nilable_string(interp, &mut pattern)? }
    };
    let pos = regexp.match_operator(interp, pattern)?;
    match pos.map(i64::try_from) {
        Some(Ok(pos)) => Ok(interp.convert(pos)),
        Some(Err(_)) => Err(ArgumentError::with_message("string too long").into()),
        None => Ok(Value::nil()),
    }
}

pub fn is_casefold(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let is_casefold = regexp.is_casefold();
    Ok(interp.convert(is_casefold))
}

pub fn is_fixed_encoding(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let is_fixed_encoding = regexp.is_fixed_encoding();
    Ok(interp.convert(is_fixed_encoding))
}

pub fn hash(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    #[allow(clippy::cast_possible_wrap)]
    let hash = regexp.hash() as i64;
    Ok(interp.convert(hash))
}

pub fn inspect(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let inspect = regexp.inspect();
    interp.try_convert_mut(inspect)
}

pub fn named_captures(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let named_captures = regexp.named_captures()?;
    interp.try_convert_mut(named_captures)
}

pub fn names(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let names = regexp.names();
    interp.try_convert_mut(names)
}

pub fn options(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let opts = regexp.options();
    Ok(interp.convert(opts))
}

pub fn source(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let source = regexp.source();
    interp.try_convert_mut(source)
}

pub fn to_s(interp: &mut Artichoke, mut regexp: Value) -> Result<Value, Error> {
    let regexp = unsafe { Regexp::unbox_from_value(&mut regexp, interp)? };
    let s = regexp.string();
    interp.try_convert_mut(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::prelude::*;

    #[test]
    fn should_raise_frozen_error() {
        let mut interp = interpreter();
        let pattern = interp.try_convert_mut("xyz").unwrap();
        let options = None;
        let encoding = None;
        let slf = interp.eval(b"/abc/").unwrap();
        let result = initialize(&mut interp, pattern, options, encoding, slf);
        assert_eq!(
            "FrozenError (can't modify literal regexp)",
            result.unwrap_err().to_string()
        );
    }
}
