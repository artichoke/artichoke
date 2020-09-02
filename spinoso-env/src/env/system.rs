use bstr::{ByteSlice, ByteVec};
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;

use crate::{ArgumentError, Error, InvalidError};

type Bytes = Vec<u8>;

/// ENV is a hash-like accessor for environment variables using the platform
/// APIs for accessing the environment.
///
/// `System` is an accessor to the host system's environment variables using the
/// functions provided by the [Rust Standard Library] in the
/// [`std::env`](module@env) module.
///
/// Use of this `ENV` backend allows Ruby code to access and modify the host
/// system. It is not appropriate to use this backend in embedded or untrusted
/// contexts.
///
/// # Examples
///
/// Fetching an environment variable:
///
/// ```
/// # use spinoso_env::System;
/// const ENV: System = System::new();
/// assert!(matches!(ENV.get(b"PATH"), Ok(Some(_))));
/// ```
///
/// Setting an environment variable:
///
/// ```
/// # use spinoso_env::System;
/// const ENV: System = System::new();
/// # fn example() -> Result<(), spinoso_env::Error> {
/// ENV.put(b"ENV_BACKEND", Some(b"spinoso_env::System"))?;
/// assert_eq!(std::env::var("ENV_BACKEND"), Ok(String::from("spinoso_env::System")));
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// [Rust Standard Library]: std
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct System {
    _private: (),
}

impl System {
    /// Constructs a new, default ENV `System` backend.
    ///
    /// The resulting environment variable accessor has access to the underlying
    /// host operating system.
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
    /// This method accesses the host system's environment using
    /// [`env::var_os`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::System;
    /// const ENV: System = System::new();
    /// assert!(matches!(ENV.get(b"PATH"), Ok(Some(_))));
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
    pub fn get<'a>(&'a self, name: &[u8]) -> Result<Option<Cow<'a, [u8]>>, ArgumentError> {
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
            return Ok(None);
        }
        if name.find_byte(b'\0').is_some() {
            let message = "bad environment variable name: contains null byte";
            return Err(ArgumentError::with_message(message));
        }
        if name.find_byte(b'=').is_some() {
            // MRI accepts names containing '=' on get and should always return
            // `nil` since these names are invalid at the OS level.
            Ok(None)
        } else {
            let name = name.to_os_str().map_err(|_| {
                ArgumentError::with_message("name could not be converted to a platform string")
            })?;
            if let Some(value) = env::var_os(name) {
                let value = Vec::from_os_string(value).map(Cow::Owned);
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
    /// This method accesses the host system's environment using
    /// [`env::set_var`] and [`env::remove_var`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::System;
    /// # use std::borrow::Cow;
    /// const ENV: System = System::new();
    /// # fn example() -> Result<(), spinoso_env::Error> {
    /// ENV.put(b"RUBY", Some(b"Artichoke"))?;
    /// assert_eq!(ENV.get(b"RUBY")?, Some(Cow::Borrowed(&b"Artichoke"[..])));
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
    pub fn put(&mut self, name: &[u8], value: Option<&[u8]>) -> Result<(), Error> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() {
            if value.is_none() {
                return Ok(());
            }
            // TODO: This should raise `Errno::EINVAL`.
            return Err(ArgumentError::with_message("Invalid argument - setenv()").into());
        }
        if name.find_byte(b'\0').is_some() {
            if value.is_none() {
                return Ok(());
            }
            let message = "bad environment variable name: contains null byte";
            return Err(InvalidError::with_message(message).into());
        }
        if name.find_byte(b'=').is_some() {
            let mut message = b"Invalid argument - setenv(".to_vec();
            message.extend(name.to_vec());
            message.push(b')');
            return Err(InvalidError::from(message).into());
        }
        if let Some(value) = value {
            if value.find_byte(b'\0').is_some() {
                let message = "bad environment variable value: contains null byte";
                return Err(ArgumentError::with_message(message).into());
            }
            let name = name.to_os_str().map_err(|_| {
                ArgumentError::with_message("name could not be converted to a platform string")
            })?;
            let value = value.to_os_str().map_err(|_| {
                ArgumentError::with_message("value could not be converted to a platform string")
            })?;
            env::set_var(name, value);
            Ok(())
        } else {
            let name = name.to_os_str().map_err(|_| {
                ArgumentError::with_message("name could not be converted to a platform string")
            })?;
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
    /// This method accesses the host system's environment using
    /// [`env::vars_os`].
    ///
    /// # Examples
    ///
    /// ```
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
    pub fn to_map(&self) -> Result<HashMap<Bytes, Bytes>, ArgumentError> {
        let mut map = HashMap::new();
        for (name, value) in env::vars_os() {
            let name = Vec::from_os_string(name).map_err(|_| {
                ArgumentError::with_message("name could not be converted to a platform string")
            })?;
            let value = Vec::from_os_string(value).map_err(|_| {
                ArgumentError::with_message("value could not be converted to a platform string")
            })?;
            map.insert(name, value);
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::System;
    use crate::ArgumentError;

    const ENV: System = System::new();

    #[test]
    fn get_name_null_byte_err() {
        let name: &[u8] = b"foo\0bar";
        assert_eq!(
            ENV.get(name),
            Err(ArgumentError::with_message(
                "bad environment variable name: contains null byte"
            ))
        );
    }

    #[test]
    fn get_name_equal_byte_unset() {
        let name: &[u8] = b"foo=bar";
        assert_eq!(ENV.get(name), Ok(None));
    }

    #[test]
    fn set_get() {
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
    fn set_unset() {
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
