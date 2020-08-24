use std::convert::TryFrom;

use crate::extn::core::array::Array;
use crate::extn::prelude::*;
use crate::gc::{MrbGarbageCollection, State as GcState};

pub fn clear(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::from("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    array.clear();
    Ok(ary)
}

pub fn element_reference(
    interp: &mut Artichoke,
    mut ary: Value,
    first: Value,
    second: Option<Value>,
) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let elem = array.element_reference(interp, first, second)?;
    Ok(interp.convert(elem))
}

pub fn element_assignment(
    interp: &mut Artichoke,
    mut ary: Value,
    first: Value,
    second: Value,
    third: Option<Value>,
) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::from("can't modify frozen Array").into());
    }
    // TODO: properly handle self-referential sets.
    if ary == first || ary == second || Some(ary) == third {
        return Ok(Value::nil());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };

    let prior_gc_state = interp.disable_gc();

    let result = array.element_assignment(interp, first, second, third);

    if let GcState::Enabled = prior_gc_state {
        interp.enable_gc();
    }
    result
}

pub fn pop(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::from("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let result = array.pop();
    Ok(interp.convert(result))
}

pub fn concat(
    interp: &mut Artichoke,
    mut ary: Value,
    other: Option<Value>,
) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::from("can't modify frozen Array").into());
    }
    if let Some(other) = other {
        let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
        array.concat(interp, other)?;
    }
    Ok(ary)
}

pub fn push(interp: &mut Artichoke, mut ary: Value, value: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::from("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    array.push(value);
    Ok(ary)
}

pub fn reverse_bang(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::from("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    array.reverse();
    Ok(ary)
}

pub fn len(interp: &mut Artichoke, mut ary: Value) -> Result<usize, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    Ok(array.len())
}

pub fn initialize(
    interp: &mut Artichoke,
    into: Value,
    first: Option<Value>,
    second: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Error> {
    let array = Array::initialize(interp, first, second, block)?;
    Array::box_into_value(array, into, interp)
}

pub fn initialize_copy(
    interp: &mut Artichoke,
    ary: Value,
    mut from: Value,
) -> Result<Value, Error> {
    let from = unsafe { Array::unbox_from_value(&mut from, interp)? };
    let result = from.clone();
    Array::box_into_value(result, ary, interp)
}

pub fn shift(interp: &mut Artichoke, mut ary: Value, count: Option<Value>) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::from("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    if let Some(count) = count {
        let count = count.implicitly_convert_to_int(interp)?;
        let count =
            usize::try_from(count).map_err(|_| ArgumentError::from("negative array size"))?;
        let shifted = array.shift_n(count);
        Array::alloc_value(shifted, interp)
    } else {
        let shifted = array.shift();
        Ok(interp.convert(shifted))
    }
}
