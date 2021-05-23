use spinoso_symbol::{CaseFold, InternerAllSymbols};

use crate::extn::core::array::Array;
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

pub fn all_symbols(interp: &mut Artichoke) -> Result<Value, Error> {
    let all_symbols = interp
        .all_symbols()
        .map(|sym| Symbol::alloc_value(sym, interp))
        .collect::<Result<Array, Error>>()?;
    Array::alloc_value(all_symbols, interp)
}

pub fn equal_equal(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { Symbol::unbox_from_value(&mut other, interp) } {
        let eql = symbol.id() == other.id();
        Ok(interp.convert(eql))
    } else {
        Ok(interp.convert(false))
    }
}

pub fn ascii_casecmp(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { Symbol::unbox_from_value(&mut other, interp) } {
        let cmp = spinoso_symbol::ascii_casecmp(interp, symbol.id(), other.id())?;
        Ok(interp.convert(cmp as i64))
    } else {
        Ok(Value::nil())
    }
}

pub fn unicode_casecmp(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { Symbol::unbox_from_value(&mut other, interp) } {
        let cmp = spinoso_symbol::unicode_case_eq(interp, symbol.id(), other.id(), CaseFold::new())?;
        Ok(interp.convert(cmp))
    } else {
        Ok(Value::nil())
    }
}

pub fn is_empty(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    let is_empty = symbol.is_empty(interp);
    Ok(interp.convert(is_empty))
}

pub fn length(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    let len = symbol.len(interp);
    interp.try_convert(len)
}

pub fn bytes(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    // These bytes must be cloned because they are owned by the interpreter.
    let bytes = symbol.bytes(interp).to_vec();
    interp.try_convert_mut(bytes)
}

pub fn inspect(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let symbol = unsafe { Symbol::unbox_from_value(&mut value, interp)? };
    let inspect = symbol.inspect(interp);
    let debug = inspect.collect::<String>();
    interp.try_convert_mut(debug)
}
