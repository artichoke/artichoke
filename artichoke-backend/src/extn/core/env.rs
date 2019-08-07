use crate::convert::Convert;
use crate::def::{ClassLike, Define};
use crate::extn::core::error::{ArgumentError, RubyException};
use log::trace;
use std::env;
use std::mem;

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
    env.borrow_mut()
        .add_method("[]=", Env::set, sys::mrb_args_req(2));

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
    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let _interp = unwrap_interpreter!(mrb);
        sys::mrb_sys_nil_value()
    }

    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        if let Some(arg_name) = Env::extract_string_arg(&interp) {
            match env::var_os(arg_name) {
                Some(value) => match Env::get_internal(&interp, value) {
                    Some(result) => result.inner(),
                    None => sys::mrb_sys_nil_value(),
                },
                None => sys::mrb_sys_nil_value(),
            }
        } else {
            ArgumentError::raise(interp, "wrong number of arguments (given 0, expected 1)");
            sys::mrb_sys_nil_value()
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

    const STRING_SINGLE_ARG_SPEC: &'static [u8] = b"o\0";

    unsafe fn extract_string_arg(interp: &Artichoke) -> Option<String> {
        let mut other = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mrb = interp.borrow().mrb;
        sys::mrb_get_args(
            mrb,
            Env::STRING_SINGLE_ARG_SPEC.as_ptr() as *const i8,
            other.as_mut_ptr(),
        );
        let other = other.assume_init();

        let arg_value = Value::new(interp, other);

        Some(arg_value.to_s())
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

    #[test]
    fn test_with_unexisting_variable() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        let non_existing_env_variable = (&interp)
            .eval(r"ENV['7da5e62c-a121-4bef-ade6-29b60d4e4510']")
            .unwrap()
            .try_into::<String>();

        assert!(non_existing_env_variable.is_err());
    }

    #[test]
    fn test_with_env_get_without_args() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        let non_existing_env_variable = (&interp).eval(r"ENV[]");

        assert!(non_existing_env_variable.is_err());
    }
}
