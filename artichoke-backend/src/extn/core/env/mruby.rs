//! FFI glue between the Rust trampolines and the mruby C interpreter.

use std::ffi::CStr;

use crate::extn::core::artichoke;
use crate::extn::core::env::{self, trampoline};
use crate::extn::prelude::*;

const ENVIRON_CSTR: &CStr = cstr::cstr!("Environ");
static ENV_RUBY_SOURCE: &[u8] = spinoso_env::RUBY_API_POLYFILLS.as_bytes();

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<env::Environ>() {
        return Ok(());
    }

    let scope = interp
        .module_spec::<artichoke::Artichoke>()?
        .map(EnclosingRubyScope::module)
        .ok_or_else(|| NotDefinedError::module("Artichoke"))?;
    let spec = class::Spec::new(
        "Environ",
        ENVIRON_CSTR,
        Some(scope),
        Some(def::box_unbox_free::<env::Environ>),
    )?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_method("[]", env_element_reference, sys::mrb_args_req(1))?
        .add_method("[]=", env_element_assignment, sys::mrb_args_req(2))?
        .add_method("initialize", env_initialize, sys::mrb_args_none())?
        .add_method("to_h", env_to_h, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<env::Environ>(spec)?;
    interp.eval(ENV_RUBY_SOURCE)?;

    Ok(())
}

unsafe extern "C" fn env_initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let slf = Value::from(slf);
    let result = trampoline::initialize(&mut guard, slf);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn env_element_reference(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let name = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let obj = Value::from(slf);
    let name = Value::from(name);
    let result = trampoline::element_reference(&mut guard, obj, name);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn env_element_assignment(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (name, value) = mrb_get_args!(mrb, required = 2);
    unwrap_interpreter!(mrb, to => guard);
    let obj = Value::from(slf);
    let name = Value::from(name);
    let value = Value::from(value);
    let result = trampoline::element_assignment(&mut guard, obj, name, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn env_to_h(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let obj = Value::from(slf);
    let result = trampoline::to_h(&mut guard, obj);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
