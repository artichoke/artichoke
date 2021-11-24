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
        .add_method("ord", string_ord, sys::mrb_args_none())?
        .add_method("scan", string_scan, sys::mrb_args_req(1))?
        .define()?;
    interp.def_class::<string::String>(spec)?;
    interp.eval(&include_bytes!("string.rb")[..])?;
    trace!("Patched String onto interpreter");
    Ok(())
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
