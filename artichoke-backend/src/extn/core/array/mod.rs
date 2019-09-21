use artichoke_core::value::Value as ValueLike;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::slice;

use crate::convert::{Convert, RustBackedValue};
use crate::def::Define;
use crate::eval::Eval;
use crate::extn::core::error::{IndexError, RubyException, RuntimeError, TypeError};
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

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

pub struct Array {
    buffer: VecDeque<Value>,
}

impl RustBackedValue for Array {}

// MRB_API mrb_value mrb_assoc_new(mrb_state *mrb, mrb_value car, mrb_value cdr);
pub fn assoc(interp: &Artichoke, car: Value, cdr: Value) -> Result<Value, Error> {
    let _ = interp;
    let buffer = VecDeque::from(vec![car, cdr]);
    let ary = Array { buffer };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

// MRB_API mrb_value mrb_assoc_new(mrb_state *mrb, mrb_value car, mrb_value cdr);
#[no_mangle]
pub unsafe extern "C" fn artichoke_assoc_new(
    mrb: *mut sys::mrb_state,
    car: sys::mrb_value,
    cdr: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let car = Value::new(&interp, car);
    let cdr = Value::new(&interp, cdr);
    let result = assoc(&interp, car, cdr);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
pub fn new(interp: &Artichoke) -> Result<Value, Error> {
    let _ = interp;
    let buffer = VecDeque::new();
    let ary = Array { buffer };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = new(&interp);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_new_capa(mrb_state*, mrb_int);
pub fn with_capacity(interp: &Artichoke, capacity: usize) -> Result<Value, Error> {
    let _ = interp;
    let buffer = VecDeque::with_capacity(capacity);
    let ary = Array { buffer };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

// MRB_API mrb_value mrb_ary_new_capa(mrb_state*, mrb_int);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new_capa(
    mrb: *mut sys::mrb_state,
    capacity: sys::mrb_int,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = usize::try_from(capacity)
        .map_err(|_| Error::Fatal)
        .and_then(|capacity| with_capacity(&interp, capacity));
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_new_from_values(mrb_state *mrb, mrb_int size, const mrb_value *vals);
pub fn from_values<'a>(interp: &'a Artichoke, values: &[Value]) -> Result<Value, Error<'a>> {
    let ary = Array {
        buffer: VecDeque::from(values.to_vec()),
    };
    unsafe { ary.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

// MRB_API mrb_value mrb_ary_new_from_values(mrb_state *mrb, mrb_int size, const mrb_value *vals);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new_from_values(
    mrb: *mut sys::mrb_state,
    size: sys::mrb_int,
    vals: *const sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = usize::try_from(size)
        .map_err(|_| Error::Fatal)
        .and_then(|size| {
            let values = slice::from_raw_parts(vals, size);
            from_values(
                &interp,
                values
                    .iter()
                    .map(|val| Value::new(&interp, *val))
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
        });
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_splat(mrb_state *mrb, mrb_value value);
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

// MRB_API mrb_value mrb_ary_splat(mrb_state *mrb, mrb_value value);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_splat(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = splat(&interp, value);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_clear(mrb_state *mrb, mrb_value self);
pub fn clear(interp: &Artichoke, ary: Value) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    borrow.buffer.clear();
    Ok(ary)
}

/// Stub for `mrb_ary_clear`.
///
/// `mrb_ary_clear` is part of the mruby public API, but it is only used in
/// `array.c` and exposed as an instance method, which this module is replacing.
///
/// ```c
/// MRB_API mrb_value mrb_ary_clear(mrb_state *mrb, mrb_value self);
/// ```
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_clear(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let _ = value;
    interp.convert(None::<Value>).inner()
}

// MRB_API mrb_value mrb_ary_entry(mrb_value ary, mrb_int offset);
// MRB_API mrb_value mrb_ary_ref(mrb_state *mrb, mrb_value ary, mrb_int n);
pub fn element_reference(
    interp: &Artichoke,
    ary: Value,
    offset: isize,
) -> Result<Option<Value>, Error> {
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

// MRB_API mrb_value mrb_ary_entry(mrb_value ary, mrb_int offset);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_entry(
    ary: sys::mrb_value,
    offset: sys::mrb_int,
) -> sys::mrb_value {
    let _ = ary;
    let _ = offset;
    // This API is not possible to implement without an interp instance.
    sys::mrb_sys_nil_value()
}

// MRB_API mrb_value mrb_ary_ref(mrb_state *mrb, mrb_value ary, mrb_int n);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_ref(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    offset: sys::mrb_int,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = isize::try_from(offset)
        .map_err(|_| Error::Fatal)
        .and_then(|offset| element_reference(&interp, ary, offset));
    match result {
        Ok(value) => interp.convert(value).inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_join(mrb_state *mrb, mrb_value ary, mrb_value sep);
pub fn join(interp: &Artichoke, ary: Value, separator: Value) -> Result<Value, Error> {
    let _ = interp;
    ary.funcall::<Value>("join", &[separator], None)
        .map_err(Error::Artichoke)
}

/// Stub for `mrb_ary_join`.
///
/// `mrb_ary_join` is part of the mruby public API, but it is only used in
/// `array.c` and exposed as an instance method, which this module is replacing.
///
/// ```c
/// MRB_API mrb_value mrb_ary_join(mrb_state *mrb, mrb_value ary, mrb_value sep);
/// ```
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_join(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    sep: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let _ = ary;
    let _ = sep;
    interp.convert(None::<Value>).inner()
}

// MRB_API mrb_value mrb_ary_pop(mrb_state *mrb, mrb_value ary);
pub fn pop(interp: &Artichoke, ary: Value) -> Result<Option<Value>, Error> {
    let ary = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = ary.borrow_mut();
    Ok(borrow.buffer.pop_back())
}

// MRB_API mrb_value mrb_ary_pop(mrb_state *mrb, mrb_value ary);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_pop(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = pop(&interp, ary);
    match result {
        Ok(value) => interp.convert(value).inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_resize(mrb_state *mrb, mrb_value ary, mrb_int new_len);
pub fn resize(interp: &Artichoke, ary: Value, new_len: usize) {
    // This API is an artifact of mruby being C code where the buffer in an
    // Array must be explicity managed. It is safe to make this a no-op in
    // Rust.
    let _ = interp;
    let _ = ary;
    let _ = new_len;
}

/// Stub for `mrb_ary_resize`.
///
/// `mrb_ary_resize` is part of the mruby public API, but it is a no-op for a
/// `VecDeque`-backed array.
///
/// ```c
/// MRB_API mrb_value mrb_ary_resize(mrb_state *mrb, mrb_value ary, mrb_int new_len);
/// ```
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_resize(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    new_len: sys::mrb_int,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let _ = ary;
    let _ = new_len;
    interp.convert(None::<Value>).inner()
}

// MRB_API mrb_value mrb_ary_shift(mrb_state *mrb, mrb_value self);
// MRB_API mrb_value mrb_ary_splice(mrb_state *mrb, mrb_value self, mrb_int head, mrb_int len, mrb_value rpl);
pub fn shift(interp: &Artichoke, ary: Value, count: Option<Value>) -> Result<Value, Error> {
    let _ = interp;
    let _ = ary;
    let _ = count;
    Err(Error::Fatal)
}

// MRB_API mrb_value mrb_ary_shift(mrb_state *mrb, mrb_value self);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_shift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let count = mrb_get_args!(mrb, optional = 1);
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = shift(&interp, ary, count.map(|count| Value::new(&interp, count)));
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

/// Stub for `mrb_ary_splice`.
///
/// `mrb_ary_splice` is part of the mruby public API, but it is only used in
/// `array.c`, which this module is replacing.
///
/// ```c
/// MRB_API mrb_value mrb_ary_splice(mrb_state *mrb, mrb_value self, mrb_int head, mrb_int len, mrb_value rpl);
/// ```
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_splice(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    head: sys::mrb_int,
    len: sys::mrb_int,
    replacement: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let _ = ary;
    let _ = head;
    let _ = len;
    let _ = replacement;
    interp.convert(None::<Value>).inner()
}

// MRB_API mrb_value mrb_ary_unshift(mrb_state *mrb, mrb_value self, mrb_value item);
pub fn unshift(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = ary;
    let _ = value;
    Err(Error::Fatal)
}

// MRB_API mrb_value mrb_ary_unshift(mrb_state *mrb, mrb_value self, mrb_value item);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_unshift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    item: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let item = Value::new(&interp, item);
    let result = unshift(&interp, ary, item);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API void mrb_ary_concat(mrb_state *mrb, mrb_value self, mrb_value other);
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

// MRB_API void mrb_ary_concat(mrb_state *mrb, mrb_value self, mrb_value other);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_concat(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    other: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let other = Value::new(&interp, other);
    let result = concat(&interp, ary, other);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API void mrb_ary_push(mrb_state *mrb, mrb_value array, mrb_value value);
pub fn push(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    borrow.buffer.push_back(value);
    Ok(ary)
}

// MRB_API void mrb_ary_push(mrb_state *mrb, mrb_value array, mrb_value value);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_push(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let value = Value::new(&interp, value);
    let result = push(&interp, ary, value);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API void mrb_ary_replace(mrb_state *mrb, mrb_value self, mrb_value other);
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

// MRB_API void mrb_ary_replace(mrb_state *mrb, mrb_value self, mrb_value other);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_replace(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    other: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let other = Value::new(&interp, other);
    let result = replace(&interp, ary, other);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

// MRB_API void mrb_ary_set(mrb_state *mrb, mrb_value ary, mrb_int n, mrb_value val);
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

// MRB_API void mrb_ary_set(mrb_state *mrb, mrb_value ary, mrb_int n, mrb_value val);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_set(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    offset: sys::mrb_int,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let value = Value::new(&interp, value);
    let result = isize::try_from(offset)
        .map_err(|_| Error::Fatal)
        .and_then(|offset| element_set(&interp, ary, offset, value));
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_clone(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let _ = value;
    interp.convert(None::<Value>).inner()
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_value_to_ary(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let _ = value;
    interp.convert(None::<Value>).inner()
}

pub fn len(interp: &Artichoke, ary: Value) -> Result<usize, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let borrow = array.borrow();
    Ok(borrow.buffer.len())
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_len(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_int {
    let interp = match crate::ffi::from_user_data(mrb) {
        Ok(interp) => interp,
        Err(_) => return 0,
    };
    let ary = Value::new(&interp, ary);
    let result =
        len(&interp, ary).and_then(|len| sys::mrb_int::try_from(len).map_err(|_| Error::Fatal));
    match result {
        Ok(len) => len,
        Err(Error::Artichoke(_)) => {
            RuntimeError::raise(interp, "artichoke error");
            0
        }
        Err(Error::Fatal) => {
            RuntimeError::raise(interp, "fatal Array error");
            0
        }
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args);
            0
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion from %S to %S", format_args);
            0
        }
    }
}
