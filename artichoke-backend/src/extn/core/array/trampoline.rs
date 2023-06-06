use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::core::array::Array;
use crate::extn::prelude::*;

pub fn plus(interp: &mut Artichoke, mut ary: Value, mut other: Value) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let result = if let Ok(other) = unsafe { Array::unbox_from_value(&mut other, interp) } {
        let mut result = Array::with_capacity(array.len() + other.len());
        result.concat(array.as_slice());
        result.concat(other.as_slice());
        result
    } else if other.respond_to(interp, "to_ary")? {
        let mut other_converted = other.funcall(interp, "to_ary", &[], None)?;
        if let Ok(other) = unsafe { Array::unbox_from_value(&mut other_converted, interp) } {
            let mut result = Array::with_capacity(array.len() + other.len());
            result.concat(array.as_slice());
            result.concat(other.as_slice());
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
    // SAFETY: Convert separator to an owned byte vec to ensure the `RString*`
    // backing `joiner` is not garbage collected when invoking `to_s` during
    // `join`.
    if let Ok(separator) = unsafe { implicitly_convert_to_string(interp, &mut joiner) } {
        let separator = separator.to_vec();
        let s = super::join(interp, &array, &separator)?;
        interp.try_convert_mut(s)
    } else {
        let n = implicitly_convert_to_int(interp, joiner)?;
        if let Ok(n) = usize::try_from(n) {
            let value = super::repeat(&array, n)?;
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

pub fn push_single(interp: &mut Artichoke, mut ary: Value, value: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };

    // SAFETY: The array is repacked without any intervening interpreter heap
    // allocations.
    let array_mut = unsafe { array.as_inner_mut() };
    array_mut.push(value);

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    Ok(ary)
}

pub fn element_reference(
    interp: &mut Artichoke,
    mut ary: Value,
    first: Value,
    second: Option<Value>,
) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    let elem = super::aref(interp, &array, first, second)?;
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

    // XXX: Ensure that `array_mut` does not allocate in between mruby
    // allocations.
    let array_mut = unsafe { array.as_inner_mut() };
    let result = super::aset(interp, array_mut, first, second, third);

    unsafe {
        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    result
}

pub fn clear(interp: &mut Artichoke, mut ary: Value) -> Result<Value, Error> {
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }
    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };

    // SAFETY: Clearing a `Vec` does not reallocate it, but it does change its
    // length. The array is repacked before any intervening interpreter heap
    // allocations occur.
    unsafe {
        let array_mut = array.as_inner_mut();
        array_mut.clear();

        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;
    }

    Ok(ary)
}

pub fn push<I>(interp: &mut Artichoke, mut ary: Value, others: I) -> Result<Value, Error>
where
    I: IntoIterator<Item = Value>,
    I::IntoIter: Clone,
{
    if ary.is_frozen(interp) {
        return Err(FrozenError::with_message("can't modify frozen Array").into());
    }

    let mut array = unsafe { Array::unbox_from_value(&mut ary, interp)? };

    // SAFETY: The array is repacked without any intervening interpreter heap
    // allocations.
    let array_mut = unsafe { array.as_inner_mut() };

    array_mut.extend(others);

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
    replacement.concat(array.as_slice());

    for mut other in others {
        if let Ok(other) = unsafe { Array::unbox_from_value(&mut other, interp) } {
            replacement.reserve(other.len());
            replacement.concat(other.as_slice());
        } else if other.respond_to(interp, "to_ary")? {
            let mut other_converted = other.funcall(interp, "to_ary", &[], None)?;
            if let Ok(other) = unsafe { Array::unbox_from_value(&mut other_converted, interp) } {
                replacement.reserve(other.len());
                replacement.concat(other.as_slice());
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
        if let Ruby::Float = num.ruby_type() {
            return Err(RangeError::with_message("bignum too big to convert into `long'").into());
        }
        let n = implicitly_convert_to_int(interp, num)?;
        if let Ok(n) = usize::try_from(n) {
            let slice = array.first_n(n);
            let result = Array::from(slice);
            Array::alloc_value(result, interp)
        } else {
            Err(ArgumentError::with_message("negative array size").into())
        }
    } else {
        let last = array.first();
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
    // Pack an empty `Array` into the given uninitialized `RArray *` so it can
    // be safely marked if an mruby allocation occurs and a GC is triggered in
    // `Array::initialize`.
    //
    // Allocations are likely in the case where a block is passed to
    // `Array#initialize` or when the first and second args must be coerced with
    // the `#to_*` family of methods.
    Array::box_into_value(Array::new(), into, interp)?;
    let array = super::initialize(interp, first, second, block)?;
    Array::box_into_value(array, into, interp)
}

pub fn initialize_copy(interp: &mut Artichoke, ary: Value, mut from: Value) -> Result<Value, Error> {
    // Pack an empty `Array` into the given uninitialized `RArray *` so it can
    // be safely marked if an mruby allocation occurs and a GC is triggered in
    // `Array::initialize`.
    //
    // This ensures the given `RArry *` is initialized even if a non-`Array`
    // object is called with `Array#initialize_copy` and the
    // `Array::unbox_from_value` call below short circuits with an error.
    Array::box_into_value(Array::new(), ary, interp)?;
    let from = unsafe { Array::unbox_from_value(&mut from, interp)? };
    let result = from.clone();
    Array::box_into_value(result, ary, interp)
}

pub fn last(interp: &mut Artichoke, mut ary: Value, num: Option<Value>) -> Result<Value, Error> {
    let array = unsafe { Array::unbox_from_value(&mut ary, interp)? };
    if let Some(num) = num {
        // Hack to detect `BigNum`
        if let Ruby::Float = num.ruby_type() {
            return Err(RangeError::with_message("bignum too big to convert into `long'").into());
        }
        let n = implicitly_convert_to_int(interp, num)?;
        if let Ok(n) = usize::try_from(n) {
            let slice = array.last_n(n);
            let result = Array::from(slice);
            Array::alloc_value(result, interp)
        } else {
            Err(ArgumentError::with_message("negative array size").into())
        }
    } else {
        let last = array.last();
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

    // SAFETY: The array is repacked without any intervening interpreter heap
    // allocations.
    let result = unsafe {
        let array_mut = array.as_inner_mut();
        let result = array_mut.pop();

        let inner = array.take();
        Array::box_into_value(inner, ary, interp)?;

        result
    };

    Ok(interp.convert(result))
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

    // SAFETY: Reversing an `Array` in place does not reallocate it. The array
    // is repacked without any intervening interpreter heap allocations.
    unsafe {
        let array_mut = array.as_inner_mut();
        array_mut.reverse();

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
    if let Some(count) = count {
        let count = implicitly_convert_to_int(interp, count)?;
        let count = usize::try_from(count).map_err(|_| ArgumentError::with_message("negative array size"))?;

        // SAFETY: The call to `Array::shift_n` will potentially invalidate the
        // raw parts stored in `ary`'s `RArray*`.
        //
        // The below call to `Array::alloc_value` will trigger an mruby heap
        // allocation which may trigger a garbage collection.
        //
        // The raw parts in `ary`'s `RArray*` must be repacked before a potential
        // garbage collection, otherwise marking the children in `ary` will have
        // undefined behavior.
        //
        // The call to `Array::alloc_value` happens outside this block after
        // the `Array` has been repacked.
        let shifted = unsafe {
            let array_mut = array.as_inner_mut();
            let shifted = array_mut.shift_n(count);

            let inner = array.take();
            Array::box_into_value(inner, ary, interp)?;

            shifted
        };

        Array::alloc_value(shifted, interp)
    } else {
        // SAFETY: The call to `Array::shift` will potentially invalidate the
        // raw parts stored in `ary`'s `RArray*`.
        //
        // The raw parts in `ary`'s `RArray *` must be repacked before a
        // potential garbage collection, otherwise marking the children in `ary`
        // will have undefined behavior.
        //
        // The call to `interp.convert` happens outside this block after the
        // `Array` has been repacked.
        let shifted = unsafe {
            let array_mut = array.as_inner_mut();
            let shifted = array_mut.shift();

            let inner = array.take();
            Array::box_into_value(inner, ary, interp)?;

            shifted
        };

        Ok(interp.convert(shifted))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::prelude::*;

    #[test]
    fn mutating_methods_may_raise_frozen_error() {
        let mut interp = interpreter();
        let mut slf = interp.eval(b"[1,2,3]").unwrap();
        slf.freeze(&mut interp).unwrap();

        let value = interp.convert(0);
        assert_eq!(
            push_single(&mut interp, slf, value).unwrap_err().to_string(),
            "FrozenError (can't modify frozen Array)",
        );

        let first = interp.convert(0);
        let second = interp.try_convert_mut(Vec::<u8>::new()).unwrap();
        assert_eq!(
            element_assignment(&mut interp, slf, first, second, None)
                .unwrap_err()
                .to_string(),
            "FrozenError (can't modify frozen Array)",
        );

        assert_eq!(
            clear(&mut interp, slf).unwrap_err().to_string(),
            "FrozenError (can't modify frozen Array)",
        );

        assert_eq!(
            push(&mut interp, slf, None).unwrap_err().to_string(),
            "FrozenError (can't modify frozen Array)",
        );

        assert_eq!(
            concat(&mut interp, slf, []).unwrap_err().to_string(),
            "FrozenError (can't modify frozen Array)",
        );

        assert_eq!(
            reverse_bang(&mut interp, slf).unwrap_err().to_string(),
            "FrozenError (can't modify frozen Array)",
        );

        assert_eq!(
            shift(&mut interp, slf, None).unwrap_err().to_string(),
            "FrozenError (can't modify frozen Array)",
        );
    }
}
