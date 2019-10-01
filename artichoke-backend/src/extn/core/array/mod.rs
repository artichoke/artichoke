use artichoke_core::value::Value as ValueLike;
use std::collections::VecDeque;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::Eval;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

mod mruby;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let array =
        interp
            .0
            .borrow_mut()
            .def_class::<Array>("Array", None, Some(rust_data_free::<Array>));
    array.borrow_mut().mrb_value_is_rust_backed(true);

    /*
    mrb_define_method(mrb, a, "+",               mrb_ary_plus,         MRB_ARGS_REQ(1));   /* 15.2.12.5.1  */
    mrb_define_method(mrb, a, "*",               mrb_ary_times,        MRB_ARGS_REQ(1));   /* 15.2.12.5.2  */
    mrb_define_method(mrb, a, "<<",              mrb_ary_push_m,       MRB_ARGS_REQ(1));   /* 15.2.12.5.3  */
    mrb_define_method(mrb, a, "[]",              mrb_ary_aget,         MRB_ARGS_ARG(1,1)); /* 15.2.12.5.4  */
    mrb_define_method(mrb, a, "[]=",             mrb_ary_aset,         MRB_ARGS_ARG(2,1)); /* 15.2.12.5.5  */
    mrb_define_method(mrb, a, "clear",           mrb_ary_clear_m,      MRB_ARGS_NONE());   /* 15.2.12.5.6  */
    mrb_define_method(mrb, a, "concat",          mrb_ary_concat_m,     MRB_ARGS_REQ(1));   /* 15.2.12.5.8  */
    mrb_define_method(mrb, a, "delete_at",       mrb_ary_delete_at,    MRB_ARGS_REQ(1));   /* 15.2.12.5.9  */
    mrb_define_method(mrb, a, "empty?",          mrb_ary_empty_p,      MRB_ARGS_NONE());   /* 15.2.12.5.12 */
    mrb_define_method(mrb, a, "first",           mrb_ary_first,        MRB_ARGS_OPT(1));   /* 15.2.12.5.13 */
    mrb_define_method(mrb, a, "index",           mrb_ary_index_m,      MRB_ARGS_REQ(1));   /* 15.2.12.5.14 */
    mrb_define_method(mrb, a, "initialize_copy", mrb_ary_replace_m,    MRB_ARGS_REQ(1));   /* 15.2.12.5.16 */
    mrb_define_method(mrb, a, "join",            mrb_ary_join_m,       MRB_ARGS_OPT(1));   /* 15.2.12.5.17 */
    mrb_define_method(mrb, a, "last",            mrb_ary_last,         MRB_ARGS_OPT(1));   /* 15.2.12.5.18 */
    mrb_define_method(mrb, a, "length",          mrb_ary_size,         MRB_ARGS_NONE());   /* 15.2.12.5.19 */
    mrb_define_method(mrb, a, "pop",             mrb_ary_pop,          MRB_ARGS_NONE());   /* 15.2.12.5.21 */
    mrb_define_method(mrb, a, "push",            mrb_ary_push_m,       MRB_ARGS_ANY());    /* 15.2.12.5.22 */
    mrb_define_method(mrb, a, "replace",         mrb_ary_replace_m,    MRB_ARGS_REQ(1));   /* 15.2.12.5.23 */
    mrb_define_method(mrb, a, "reverse",         mrb_ary_reverse,      MRB_ARGS_NONE());   /* 15.2.12.5.24 */
    mrb_define_method(mrb, a, "reverse!",        mrb_ary_reverse_bang, MRB_ARGS_NONE());   /* 15.2.12.5.25 */
    mrb_define_method(mrb, a, "rindex",          mrb_ary_rindex_m,     MRB_ARGS_REQ(1));   /* 15.2.12.5.26 */
    mrb_define_method(mrb, a, "shift",           mrb_ary_shift,        MRB_ARGS_NONE());   /* 15.2.12.5.27 */
    mrb_define_method(mrb, a, "size",            mrb_ary_size,         MRB_ARGS_NONE());   /* 15.2.12.5.28 */
    mrb_define_method(mrb, a, "slice",           mrb_ary_aget,         MRB_ARGS_ARG(1,1)); /* 15.2.12.5.29 */
    mrb_define_method(mrb, a, "unshift",         mrb_ary_unshift_m,    MRB_ARGS_ANY());    /* 15.2.12.5.30 */
    */
    array.borrow_mut().add_method(
        "[]",
        mruby::ary_element_reference,
        sys::mrb_args_req_and_opt(1, 1),
    );
    array.borrow_mut().add_method(
        "[]=",
        mruby::ary_element_assignment,
        sys::mrb_args_req_and_opt(2, 1),
    );
    array
        .borrow_mut()
        .add_method("concat", mruby::ary_concat, sys::mrb_args_any());
    array.borrow_mut().add_method(
        "initialize_copy",
        mruby::ary_initialize_copy,
        sys::mrb_args_req(1),
    );
    array
        .borrow_mut()
        .add_method("length", mruby::ary_len, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("pop", mruby::artichoke_ary_pop, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("reverse", mruby::ary_reverse, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("reverse!", mruby::ary_reverse_bang, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("size", mruby::ary_len, sys::mrb_args_none());
    array.borrow().define(interp)?;
    interp.eval(include_str!("array.rb"))?;
    Ok(())
}

#[derive(Debug)]
pub enum Error<'a> {
    Artichoke(ArtichokeError),
    CannotConvert {
        to: &'a str,
        from: &'a str,
        method: &'a str,
        gives: &'a str,
    },
    Fatal,
    IndexTooSmall {
        index: isize,
        minimum: isize,
    },
    NoImplicitConversion {
        from: &'a str,
        to: &'a str,
    },
    RangeError {
        min: isize,
        max: isize,
        exclusive: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Array {
    pub buffer: VecDeque<Value>,
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

pub fn ary_ref<'a>(
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

#[derive(Debug, Clone, Copy)]
pub enum ElementReferenceArgs {
    Index(Int),
    StartLen(Int, usize),
}

pub fn element_reference<'a>(
    interp: &'a Artichoke,
    ary: &Value,
    args: ElementReferenceArgs,
) -> Result<Value, Error<'a>> {
    let data = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    match args {
        ElementReferenceArgs::Index(index) => {
            if index < 0 {
                // Positive Int must be usize
                let index = usize::try_from(-index).map_err(|_| Error::Fatal)?;
                match borrow.buffer.len().checked_sub(index) {
                    Some(0) | None => Ok(interp.convert(None::<Value>)),
                    Some(index) => Ok(interp.convert(borrow.buffer.get(index))),
                }
            } else {
                // Positive Int must be usize
                let index = usize::try_from(index).map_err(|_| Error::Fatal)?;
                Ok(interp.convert(borrow.buffer.get(index)))
            }
        }
        ElementReferenceArgs::StartLen(start, len) => {
            let start = if start < 0 {
                // Positive i64 must be usize
                let start = usize::try_from(-start).map_err(|_| Error::Fatal)?;
                borrow.buffer.len().checked_sub(start).ok_or(Error::Fatal)?
            } else {
                // Positive i64 must be usize
                usize::try_from(start).map_err(|_| Error::Fatal)?
            };
            let mut slice = Vec::with_capacity(len);
            for index in start..(start + len) {
                slice.push(interp.convert(borrow.buffer.get(index)));
            }
            from_values(&interp, slice.as_slice())
        }
    }
}

pub fn element_assignment<'a>(
    interp: &'a Artichoke,
    ary: &Value,
    args: ElementReferenceArgs,
    other: Value,
) -> Result<Value, Error<'a>> {
    let data = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = data.borrow_mut();
    match args {
        ElementReferenceArgs::Index(index) => {
            let index = if index < 0 {
                // Positive Int must be usize
                usize::try_from(-index).map_err(|_| Error::Fatal)?
            } else {
                // Positive Int must be usize
                usize::try_from(index).map_err(|_| Error::Fatal)?
            };
            let len = borrow.buffer.len();
            if index > len {
                for _ in len..index {
                    borrow.buffer.push_back(interp.convert(None::<Value>));
                }
                borrow.buffer.push_back(other.clone());
            } else {
                borrow.buffer.insert(index, other.clone());
            };
        }
        ElementReferenceArgs::StartLen(start, len) => {
            let other_ary = to_ary(interp, other.clone())?;
            let other_data =
                unsafe { Array::try_from_ruby(interp, &other_ary) }.map_err(|_| Error::Fatal)?;
            let other_borrow = other_data.borrow();

            let start = if start < 0 {
                // Positive i64 must be usize
                let start = usize::try_from(-start).map_err(|_| Error::Fatal)?;
                borrow.buffer.len().checked_sub(start).ok_or(Error::Fatal)?
            } else {
                // Positive i64 must be usize
                usize::try_from(start).map_err(|_| Error::Fatal)?
            };
            let buf_len = borrow.buffer.len();
            let other_len = other_borrow.buffer.len();
            if start > buf_len {
                for _ in buf_len..start {
                    borrow.buffer.push_back(interp.convert(None::<Value>));
                }
                borrow.buffer.extend(other_borrow.buffer.clone());
                if start + len > other_len {
                    for _ in (start + len)..other_len {
                        borrow.buffer.push_back(interp.convert(None::<Value>));
                    }
                }
            } else if other_len < len {
                for index in start..other_len {
                    let idx = start + index;
                    if start + idx >= borrow.buffer.len() {
                        borrow.buffer.push_back(other_borrow.buffer[index].clone());
                    } else {
                        borrow.buffer[start + index] = other_borrow.buffer[index].clone();
                    }
                }
                for _ in other_len..start + len {
                    borrow.buffer.remove(len);
                }
            };
        }
    }
    Ok(other)
}

pub fn pop<'a>(interp: &'a Artichoke, ary: &Value) -> Result<Option<Value>, Error<'a>> {
    let ary = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = ary.borrow_mut();
    Ok(borrow.buffer.pop_back())
}

pub fn shift<'a>(interp: &'a Artichoke, ary: &Value, count: usize) -> Result<Value, Error<'a>> {
    let ary = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = ary.borrow_mut();
    let mut popped = VecDeque::with_capacity(count);
    for _ in 0..count {
        let item = borrow.buffer.pop_front();
        if item.is_none() {
            break;
        }
        popped.push_back(interp.convert(item));
    }
    let popped = Array { buffer: popped };
    unsafe { popped.try_into_ruby(interp, None) }.map_err(|_| Error::Fatal)
}

pub fn unshift(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    borrow.buffer.push_front(value);
    Ok(ary)
}

pub fn concat(interp: &Artichoke, ary: Value, other: Value) -> Result<Value, Error> {
    let ary_type = ary.pretty_name();
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    let ruby_type = other.pretty_name();
    if let Ok(other) = unsafe { Array::try_from_ruby(interp, &other) } {
        borrow.buffer.extend(other.borrow().buffer.clone());
        Ok(ary)
    } else if let Ok(other) = other.funcall("to_a", &[], None) {
        if let Ok(other) = unsafe { Array::try_from_ruby(interp, &other) } {
            borrow.buffer.extend(other.borrow().buffer.clone());
            Ok(ary)
        } else {
            Err(Error::CannotConvert {
                to: "Array",
                from: ruby_type,
                method: "to_ary",
                gives: other.pretty_name(),
            })
        }
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

pub fn reverse(interp: &Artichoke, ary: Value) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let borrow = array.borrow();
    let len = borrow.buffer.len();
    let mut buffer = VecDeque::with_capacity(borrow.buffer.len());
    for offset in 1..=len {
        buffer.push_back(borrow.buffer[len - offset].clone());
    }
    let result = Array { buffer };
    unsafe { result.try_into_ruby(interp, None) }.map_err(Error::Artichoke)
}

pub fn reverse_bang(interp: &Artichoke, ary: Value) -> Result<Value, Error> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| Error::Fatal)?;
    let mut borrow = array.borrow_mut();
    let mut front = 0;
    let mut back = borrow.buffer.len() - 1;
    while front < back {
        borrow.buffer.swap(front, back);
        front += 1;
        back -= 1;
    }
    Ok(ary)
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

pub fn initialize_copy<'a>(
    interp: &'a Artichoke,
    ary: Value,
    other: &Value,
) -> Result<Value, Error<'a>> {
    let other = unsafe { Array::try_from_ruby(interp, other) }.map_err(|_| Error::Fatal)?;
    let clone = Array {
        buffer: other.borrow().buffer.clone(),
    };
    unsafe { clone.try_into_ruby(interp, Some(ary.inner())) }.map_err(Error::Artichoke)
}

pub fn to_ary(interp: &Artichoke, ary: Value) -> Result<Value, Error> {
    if unsafe { Array::try_from_ruby(interp, &ary) }.is_ok() {
        Ok(ary)
    } else {
        from_values(interp, &[ary])
    }
}
