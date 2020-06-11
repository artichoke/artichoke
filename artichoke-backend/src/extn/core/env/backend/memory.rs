use bstr::ByteSlice;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

use crate::extn::core::env::backend::{EnvArgumentError, EnvType};
use crate::extn::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    store: HashMap<Vec<u8>, Vec<u8>>,
}

impl Memory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl EnvType for Memory {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn get<'a>(&'a self, name: &[u8]) -> Result<Option<Cow<'a, [u8]>>, Exception> {
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
            return Err(Exception::from(EnvArgumentError::from(
                "bad environment variable name: contains null byte",
            )));
        }
        if name.find_byte(b'=').is_some() {
            // MRI accepts names containing '=' on get and should always return
            // `nil` since these names are invalid at the OS level.
            Ok(None)
        } else {
            Ok(self.store.get(name).map(Cow::from))
        }
    }

    fn put(&mut self, name: &[u8], value: Option<&[u8]>) -> Result<(), Exception> {
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
            return Err(Exception::from(EnvArgumentError::from(
                "Invalid argument - setenv()",
            )));
        }
        if name.find_byte(b'\0').is_some() {
            if value.is_none() {
                return Ok(());
            }
            return Err(Exception::from(EnvArgumentError::from(
                "bad environment variable name: contains null byte",
            )));
        }
        if name.find_byte(b'=').is_some() {
            if value.is_none() {
                return Ok(());
            }
            let mut message = b"Invalid argument - setenv(".to_vec();
            message.extend(name.to_vec());
            message.push(b')');
            // TODO: This should raise `Errno::EINVAL`.
            return Err(Exception::from(EnvArgumentError::from(message)));
        }
        if let Some(value) = value {
            if value.find_byte(b'\0').is_some() {
                return Err(Exception::from(EnvArgumentError::from(
                    "bad environment variable value: contains null byte",
                )));
            }
            self.store.insert(name.to_vec(), value.to_vec());
            Ok(())
        } else {
            self.store.remove(name);
            Ok(())
        }
    }

    fn to_map(&self) -> Result<HashMap<Vec<u8>, Vec<u8>>, Exception> {
        Ok(self.store.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::extn::core::env::backend::memory::Memory;
    use crate::extn::core::env::backend::EnvType;

    #[test]
    fn test_hashmap_backend_set_get() {
        // given
        let mut backend = Memory::new();
        let env_name = "308a3d98-2f87-46fd-b996-ae471a76b64e";
        let env_value = "value";

        // when
        backend
            .put(env_name.as_bytes(), Some(env_value.as_bytes()))
            .unwrap();
        let value = backend.get(env_name.as_bytes());

        // then
        assert_eq!(
            Some(env_value.as_bytes()),
            value.unwrap().map(|value| value.into_owned()).as_deref()
        );
    }

    #[test]
    fn test_hashmap_backend_set_unset() {
        // given
        let mut backend = Memory::new();
        let env_name = "7a6885c3-0c17-4310-a5e7-ed971cac69b6";
        let env_value = "value";

        // when
        backend
            .put(env_name.as_bytes(), Some(env_value.as_bytes()))
            .unwrap();
        backend.put(env_name.as_bytes(), None).unwrap();
        let value = backend.get(env_name.as_bytes());

        // then
        assert!(value.unwrap().is_none());
    }

    #[test]
    fn test_hashmap_backend_to_hashmap() {
        // given
        let mut backend = Memory::new();
        let env1_name = "3ab42e94-9b7f-4e96-b9c7-ba1738c61f89";
        let env1_value = "value1";
        let env2_name = "3e7bf2b3-9517-444b-bda8-7f5dd3b36648";
        let env2_value = "value2";

        // when
        let size_before = backend.to_map().unwrap().len();
        backend
            .put(env1_name.as_bytes(), Some(env1_value.as_bytes()))
            .unwrap();
        backend
            .put(env2_name.as_bytes(), Some(env2_value.as_bytes()))
            .unwrap();
        let data = backend.to_map().unwrap();
        let size_after = data.len();

        // then
        assert_eq!(2, size_after - size_before);
        let value1 = data.get(env1_name.as_bytes());
        let value2 = data.get(env2_name.as_bytes());
        assert!(value1.is_some());
        assert!(value2.is_some());
        assert_eq!(env1_value.as_bytes(), value1.unwrap().as_slice());
        assert_eq!(env2_value.as_bytes(), value2.unwrap().as_slice());
    }
}
