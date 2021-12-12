//! FFI glue between the Rust trampolines and the mruby C interpreter.

use std::ffi::CStr;

use super::{trampoline, Rng};
use crate::extn::prelude::*;

const RANDOM_CSTR: &CStr = cstr::cstr!("Random");
static RANDOM_RUBY_SOURCE: &[u8] = include_bytes!("random.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Rng>() {
        return Ok(());
    }

    let spec = class::Spec::new("Random", RANDOM_CSTR, None, Some(def::box_unbox_free::<Rng>))?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_self_method("new_seed", random_self_new_seed, sys::mrb_args_req(1))?
        .add_self_method("srand", random_self_srand, sys::mrb_args_opt(1))?
        .add_self_method("urandom", random_self_urandom, sys::mrb_args_req(1))?
        .add_method("initialize", random_initialize, sys::mrb_args_opt(1))?
        .add_method("==", random_eq, sys::mrb_args_opt(1))?
        .add_method("bytes", random_bytes, sys::mrb_args_req(1))?
        .add_method("rand", random_rand, sys::mrb_args_opt(1))?
        .add_method("seed", random_seed, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<Rng>(spec)?;

    let default = Rng::Global;
    let default = Rng::alloc_value(default, interp).map_err(|_| NotDefinedError::class_constant("Random::DEFAULT"))?;
    interp.define_class_constant::<Rng>("DEFAULT", default)?;
    interp.eval(RANDOM_RUBY_SOURCE)?;

    Ok(())
}

unsafe extern "C" fn random_initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let seed = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let slf = Value::from(slf);
    let seed = seed.map(Value::from);
    let result = trampoline::initialize(&mut guard, seed, slf);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn random_eq(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let rand = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::equal(&mut guard, rand, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn random_bytes(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let size = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let rand = Value::from(slf);
    let size = Value::from(size);
    let result = trampoline::bytes(&mut guard, rand, size);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn random_rand(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let max = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let rand = Value::from(slf);
    let max = max.map(Value::from);
    let result = trampoline::rand(&mut guard, rand, max);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn random_seed(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let rand = Value::from(slf);
    let result = trampoline::seed(&mut guard, rand);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn random_self_new_seed(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::new_seed(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn random_self_srand(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let number = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let number = number.map(Value::from);
    let result = trampoline::srand(&mut guard, number);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn random_self_urandom(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let size = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let size = Value::from(size);
    let result = trampoline::urandom(&mut guard, size);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
