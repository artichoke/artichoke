use std::ffi::CStr;

use crate::extn::core::artichoke;
use crate::extn::core::kernel::{self, trampoline};
use crate::extn::prelude::*;

const KERNEL_CSTR: &CStr = qed::const_cstr_from_str!("Kernel\0");
static KERNEL_RUBY_SOURCE: &[u8] = include_bytes!("kernel.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<kernel::Kernel>() {
        return Ok(());
    }

    let spec = module::Spec::new(interp, "Kernel", KERNEL_CSTR, None)?;
    module::Builder::for_spec(interp, &spec)
        .add_method("require", kernel_require, sys::mrb_args_rest())?
        .add_method("require_relative", kernel_require_relative, sys::mrb_args_rest())?
        .add_method("load", kernel_load, sys::mrb_args_rest())?
        .add_method("p", kernel_p, sys::mrb_args_rest())?
        .add_method("print", kernel_print, sys::mrb_args_rest())?
        .add_method("puts", kernel_puts, sys::mrb_args_rest())?
        .define()?;
    interp.def_module::<kernel::Kernel>(spec)?;
    interp.eval(KERNEL_RUBY_SOURCE)?;

    // Some `Kernel` functions are implemented with methods in the
    // `Artichoke::Kernel` module. These functions are delegated to by Ruby
    // implementations of the `Kernel` methods that marshal arguments and handle
    // exceptions.
    let scope = interp
        .module_spec::<artichoke::Artichoke>()?
        .map(EnclosingRubyScope::module)
        .ok_or_else(|| NotDefinedError::module("Artichoke"))?;
    let spec = module::Spec::new(interp, "Kernel", KERNEL_CSTR, Some(scope))?;
    module::Builder::for_spec(interp, &spec)
        .add_method("Integer", kernel_integer, sys::mrb_args_req_and_opt(1, 1))?
        .add_self_method("Integer", kernel_integer, sys::mrb_args_req_and_opt(1, 1))?
        .define()?;
    interp.def_module::<artichoke::Kernel>(spec)?;

    Ok(())
}

unsafe extern "C-unwind" fn kernel_integer(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (arg, base) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let arg = Value::from(arg);
    let base = base.map(Value::from);
    let result = trampoline::integer(&mut guard, arg, base);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn kernel_load(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let file = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let file = Value::from(file);
    let result = trampoline::load(&mut guard, file);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn kernel_p(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let args = args.iter().copied().map(Value::from);
    let result = trampoline::p(&mut guard, args);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn kernel_print(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let args = args.iter().copied().map(Value::from);
    let result = trampoline::print(&mut guard, args);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn kernel_puts(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let args = args.iter().copied().map(Value::from);
    let result = trampoline::puts(&mut guard, args);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn kernel_require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let file = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let file = Value::from(file);
    let result = trampoline::require(&mut guard, file);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C-unwind" fn kernel_require_relative(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let file = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let file = Value::from(file);
    let result = trampoline::require_relative(&mut guard, file);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
