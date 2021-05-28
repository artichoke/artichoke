use std::ffi::CStr;

use crate::extn::core::string::{self, trampoline};
use crate::extn::prelude::*;

const STRING_CSTR: &CStr = cstr::cstr!("String");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<string::String>() {
        return Ok(());
    }
    let spec = class::Spec::new("String", STRING_CSTR, None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_method("<=>", string_cmp_rocket, sys::mrb_args_req(1))?
        .add_method("==", string_equals_equals, sys::mrb_args_req(1))?
        .add_method("+", string_add, sys::mrb_args_req(1))?
        .add_method("*", string_mul, sys::mrb_args_req(1))?
        .add_method("bytes", string_bytes, sys::mrb_args_none())?
        .add_method("bytesize", string_bytesize, sys::mrb_args_none())?
        .add_method("eql?", string_eql, sys::mrb_args_req(1))?
        .add_method("inspect", string_inspect, sys::mrb_args_none())?
        .add_method("length", string_length, sys::mrb_args_none())?
        .add_method("ord", string_ord, sys::mrb_args_none())?
        .add_method("scan", string_scan, sys::mrb_args_req(1))?
        .add_method("size", string_length, sys::mrb_args_none())?
        .add_method("to_s", string_to_s, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<string::String>(spec)?;
    // interp.eval(&include_bytes!("string.rb")[..])?;
    trace!("Patched String onto interpreter");
    Ok(())
}

unsafe extern "C" fn string_cmp_rocket(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::cmp_rocket(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_equals_equals(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::equals_equals(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_add(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::add(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_mul(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::mul(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_bytes(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::bytes(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_bytesize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::bytesize(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::eql(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::inspect(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_length(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::length(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_ord(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::ord(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_scan(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, block) = mrb_get_args!(mrb, required = 1, &block);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let pattern = Value::from(pattern);
    let result = trampoline::scan(&mut guard, value, pattern, block);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let _ = mrb;
    // TODO: dup `slf` when slf is a subclass of `String`.
    slf
}
