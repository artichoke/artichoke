#[cfg(feature = "artichoke-array")]
use std::convert::TryFrom;

#[cfg(feature = "artichoke-array")]
use crate::extn::core::array;
use crate::extn::prelude::*;

#[cfg(feature = "artichoke-array")]
pub fn init(interp: &Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<array::Array>().is_some() {
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
    interp.0.borrow_mut().def_class::<array::Array>(spec);
    let _ = interp.eval(&include_bytes!("array.rb")[..])?;
    trace!("Patched Array onto interpreter");
    Ok(())
}

#[cfg(not(feature = "artichoke-array"))]
pub fn init(interp: &Artichoke) -> InitializeResult<()> {
    let _ = interp.eval(&include_bytes!("array.rb")[..])?;
    trace!("Patched Array onto interpreter");
    Ok(())
}

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_pop(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let result = array::trampoline::pop(&interp, array);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_len(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = array::trampoline::len(&interp, ary).and_then(|len| {
        sys::mrb_int::try_from(len).map_err(|_| {
            Exception::from(Fatal::new(
                &interp,
                "Array length does not fit in mruby Integer max",
            ))
        })
    });
    match result {
        Ok(len) => interp.convert(len).inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_concat(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    println!("ary concat C");
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

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_initialize(
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

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_initialize_copy(
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

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_reverse_bang(
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

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_element_reference(
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

#[cfg(feature = "artichoke-array")]
unsafe extern "C" fn ary_element_assignment(
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
