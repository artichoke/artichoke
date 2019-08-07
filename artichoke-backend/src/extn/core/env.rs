use log::trace;
use std::env;

use crate::convert::Convert;
use crate::def::{ClassLike, Define};
use crate::eval::Eval;
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;
use crate::ArtichokeError;
use std::ffi::OsString;

pub fn patch(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.borrow().class_spec::<Env>().is_some() {
        return Ok(());
    }

    let env = interp.borrow_mut().def_class::<Env>("EnvClass", None, None);

    env.borrow_mut()
        .add_method("[]", Env::get, sys::mrb_args_req(1));

    env.borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;

    interp.eval(include_str!("env.rb"))?;

    trace!("Patched ENV onto interpreter");

    Ok(())
}

pub struct Env {}

#[allow(dead_code)]
impl Env {
    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        let key = "PATH";
        match env::var_os(key) {
            Some(value) => match Env::get_internal(&interp, value) {
                Some(result) => result.inner(),
                None => sys::mrb_sys_nil_value(),
            },
            None => sys::mrb_sys_nil_value(),
        }
    }

    fn get_internal(interp: &Artichoke, value: OsString) -> Option<Value> {
        let gc_was_enabled = interp.disable_gc();
        let string_value = value.to_str().unwrap();

        let result = Value::convert(interp, string_value);
        if gc_was_enabled {
            interp.enable_gc();
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::extn::core::env;

    #[test]
    fn test_env_initialized() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_env_has_PATH() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        let PATH_variable_value: String = (&interp)
            .eval(r"ENV['PATH']")
            .unwrap()
            .try_into::<String>()
            .unwrap();

        assert_eq!(PATH_variable_value.is_empty(), false);
    }
}
