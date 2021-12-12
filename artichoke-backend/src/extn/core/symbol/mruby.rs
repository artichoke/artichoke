use std::ffi::CStr;

use crate::extn::core::symbol::{self, trampoline};
use crate::extn::prelude::*;

const SYMBOL_CSTR: &CStr = cstr::cstr!("Symbol");
static SYMBOL_RUBY_SOURCE: &[u8] = include_bytes!("symbol.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<symbol::Symbol>() {
        return Ok(());
    }

    let spec = class::Spec::new("Symbol", SYMBOL_CSTR, None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_self_method("all_symbols", symbol_all_symbols, sys::mrb_args_none())?
        .add_method("==", symbol_equal_equal, sys::mrb_args_req(1))?
        .add_method("casecmp", symbol_ascii_casecmp, sys::mrb_args_req(1))?
        .add_method("casecmp?", symbol_unicode_casecmp, sys::mrb_args_req(1))?
        .add_method("empty?", symbol_empty, sys::mrb_args_none())?
        .add_method("inspect", symbol_inspect, sys::mrb_args_none())?
        .add_method("length", symbol_length, sys::mrb_args_none())?
        .add_method("to_s", symbol_to_s, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<symbol::Symbol>(spec)?;
    interp.eval(SYMBOL_RUBY_SOURCE)?;

    Ok(())
}

unsafe extern "C" fn symbol_all_symbols(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::all_symbols(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn symbol_equal_equal(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let sym = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::equal_equal(&mut guard, sym, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn symbol_ascii_casecmp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let sym = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::ascii_casecmp(&mut guard, sym, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn symbol_unicode_casecmp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let sym = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::unicode_casecmp(&mut guard, sym, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn symbol_empty(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let sym = Value::from(slf);
    let result = trampoline::is_empty(&mut guard, sym);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn symbol_inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::inspect(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn symbol_length(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let sym = Value::from(slf);
    let result = trampoline::length(&mut guard, sym);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn symbol_to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let sym = Value::from(slf);
    let result = trampoline::bytes(&mut guard, sym);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
