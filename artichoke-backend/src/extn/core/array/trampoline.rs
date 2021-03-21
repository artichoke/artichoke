use std::convert::TryFrom;

use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::core::array::Array;
use crate::extn::prelude::*;
use crate::gc::{MrbGarbageCollection, State as GcState};

pub fn plus(interp: &mut Artichoke, mut ary: Value, mut other: Value) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let result = if let Ok(other) = unsafe { Array::unbox_from_value(&mut other, interp) } {
        let mut result = Array::with_capacity(array.len() + other.len());
        result.0.concat(array.0.as_slice());
        result.0.concat(other.0.as_slice());
        result
    } else if other.respond_to(interp, "to_ary")? {
        let mut other_converted = other.funcall(interp, "to_ary", &[], None)?;
        if let Ok(other) = unsafe { Array::unbox_from_value(&mut other_converted, interp) } {
            let mut result = Array::with_capacity(array.len() + other.len());
            result.0.concat(array.0.as_slice());
            result.0.concat(other.0.as_slice());
            result
        } else {
            let mut message = String::from("can't convert ");
            let name = interp.inspect_type_name_for_value(other);
            message.push_str(name);
            message.push_str(" to Array (");
            message.push_str(name);
            message.push_str("#to_ary gives ");
            message.push_str(interp.inspect_type_name_for_value(other_converted));
            return Err(TypeError::from(message).into());
        }
    } else {
        let mut message = String::from("no implicit conversion of ");
        message.push_str(interp.inspect_type_name_for_value(other));
        message.push_str(" into Array");
        return Err(TypeError::from(message).into());
    };
    Array::alloc_value(result, interp)
}

pub fn mul(interp: &mut Artichoke, mut ary: Value, mut joiner: Value) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    // Safety:
    //
    // Convert separator to an owned byte vec to ensure the `RString` backing
    // `joiner` is not garbage collected when invoking `to_s` during `join`.
    if let Ok(separator) = unsafe { implicitly_convert_to_string(interp, &mut joiner) } {
        let separator = separator.to_vec();
        let s = array.join(interp, &separator)?;
        Ok(interp.convert_mut(s))
    } else {
        let n = implicitly_convert_to_int(interp, joiner)?;
        if let Ok(n) = usize::try_from(n) {
            let value = array.repeat(n)?;
            let result = Array::alloc_value(value, interp)?;
            let result_value = result.inner();
            let ary_value = ary.inner();
            unsafe {
                let ary_rbasic = ary_value.value.p.cast::<sys::RBasic>();
                let result_rbasic = result_value.value.p.cast::<sys::RBasic>();
                (*result_rbasic).c = (*ary_rbasic).c;
            }
            Ok(result)
        } else {
            Err(ArgumentError::with_message("negative argument").into())
        }
    }
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
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    // TODO: properly handle self-referential sets.
    if ary == first || ary == second || Some(ary) == third {
        return Ok(Value::nil());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };

    let prior_gc_state = interp.disable_gc();

    let result = array.element_assignment(interp, first, second, third);

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    if let GcState::Enabled = prior_gc_state {
        interp.enable_gc();
    }
    result
}

pub fn clear(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    array.clear();

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    Ok(ary)
}

pub fn concat<I>(interp: &mut Artichoke, mut ary: Value, others: I) -> Result<Value, Error>
where
    I: IntoIterator<Item = Value>,
    I::IntoIter: Clone,
{
    // Assumption for the average length of an `Array` that will be concatenated
    // into `ary`.
    const OTHER_ARRAYS_AVG_LENGTH: usize = 5;

    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let others = others.into_iter();

    // Allocate a new buffer and concatenate into it to allow preserving the
    // original `Array`'s items if `ary` is concatenated with itself.
    //
    // This allocation assumes that each `Array` yielded by the iterator is
    // "small", where small means 5 elements or fewer.
    let mut replacement = Array::with_capacity(array.len() + others.clone().count() * OTHER_ARRAYS_AVG_LENGTH);
    replacement.0.concat(array.0.as_slice());

    for mut other in others {
        if let Ok(other) = unsafe { Array::unbox_from_value(&mut other, interp) } {
            replacement.0.reserve(other.len());
            replacement.0.concat(other.0.as_slice());
        } else if other.respond_to(interp, "to_ary")? {
            let mut other_converted = other.funcall(interp, "to_ary", &[], None)?;
            if let Ok(other) = unsafe { Array::unbox_from_value(&mut other_converted, interp) } {
                replacement.0.reserve(other.len());
                replacement.0.concat(other.0.as_slice());
            } else {
                let mut message = String::from("can't convert ");
                let name = interp.inspect_type_name_for_value(other);
                message.push_str(name);
                message.push_str(" to Array (");
                message.push_str(name);
                message.push_str("#to_ary gives ");
                message.push_str(interp.inspect_type_name_for_value(other_converted));
                return Err(TypeError::from(message).into());
            }
        } else {
            let mut message = String::from("no implicit conversion of ");
            message.push_str(interp.inspect_type_name_for_value(other));
            message.push_str(" into Array");
            return Err(TypeError::from(message).into());
        }
    }

    unsafe {
        let _old = array.take();
        Array::box_into_value(replacement, ary, interp)?;
    }

    Ok(ary)
}

pub fn first(interp: &mut Artichoke, mut ary: Value, num: Option<Value>) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    if let Some(num) = num {
        // Hack to detect `BigNum`
        if matches!(num.ruby_type(), Ruby::Float) {
            return Err(RangeError::with_message("bignum too big to convert into `long'").into());
        }
        let n = implicitly_convert_to_int(interp, num)?;
        if let Ok(n) = usize::try_from(n) {
            let slice = array.0.first_n(n);
            let result = Array::from(slice);
            Array::alloc_value(result, interp)
        } else {
            Err(ArgumentError::with_message("negative array size").into())
        }
    } else {
        let last = array.0.first().copied().map(Value::from);
        Ok(interp.convert(last))
    }
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

pub fn initialize_copy(interp: &mut Artichoke, ary: Value, mut from: Value) -> Result<Value, Error> {
    let from = unsafe { Array::unbox_from_value(&mut from, interp)? };
    let result = from.clone();
    Array::box_into_value(result, ary, interp)
}

pub fn last(interp: &mut Artichoke, mut ary: Value, num: Option<Value>) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    if let Some(num) = num {
        // Hack to detect `BigNum`
        if matches!(num.ruby_type(), Ruby::Float) {
            return Err(RangeError::with_message("bignum too big to convert into `long'").into());
        }
        let n = implicitly_convert_to_int(interp, num)?;
        if let Ok(n) = usize::try_from(n) {
            let slice = array.0.last_n(n);
            let result = Array::from(slice);
            Array::alloc_value(result, interp)
        } else {
            Err(ArgumentError::with_message("negative array size").into())
        }
    } else {
        let last = array.0.last().copied().map(Value::from);
        Ok(interp.convert(last))
    }
}

pub fn len(interp: &mut Artichoke, mut ary: Value) -> Result<usize, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    Ok(array.len())
}

pub fn pop(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let result = array.pop();

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    Ok(interp.convert(result))
}

pub fn push(interp: &mut Artichoke, mut ary: Value, value: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    array.push(value);

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    Ok(ary)
}

pub fn reverse(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let mut reversed = array.clone();
    reversed.reverse();
    Array::alloc_value(reversed, interp)
}

pub fn reverse_bang(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    array.reverse();

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    Ok(ary)
}

pub fn shift(interp: &mut Artichoke, mut ary: Value, count: Option<Value>) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let result = if let Some(count) = count {
        let count = implicitly_convert_to_int(interp, count)?;
        let count = usize::try_from(count).map_err(|_| ArgumentError::with_message("negative array size"))?;
        let shifted = array.shift_n(count);

        Array::alloc_value(shifted, interp)
    } else {
        let shifted = array.shift();

        Ok(interp.convert(shifted))
    };

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    result
}
