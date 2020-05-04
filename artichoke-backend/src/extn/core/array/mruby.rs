use std::convert::TryFrom;

use crate::extn::core::array;
use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<array::Array>() {
        return Ok(());
    }
    let spec = class::Spec::new("Array", None, Some(def::rust_data_free::<array::Array>))?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_method("[]", ary_element_reference, sys::mrb_args_req_and_opt(1, 1))?
        .add_method(
            "[]=",
            ary_element_assignment,
            sys::mrb_args_req_and_opt(2, 1),
        )?
        .add_method("concat", ary_concat, sys::mrb_args_any())?
        .add_method(
            "initialize",
            ary_initialize,
            sys::mrb_args_opt(2) | sys::mrb_args_block(),
        )?
        .add_method("initialize_copy", ary_initialize_copy, sys::mrb_args_req(1))?
        .add_method("length", ary_len, sys::mrb_args_none())?
        .add_method("pop", ary_pop, sys::mrb_args_none())?
        .add_method("reverse!", ary_reverse_bang, sys::mrb_args_none())?
        .add_method("size", ary_len, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<array::Array>(spec)?;
    let _ = interp.eval(&include_bytes!("array.rb")[..])?;
    trace!("Patched Array onto interpreter");
    Ok(())
}

unsafe extern "C" fn ary_pop(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let result = array::trampoline::pop(&mut interp, array);
    match result {
        Ok(value) => {
            let _ = Artichoke::into_raw(interp);
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn ary_len(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = array::trampoline::len(&mut interp, ary).and_then(|len| {
        if let Ok(len) = sys::mrb_int::try_from(len) {
            Ok(len)
        } else {
            Err(Fatal::new(&interp, "Array length does not fit in mruby Integer max").into())
        }
    });
    match result {
        Ok(len) => {
            let len = interp.convert(len);
            let _ = Artichoke::into_raw(interp);
            len.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn ary_concat(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let other = other.map(|other| Value::new(&interp, other));
    let result = array::trampoline::concat(&mut interp, array, other);
    match result {
        Ok(value) => {
            let _ = Artichoke::into_raw(interp);
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn ary_initialize(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let (first, second, block) = mrb_get_args!(mrb, optional = 2, &block);
    let mut interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let first = first.map(|first| Value::new(&interp, first));
    let second = second.map(|second| Value::new(&interp, second));
    let result = array::trampoline::initialize(&mut interp, array, first, second, block);
    match result {
        Ok(value) => {
            let _ = Artichoke::into_raw(interp);
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn ary_initialize_copy(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let other = Value::new(&interp, other);
    let result = array::trampoline::initialize_copy(&mut interp, array, other);
    match result {
        Ok(value) => {
            let _ = Artichoke::into_raw(interp);
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn ary_reverse_bang(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let result = array::trampoline::reverse_bang(&mut interp, array);
    match result {
        Ok(value) => {
            let _ = Artichoke::into_raw(interp);
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn ary_element_reference(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let (elem, len) = mrb_get_args!(mrb, required = 1, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let elem = Value::new(&interp, elem);
    let len = len.map(|len| Value::new(&interp, len));
    let array = Value::new(&interp, ary);
    let result = array::trampoline::element_reference(&mut interp, array, elem, len);
    match result {
        Ok(value) => {
            let _ = Artichoke::into_raw(interp);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn ary_element_assignment(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let (first, second, third) = mrb_get_args!(mrb, required = 2, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let first = Value::new(&interp, first);
    let second = Value::new(&interp, second);
    let third = third.map(|third| Value::new(&interp, third));
    let array = Value::new(&interp, ary);
    let result = array::trampoline::element_assignment(&mut interp, array, first, second, third);
    match result {
        Ok(value) => {
            let _ = Artichoke::into_raw(interp);
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}
