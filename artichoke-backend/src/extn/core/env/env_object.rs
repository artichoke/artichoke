use crate::convert::Convert;
use crate::extn::core::error::{ArgumentError, RubyException};
use std::marker::PhantomData;
use std::mem;

use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

use super::backends::EnvBackend;
use super::errors::EnvError;
use mruby_sys::mrb_state;
use std::collections::HashMap;

pub trait RubyEnvNativeApi {
    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value;
    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value;
    unsafe extern "C" fn env_to_h(mrb: *mut sys::mrb_state, _slf: sys::mrb_value)
        -> sys::mrb_value;
}

pub struct Env<T: EnvBackend> {
    phantom: PhantomData<T>,
}

impl<T: EnvBackend> Env<T> {
    const TWO_STRINGS_ARGS_SPEC: &'static [u8] = b"SS!\0";

    unsafe fn extract_two_string_args(interp: &Artichoke) -> (String, Option<String>) {
        let mut key = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut value = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mrb: *mut mrb_state = interp.borrow().mrb;
        sys::mrb_get_args(
            mrb,
            Self::TWO_STRINGS_ARGS_SPEC.as_ptr() as *const i8,
            key.as_mut_ptr(),
            value.as_mut_ptr(),
        );
        let key = key.assume_init();
        let value = value.assume_init();

        let key_v = Value::new(interp, key);
        let value_v = Value::new(interp, value);

        if sys::mrb_sys_value_is_nil(value_v.inner()) {
            (key_v.to_s(), None)
        } else {
            (key_v.to_s(), Some(value_v.to_s()))
        }
    }

    // set_env
    // This function may panic if key is empty, contains an ASCII equals sign '='
    //      or the NUL character '\0', or when the value contains the NUL character.
    fn validate_set_args(key: &str, value: &Option<String>) -> Result<(), EnvError> {
        if key.find('=').is_some() || key.find('\0').is_some() {
            return Err(EnvError::InvalidSetKey);
        }

        if value.is_some() && value.clone().unwrap().find('\0').is_some() {
            return Err(EnvError::InvalidSetValue);
        }

        Ok(())
    }
    fn set_internal(key: &str, value: Option<&String>) {
        T::set_value(key, value);
    }

    fn string_to_value(interp: &Artichoke, key: &str) -> Value {
        let gc_was_enabled = interp.disable_gc();

        let result = Value::convert(interp, key);

        if gc_was_enabled {
            interp.enable_gc();
        }

        result
    }

    fn hash_map_to_value(interp: &Artichoke, data: HashMap<String, String>) -> Value {
        let gc_was_enabled = interp.disable_gc();

        let result = Value::convert(interp, data);

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
            Self::STRING_SINGLE_ARG_SPEC.as_ptr() as *const i8,
            other.as_mut_ptr(),
        );
        let other = other.assume_init();

        let arg_value = Value::new(interp, other);

        Some(arg_value.to_s())
    }

    unsafe fn get_internal(interp: &Artichoke, arg_name: &str) -> sys::mrb_value {
        let value = T::get_value(arg_name);

        match value {
            Some(variable_value) => Self::string_to_value(interp, &variable_value).inner(),
            None => sys::mrb_sys_nil_value(),
        }
    }

    unsafe fn env_to_h_internal(interp: &Artichoke) -> sys::mrb_value {
        let env = T::convert_to_h();

        Self::hash_map_to_value(interp, env).inner()
    }
}

impl<T: EnvBackend> RubyEnvNativeApi for Env<T> {
    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        if let Some(arg_name) = Self::extract_string_arg(&interp) {
            Self::get_internal(&interp, &arg_name)
        } else {
            ArgumentError::raise(interp, "ENV[..] incorrect arguments");
            sys::mrb_sys_nil_value()
        }
    }

    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        let (key, value) = Self::extract_two_string_args(&interp);

        match Self::validate_set_args(&key, &value) {
            Ok(_res) => {
                Self::set_internal(&key, value.as_ref());
                Self::get_internal(&interp, &key)
            }
            Err(error) => {
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
    }

    unsafe extern "C" fn env_to_h(
        mrb: *mut sys::mrb_state,
        _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        Self::env_to_h_internal(&interp)
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::extn::core::env;
    use crate::sys;

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

    #[test]
    fn test_set_get() {
        // given
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");
        let var_name = "81fdf184-01b4-4248-82db-3b3e8482abf6";
        let var_value = "val";
        let set_var_cmd = format!(r"ENV['{0}'] = '{1}'", var_name, var_value);
        let get_var_cmd = format!(r"ENV['{0}']", var_name);

        // when
        (&interp).eval(set_var_cmd).unwrap();
        let get_result = (&interp).eval(get_var_cmd).unwrap().try_into::<String>();

        // then
        assert!(get_result.is_ok());
        assert_eq!(var_value, get_result.unwrap());
    }

    #[test]
    fn test_set_nil() {
        // given
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");
        let var_name = "9a557fda-73a6-4de8-8999-ddeda18703f2";
        let var_value = "val";
        let set_var_cmd = format!(r"ENV['{0}'] = '{1}'", var_name, var_value);
        let set_nil = format!(r"ENV['{0}'] = nil", var_name);
        let get_var_cmd = format!(r"ENV['{0}']", var_name);

        // when
        (&interp).eval(set_var_cmd).unwrap();
        let first_result = (&interp).eval(&get_var_cmd).unwrap().try_into::<String>();
        (&interp).eval(set_nil).unwrap();
        let last_result = (&interp).eval(&get_var_cmd).unwrap();

        // then
        assert!(first_result.is_ok());
        assert_eq!(var_value, first_result.unwrap());
        unsafe {
            assert!(sys::mrb_sys_value_is_nil(last_result.inner()));
        }
    }

}
