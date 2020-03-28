use crate::extn::core::integer::{self, Quotient};
use crate::extn::prelude::*;

pub fn chr(
    interp: &mut Artichoke,
    value: Value,
    encoding: Option<Value>,
) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    let s = integer::chr(interp, value, encoding)?;
    Ok(interp.convert_mut(s))
}

pub fn element_reference(
    interp: &mut Artichoke,
    value: Value,
    bit: Value,
) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    let bit = integer::element_reference(interp, value, bit)?;
    Ok(interp.convert(bit))
}

pub fn div(interp: &mut Artichoke, value: Value, denominator: Value) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    let quotient = integer::div(interp, value, denominator)?;
    match quotient {
        Quotient::Int(num) => Ok(interp.convert(num)),
        Quotient::Float(num) => Ok(interp.convert_mut(num)),
    }
}

pub fn size(interp: &Artichoke, value: Value) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    // This `as` cast is lossless because size_of::<Int> is guaranteed to be
    // less than `Int::MAX`.
    let size = integer::size(interp, value) as Int;
    Ok(interp.convert(size))
}
