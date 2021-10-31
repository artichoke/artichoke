use crate::convert::implicitly_convert_to_int;
use crate::extn::core::integer::Integer;
use crate::extn::prelude::*;

pub fn chr(interp: &mut Artichoke, value: Value, encoding: Option<Value>) -> Result<Value, Error> {
    let value = value.try_convert_into::<Integer>(interp)?;
    let s = value.chr(interp, encoding)?;
    interp.try_convert_mut(s)
}

pub fn element_reference(interp: &mut Artichoke, value: Value, bit: Value) -> Result<Value, Error> {
    let value = value.try_convert_into::<Integer>(interp)?;
    let bit = implicitly_convert_to_int(interp, bit)?;
    let bit = value.bit(bit)?;
    Ok(interp.convert(bit))
}

pub fn div(interp: &mut Artichoke, value: Value, denominator: Value) -> Result<Value, Error> {
    let value = value.try_convert_into::<Integer>(interp)?;
    let quotient = value.div(interp, denominator)?;
    Ok(interp.convert_mut(quotient))
}

pub fn is_allbits(interp: &mut Artichoke, value: Value, mask: Value) -> Result<Value, Error> {
    let value = value.try_convert_into::<Integer>(interp)?;
    let mask = implicitly_convert_to_int(interp, mask)?;
    let result = value.is_allbits(mask);
    Ok(interp.convert(result))
}

pub fn is_anybits(interp: &mut Artichoke, value: Value, mask: Value) -> Result<Value, Error> {
    let value = value.try_convert_into::<Integer>(interp)?;
    let mask = implicitly_convert_to_int(interp, mask)?;
    let result = value.is_anybits(mask);
    Ok(interp.convert(result))
}

pub fn is_nobits(interp: &mut Artichoke, value: Value, mask: Value) -> Result<Value, Error> {
    let value = value.try_convert_into::<Integer>(interp)?;
    let mask = implicitly_convert_to_int(interp, mask)?;
    let result = value.is_nobits(mask);
    Ok(interp.convert(result))
}

pub fn size(interp: &Artichoke) -> Result<Value, Error> {
    // This `as` cast is lossless because `size_of::<i64>` is guaranteed to be
    // less than `i64::MAX`.
    const SIZE: i64 = Integer::size() as i64;
    Ok(interp.convert(SIZE))
}
