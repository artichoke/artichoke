use std::convert::TryFrom;
use std::slice;

use crate::convert::Convert;
use crate::extn::core::array::{self, Error};
use crate::extn::core::error::{IndexError, RubyException, RuntimeError, TypeError};
use crate::sys;
use crate::value::Value;

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = array::new(&interp);
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
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_concat(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    other: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let other = Value::new(&interp, other);
    let result = array::concat(&interp, ary, other);
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

// MRB_API mrb_value mrb_ary_pop(mrb_state *mrb, mrb_value ary);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_pop(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = array::pop(&interp, &ary);
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
    let result = array::push(&interp, ary, value);
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
        .and_then(|offset| array::element_reference(&interp, &ary, offset));
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
        .and_then(|offset| array::element_set(&interp, ary, offset, value));
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
    let ary = Value::new(&interp, value);
    let result = array::clone(&interp, &ary);
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
