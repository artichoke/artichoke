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
use mruby_sys::mrb_state;

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
    fn validate_set_args(key: &String, value: &String) -> Result<(), EnvError> {
        if key.find('=').is_some() || key.find('\0').is_some() {
            return Err(EnvError::InvalidSetKey);
        }

        if value.find('\0').is_some() {
            return Err(EnvError::InvalidSetValue);
        }

        Ok(())
    }
    fn set_internal(key: &String, value: &String) {
        env::set_var(key, value);
    }

    fn os_string_to_value(interp: &Artichoke, key: OsString) -> Value {
        let gc_was_enabled = interp.disable_gc();

        let string_value = key.to_str().unwrap();
        let result = Value::convert(interp, string_value);

        if gc_was_enabled {
            interp.enable_gc();
        }

        result
    }

    const STRING_SINGLE_ARG_SPEC: &'static [u8] = b"o\0";

    unsafe fn extract_string_arg(interp: &Artichoke) -> Option<String> {
        let mut other = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mrb: *mut mrb_state = interp.borrow().mrb;
        sys::mrb_get_args(
            mrb,
            Env::STRING_SINGLE_ARG_SPEC.as_ptr() as *const i8,
            other.as_mut_ptr(),
        );
        let other = other.assume_init();

        let arg_value = Value::new(interp, other);

        Some(arg_value.to_s())
    }

    unsafe fn get_internal(interp: &Artichoke, arg_name: String) -> sys::mrb_value {
        if let Some(variable_value) = env::var_os(arg_name) {
            Env::os_string_to_value(interp, variable_value).inner()
        } else {
            sys::mrb_sys_nil_value()
        }
    }
}

impl RubyEnvNativeApi for Env {
    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        if let Some(arg_name) = Env::extract_string_arg(&interp) {
            Env::get_internal(&interp, arg_name)
        } else {
            ArgumentError::raise(interp, "ENV[..] incorrect arguments");
            sys::mrb_sys_nil_value()
        }
    }

    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        if let Some((key, value)) = Env::extract_two_string_args(&interp) {
            match Env::validate_set_args(&key, &value) {
                Ok(_res) => {
                    Env::set_internal(&key, &value);
                    Env::get_internal(&interp, key)
                }
                Err(error) => {
                    // TODO we might need to set errno here...
                    match error {
                        EnvError::InvalidSetKey => {
                            ArgumentError::raise(interp, "Invalid key for ENV set")
                        }
                        EnvError::InvalidSetValue => {
                            ArgumentError::raise(interp, "Invalid value for ENV set")
                        }
                    };
                    sys::mrb_sys_nil_value()
                }
            }
        } else {
            sys::mrb_sys_nil_value()
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
        // given
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        // when
        let PATH_variable_value: String = (&interp)
            .eval(r"ENV['PATH']")
            .unwrap()
            .try_into::<String>()
            .unwrap();

        // then
        assert_eq!(PATH_variable_value.is_empty(), false);
    }

    #[test]
    fn test_env_get_unexisting_variable() {
        // given
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        // when
        let non_existing_env_variable = (&interp)
            .eval(r"ENV['7da5e62c-a121-4bef-ade6-29b60d4e4510']")
            .unwrap()
            .try_into::<String>();

        // then
        assert!(non_existing_env_variable.is_err());
    }

    #[test]
    fn test_env_get_with_incorrect_number_of_args() {
        // given
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        // when
        let env_get_no_args = (&interp).eval(r"ENV[]");
        let env_get_two_args = (&interp).eval(r"ENV['abc', 'def']");

        // then
        assert!(env_get_no_args.is_err());
        assert!(env_get_two_args.is_err());
    }

    #[test]
    fn test_env_set() {
        // given
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        // when
        let env_set_random_var =
            (&interp).eval(r"ENV['8197f6f8-8a35-410b-af99-c94c285b6aba'] = 'val'");

        // then
        assert!(env_set_random_var.is_ok());
        let actual_value = env_set_random_var.unwrap().try_into::<String>().unwrap();
        assert_eq!("val", actual_value);
    }

    #[test]
    fn test_two_set() {
        // given
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        // when
        let _env_set_1 = (&interp).eval(r"ENV['f38e2156-0633-4b06-80e7-9d5fa4b5a553'] = 'val1'");
        let env_set_2_value = (&interp)
            .eval(r"ENV['f38e2156-0633-4b06-80e7-9d5fa4b5a553'] = 'val2'")
            .unwrap()
            .try_into::<String>()
            .unwrap();

        // then
        assert_eq!("val2", env_set_2_value);
    }
}
