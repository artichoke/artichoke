use std::ffi::CStr;

use crate::extn::core::integer::trampoline;
use crate::extn::core::integer::Integer;
use crate::extn::prelude::*;

const INTEGER_CSTR: &CStr = qed::const_cstr_from_str!("Integer\0");
static INTEGER_RUBY_SOURCE: &[u8] = include_bytes!("integer.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Integer>() {
        return Ok(());
    }

    let spec = class::Spec::new("Integer", INTEGER_CSTR, None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_method("chr", integer_chr, sys::mrb_args_opt(1))?
        .add_method("[]", integer_element_reference, sys::mrb_args_req(1))?
        .add_method("/", integer_div, sys::mrb_args_req(1))?
        .add_method("allbits?", integer_is_allbits, sys::mrb_args_req(1))?
        .add_method("anybits?", integer_is_anybits, sys::mrb_args_req(1))?
        .add_method("nobits?", integer_is_nobits, sys::mrb_args_req(1))?
        .add_method("size", integer_size, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<Integer>(spec)?;
    interp.eval(INTEGER_RUBY_SOURCE)?;

    Ok(())
}

unsafe extern "C" fn integer_chr(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let encoding = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let encoding = encoding.map(Value::from);
    let result = trampoline::chr(&mut guard, value, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn integer_element_reference(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let bit = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let bit = Value::from(bit);
    let result = trampoline::element_reference(&mut guard, value, bit);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn integer_div(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let denominator = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let denominator = Value::from(denominator);
    let result = trampoline::div(&mut guard, value, denominator);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn integer_is_allbits(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let mask = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let mask = Value::from(mask);
    let result = trampoline::is_allbits(&mut guard, value, mask);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn integer_is_anybits(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let mask = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let mask = Value::from(mask);
    let result = trampoline::is_anybits(&mut guard, value, mask);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn integer_is_nobits(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let mask = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let mask = Value::from(mask);
    let result = trampoline::is_nobits(&mut guard, value, mask);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn integer_size(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::size(&guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
