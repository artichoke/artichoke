use artichoke_core::value::Value as ValueLike;
use std::collections::VecDeque;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::def::Define;
use crate::eval::Eval;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

mod mruby;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let array = interp
        .0
        .borrow_mut()
        .def_class::<Array>("Array", None, None);
    array.borrow().define(interp)?;
    interp.eval(include_str!("array.rb"))?;
    Ok(())
}

pub enum Error<'a> {
    Artichoke(ArtichokeError),
    Fatal,
    IndexTooSmall { index: isize, minimum: isize },
    NoImplicitConversion { from: &'a str, to: &'a str },
}

#[derive(Debug, Clone)]
pub struct Array {
    buffer: VecDeque<Value>,
}

impl RustBackedValue for Array {}

#[allow(clippy::similar_names)]
pub fn assoc(interp: &Artichoke, car: Value, cdr: Value) -> Result<Value, Error> {
    let _ = interp;
    let buffer = VecDeque::from(vec![car, cdr]);
    let ary = Array { buffer };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

pub fn new(interp: &Artichoke) -> Result<Value, Error> {
    let _ = interp;
    let buffer = VecDeque::new();
    let ary = Array { buffer };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

pub fn with_capacity(interp: &Artichoke, capacity: usize) -> Result<Value, Error> {
    let _ = interp;
    let buffer = VecDeque::with_capacity(capacity);
    let ary = Array { buffer };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

pub fn from_values<'a>(interp: &'a Artichoke, values: &[Value]) -> Result<Value, Error<'a>> {
    let ary = Array {
        buffer: VecDeque::from(values.to_vec()),
    };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

pub fn splat(interp: &Artichoke, value: Value) -> Result<Value, Error> {
    let buffer = if value.respond_to("to_a").map_err(Error::Artichoke)? {
        value
            .funcall::<Vec<Value>>("to_a", &[], None)
            .map_err(Error::Artichoke)?
    } else {
        vec![value]
    };
    let ary = Array {
        buffer: VecDeque::from(buffer),
    };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

pub fn clear(interp: &Artichoke, ary: Value) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    borrow.buffer.clear();
    Ok(ary)
}

pub fn element_reference<'a>(
    interp: &'a Artichoke,
    ary: &Value,
    offset: isize,
) -> Result<Option<Value>, Error<'a>> {
    let ary = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let borrow = ary.borrow();
    let offset = if offset >= 0 {
        usize::try_from(offset).map_err(|_| Error::Fatal)?
    } else {
        let wrapped_offset = usize::try_from(-offset).map_err(|_| Error::Fatal)?;
        let wrapped_offset = borrow.buffer.len().checked_sub(wrapped_offset);
        if let Some(offset) = wrapped_offset {
            offset
        } else {
            let minimum = isize::try_from(borrow.buffer.len())
                .ok()
                .and_then(|min| min.checked_mul(-1))
                .ok_or(Error::Fatal)?;
            return Err(Error::IndexTooSmall {
                index: offset,
                minimum,
            });
        }
    };
    Ok(borrow.buffer.get(offset).cloned())
}

pub fn pop<'a>(interp: &'a Artichoke, ary: &Value) -> Result<Option<Value>, Error<'a>> {
    let ary = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = ary.borrow_mut();
    Ok(borrow.buffer.pop_back())
}

pub fn unshift(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = ary;
    let _ = value;
    Err(Error::Fatal)
}

pub fn concat(interp: &Artichoke, ary: Value, other: Value) -> Result<Value, Error> {
    let ary_type = ary.pretty_name();
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    let ruby_type = other.pretty_name();
    if let Ok(other) = other.try_into::<Vec<Value>>() {
        borrow.buffer.extend(other);
        Ok(ary)
    } else {
        Err(Error::NoImplicitConversion {
            from: ruby_type,
            to: ary_type,
        })
    }
}

pub fn push(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    borrow.buffer.push_back(value);
    Ok(ary)
}

pub fn replace(interp: &Artichoke, ary: Value, other: Value) -> Result<Value, Error> {
    let ary_type = ary.pretty_name();
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    let ruby_type = other.pretty_name();
    if let Ok(other) = other.try_into::<Vec<Value>>() {
        borrow.buffer = VecDeque::from(other);
        Ok(ary)
    } else {
        Err(Error::NoImplicitConversion {
            from: ruby_type,
            to: ary_type,
        })
    }
}

pub fn element_set(
    interp: &Artichoke,
    ary: Value,
    offset: isize,
    value: Value,
) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    let offset = if offset >= 0 {
        usize::try_from(offset).map_err(|_| Error::Fatal)?
    } else {
        let wrapped_offset = usize::try_from(-offset).map_err(|_| Error::Fatal)?;
        let wrapped_offset = borrow.buffer.len().checked_sub(wrapped_offset);
        if let Some(offset) = wrapped_offset {
            offset
        } else {
            let minimum = isize::try_from(borrow.buffer.len())
                .ok()
                .and_then(|min| min.checked_mul(-1))
                .ok_or(Error::Fatal)?;
            return Err(Error::IndexTooSmall {
                index: offset,
                minimum,
            });
        }
    };
    let fill = offset.checked_sub(borrow.buffer.len()).unwrap_or_default();
    for _ in 0..fill {
        borrow.buffer.push_back(interp.convert(None::<Value>));
    }
    borrow.buffer.insert(offset, value);
    Ok(ary)
}

pub fn len<'a>(interp: &'a Artichoke, ary: &Value) -> Result<usize, Error<'a>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let borrow = array.borrow();
    Ok(borrow.buffer.len())
}

pub fn clone<'a>(interp: &'a Artichoke, ary: &Value) -> Result<Value, Error<'a>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let borrow = array.borrow();
    let clone = borrow.clone();
    unsafe { clone.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

pub fn to_ary(interp: &Artichoke, ary: Value) -> Result<Value, Error> {
    if unsafe { Array::try_from_ruby(interp, &ary) }.is_ok() {
        Ok(ary)
    } else {
        from_values(interp, &[ary])
    }
}
