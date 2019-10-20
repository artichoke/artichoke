use std::convert::TryFrom;
use std::slice;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::array;
use crate::extn::core::exception;
use crate::sys;
use crate::value::Value;

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = array::new(&interp);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_ary_new_capa(mrb_state*, mrb_int);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new_capa(
    mrb: *mut sys::mrb_state,
    capacity: sys::mrb_int,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let capacity = usize::try_from(capacity).unwrap_or_default();
    let result = array::with_capacity(&interp, capacity);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
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
    let size = usize::try_from(size).unwrap_or_default();
    let values = slice::from_raw_parts(vals, size);
    let values = values
        .iter()
        .map(|val| Value::new(&interp, *val))
        .collect::<Vec<_>>();
    let result = array::from_values(&interp, values.as_slice());
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
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
    let offset = isize::try_from(offset).unwrap_or_default();
    let result = array::ary_ref(&interp, &ary, offset);
    match result {
        Ok(value) => interp.convert(value).inner(),
        Err(exception) => exception::raise(interp, exception),
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
    let offset = isize::try_from(offset).unwrap_or_default();
    let result = array::element_set(&interp, array, offset, value);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
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
    let result =
        array::len(&interp, &ary).map(|len| sys::mrb_int::try_from(len).unwrap_or_default());
    match result {
        Ok(len) => len,
        Err(exception) => exception::raise(interp, exception),
    }
}

pub unsafe extern "C" fn ary_len(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result =
        array::len(&interp, &ary).map(|len| sys::mrb_int::try_from(len).unwrap_or_default());
    match result {
        Ok(len) => interp.convert(len).inner(),
        Err(exception) => exception::raise(interp, exception),
    }
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
        Err(exception) => exception::raise(interp, exception),
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
    let result = array::initialize_copy(&interp, &array, &other);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
    }
}

pub unsafe extern "C" fn ary_reverse(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = array::reverse(&interp, &ary);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

pub unsafe extern "C" fn ary_reverse_bang(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let result = array::reverse_bang(&interp, array);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
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
        Err(exception) => exception::raise(interp, exception),
    }
}

pub unsafe extern "C" fn ary_element_reference(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let (elem, len) = mrb_get_args!(mrb, required = 1, optional = 1);
    let interp = unwrap_interpreter!(mrb);
    let elem = Value::new(&interp, elem);
    let len = len.map(|len| Value::new(&interp, len));
    let array = Value::new(&interp, ary);
    let result = array::len(&interp, &array)
        .and_then(|length| array::ElementReferenceArgs::extract(&interp, elem, len, length))
        .and_then(|args| array::element_reference(&interp, args, &array));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

pub unsafe extern "C" fn ary_element_assignment(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let (first, second, third) = mrb_get_args!(mrb, required = 2, optional = 1);
    let interp = unwrap_interpreter!(mrb);
    let first = Value::new(&interp, first);
    let second = Value::new(&interp, second);
    let third = third.map(|third| Value::new(&interp, third));
    let array = Value::new(&interp, ary);
    let result = array::element_assignment(&interp, array, first, second, third);
    if result.is_ok() {
        let basic = sys::mrb_sys_basic_ptr(ary);
        sys::mrb_write_barrier(mrb, basic);
    }
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
