//! FFI glue between the Rust trampolines and the mruby C interpreter.

use std::ffi::CStr;

use crate::extn::prelude::*;
use crate::extn::stdlib::securerandom::{self, trampoline};

const SECURE_RANDOM_CSTR: &CStr = qed::const_cstr_from_str!("SecureRandom\0");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    interp.def_file_for_type::<_, SecureRandomFile>("securerandom.rb")?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct SecureRandomFile {
    // Ensure this type is not constructable
    _private: (),
}

impl File for SecureRandomFile {
    type Artichoke = Artichoke;
    type Error = Error;

    fn require(interp: &mut Self::Artichoke) -> Result<(), Self::Error> {
        if interp.is_module_defined::<securerandom::SecureRandom>() {
            return Ok(());
        }

        let spec = module::Spec::new(interp, "SecureRandom", SECURE_RANDOM_CSTR, None)?;
        module::Builder::for_spec(interp, &spec)
            .add_self_method("alphanumeric", securerandom_alphanumeric, sys::mrb_args_opt(1))?
            .add_self_method("base64", securerandom_base64, sys::mrb_args_opt(1))?
            .add_self_method("urlsafe_base64", securerandom_urlsafe_base64, sys::mrb_args_opt(2))?
            .add_self_method("hex", securerandom_hex, sys::mrb_args_opt(1))?
            .add_self_method("random_bytes", securerandom_random_bytes, sys::mrb_args_opt(1))?
            .add_self_method("random_number", securerandom_random_number, sys::mrb_args_opt(1))?
            .add_self_method("uuid", securerandom_uuid, sys::mrb_args_none())?
            .define()?;
        interp.def_module::<securerandom::SecureRandom>(spec)?;

        Ok(())
    }
}

unsafe extern "C-unwind" fn securerandom_alphanumeric(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let len = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let len = len.map(Value::from).and_then(|len| guard.convert(len));
    let result = trampoline::alphanumeric(&mut guard, len);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn securerandom_base64(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let len = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let len = len.map(Value::from).and_then(|len| guard.convert(len));
    let result = trampoline::base64(&mut guard, len);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn securerandom_urlsafe_base64(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (len, padding) = mrb_get_args!(mrb, optional = 2);
    unwrap_interpreter!(mrb, to => guard);
    let len = len.map(Value::from).and_then(|len| guard.convert(len));
    let padding = padding.map(Value::from).and_then(|padding| guard.convert(padding));
    let result = trampoline::urlsafe_base64(&mut guard, len, padding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn securerandom_hex(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let len = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let len = len.map(Value::from).and_then(|len| guard.convert(len));
    let result = trampoline::hex(&mut guard, len);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn securerandom_random_bytes(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let len = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let len = len.map(Value::from).and_then(|len| guard.convert(len));
    let result = trampoline::random_bytes(&mut guard, len);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn securerandom_random_number(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let max = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let max = max.map(Value::from).and_then(|max| guard.convert(max));
    let result = trampoline::random_number(&mut guard, max);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn securerandom_uuid(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::uuid(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
