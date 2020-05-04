use crate::extn::core::random::{self, trampoline};
use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<random::Random>() {
        return Ok(());
    }
    let spec = class::Spec::new("Random", None, Some(def::rust_data_free::<random::Random>))?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_self_method(
            "new_seed",
            artichoke_random_self_new_seed,
            sys::mrb_args_req(1),
        )?
        .add_self_method("srand", artichoke_random_self_srand, sys::mrb_args_opt(1))?
        .add_self_method(
            "urandom",
            artichoke_random_self_urandom,
            sys::mrb_args_req(1),
        )?
        .add_method(
            "initialize",
            artichoke_random_initialize,
            sys::mrb_args_opt(1),
        )?
        .add_method("==", artichoke_random_eq, sys::mrb_args_opt(1))?
        .add_method("bytes", artichoke_random_bytes, sys::mrb_args_req(1))?
        .add_method("rand", artichoke_random_rand, sys::mrb_args_opt(1))?
        .add_method("seed", artichoke_random_seed, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<random::Random>(spec)?;

    let default = random::Random::interpreter_prng_delegate();
    let default = default
        .try_into_ruby(interp, None)
        .map_err(|_| NotDefinedError::class_constant("Random::DEFAULT"))?;
    interp.define_class_constant::<random::Random>("DEFAULT", default)?;
    let _ = interp.eval(&include_bytes!("random.rb")[..])?;
    trace!("Patched Random onto interpreter");
    Ok(())
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_initialize(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let seed = mrb_get_args!(mrb, optional = 1);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let slf = Value::new(guard.interp(), slf);
    let seed = seed.map(|seed| Value::new(guard.interp(), seed));
    let result = trampoline::initialize(guard.interp(), seed, slf);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_eq(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let rand = Value::new(guard.interp(), slf);
    let other = Value::new(guard.interp(), other);
    let result = trampoline::equal(guard.interp(), rand, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_bytes(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let size = mrb_get_args!(mrb, required = 1);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let rand = Value::new(guard.interp(), slf);
    let size = Value::new(guard.interp(), size);
    let result = trampoline::bytes(guard.interp(), rand, size);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_rand(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let max = mrb_get_args!(mrb, optional = 1);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let rand = Value::new(guard.interp(), slf);
    let max = max.map(|max| Value::new(guard.interp(), max));
    let result = trampoline::rand(guard.interp(), rand, max);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_seed(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let rand = Value::new(guard.interp(), slf);
    let result = trampoline::seed(guard.interp(), rand);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_self_new_seed(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let result = trampoline::new_seed(guard.interp());
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_self_srand(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let number = mrb_get_args!(mrb, optional = 1);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let number = number.map(|number| Value::new(guard.interp(), number));
    let result = trampoline::srand(guard.interp(), number);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_random_self_urandom(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let size = mrb_get_args!(mrb, required = 1);
    let (mut interp, guard) = unwrap_interpreter!(mrb);
    let size = Value::new(guard.interp(), size);
    let result = trampoline::urandom(guard.interp(), size);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}
