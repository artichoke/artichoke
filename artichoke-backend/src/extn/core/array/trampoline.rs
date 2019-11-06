use artichoke_core::value::Value as _;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::array::{backend, Array};
use crate::extn::core::exception::{Fatal, FrozenError, IndexError, RubyException, TypeError};
use crate::gc::MrbGarbageCollection;
use crate::value::{Block, Value};
use crate::Artichoke;

#[allow(clippy::similar_names)]
pub fn assoc(interp: &Artichoke, car: Value, cdr: Value) -> Result<Value, Box<dyn RubyException>> {
    let result = backend::fixed::two(car, cdr);
    let result = Array(result);
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn new(interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
    let result = backend::fixed::empty();
    let result = Array(result);
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn with_capacity(interp: &Artichoke, capacity: usize) -> Result<Value, Box<dyn RubyException>> {
    let result = if capacity == 0 {
        backend::fixed::empty()
    } else {
        let buffer = backend::buffer::Buffer::with_capacity(capacity);
        Box::new(buffer)
    };
    let result = Array(result);
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn from_values<'a>(
    interp: &'a Artichoke,
    values: &[Value],
) -> Result<Value, Box<dyn RubyException>> {
    let result = if values.is_empty() {
        backend::fixed::empty()
    } else if values.len() == 1 {
        backend::fixed::one(values[0].clone())
    } else if values.len() == 2 {
        backend::fixed::two(values[0].clone(), values[1].clone())
    } else {
        let buffer = backend::buffer::Buffer::from(values);
        Box::new(buffer)
    };
    let result = Array(result);
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn splat(interp: &Artichoke, value: Value) -> Result<Value, Box<dyn RubyException>> {
    if unsafe { Array::try_from_ruby(interp, &value) }.is_ok() {
        return Ok(value);
    }
    if value
        .respond_to(interp, "to_a")
        .map_err(|_| Fatal::new(interp, "Error calling #respond_to?(:to_a)"))?
    {
        let value_type = value.pretty_name(interp);
        let value = value
            .funcall::<Value>(interp, "to_a", &[], None)
            // TODO: propagate exceptions thrown by `value#to_a`.
            .map_err(|_| Fatal::new(interp, "Error calling #to_a even though it exists"))?;
        if unsafe { Array::try_from_ruby(interp, &value) }.is_ok() {
            Ok(value)
        } else if value.is_nil() {
            let result = backend::fixed::one(value);
            let result = Array(result);
            let result = unsafe { result.try_into_ruby(interp, None) }.map_err(|_| {
                Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array")
            })?;
            Ok(result)
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!(
                    "can't convert {classname} to Array ({classname}#to_a gives {gives})",
                    classname = value_type,
                    gives = value.pretty_name(interp)
                ),
            )))
        }
    } else {
        let result = backend::fixed::one(value);
        let result = Array(result);
        let result = unsafe { result.try_into_ruby(interp, None) }
            .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
        Ok(result)
    }
}

pub fn to_ary(interp: &Artichoke, value: Value) -> Result<Value, Box<dyn RubyException>> {
    if unsafe { Array::try_from_ruby(interp, &value) }.is_ok() {
        Ok(value)
    } else if let Ok(ary) = value.funcall::<Value>(interp, "to_a", &[], None) {
        let ruby_type = ary.pretty_name(interp);
        if unsafe { Array::try_from_ruby(interp, &ary) }.is_ok() {
            Ok(ary)
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!(
                    "can't convert {classname} to Array ({classname}#to_a gives {gives})",
                    classname = value.pretty_name(interp),
                    gives = ruby_type
                ),
            )))
        }
    } else {
        let result = backend::fixed::one(value);
        let result = Array(result);
        let result = unsafe { result.try_into_ruby(interp, None) }
            .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
        Ok(result)
    }
}

pub fn ary_ref(
    interp: &Artichoke,
    ary: Value,
    offset: isize,
) -> Result<Option<Value>, Box<dyn RubyException>> {
    let ary = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let offset =
        usize::try_from(offset).map_err(|_| Fatal::new(interp, "Offset does not fit in usize"))?;
    let result = ary.borrow().0.get(interp, offset)?;
    Ok(interp.convert(result))
}

pub fn clear(interp: &Artichoke, ary: Value) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let mut borrow = array.borrow_mut();
    borrow.clear();
    Ok(ary)
}

pub fn element_reference(
    interp: &Artichoke,
    ary: Value,
    first: Value,
    second: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = array.borrow();
    borrow.element_reference(interp, first, second)
}

pub fn element_assignment(
    interp: &Artichoke,
    ary: Value,
    first: Value,
    second: Value,
    third: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    // TODO: properly handle self-referential sets.
    if ary == first || ary == second || Some(ary) == third {
        return Ok(interp.convert(None::<Value>));
    }
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    let result = borrow.element_assignment(interp, first, second, third);
    if gc_was_enabled {
        interp.enable_gc();
    }
    result
}

pub fn pop(interp: &Artichoke, ary: Value) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    let result = borrow.pop(interp);
    if gc_was_enabled {
        interp.enable_gc();
    }
    result
}

pub fn shift(
    interp: &Artichoke,
    ary: Value,
    count: Option<usize>,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    if let Some(count) = count {
        let popped = {
            let mut borrow = array.borrow_mut();
            let gc_was_enabled = interp.disable_gc();
            let result = borrow.slice(interp, 0, count)?;
            borrow.set_slice(interp, 0, count, backend::fixed::empty())?;
            if gc_was_enabled {
                interp.enable_gc();
            }
            result
        };
        let popped = Array(popped);
        let result = unsafe { popped.try_into_ruby(interp, None) }
            .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
        Ok(result)
    } else {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.pop(interp);
        if gc_was_enabled {
            interp.enable_gc();
        }
        result
    }
}

pub fn unshift(
    interp: &Artichoke,
    ary: Value,
    value: Value,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    borrow.set_with_drain(interp, 0, 0, value)?;
    if gc_was_enabled {
        interp.enable_gc();
    }
    Ok(ary)
}

pub fn concat(
    interp: &Artichoke,
    ary: Value,
    other: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    if let Some(other) = other {
        let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
            Fatal::new(
                interp,
                "Unable to extract Rust Array from Ruby Array receiver",
            )
        })?;
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        borrow.concat(interp, other)?;
        if gc_was_enabled {
            interp.enable_gc();
        }
    }
    Ok(ary)
}

pub fn push(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let idx = array.borrow().len_usize();
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    borrow.set(interp, idx, value)?;
    if gc_was_enabled {
        interp.enable_gc();
    }
    Ok(ary)
}

pub fn reverse(interp: &Artichoke, ary: Value) -> Result<Value, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = array.borrow();
    borrow.reverse(interp)
}

pub fn reverse_bang(interp: &Artichoke, ary: Value) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    borrow.reverse_in_place(interp)?;
    if gc_was_enabled {
        interp.enable_gc();
    }
    Ok(ary)
}

pub fn element_set(
    interp: &Artichoke,
    ary: Value,
    offset: isize,
    value: Value,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen(interp) {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let offset = if offset >= 0 {
        usize::try_from(offset)
            .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?
    } else {
        let len = array.borrow().len_usize();
        // Positive Int must be usize
        let idx = usize::try_from(-offset)
            .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?;
        if let Some(offset) = len.checked_sub(idx) {
            offset
        } else {
            return Err(Box::new(IndexError::new(
                interp,
                format!("index {} too small for array; minimum: {}", offset, len),
            )));
        }
    };
    // TODO: properly handle self-referential sets.
    if ary == value {
        return Ok(interp.convert(None::<Value>));
    }
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    borrow.set(interp, offset, value)?;
    if gc_was_enabled {
        interp.enable_gc();
    }
    Ok(ary)
}

pub fn len(interp: &Artichoke, ary: Value) -> Result<usize, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = array.borrow();
    Ok(borrow.len_usize())
}

pub fn clone(interp: &Artichoke, ary: Value) -> Result<Value, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = array.borrow();
    let result = borrow.clone();
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn initialize(
    interp: &Artichoke,
    ary: Value,
    first: Option<Value>,
    second: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Box<dyn RubyException>> {
    Array::initialize(interp, first, second, block, ary)
}

pub fn initialize_copy(
    interp: &Artichoke,
    ary: Value,
    from: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let from = unsafe { Array::try_from_ruby(interp, &from) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = from.borrow();
    let result = borrow.clone();
    let result = unsafe { result.try_into_ruby(interp, Some(ary.inner())) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}
