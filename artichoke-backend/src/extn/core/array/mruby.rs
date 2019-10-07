use artichoke_core::types::Ruby;
use artichoke_core::value::Value as ValueLike;
use std::convert::TryFrom;
use std::mem;
use std::slice;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::array::{self, Error};
use crate::extn::core::exception::{
    FrozenError, IndexError, RangeError, RubyException, RuntimeError, TypeError,
};
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = array::new(&interp);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
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
        .and_then(|capacity| array::with_capacity(&interp, capacity));
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
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
            let values = values
                .iter()
                .map(|val| Value::new(&interp, *val))
                .collect::<Vec<_>>();
            array::from_values(&interp, values.as_slice())
        });
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_splat(mrb_state *mrb, mrb_value value);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_splat(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = array::splat(&interp, value);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
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
    let array = Value::new(&interp, ary);
    let other = Value::new(&interp, other);
    let result = array::concat(&interp, array, Some(other));
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

// MRB_API mrb_value mrb_ary_pop(mrb_state *mrb, mrb_value ary);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_pop(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let result = array::pop(&interp, &array);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => interp.convert(value).inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

// MRB_API void mrb_ary_push(mrb_state *mrb, mrb_value array, mrb_value value);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_push(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let value = Value::new(&interp, value);
    let result = array::push(&interp, array, value);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
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
        .and_then(|offset| array::ary_ref(&interp, &ary, offset));
    match result {
        Ok(Some(value)) => value.inner(),
        Ok(None) => sys::mrb_sys_nil_value(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
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
    let array = Value::new(&interp, ary);
    let value = Value::new(&interp, value);
    let result = isize::try_from(offset)
        .map_err(|_| Error::Fatal)
        .and_then(|offset| array::element_set(&interp, array, offset, value));
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_clone(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, value);
    let result = array::clone(&interp, &ary);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_value_to_ary(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = array::to_ary(&interp, value);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
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
    let result = array::len(&interp, &ary)
        .and_then(|len| sys::mrb_int::try_from(len).map_err(|_| Error::Fatal));
    match result {
        Ok(len) => len,
        Err(Error::Artichoke(_)) => {
            RuntimeError::raise(interp, "artichoke error");
            0
        }
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args);
            0
        }
        Err(Error::Fatal) => {
            RuntimeError::raise(interp, "fatal Array error");
            0
        }
        Err(Error::Frozen) => {
            FrozenError::raise(interp, "can't modify frozen Array");
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
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args);
            0
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args);
            0
        }
    }
}

pub unsafe extern "C" fn ary_len(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let len = artichoke_ary_len(mrb, ary);
    let interp = unwrap_interpreter!(mrb);
    interp.convert(len).inner()
}

pub unsafe extern "C" fn ary_concat(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, optional = 1);
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let other = other.map(|other| Value::new(&interp, other));
    let result = array::concat(&interp, array, other);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

pub unsafe extern "C" fn ary_initialize_copy(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let other = Value::new(&interp, other);
    let result = array::initialize_copy(&interp, array, &other);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_check(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_bool {
    let interp = match crate::ffi::from_user_data(mrb) {
        Ok(interp) => interp,
        Err(_) => return 0,
    };
    let ary = Value::new(&interp, ary);
    if array::Array::try_from_ruby(&interp, &ary).is_ok() {
        1_u8
    } else {
        0_u8
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_shift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = array::shift(&interp, &ary, Some(1));
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary.inner());
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

pub unsafe extern "C" fn ary_reverse(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = array::reverse(&interp, ary);
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

pub unsafe extern "C" fn ary_reverse_bang(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let result = array::reverse_bang(&interp, array);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_unshift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    val: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let val = Value::new(&interp, val);
    let result = array::unshift(&interp, array, val);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

pub unsafe extern "C" fn ary_element_reference(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    fn ary_element_reference_args(
        interp: &Artichoke,
        first: Value,
        second: Option<Value>,
        len: Int,
    ) -> Result<Option<array::ElementReferenceArgs>, Error> {
        if let Some(length) = second {
            let start_type = first.pretty_name();
            let start = first
                .try_into::<Int>()
                .map_err(|_| Error::NoImplicitConversion {
                    from: start_type,
                    to: "Integer",
                })?;
            let len_type = length.pretty_name();
            let len = length
                .try_into::<usize>()
                .map_err(|_| Error::NoImplicitConversion {
                    from: len_type,
                    to: "Integer",
                })?;
            Ok(Some(array::ElementReferenceArgs::StartLen(start, len)))
        } else if let Ruby::Data = first.ruby_type() {
            Err(Error::NoImplicitConversion {
                from: first.pretty_name(),
                to: "Integer",
            })
        } else if let Ok(index) = first.clone().try_into::<Int>() {
            Ok(Some(array::ElementReferenceArgs::Index(index)))
        } else if let Ruby::Range = first.ruby_type() {
            if let Some(args) = unsafe { is_range(interp, &first, len)? } {
                Ok(Some(args))
            } else {
                Ok(None)
            }
        } else {
            Err(Error::NoImplicitConversion {
                from: first.pretty_name(),
                to: "Integer",
            })
        }
    }

    unsafe fn is_range<'a>(
        interp: &'a Artichoke,
        first: &Value,
        length: Int,
    ) -> Result<Option<array::ElementReferenceArgs>, Error<'a>> {
        let mut start = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        let mut len = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        let mrb = interp.0.borrow().mrb;
        // `mrb_range_beg_len` can raise.
        // TODO: Wrap this in a call to `mrb_protect`.
        let check_range = sys::mrb_range_beg_len(
            mrb,
            first.inner(),
            start.as_mut_ptr(),
            len.as_mut_ptr(),
            length,
            0_u8,
        );
        let start = start.assume_init();
        let len = len.assume_init();
        if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
            let len = usize::try_from(len).map_err(|_| Error::NoImplicitConversion {
                from: "Integer",
                to: "Integer",
            })?;
            Ok(Some(array::ElementReferenceArgs::StartLen(start, len)))
        } else {
            Ok(None)
        }
    }

    let (first, second) = mrb_get_args!(mrb, required = 1, optional = 1);
    let interp = unwrap_interpreter!(mrb);
    let first = Value::new(&interp, first);
    let second = second.map(|second| Value::new(&interp, second));
    let args = ary_element_reference_args(&interp, first, second, artichoke_ary_len(mrb, ary));
    let ary = Value::new(&interp, ary);
    let result = args.and_then(|args| {
        if let Some(args) = args {
            array::element_reference(&interp, &ary, args)
        } else {
            Ok(interp.convert(None::<Value>))
        }
    });
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}

pub unsafe extern "C" fn ary_element_assignment(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    fn ary_element_assignment_args(
        interp: &Artichoke,
        first: Value,
        second: Option<Value>,
        third: Option<Value>,
        len: Int,
    ) -> Result<Option<(array::ElementReferenceArgs, Value)>, Error> {
        if let Some(value) = third {
            let length = second.ok_or(Error::Fatal)?;
            let start_type = first.pretty_name();
            let start = first
                .try_into::<Int>()
                .map_err(|_| Error::NoImplicitConversion {
                    from: start_type,
                    to: "Integer",
                })?;
            let len_type = length.pretty_name();
            let length = length
                .try_into::<usize>()
                .map_err(|_| Error::NoImplicitConversion {
                    from: len_type,
                    to: "Integer",
                })?;
            Ok(Some((
                array::ElementReferenceArgs::StartLen(start, length),
                value,
            )))
        } else if let Some(value) = second {
            if let Ok(index) = first.clone().try_into::<Int>() {
                Ok(Some((array::ElementReferenceArgs::Index(index), value)))
            } else if first.ruby_type() == Ruby::Range {
                unsafe { is_range(interp, &first, len) }.map(|args| Some((args, value)))
            } else {
                Ok(None)
            }
        } else {
            Err(Error::Fatal)
        }
    }

    unsafe fn is_range<'a>(
        interp: &'a Artichoke,
        first: &Value,
        length: Int,
    ) -> Result<array::ElementReferenceArgs, Error<'a>> {
        let mut start = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        let mut len = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        let mrb = interp.0.borrow().mrb;
        // `mrb_range_beg_len` can raise.
        // TODO: Wrap this in a call to `mrb_protect`.
        let check_range = sys::mrb_range_beg_len(
            mrb,
            first.inner(),
            start.as_mut_ptr(),
            len.as_mut_ptr(),
            length,
            0_u8,
        );
        let start = start.assume_init();
        let len = len.assume_init();
        let exclusive = sys::mrb_sys_range_excl(mrb, first.inner());
        if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
            let len = usize::try_from(len).map_err(|_| Error::NoImplicitConversion {
                from: "Integer",
                to: "Integer",
            })?;
            Ok(array::ElementReferenceArgs::StartLen(start, len))
        } else {
            let min = isize::try_from(start).map_err(|_| Error::Fatal)?;
            let len = isize::try_from(len).map_err(|_| Error::Fatal)?;
            Err(Error::Range {
                min,
                max: min + len,
                exclusive,
            })
        }
    }

    let (first, second, third) = mrb_get_args!(mrb, required = 1, optional = 2);
    let interp = unwrap_interpreter!(mrb);
    let first = Value::new(&interp, first);
    let second = second.map(|second| Value::new(&interp, second));
    let third = third.map(|third| Value::new(&interp, third));
    let ary = Value::new(&interp, ary);
    let args = if ary.is_frozen() {
        Err(Error::Frozen)
    } else {
        ary_element_assignment_args(
            &interp,
            first,
            second,
            third,
            artichoke_ary_len(mrb, ary.inner()),
        )
    };
    let result = args.and_then(|args| {
        if let Some((args, value)) = args {
            array::element_assignment(&interp, &ary, args, value)
        } else if ary.is_frozen() {
            Err(Error::Frozen)
        } else {
            Ok(interp.convert(None::<Value>))
        }
    });
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary.inner());
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(Error::Artichoke(_)) => RuntimeError::raise(interp, "artichoke error"),
        Err(Error::CannotConvert {
            to,
            from,
            method,
            gives,
        }) => {
            let format_args = vec![interp.convert(format!(
                "can't convert {from} to {to} ({from}#{method} gives {gives})",
                to = to,
                from = from,
                method = method,
                gives = gives
            ))];
            RangeError::raisef(interp, "%S", format_args)
        }
        Err(Error::Fatal) => RuntimeError::raise(interp, "fatal Array error"),
        Err(Error::Frozen) => FrozenError::raise(interp, "can't modify frozen Array"),
        Err(Error::IndexTooSmall { index, minimum }) => {
            let format_args = vec![interp.convert(format!(
                "index {} too small for array; minimum: {}",
                index, minimum
            ))];
            IndexError::raisef(interp, "%S", format_args)
        }
        Err(Error::NoImplicitConversion { from, to }) => {
            let format_args = vec![interp.convert(from), interp.convert(to)];
            TypeError::raisef(interp, "No implicit conversion of %S into %S", format_args)
        }
        Err(Error::Range {
            min,
            max,
            exclusive,
        }) => {
            let format_args = if exclusive {
                vec![interp.convert(format!("{}..{} out of range", min, max))]
            } else {
                vec![interp.convert(format!("{}...{} out of range", min, max))]
            };
            RangeError::raisef(interp, "%S", format_args)
        }
    }
}
