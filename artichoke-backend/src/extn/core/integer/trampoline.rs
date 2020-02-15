use crate::extn::core::integer;
use crate::extn::prelude::*;

pub fn chr(
    interp: &mut Artichoke,
    value: Value,
    encoding: Option<Value>,
) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    integer::chr(interp, value, encoding)
}

pub fn element_reference(
    interp: &mut Artichoke,
    value: Value,
    bit: Value,
) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    integer::element_reference(interp, value, bit)
}

pub fn div(interp: &mut Artichoke, value: Value, denominator: Value) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    integer::div(interp, value, denominator)
}

pub fn size(interp: &Artichoke, value: Value) -> Result<Value, Exception> {
    let value = value.try_into::<Int>()?;
    integer::size(interp, value).map_err(Exception::from)
}
