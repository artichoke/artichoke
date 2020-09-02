use bstr::ByteSlice;
use std::borrow::Cow;
use std::collections::HashMap;

use crate::{ArgumentError, Error, InvalidError};

type Bytes = Vec<u8>;

/// An in-memory fake store of environment variables.
///
/// `Memory` offers the same API as other environment variable backends in this
/// crate, but does not access or mutate the underlying system.
///
/// This backend is suitable for running in untrusted environments such as a
/// WebAssembly binary, testing environments, or in embedded contexts.
///
/// # Examples
///
/// Fetching an environment variable:
///
/// ```
/// # use spinoso_env::Memory;
/// let env = Memory::new();
/// // `Memory` backends start out empty.
/// assert_eq!(env.get(b"PATH"), Ok(None));
/// ```
///
/// Setting an environment variable:
///
/// ```
/// # use spinoso_env::Memory;
/// # use std::borrow::Cow;
/// # fn example() -> Result<(), spinoso_env::Error> {
/// let mut env = Memory::new();
/// env.put(b"ENV_BACKEND", Some(b"spinoso_env::Memory"))?;
/// assert_eq!(env.get(b"ENV_BACKEND")?, Some(Cow::Borrowed(&b"spinoso_env::Memory"[..])));
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    store: HashMap<Bytes, Bytes>,
}

impl Memory {
    /// Constructs a new, empty ENV `Memory` backend.
    ///
    /// The resulting environment variable accessor has no access to the
    /// underlying host operating system. The returned accessor uses a virtual
    /// environment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::Memory;
    /// let env = Memory::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let store = HashMap::new();
        Self { store }
    }

    /// Retrieves the value for environment variable `name`.
    ///
    /// Returns [`None`] if the named variable does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::Memory;
    /// let env = Memory::new();
    /// assert_eq!(env.get(b"PATH"), Ok(None));
    /// ```
    ///
    /// # Errors
    ///
    /// If `name` contains a NUL byte, e.g. `b'\0'`, an error is returned.
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
        } else if let Some(value) = self.store.get(name) {
            Ok(Some(Cow::Borrowed(value)))
        } else {
            Ok(None)
        }
    }

    /// Sets the environment variable `name` to `value`.
    ///
    /// If the value given is [`None`] the environment variable is deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::Memory;
    /// # use std::borrow::Cow;
    /// # fn example() -> Result<(), spinoso_env::Error> {
    /// let mut env = Memory::new();
    /// env.put(b"RUBY", Some(b"Artichoke"))?;
    /// assert_eq!(env.get(b"RUBY")?, Some(Cow::Borrowed(&b"Artichoke"[..])));
    /// env.put(b"RUBY", None)?;
    /// assert_eq!(env.get(b"RUBY")?, None);
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
            let message = "Invalid argument - setenv()";
            return Err(InvalidError::with_message(message).into());
        }
        if name.find_byte(b'\0').is_some() {
            let message = "bad environment variable name: contains null byte";
            return Err(ArgumentError::with_message(message).into());
        }
        if name.find_byte(b'=').is_some() {
            if value.is_none() {
                return Ok(());
            }
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
            self.store.insert(name.to_vec(), value.to_vec());
            Ok(())
        } else {
            self.store.remove(name);
            Ok(())
        }
    }

    /// Serialize the environ to a [`HashMap`].
    ///
    /// Map keys are environment variable names and map values are environment
    /// variable values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::Memory;
    /// # fn example() -> Result<(), spinoso_env::Error> {
    /// let env = Memory::new();
    /// let map = env.to_map()?;
    /// assert!(map.is_empty());
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// # Errors
    ///
    /// This function is infallible.
    #[inline]
    pub fn to_map(&self) -> Result<HashMap<Bytes, Bytes>, ArgumentError> {
        Ok(self.store.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use crate::ArgumentError;

    #[test]
    fn get_name_null_byte_err() {
        let backend = Memory::new();
        let name: &[u8] = b"foo\0bar";
        assert_eq!(
            backend.get(name),
            Err(ArgumentError::with_message(
                "bad environment variable name: contains null byte"
            ))
        );
    }

    #[test]
    fn get_name_equal_byte_unset() {
        let backend = Memory::new();
        let name: &[u8] = b"foo=bar";
        assert_eq!(backend.get(name), Ok(None));
    }

    #[test]
    fn set_get() {
        // given
        let mut backend = Memory::new();
        let name: &[u8] = b"308a3d98-2f87-46fd-b996-ae471a76b64e";
        let value: &[u8] = b"value";
        assert_eq!(backend.get(name), Ok(None));

        // when
        backend.put(name, Some(value)).unwrap();
        let retrieved = backend.get(name);

        // then
        assert_eq!(retrieved.unwrap().unwrap(), value);
    }

    #[test]
    fn set_unset() {
        // given
        let mut backend = Memory::new();
        let name: &[u8] = b"7a6885c3-0c17-4310-a5e7-ed971cac69b6";
        let value: &[u8] = b"value";
        assert_eq!(backend.get(name), Ok(None));

        // when
        backend.put(name, Some(value)).unwrap();
        backend.put(name, None).unwrap();
        let value = backend.get(name);

        // then
        assert!(value.unwrap().is_none());
    }

    #[test]
    fn to_h() {
        // given
        let mut backend = Memory::new();
        let name_a: &[u8] = b"3ab42e94-9b7f-4e96-b9c7-ba1738c61f89";
        let value_a: &[u8] = b"value1";
        let name_b: &[u8] = b"3e7bf2b3-9517-444b-bda8-7f5dd3b36648";
        let value_b: &[u8] = b"value2";

        // when
        let size_before = backend.to_map().unwrap().len();
        backend.put(name_a, Some(value_a)).unwrap();
        backend.put(name_b, Some(value_b)).unwrap();
        let data = backend.to_map().unwrap();
        let size_after = data.len();

        // then
        assert_eq!(size_after - size_before, 2);
        let value1 = data.get(name_a);
        let value2 = data.get(name_b);
        assert!(value1.is_some());
        assert!(value2.is_some());
        assert_eq!(value1.unwrap(), &value_a);
        assert_eq!(value2.unwrap(), &value_b);
    }
}
