use std::borrow::Cow;
use std::collections::HashMap;
use std::env;

use bstr::ByteSlice;
use scolapasta_path::{bytes_to_os_str, os_string_to_bytes};

use crate::{ArgumentError, Error, InvalidError};

type Bytes = Vec<u8>;

/// A hash-like accessor for environment variables using platform APIs.
///
/// `System` is an accessor to the host system's environment variables using the
/// functions provided by the [Rust Standard Library] in the
/// [`std::env`] module.
///
/// Use of this `ENV` backend allows Ruby code to access and modify the host
/// system. It is not appropriate to use this backend in embedded or untrusted
/// contexts.
///
/// # Examples
///
/// Fetching an environment variable:
///
/// ```no_run
/// # use spinoso_env::System;
/// # fn example() -> Result<(), spinoso_env::Error> {
/// const ENV: System = System::new();
/// assert!(ENV.get(b"PATH")?.is_some());
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// Setting an environment variable:
///
/// ```no_run
/// # use spinoso_env::System;
/// const ENV: System = System::new();
/// # fn example() -> Result<(), spinoso_env::Error> {
/// ENV.put(b"ENV_BACKEND", Some(b"spinoso_env::System"))?;
/// assert_eq!(
///     std::env::var("ENV_BACKEND").as_deref(),
///     Ok("spinoso_env::System")
/// );
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// [Rust Standard Library]: std
/// [`std::env`]: module@env
#[cfg_attr(docsrs, doc(cfg(feature = "system-env")))]
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct System {
    _private: (),
}

impl System {
    /// Constructs a new, default ENV `System` backend.
    ///
    /// The resulting environment variable accessor has access to the host
    /// system via platform APIs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::System;
    /// const ENV: System = System::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieves the value for environment variable `name`.
    ///
    /// Returns [`None`] if the named variable does not exist. If the retrieved
    /// environment variable value cannot be converted from a [platform string]
    /// to a byte vector, [`None`] is returned.
    ///
    /// # Implementation notes
    ///
    /// This method accesses the host system's environment using [`env::var_os`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use spinoso_env::System;
    /// # fn example() -> Result<(), spinoso_env::Error> {
    /// const ENV: System = System::new();
    /// assert!(ENV.get(b"PATH")?.is_some());
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// # Errors
    ///
    /// If `name` contains a NUL byte, e.g. `b'\0'`, an error is returned.
    ///
    /// If the environment variable name or value cannot be converted from a
    /// byte vector to a [platform string], an error is returned.
    ///
    /// [platform string]: std::ffi::OsString
    #[inline]
    pub fn get(self, name: &[u8]) -> Result<Option<Cow<'static, [u8]>>, ArgumentError> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() {
            // MRI accepts empty names on get and should always return `nil`
            // since empty names are invalid at the OS level.
            Ok(None)
        } else if name.find_byte(b'\0').is_some() {
            let message = "bad environment variable name: contains null byte";
            Err(ArgumentError::with_message(message))
        } else if name.find_byte(b'=').is_some() {
            // MRI accepts names containing '=' on get and should always return
            // `nil` since these names are invalid at the OS level.
            Ok(None)
        } else {
            let name = bytes_to_os_str(name)?;
            if let Some(value) = env::var_os(name) {
                let value = os_string_to_bytes(value).map(Cow::Owned);
                Ok(value.ok())
            } else {
                Ok(None)
            }
        }
    }

    /// Sets the environment variable `name` to `value`.
    ///
    /// If the value given is [`None`] the environment variable is deleted.
    ///
    /// # Implementation notes
    ///
    /// This method accesses the host system's environment using [`env::set_var`]
    /// and [`env::remove_var`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use spinoso_env::System;
    /// # use std::borrow::Cow;
    /// const ENV: System = System::new();
    /// # fn example() -> Result<(), spinoso_env::Error> {
    /// ENV.put(b"RUBY", Some(b"Artichoke"))?;
    /// assert_eq!(ENV.get(b"RUBY")?.as_deref(), Some(&b"Artichoke"[..]));
    /// ENV.put(b"RUBY", None)?;
    /// assert_eq!(ENV.get(b"RUBY")?, None);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If `name` contains a NUL byte, e.g. `b'\0'`, an argument error is
    /// returned.
    ///
    /// If `name` contains an '=' byte, e.g. `b'='`, an `EINVAL` error is
    /// returned.
    ///
    /// If `value` is [`Some`] and contains a NUL byte, e.g. `b'\0'`, an
    /// argument error is returned.
    ///
    /// If the environment variable name or value cannot be converted from a
    /// byte vector to a [platform string], an error is returned.
    ///
    /// [platform string]: std::ffi::OsString
    #[inline]
    pub fn put(self, name: &[u8], value: Option<&[u8]>) -> Result<(), Error> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.find_byte(b'\0').is_some() {
            let message = "bad environment variable name: contains null byte";
            Err(ArgumentError::with_message(message).into())
        } else if let Some(value) = value {
            if value.find_byte(b'\0').is_some() {
                let message = "bad environment variable value: contains null byte";
                return Err(ArgumentError::with_message(message).into());
            }
            if name.find_byte(b'=').is_some() {
                let mut message = b"Invalid argument - setenv(".to_vec();
                message.extend_from_slice(name);
                message.push(b')');
                return Err(InvalidError::from(message).into());
            }
            if name.is_empty() {
                let message = "Invalid argument - setenv()";
                return Err(InvalidError::with_message(message).into());
            }
            let name = bytes_to_os_str(name)?;
            let value = bytes_to_os_str(value)?;
            env::set_var(name, value);
            Ok(())
        } else if name.is_empty() || name.find_byte(b'=').is_some() {
            Ok(())
        } else {
            let name = bytes_to_os_str(name)?;
            env::remove_var(name);
            Ok(())
        }
    }

    /// Serialize the environ to a [`HashMap`].
    ///
    /// Map keys are environment variable names and map values are environment
    /// variable values.
    ///
    /// # Implementation notes
    ///
    /// This method accesses the host system's environment using [`env::vars_os`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use spinoso_env::System;
    /// const ENV: System = System::new();
    /// # fn example() -> Result<(), spinoso_env::Error> {
    /// let map = ENV.to_map()?;
    /// assert!(map.contains_key(&b"PATH"[..]));
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// # Errors
    ///
    /// If any environment variable name or value cannot be converted from a
    /// [platform string] to a byte vector, an error is returned.
    ///
    /// [platform string]: std::ffi::OsString
    #[inline]
    pub fn to_map(self) -> Result<HashMap<Bytes, Bytes>, ArgumentError> {
        let mut map = HashMap::new();
        for (name, value) in env::vars_os() {
            let name = os_string_to_bytes(name)?;
            let value = os_string_to_bytes(value)?;
            map.insert(name, value);
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::System;
    use crate::{ArgumentError, Error, InvalidError};

    const ENV: System = System::new();

    // ```console
    // $ ruby -e 'puts ENV[""].inspect'
    // nil
    // ```
    #[test]
    fn get_name_empty() {
        let name: &[u8] = b"";
        assert_eq!(ENV.get(name), Ok(None));
    }

    // ```consol
    // $ ruby -e 'puts ENV["980b1f2f-a155-4cc6-97f3-cafc3cea2b1a-foo\0bar"].inspect'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]': bad environment variable name: contains null byte (ArgumentError)
    // ```
    #[test]
    fn get_name_nul_byte_err() {
        let name: &[u8] = b"980b1f2f-a155-4cc6-97f3-cafc3cea2b1a-foo\0bar";
        assert_eq!(
            ENV.get(name),
            Err(ArgumentError::with_message(
                "bad environment variable name: contains null byte"
            ))
        );
    }

    // ```console
    // $ ruby -e 'puts ENV["fa7575b4-3224-4fbb-9201-85d54ea95b93-foo=bar"].inspect'
    // nil
    // ```
    #[test]
    fn get_name_equal_byte_unset() {
        let name: &[u8] = b"fa7575b4-3224-4fbb-9201-85d54ea95b93-foo=bar";
        assert_eq!(ENV.get(name), Ok(None));
    }

    // ```console
    // $ ruby -e 'ENV["0f87d787-bf18-437a-a205-ed38d81fa4da-foo\0bar"] = "3427d141-700f-494f-bfa6-877147333249-baz"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': bad environment variable name: contains null byte (ArgumentError)
    // ```
    #[test]
    fn put_name_null_byte_err_set_value() {
        let name: &[u8] = b"0f87d787-bf18-437a-a205-ed38d81fa4da-foo\0bar";
        let value: &[u8] = b"3427d141-700f-494f-bfa6-877147333249-baz";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Argument(ArgumentError::with_message(
                "bad environment variable name: contains null byte"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV["1437e58a-b7e3-4c5e-9b1f-a67b78fe1e42-foo\0bar"] = nil'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': bad environment variable name: contains null byte (ArgumentError)
    // ```
    #[test]
    fn put_name_nul_byte_err_unset_value() {
        let name: &[u8] = b"1437e58a-b7e3-4c5e-9b1f-a67b78fe1e42-foo\0bar";
        assert_eq!(
            ENV.put(name, None),
            Err(Error::Argument(ArgumentError::with_message(
                "bad environment variable name: contains null byte"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV["75b8c10e-4a1d-4f61-9800-5f5c29087edd-foo\0bar"] = "a19660e3-304d-45b8-8746-297a2065a076-baz\0quux"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': bad environment variable name: contains null byte (ArgumentError)
    // ```
    #[test]
    fn put_name_null_byte_set_value_nul_byte_err() {
        let name: &[u8] = b"75b8c10e-4a1d-4f61-9800-5f5c29087edd-foo\0bar";
        let value: &[u8] = b"a19660e3-304d-45b8-8746-297a2065a076-baz\0quux";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Argument(ArgumentError::with_message(
                "bad environment variable name: contains null byte"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV["044f35c0-f711-4b80-8de5-4579075cd754-foo-bar"] = "52bb4d27-6d8a-4a83-90f8-51940ce1f1a7-baz\0quux"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': bad environment variable value: contains null byte (ArgumentError)
    // ```
    #[test]
    fn put_name_set_value_nul_byte_err() {
        let name: &[u8] = b"044f35c0-f711-4b80-8de5-4579075cd754-foo-bar";
        let value: &[u8] = b"52bb4d27-6d8a-4a83-90f8-51940ce1f1a7-baz\0quux";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Argument(ArgumentError::with_message(
                "bad environment variable value: contains null byte"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV["="] = nil'
    // ```
    #[test]
    fn put_name_eq_unset() {
        let name: &[u8] = b"=";
        assert_eq!(ENV.put(name, None), Ok(()));
    }

    // ```console
    // $ ruby -e 'ENV["="] = ""'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': Invalid argument - setenv(=) (Errno::EINVAL)
    // ```
    #[test]
    fn put_name_eq_set_value_empty_byte_err() {
        let name: &[u8] = b"=";
        let value: &[u8] = b"";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Invalid(InvalidError::with_message(
                "Invalid argument - setenv(=)"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV["="] = "4ac79e15-2b8c-4771-8fc8-ff0b095ce7d0-baz-quux"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': Invalid argument - setenv(=) (Errno::EINVAL)
    // ```
    #[test]
    fn put_name_eq_set_value_non_empty_err() {
        let name: &[u8] = b"=";
        let value: &[u8] = b"4ac79e15-2b8c-4771-8fc8-ff0b095ce7d0-baz-quux";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Invalid(InvalidError::with_message(
                "Invalid argument - setenv(=)"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV["="] = "42db3f11-46f5-4cab-93f4-ee543c1634f9-baz\0quux"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': bad environment variable value: contains null byte (ArgumentError)
    // ```
    #[test]
    fn put_name_eq_set_value_null_byte_err() {
        let name: &[u8] = b"=";
        let value: &[u8] = b"42db3f11-46f5-4cab-93f4-ee543c1634f9-baz\0quux";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Argument(ArgumentError::with_message(
                "bad environment variable value: contains null byte"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV["=71cb1499-3a0d-476a-8334-aa7a334f387e-\0"] = "42db3f11-46f5-4cab-93f4-ee543c1634f9-baz\0quux"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': bad environment variable name: contains null byte (ArgumentError)
    // ```
    #[test]
    fn put_name_eq_nul_set_value_null_byte_err() {
        let name: &[u8] = b"=71cb1499-3a0d-476a-8334-aa7a334f387e-\0";
        let value: &[u8] = b"42db3f11-46f5-4cab-93f4-ee543c1634f9-baz\0quux";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Argument(ArgumentError::with_message(
                "bad environment variable name: contains null byte"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV[""] = nil'
    // ```
    #[test]
    fn put_name_empty_value_unset() {
        let name: &[u8] = b"";
        assert_eq!(ENV.put(name, None), Ok(()));
    }

    // ```console
    // $ ruby -e 'ENV[""] = ""'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': Invalid argument - setenv() (Errno::EINVAL)
    // ```
    #[test]
    fn put_name_empty_set_value_empty_err() {
        let name: &[u8] = b"";
        let value: &[u8] = b"";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Invalid(InvalidError::with_message(
                "Invalid argument - setenv()"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV[""] = "157f6920-04e5-4561-8f06-6f00d09c3610-foo"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': Invalid argument - setenv() (Errno::EINVAL)
    // ```
    #[test]
    fn put_name_empty_set_value_non_empty_err() {
        let name: &[u8] = b"";
        let value: &[u8] = b"157f6920-04e5-4561-8f06-6f00d09c3610-foo";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Invalid(InvalidError::with_message(
                "Invalid argument - setenv()"
            )))
        );
    }

    // ```console
    // $ ruby -e 'ENV[""] = "1d50869d-e71a-4347-8b28-b274f34e2892-foo\0bar"'
    // Traceback (most recent call last):
    // 	1: from -e:1:in `<main>'
    // -e:1:in `[]=': bad environment variable value: contains null byte (ArgumentError)
    // ```
    #[test]
    fn put_name_empty_set_value_non_empty_nul_byte_err() {
        let name: &[u8] = b"";
        let value: &[u8] = b"1d50869d-e71a-4347-8b28-b274f34e2892-foo\0bar";
        assert_eq!(
            ENV.put(name, Some(value)),
            Err(Error::Argument(ArgumentError::with_message(
                "bad environment variable value: contains null byte"
            )))
        );
    }

    #[test]
    fn set_get_happy_path() {
        // given
        let name: &[u8] = b"308a3d98-2f87-46fd-b996-ae471a76b64e";
        let value: &[u8] = b"value";
        assert_eq!(ENV.get(name), Ok(None));

        // when
        ENV.put(name, Some(value)).unwrap();
        let retrieved = ENV.get(name);

        // then
        assert_eq!(retrieved.unwrap().unwrap(), value);
    }

    #[test]
    fn set_unset_happy_path() {
        // given
        let name: &[u8] = b"7a6885c3-0c17-4310-a5e7-ed971cac69b6";
        let value: &[u8] = b"value";
        assert_eq!(ENV.get(name), Ok(None));

        // when
        ENV.put(name, Some(value)).unwrap();
        ENV.put(name, None).unwrap();
        let value = ENV.get(name);

        // then
        assert!(value.unwrap().is_none());
    }

    #[test]
    fn to_h() {
        // given
        let name_a: &[u8] = b"3ab42e94-9b7f-4e96-b9c7-ba1738c61f89";
        let value_a: &[u8] = b"value1";
        let name_b: &[u8] = b"3e7bf2b3-9517-444b-bda8-7f5dd3b36648";
        let value_b: &[u8] = b"value2";

        // when
        ENV.put(name_a, Some(value_a)).unwrap();
        ENV.put(name_b, Some(value_b)).unwrap();
        let data = ENV.to_map().unwrap();

        // then
        let value1 = data.get(name_a);
        let value2 = data.get(name_b);
        assert!(value1.is_some());
        assert!(value2.is_some());
        assert_eq!(value1.unwrap(), &value_a);
        assert_eq!(value2.unwrap(), &value_b);
    }
}
