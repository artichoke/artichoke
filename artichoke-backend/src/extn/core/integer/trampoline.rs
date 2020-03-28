use crate::extn::core::integer::Integer;
use crate::extn::prelude::*;

pub fn chr(
    interp: &mut Artichoke,
    value: Value,
    encoding: Option<Value>,
) -> Result<Value, Exception> {
    let value = value.try_into::<Integer>()?;
    let s = value.chr(interp, encoding)?;
    Ok(interp.convert_mut(s))
}

pub fn element_reference(
    interp: &mut Artichoke,
    value: Value,
    bit: Value,
) -> Result<Value, Exception> {
    let value = value.try_into::<Integer>()?;
    let bit = value.bit(interp, bit)?;
    Ok(interp.convert(bit))
}

pub fn div(interp: &mut Artichoke, value: Value, denominator: Value) -> Result<Value, Exception> {
    let value = value.try_into::<Integer>()?;
    let quotient = value.div(interp, denominator)?;
    Ok(interp.convert_mut(quotient))
}

pub fn size(interp: &Artichoke, value: Value) -> Result<Value, Exception> {
    let value = value.try_into::<Integer>()?;
    // This `as` cast is lossless because size_of::<Int> is guaranteed to be
    // less than `Int::MAX`.
    let size = value.size(interp) as Int;
    Ok(interp.convert(size))
}
