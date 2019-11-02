use crate::def::{ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::env;
use crate::extn::core::exception;
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().class_spec::<env::ENV>().is_some() {
        return Ok(());
    }
    interp.eval(include_str!("env.rb"))?;

    let env = interp
        .0
        .borrow_mut()
        .def_class::<env::ENV>("EnvClass", None, None);

    env.borrow_mut().mrb_value_is_rust_backed(true);

    env.borrow_mut()
        .add_method("initialize", artichoke_env_initialize, sys::mrb_args_none());
    env.borrow_mut()
        .add_method("[]", artichoke_env_element_reference, sys::mrb_args_req(1));
    env.borrow_mut().add_method(
        "[]=",
        artichoke_env_element_assignment,
        sys::mrb_args_req(2),
    );
    env.borrow_mut()
        .add_method("to_h", artichoke_env_to_h, sys::mrb_args_none());

    env.borrow().define(interp)?;

    interp.eval("ENV = EnvClass.new")?;

    trace!("Patched ENV onto interpreter");

    Ok(())
}

#[no_mangle]
unsafe extern "C" fn artichoke_env_initialize(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let result = env::initialize(&interp, Some(slf));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_env_element_reference(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let name = mrb_get_args!(mrb, required = 1);
    let interp = unwrap_interpreter!(mrb);
    let obj = Value::new(&interp, slf);
    let name = Value::new(&interp, name);
    let result = env::element_reference(&interp, obj, &name);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_env_element_assignment(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let (name, value) = mrb_get_args!(mrb, required = 2);
    let interp = unwrap_interpreter!(mrb);
    let obj = Value::new(&interp, slf);
    let name = Value::new(&interp, name);
    let value = Value::new(&interp, value);
    let result = env::element_assignment(&interp, obj, &name, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_env_to_h(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let obj = Value::new(&interp, slf);
    let result = env::to_h(&interp, obj);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
