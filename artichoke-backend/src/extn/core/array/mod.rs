use artichoke_core::value::Value as ValueLike;
use std::cmp;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::mem;

use crate::convert::{Convert, RustBackedValue};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::exception::{
    Fatal, FrozenError, IndexError, RangeError, RubyException, TypeError,
};
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub mod mruby;

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

#[derive(Debug, Clone)]
pub struct Array {
    pub buffer: VecDeque<Value>,
}

impl RustBackedValue for Array {}

#[allow(clippy::similar_names)]
pub fn assoc(interp: &Artichoke, car: Value, cdr: Value) -> Result<Value, Box<dyn RubyException>> {
    let _ = interp;
    let buffer = VecDeque::from(vec![car, cdr]);
    let result = Array { buffer };
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn new(interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
    let _ = interp;
    let buffer = VecDeque::new();
    let result = Array { buffer };
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn with_capacity(interp: &Artichoke, capacity: usize) -> Result<Value, Box<dyn RubyException>> {
    let _ = interp;
    let buffer = VecDeque::with_capacity(capacity);
    let result = Array { buffer };
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn from_values<'a>(
    interp: &'a Artichoke,
    values: &[Value],
) -> Result<Value, Box<dyn RubyException>> {
    let result = Array {
        buffer: VecDeque::from(values.to_vec()),
    };
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn splat(interp: &Artichoke, value: Value) -> Result<Value, Box<dyn RubyException>> {
    if unsafe { Array::try_from_ruby(interp, &value) }.is_ok() {
        return Ok(value);
    }
    if value
        .respond_to("to_a")
        .map_err(|_| Fatal::new(interp, "Error calling #respond_to?(:to_a)"))?
    {
        let value_type = value.pretty_name();
        let value = value
            .funcall::<Value>("to_a", &[], None)
            // TODO: propagate exceptions thrown by `value#to_a`.
            .map_err(|_| Fatal::new(interp, "Error calling #to_a even though it exists"))?;
        if unsafe { Array::try_from_ruby(interp, &value) }.is_ok() {
            Ok(value)
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!(
                    "can't convert {classname} to Array ({classname}#to_a gives {gives})",
                    classname = value_type,
                    gives = value.pretty_name()
                ),
            )))
        }
    } else {
        let buffer = vec![value];
        let result = Array {
            buffer: VecDeque::from(buffer),
        };
        let result = unsafe { result.try_into_ruby(interp, None) }
            .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
        Ok(result)
    }
}

pub fn clear(interp: &Artichoke, ary: Value) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
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
    borrow.buffer.clear();
    Ok(ary)
}

pub fn ary_ref<'a>(
    interp: &'a Artichoke,
    ary: &Value,
    offset: isize,
) -> Result<Option<Value>, Box<dyn RubyException>> {
    let ary = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = ary.borrow();
    let offset = if offset >= 0 {
        usize::try_from(offset)
            .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?
    } else {
        // Positive Int must be usize
        let idx = usize::try_from(-offset)
            .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?;
        if let Some(offset) = borrow.buffer.len().checked_sub(idx) {
            offset
        } else {
            return Err(Box::new(IndexError::new(
                interp,
                format!(
                    "index {} too small for array; minimum: {}",
                    offset,
                    borrow.buffer.len()
                ),
            )));
        }
    };
    Ok(borrow.buffer.get(offset).cloned())
}

#[derive(Debug, Clone, Copy)]
pub enum ElementReferenceArgs {
    Empty,
    Index(Int),
    StartLen(Int, usize),
}

impl ElementReferenceArgs {
    pub fn extract(
        interp: &Artichoke,
        elem: Value,
        len: Option<Value>,
        ary_len: usize,
    ) -> Result<Self, Box<dyn RubyException>> {
        if let Some(len) = len {
            let start = if let Ok(start) = elem.clone().try_into::<Int>() {
                start
            } else if let Ok(start) = elem.funcall::<Int>("to_int", &[], None) {
                start
            } else {
                let elem_type_name = elem.pretty_name();
                return Err(Box::new(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", elem_type_name),
                )));
            };
            let len = if let Ok(len) = len.clone().try_into::<Int>() {
                len
            } else if let Ok(len) = len.funcall::<Int>("to_int", &[], None) {
                len
            } else {
                let len_type_name = len.pretty_name();
                return Err(Box::new(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", len_type_name),
                )));
            };
            if let Ok(len) = usize::try_from(len) {
                Ok(Self::StartLen(start, len))
            } else {
                Ok(Self::Empty)
            }
        } else {
            let name = elem.pretty_name();
            if let Ok(index) = elem.clone().try_into::<Int>() {
                Ok(Self::Index(index))
            } else if let Ok(index) = elem.funcall::<Int>("to_int", &[], None) {
                Ok(Self::Index(index))
            } else {
                let rangelen = Int::try_from(ary_len)
                    .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
                match unsafe { is_range(interp, &elem, rangelen) } {
                    Ok(Some((start, len))) => Ok(Self::StartLen(start, len)),
                    Ok(None) => Ok(Self::Empty),
                    Err(_) => Err(Box::new(TypeError::new(
                        interp,
                        format!("no implicit conversion of {} into Integer", name),
                    ))),
                }
            }
        }
    }
}

pub fn element_reference<'a>(
    interp: &'a Artichoke,
    args: ElementReferenceArgs,
    ary: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = array.borrow();
    match args {
        ElementReferenceArgs::Empty => Ok(interp.convert(None::<Value>)),
        ElementReferenceArgs::Index(index) => {
            let buf_len = borrow.buffer.len();
            if index < 0 {
                // Positive Int must be usize
                let idx = usize::try_from(-index).map_err(|_| {
                    Fatal::new(interp, "Expected positive index to convert to usize")
                })?;
                if let Some(index) = buf_len.checked_sub(idx) {
                    Ok(interp.convert(borrow.buffer.get(index)))
                } else {
                    Ok(interp.convert(None::<Value>))
                }
            } else {
                let idx = usize::try_from(index).map_err(|_| {
                    Fatal::new(interp, "Expected positive index to convert to usize")
                })?;
                Ok(interp.convert(borrow.buffer.get(idx)))
            }
        }
        ElementReferenceArgs::StartLen(start, len) => {
            let buf_len = borrow.buffer.len();
            let start = if start < 0 {
                // Positive Int must be usize
                let idx = usize::try_from(-start).map_err(|_| {
                    Fatal::new(interp, "Expected positive index to convert to usize")
                })?;
                if let Some(index) = buf_len.checked_sub(idx) {
                    index
                } else {
                    return Ok(interp.convert(None::<Value>));
                }
            } else {
                let idx = usize::try_from(start).map_err(|_| {
                    Fatal::new(interp, "Expected positive index to convert to usize")
                })?;
                if idx > buf_len {
                    return Ok(interp.convert(None::<Value>));
                }
                idx
            };
            let mut result = VecDeque::with_capacity(len);
            for index in start..cmp::min(start + len, buf_len) {
                result.push_back(interp.convert(borrow.buffer.get(index)));
            }
            let result = Array { buffer: result };
            let result = unsafe { result.try_into_ruby(interp, None) }.map_err(|_| {
                Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array")
            })?;
            Ok(result)
        }
    }
}

#[allow(clippy::cognitive_complexity)]
pub fn element_assignment(
    interp: &Artichoke,
    ary: Value,
    first: Value,
    second: Value,
    third: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
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
    let (start, len, elem) = if let Some(elem) = third {
        let start = first;
        let start_type_name = start.pretty_name();
        let start = if let Ok(start) = start.clone().try_into::<Int>() {
            start
        } else if let Ok(start) = start.funcall::<Int>("to_int", &[], None) {
            start
        } else {
            return Err(Box::new(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", start_type_name),
            )));
        };
        let start = if let Ok(start) = usize::try_from(start) {
            start
        } else {
            let start = usize::try_from(start)
                .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
            if start < array.borrow().buffer.len() {
                array.borrow().buffer.len() - start
            } else {
                return Err(Box::new(IndexError::new(
                    interp,
                    format!(
                        "index {} too small for array; minimum: -{}",
                        start,
                        array.borrow().buffer.len()
                    ),
                )));
            }
        };
        let len = second;
        let len_type_name = len.pretty_name();
        let len = if let Ok(len) = len.clone().try_into::<Int>() {
            len
        } else if let Ok(len) = len.funcall::<Int>("to_int", &[], None) {
            len
        } else {
            return Err(Box::new(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", len_type_name),
            )));
        };
        if let Ok(len) = usize::try_from(len) {
            (start, Some(len), elem)
        } else {
            return Err(Box::new(IndexError::new(
                interp,
                format!("negative length ({})", len),
            )));
        }
    } else if let Ok(index) = first.clone().try_into::<Int>() {
        if let Ok(index) = usize::try_from(index) {
            (index, None, second)
        } else {
            return Err(Box::new(IndexError::new(
                interp,
                format!("index {} too small for array; minimum: 0", index),
            )));
        }
    } else if let Ok(index) = first.funcall::<Int>("to_int", &[], None) {
        if let Ok(index) = usize::try_from(index) {
            (index, None, second)
        } else {
            return Err(Box::new(IndexError::new(
                interp,
                format!("index {} too small for array; minimum: 0", index),
            )));
        }
    } else {
        let rangelen = Int::try_from(array.borrow().buffer.len())
            .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
        match unsafe { is_range(interp, &first, rangelen) } {
            Ok(Some((start, len))) => {
                if let Ok(start) = usize::try_from(start) {
                    (start, Some(len), second)
                } else {
                    (0, Some(len), second)
                }
            }
            Ok(None) => {
                let start = if let Ok(start) = first.funcall::<Value>("begin", &[], None) {
                    start
                } else {
                    return Err(Box::new(Fatal::new(
                        interp,
                        "Unable to extract first from Range",
                    )));
                };
                let start = if let Ok(start) = start.clone().try_into::<Int>() {
                    start
                } else if let Ok(start) = start.funcall::<Int>("to_int", &[], None) {
                    start
                } else {
                    return Err(Box::new(TypeError::new(
                        interp,
                        format!(
                            "no implicit conversion of {} into Integer",
                            start.pretty_name()
                        ),
                    )));
                };
                let end = if let Ok(end) = first.funcall::<Value>("last", &[], None) {
                    end
                } else {
                    return Err(Box::new(Fatal::new(
                        interp,
                        "Unable to extract first from Range",
                    )));
                };
                let end = if let Ok(end) = end.clone().try_into::<Int>() {
                    end
                } else if let Ok(end) = end.funcall::<Int>("to_int", &[], None) {
                    end
                } else {
                    return Err(Box::new(TypeError::new(
                        interp,
                        format!(
                            "no implicit conversion of {} into Integer",
                            end.pretty_name()
                        ),
                    )));
                };
                if start + (end - start) < 0 {
                    return Err(Box::new(RangeError::new(
                        interp,
                        format!("{}..{} out of range", start, end),
                    )));
                }
                match (usize::try_from(start), usize::try_from(end)) {
                    (Ok(start), Ok(end)) => {
                        if end > start {
                            (start, Some(end - start), second)
                        } else {
                            (start, None, second)
                        }
                    }
                    (Err(_), Ok(end)) => {
                        let start = usize::try_from(start)
                            .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
                        if start < array.borrow().buffer.len() {
                            let start = array.borrow().buffer.len() - start;
                            if end > start {
                                (start, Some(end - start), second)
                            } else {
                                (start, None, second)
                            }
                        } else {
                            return Err(Box::new(IndexError::new(
                                interp,
                                format!(
                                    "index {} too small for array; minimum: -{}",
                                    start,
                                    array.borrow().buffer.len()
                                ),
                            )));
                        }
                    }
                    (Ok(start), Err(_)) => (start, None, second),
                    (Err(_), Err(_)) => {
                        return Err(Box::new(IndexError::new(
                            interp,
                            format!(
                                "index {} too small for array; minimum: -{}",
                                start,
                                array.borrow().buffer.len()
                            ),
                        )))
                    }
                }
            }
            Err(_) => {
                let index_type_name = first.pretty_name();
                return Err(Box::new(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", index_type_name),
                )));
            }
        }
    };

    if let Some(len) = len {
        let mut other_buffer = if let Ok(other) = unsafe { Array::try_from_ruby(interp, &elem) } {
            other.borrow().buffer.clone()
        } else if let Ok(true) = elem.respond_to("to_ary") {
            let ruby_type = elem.pretty_name();
            if let Ok(other) = elem.funcall("to_ary", &[], None) {
                if let Ok(other) = unsafe { Array::try_from_ruby(interp, &other) } {
                    other.borrow().buffer.clone()
                } else {
                    return Err(Box::new(TypeError::new(
                        interp,
                        format!(
                            "can't convert {classname} to Array ({classname}#to_ary gives {gives})",
                            classname = ruby_type,
                            gives = other.pretty_name()
                        ),
                    )));
                }
            } else {
                // TODO: propagate exceptions thrown by `value#to_a`.
                return Err(Box::new(Fatal::new(
                    interp,
                    "Error calling #to_a even though it exists",
                )));
            }
        } else {
            let mut buffer = VecDeque::with_capacity(1);
            buffer.push_back(elem.clone());
            buffer
        };
        let mut borrow = array.borrow_mut();
        let buf_len = borrow.buffer.len();
        let other_len = other_buffer.len();
        if len == 0 {
            let mut idx = start;
            for _ in buf_len..start {
                borrow.buffer.push_back(interp.convert(None::<Value>));
            }
            for item in other_buffer.drain(0..) {
                borrow.buffer.insert(idx, item);
                idx += 1;
            }
        } else if start > buf_len {
            for _ in buf_len..start {
                borrow.buffer.push_back(interp.convert(None::<Value>));
            }
            borrow.buffer.extend(other_buffer);
        } else if start + other_len < buf_len {
            if other_len == len {
                for (index, item) in other_buffer.drain(0..).enumerate() {
                    let idx = start + index;
                    borrow.buffer[idx] = item;
                }
            } else if other_len < len {
                for (index, item) in other_buffer.drain(0..).enumerate() {
                    let idx = start + index;
                    borrow.buffer[idx] = item;
                }
                let at = start + other_len;
                for _ in other_len..len {
                    borrow.buffer.remove(at);
                }
            } else {
                for (index, item) in other_buffer.drain(0..len).enumerate() {
                    let idx = start + index;
                    borrow.buffer[idx] = item;
                }
                for (index, item) in other_buffer.drain(0..len).enumerate() {
                    let at = start + other_len + index;
                    borrow.buffer.insert(at, item);
                }
            }
        } else {
            // we are guaranteed to need to call push_back.
            let mut idx = start;
            for item in other_buffer.drain(0..) {
                if idx < buf_len {
                    borrow.buffer[idx] = item;
                    idx += 1;
                } else if idx > buf_len {
                    borrow.buffer.push_back(item);
                    idx += 1;
                } else {
                    borrow.buffer.insert(idx, item);
                    idx += 1;
                }
            }
        }
    } else {
        let mut borrow = array.borrow_mut();
        let len = borrow.buffer.len();
        for _ in len..=start {
            borrow.buffer.push_back(interp.convert(None::<Value>));
        }
        borrow.buffer[start] = elem.clone();
    }
    Ok(elem)
}

pub fn pop(interp: &Artichoke, ary: &Value) -> Result<Option<Value>, Box<dyn RubyException>> {
    if ary.is_frozen() {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let mut borrow = array.borrow_mut();
    Ok(borrow.buffer.pop_back())
}

pub fn shift(
    interp: &Artichoke,
    ary: &Value,
    count: Option<usize>,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    if let Some(count) = count {
        let mut popped = VecDeque::with_capacity(count);
        {
            let mut borrow = array.borrow_mut();
            for _ in 0..count {
                let item = borrow.buffer.pop_front();
                if item.is_none() {
                    break;
                }
                popped.push_back(interp.convert(item));
            }
        }
        let popped = Array { buffer: popped };
        let result = unsafe { popped.try_into_ruby(interp, None) }
            .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
        Ok(result)
    } else {
        let popped = {
            let mut borrow = array.borrow_mut();
            borrow.buffer.pop_front()
        };
        Ok(interp.convert(popped))
    }
}

pub fn unshift(
    interp: &Artichoke,
    ary: Value,
    value: Value,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
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
    borrow.buffer.push_front(value);
    Ok(ary)
}

pub fn concat(
    interp: &Artichoke,
    ary: Value,
    other: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
        return Err(Box::new(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let other = if let Some(other) = other {
        other
    } else {
        return Ok(ary);
    };
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let mut borrow = array.borrow_mut();
    let ruby_type = other.pretty_name();
    if ary == other {
        let copy = borrow.buffer.clone();
        borrow.buffer.extend(copy);
        Ok(ary)
    } else if let Ok(other) = unsafe { Array::try_from_ruby(interp, &other) } {
        borrow.buffer.extend(other.borrow().buffer.clone());
        Ok(ary)
    } else if let Ok(other) = other.funcall("to_ary", &[], None) {
        if let Ok(other) = unsafe { Array::try_from_ruby(interp, &other) } {
            borrow.buffer.extend(other.borrow().buffer.clone());
            Ok(ary)
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!(
                    "can't convert {classname} to Array ({classname}#to_ary gives {gives})",
                    classname = ruby_type,
                    gives = other.pretty_name()
                ),
            )))
        }
    } else {
        Err(Box::new(TypeError::new(
            interp,
            format!(
                "no implicit conversion of {classname} into Array",
                classname = ruby_type,
            ),
        )))
    }
}

pub fn push(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
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
    borrow.buffer.push_back(value);
    Ok(ary)
}

pub fn replace(
    interp: &Artichoke,
    ary: Value,
    other: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
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
    let ruby_type = other.pretty_name();
    if let Ok(other) = unsafe { Array::try_from_ruby(interp, &other) } {
        borrow.buffer.extend(other.borrow().buffer.clone());
        Ok(ary)
    } else if let Ok(other) = other.funcall("to_ary", &[], None) {
        if let Ok(other) = unsafe { Array::try_from_ruby(interp, &other) } {
            borrow.buffer.extend(other.borrow().buffer.clone());
            Ok(ary)
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!(
                    "can't convert {classname} to Array ({classname}#to_ary gives {gives})",
                    classname = ruby_type,
                    gives = other.pretty_name()
                ),
            )))
        }
    } else {
        Err(Box::new(TypeError::new(
            interp,
            format!(
                "no implicit conversion of {classname} into Array",
                classname = ruby_type,
            ),
        )))
    }
}

pub fn reverse(interp: &Artichoke, ary: &Value) -> Result<Value, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = array.borrow();
    let len = borrow.buffer.len();
    let mut buffer = VecDeque::with_capacity(borrow.buffer.len());
    for offset in 1..=len {
        buffer.push_back(borrow.buffer[len - offset].clone());
    }
    let result = Array { buffer };
    let result = unsafe { result.try_into_ruby(interp, None) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn reverse_bang(interp: &Artichoke, ary: Value) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
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
    if borrow.buffer.is_empty() {
        return Ok(ary);
    }
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
) -> Result<Value, Box<dyn RubyException>> {
    if ary.is_frozen() {
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
    let offset = if offset >= 0 {
        usize::try_from(offset)
            .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?
    } else {
        // Positive Int must be usize
        let idx = usize::try_from(-offset)
            .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?;
        if let Some(offset) = borrow.buffer.len().checked_sub(idx) {
            offset
        } else {
            return Err(Box::new(IndexError::new(
                interp,
                format!(
                    "index {} too small for array; minimum: {}",
                    offset,
                    borrow.buffer.len()
                ),
            )));
        }
    };
    for _ in 0..=offset {
        borrow.buffer.push_back(interp.convert(None::<Value>));
    }
    borrow.buffer[offset] = value;
    Ok(ary)
}

pub fn len(interp: &Artichoke, ary: &Value) -> Result<usize, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let borrow = array.borrow();
    Ok(borrow.buffer.len())
}

pub fn clone(interp: &Artichoke, ary: &Value) -> Result<Value, Box<dyn RubyException>> {
    let array = unsafe { Array::try_from_ruby(interp, ary) }.map_err(|_| {
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

pub fn initialize_copy(
    interp: &Artichoke,
    ary: &Value,
    other: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    let other = unsafe { Array::try_from_ruby(interp, other) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Array from Ruby Array receiver",
        )
    })?;
    let result = Array {
        buffer: other.borrow().buffer.clone(),
    };
    let result = unsafe { result.try_into_ruby(interp, Some(ary.inner())) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
    Ok(result)
}

pub fn to_ary(interp: &Artichoke, value: Value) -> Result<Value, Box<dyn RubyException>> {
    if unsafe { Array::try_from_ruby(interp, &value) }.is_ok() {
        Ok(value)
    } else if let Ok(ary) = value.funcall::<Value>("to_a", &[], None) {
        let ruby_type = ary.pretty_name();
        if unsafe { Array::try_from_ruby(interp, &ary) }.is_ok() {
            Ok(ary)
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!(
                    "can't convert {classname} to Array ({classname}#to_a gives {gives})",
                    classname = value.pretty_name(),
                    gives = ruby_type
                ),
            )))
        }
    } else {
        from_values(interp, &[value])
    }
}

unsafe fn is_range(
    interp: &Artichoke,
    range: &Value,
    length: Int,
) -> Result<Option<(Int, usize)>, Box<dyn RubyException>> {
    let mut start = <mem::MaybeUninit<sys::mrb_int>>::uninit();
    let mut len = <mem::MaybeUninit<sys::mrb_int>>::uninit();
    let mrb = interp.0.borrow().mrb;
    // `mrb_range_beg_len` can raise.
    // TODO: Wrap this in a call to `mrb_protect`.
    let check_range = sys::mrb_range_beg_len(
        mrb,
        range.inner(),
        start.as_mut_ptr(),
        len.as_mut_ptr(),
        length,
        0_u8,
    );
    let start = start.assume_init();
    let len = len.assume_init();
    if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
        let len = usize::try_from(len)
            .map_err(|_| TypeError::new(interp, "no implicit conversion into Integer"))?;
        Ok(Some((start, len)))
    } else {
        Ok(None)
    }
}
