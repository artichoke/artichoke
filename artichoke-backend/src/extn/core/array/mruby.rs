use std::convert::TryFrom;
use std::slice;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::array::{self, Error};
use crate::extn::core::error::{IndexError, RubyException, RuntimeError, TypeError};
use crate::sys;
use crate::types::Int;
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
        .and_then(|offset| array::ary_ref(&interp, &ary, offset));
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

pub unsafe extern "C" fn ary_concat(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
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

pub unsafe extern "C" fn ary_len(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let len = artichoke_ary_len(mrb, ary);
    sys::mrb_sys_fixnum_value(len)
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

pub unsafe fn extract(interp: &Artichoke, num_captures: usize) -> Result<Self, Error> {
    let num_captures = Int::try_from(num_captures).map_err(|_| Error::Fatal)?;
    let mut first = <mem::MaybeUninit<sys::mrb_value>>::uninit();
    let mut second = <mem::MaybeUninit<sys::mrb_value>>::uninit();
    let mut has_second = <mem::MaybeUninit<sys::mrb_bool>>::uninit();
    sys::mrb_get_args(
        interp.0.borrow().mrb,
        Self::ARGSPEC.as_ptr() as *const i8,
        first.as_mut_ptr(),
        second.as_mut_ptr(),
        has_second.as_mut_ptr(),
    );
    let first = first.assume_init();
    let second = second.assume_init();
    let has_length = has_second.assume_init() != 0;
}

unsafe fn is_range(
    interp: &Artichoke,
    first: sys::mrb_value,
    num_captures: Int,
) -> Result<Option<Self>, Error> {
    let mut start = <mem::MaybeUninit<sys::mrb_int>>::uninit();
    let mut len = <mem::MaybeUninit<sys::mrb_int>>::uninit();
    let check_range = sys::mrb_range_beg_len(
        interp.0.borrow().mrb,
        first,
        start.as_mut_ptr(),
        len.as_mut_ptr(),
        num_captures + 1,
        0_u8,
    );
    let start = start.assume_init();
    let len = len.assume_init();
    if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
        let len = usize::try_from(len).map_err(|_| Error::LengthType)?;
        Ok(Some(Args::StartLen(start, len)))
    } else {
        Ok(None)
    }
}

fn ary_element_reference_args<'a>(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> Result<sys::mrb_value, Error<'a>> {
    let len = artichoke_ary_len(mrb, ary);
    let (first, second) = mrb_get_args!(mrb, required = 1, optional = 1);
    let interp = unwrap_interpreter!(mrb);
    if let Some(length) = second {
        let start = Value::new(&interp, first);
        let start_type = start.pretty_name();
        let start = start.try_into::<Int>().map_err(|_| {
            Err(Error::NoImplicitConversion {
                from: start_type,
                to: "Integer",
            })
        })?;
        let len = Value::new(&interp, length);
        let len_type = len.pretty_name();
        let len = len.try_into::<usize>().map_err(|_| {
            Err(Error::NoImplicitConversion {
                from: len_type,
                to: "Integer",
            })
        })?;
        Ok(Args::StartLen(start, len))
    } else if let Ok(index) = interp.try_convert(Value::new(interp, first)) {
        Ok(Args::Index(index))
    } else if let Ok(name) = interp.try_convert(Value::new(interp, first)) {
        Ok(Args::Name(name))
    } else if let Some(args) = Self::is_range(interp, first, num_captures)? {
        Ok(args)
    } else {
        Err(Error::IndexType)
    }
}
