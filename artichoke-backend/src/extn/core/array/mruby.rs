use std::convert::TryFrom;
use std::slice;

use crate::convert::{Convert, RustBackedValue};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::array;
use crate::extn::core::exception;
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let array = interp.0.borrow_mut().def_class::<array::Array>(
        "Array",
        None,
        Some(rust_data_free::<array::Array>),
    );
    array.borrow_mut().mrb_value_is_rust_backed(true);

    array
        .borrow_mut()
        .add_method("[]", ary_element_reference, sys::mrb_args_req_and_opt(1, 1));
    array.borrow_mut().add_method(
        "[]=",
        ary_element_assignment,
        sys::mrb_args_req_and_opt(2, 1),
    );
    array
        .borrow_mut()
        .add_method("concat", ary_concat, sys::mrb_args_any());
    array.borrow_mut().add_method(
        "initialize",
        ary_initialize,
        sys::mrb_args_opt(2) | sys::mrb_args_block(),
    );
    array
        .borrow_mut()
        .add_method("initialize_copy", ary_initialize_copy, sys::mrb_args_req(1));
    array
        .borrow_mut()
        .add_method("length", ary_len, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("pop", artichoke_ary_pop, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("reverse", ary_reverse, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("reverse!", ary_reverse_bang, sys::mrb_args_none());
    array
        .borrow_mut()
        .add_method("size", ary_len, sys::mrb_args_none());
    array.borrow().define(interp)?;

    interp.eval(&include_bytes!("array.rb")[..])?;
    Ok(())
}

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
pub unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = array::trampoline::new(&interp);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::with_capacity(&interp, capacity);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::from_values(&interp, values.as_slice());
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::splat(&interp, value);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::concat(&interp, array, Some(other));
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::pop(&interp, &array);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::push(&interp, array, value);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::ary_ref(&interp, &ary, offset);
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
    let result = array::trampoline::element_set(&interp, array, offset, value);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::clone(&interp, ary);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::to_ary(&interp, value);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::len(&interp, ary)
        .map(|len| sys::mrb_int::try_from(len).unwrap_or_default());
    match result {
        Ok(len) => len,
        Err(exception) => exception::raise(interp, exception),
    }
}

pub unsafe extern "C" fn ary_len(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = array::trampoline::len(&interp, ary)
        .map(|len| sys::mrb_int::try_from(len).unwrap_or_default());
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
    let result = array::trampoline::concat(&interp, array, other);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

pub unsafe extern "C" fn ary_initialize(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let (first, second, block) = mrb_get_args!(mrb, optional = 2, &block);
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let first = first.map(|first| Value::new(&interp, first));
    let second = second.map(|second| Value::new(&interp, second));
    let result = array::trampoline::initialize(&interp, array, first, second, block);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::initialize_copy(&interp, array, other);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let array = Value::new(&interp, ary);
    let result = array::trampoline::shift(&interp, &array, Some(1));
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::reverse(&interp, ary);
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
    let result = array::trampoline::reverse_bang(&interp, array);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::unshift(&interp, array, val);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
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
    let result = array::trampoline::element_reference(&interp, array, elem, len);
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
    let result = array::trampoline::element_assignment(&interp, array, first, second, third);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_gc_mark_ary(mrb: *mut sys::mrb_state, ary: sys::mrb_value) {
    if let Ok(interp) = crate::ffi::from_user_data(mrb) {
        let array = Value::new(&interp, ary);
        if let Ok(array) = array::Array::try_from_ruby(&interp, &array) {
            let borrow = array.borrow();
            borrow.gc_mark(&interp);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn artichoke_gc_mark_ary_size(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> usize {
    if let Ok(interp) = crate::ffi::from_user_data(mrb) {
        let array = Value::new(&interp, ary);
        if let Ok(array) = array::Array::try_from_ruby(&interp, &array) {
            let borrow = array.borrow();
            return borrow.real_children();
        }
    }
    0
}
