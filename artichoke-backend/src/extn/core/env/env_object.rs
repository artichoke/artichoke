use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::extn::core::error::{ArgumentError, RubyException, RuntimeError};
use std::mem;

use crate::extn::core::env::backends::EnvBackend;
use crate::extn::core::env::Error;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

pub trait RubyEnvNativeApi {
    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value;
    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;
    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;
    unsafe extern "C" fn env_to_h(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;
}

pub struct Env<T: EnvBackend> {
    backend: T,
}

impl<T: EnvBackend> Env<T> {
    pub fn new() -> Self {
        Self { backend: T::new() }
    }

    const TWO_STRINGS_ARGS_SPEC: &'static [u8] = b"SS!\0";

    unsafe fn extract_two_string_args(interp: &Artichoke) -> (String, Option<String>) {
        let mut key = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut value = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mrb = interp.0.borrow().mrb;

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

    unsafe fn set_internal(&self, interp: &Artichoke) -> Result<Value, Error> {
        let (key, value) = Self::extract_two_string_args(interp);

        self.backend
            .set_value(&key, value.as_ref().map(String::as_str))
            .map(|_| Value::convert(&interp, value))
    }

    const STRING_SINGLE_ARG_SPEC: &'static [u8] = b"S\0";

    unsafe fn extract_string_arg(interp: &Artichoke) -> Result<String, Error> {
        let mut other = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mrb = interp.0.borrow().mrb;
        sys::mrb_get_args(
            mrb,
            Self::STRING_SINGLE_ARG_SPEC.as_ptr() as *const i8,
            other.as_mut_ptr(),
        );
        let other = other.assume_init();

        let name = Value::new(interp, other);
        // argspec guarantees a String
        String::try_convert(interp, name).map_err(|_| Error::Fatal)
    }

    unsafe fn get_internal(&self, interp: &Artichoke) -> Result<Value, Error> {
        let name = Self::extract_string_arg(interp)?;
        self.backend
            .get_value(&name)
            .map(|value| Value::convert(&interp, value))
    }

    unsafe fn env_to_h_internal(&self, interp: &Artichoke) -> Value {
        let env = self.backend.as_map();
        Value::convert(interp, env)
    }
}

impl<T: EnvBackend> RubyEnvNativeApi for Env<T>
where
    T: 'static,
{
    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        let backend = Self::new();
        let backend = backend.try_into_ruby(&interp, Some(slf));

        if let Ok(backend) = backend {
            backend.inner()
        } else {
            RuntimeError::raise(interp, "Cannot initialize new ENV object")
        }
    }

    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        let result = Self::try_from_ruby(&interp, &Value::new(&interp, slf))
            .map_err(|_| Error::Fatal)
            .and_then(|env| {
                let borrow = env.borrow();
                borrow.get_internal(&interp)
            });
        match result {
            Ok(value) => value.inner(),
            Err(Error::Fatal) => RuntimeError::raise(interp, "fatal ENV error"),
            Err(Error::NameContainsNullByte) => {
                ArgumentError::raise(interp, "bad environment variable name: contains null byte")
            }
            Err(Error::Os(arg)) => {
                let fmt = Value::convert(&interp, arg);
                RuntimeError::raisef(interp, "invalid argument - setenv(%S)", vec![fmt])
            }
            Err(Error::ValueContainsNullByte) => {
                ArgumentError::raise(interp, "bad environment variable value: contains null byte")
            }
        }
    }

    unsafe extern "C" fn set(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);

        let result = Self::try_from_ruby(&interp, &Value::new(&interp, slf))
            .map_err(|_| Error::Fatal)
            .and_then(|env| {
                let borrow = env.borrow();
                borrow.set_internal(&interp)
            });
        match result {
            Ok(value) => value.inner(),
            Err(Error::Fatal) => RuntimeError::raise(interp, "fatal ENV error"),
            Err(Error::NameContainsNullByte) => {
                ArgumentError::raise(interp, "bad environment variable name: contains null byte")
            }
            Err(Error::Os(arg)) => {
                let fmt = Value::convert(&interp, arg);
                RuntimeError::raisef(interp, "invalid argument - setenv(%S)", vec![fmt])
            }
            Err(Error::ValueContainsNullByte) => {
                ArgumentError::raise(interp, "bad environment variable value: contains null byte")
            }
        }
    }

    unsafe extern "C" fn env_to_h(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp: Artichoke = unwrap_interpreter!(mrb);

        if let Ok(env) = Self::try_from_ruby(&interp, &Value::new(&interp, slf)) {
            let borrow = env.borrow();
            borrow.env_to_h_internal(&interp).inner()
        } else {
            RuntimeError::raise(interp, "fatal ENV error")
        }
    }
}

impl<T: EnvBackend> RustBackedValue for Env<T> where T: 'static {}

#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::extn::core::env;
    use crate::value::Value;
    use crate::ArtichokeError;

    #[test]
    fn test_env_initialized() {
        let interp = crate::interpreter().expect("init");
        env::init(&interp).expect("env init");
    }

    #[test]
    fn test_env_get_unexisting_variable() {
        // given
        let interp = crate::interpreter().expect("init");
        env::init(&interp).expect("env init");

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
        env::init(&interp).expect("env init");

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
        env::init(&interp).expect("env init");

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
        env::init(&interp).expect("env init");

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
        env::init(&interp).expect("env init");
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
        env::init(&interp).expect("env init");
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
        assert!(last_result.try_into::<Option<Value>>().unwrap().is_none());
    }

    #[test]
    fn test_get_name_with_null_byte() {
        // given
        let interp = crate::interpreter().expect("init");
        env::init(&interp).expect("env init");

        // when
        let result = interp.eval(r#"ENV["bar\0"]"#);

        // then
        assert!(result.is_err());
        let expected_backtrace = r#"
(eval):1: bad environment variable name: contains null byte (ArgumentError)
(eval):1
        "#;
        assert_eq!(
            result.map(|_| ()),
            Err(ArtichokeError::Exec(expected_backtrace.trim().to_owned()))
        );
    }

    #[test]
    fn test_set_name_with_null_byte() {
        // given
        let interp = crate::interpreter().expect("init");
        env::init(&interp).expect("env init");

        // when
        let result = interp.eval(r#"ENV["bar\0"] = "foo""#);

        // then
        assert!(result.is_err());
        let expected_backtrace = r#"
(eval):1: bad environment variable name: contains null byte (ArgumentError)
(eval):1
        "#;
        assert_eq!(
            result.map(|_| ()),
            Err(ArtichokeError::Exec(expected_backtrace.trim().to_owned()))
        );
    }

    #[test]
    fn test_set_value_with_null_byte() {
        // given
        let interp = crate::interpreter().expect("init");
        env::init(&interp).expect("env init");

        // when
        let result = interp.eval(r#"ENV['bar'] = "foo\0""#);

        // then
        assert!(result.is_err());
        let expected_backtrace = r#"
(eval):1: bad environment variable value: contains null byte (ArgumentError)
(eval):1
        "#;
        assert_eq!(
            result.map(|_| ()),
            Err(ArtichokeError::Exec(expected_backtrace.trim().to_owned()))
        );
    }

    #[test]
    fn test_set_value_with_equal() {
        // given
        let interp = crate::interpreter().expect("init");
        env::init(&interp).expect("env init");

        // when
        let result = interp.eval(r#"ENV['bar='] = "foo""#);

        // then
        assert!(result.is_err());
        let expected_backtrace = r#"
(eval):1: invalid argument - setenv(bar=) (RuntimeError)
(eval):1
        "#;
        assert_eq!(
            result.map(|_| ()),
            Err(ArtichokeError::Exec(expected_backtrace.trim().to_owned()))
        );
    }
}
