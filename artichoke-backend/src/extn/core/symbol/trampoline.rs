use crate::extn::core::array::Array;
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

pub fn all_symbols(interp: &mut Artichoke) -> Result<Value, Exception> {
    let all_symbols = Symbol::all_symbols(interp)?;
    Array::alloc_value(all_symbols, interp)
}

pub fn equal_equal(
    interp: &mut Artichoke,
    mut value: Value,
    mut other: Value,
) -> Result<Value, Exception> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { Symbol::unbox_from_value(&mut other, interp) } {
        let eql = symbol.id() == other.id();
        Ok(interp.convert(eql))
    } else {
        Ok(interp.convert(false))
    }
}

pub fn is_empty(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    let is_empty = symbol.is_empty(interp);
    Ok(interp.convert(is_empty))
}

pub fn length(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    let len = symbol.len(interp);
    interp.try_convert(len)
}

pub fn bytes(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    // These bytes must be cloned because they are owned by the interpreter.
    let bytes = symbol.bytes(interp).to_vec();
    Ok(interp.convert_mut(bytes))
}

pub fn inspect(interp: &mut Artichoke, mut value: Value) -> Result<Value, Exception> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    let inspect = symbol.inspect(interp);
    Ok(interp.convert_mut(inspect))
}
