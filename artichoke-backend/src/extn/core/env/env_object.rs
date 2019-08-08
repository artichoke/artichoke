use crate::convert::Convert;
use crate::extn::core::error::{ArgumentError, RubyException};
use std::env;
use std::ffi::OsString;
use std::mem;

use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

use super::errors::EnvError;

pub trait RubyEnvNativeApi {
    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value;
    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value;
}

pub struct Env;

impl Env {
    fn extract_two_string_args(_interp: &Artichoke) -> Option<(String, String)> {
        None
    }
    // set_env
    // This function may panic if key is empty, contains an ASCII equals sign '='
    //      or the NUL character '\0', or when the value contains the NUL character.
    fn set_internal(
        _interp: &Artichoke,
        _key: String,
        _value: String,
    ) -> Result<sys::mrb_value, EnvError> {
        // env::set_var(key, value);
        Err(EnvError::InvalidSetArguments)
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

impl RubyEnvNativeApi for Env {
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
            ArgumentError::raise(interp, "ENV[..] incorrect arguments");
            sys::mrb_sys_nil_value()
        }
    }

    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        let _arguments = Env::extract_two_string_args(&interp);

        match Env::set_internal(&interp, "".to_string(), "".to_string()) {
            Ok(_value) => sys::mrb_sys_nil_value(),
            Err(_error) => {
                ArgumentError::raise(interp, "ENV[..] incorrect arguments");
                sys::mrb_sys_nil_value()
            }
        }
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
    fn test_env_get_PATH() {
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
    fn test_env_get_unexisting_variable() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        let non_existing_env_variable = (&interp)
            .eval(r"ENV['7da5e62c-a121-4bef-ade6-29b60d4e4510']")
            .unwrap()
            .try_into::<String>();

        assert!(non_existing_env_variable.is_err());
    }

    #[test]
    fn test_env_get_with_incorrect_number_of_args() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        let env_get_no_args = (&interp).eval(r"ENV[]");
        assert!(env_get_no_args.is_err());

        let env_get_two_args = (&interp).eval(r"ENV['abc', 'def']");
        assert!(env_get_two_args.is_err());
    }
}
