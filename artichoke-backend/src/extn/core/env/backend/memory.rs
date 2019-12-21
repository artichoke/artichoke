use std::collections::HashMap;

use crate::convert::Convert;
use crate::extn::core::env::Env;
use crate::extn::core::exception::{ArgumentError, RubyException};
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Default, Clone)]
pub struct Memory {
    store: HashMap<Vec<u8>, Vec<u8>>,
}

impl Memory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Env for Memory {
    fn get(&self, interp: &Artichoke, name: &[u8]) -> Result<Value, Box<dyn RubyException>> {
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
            Ok(interp.convert(None::<Value>))
        } else if memchr::memchr(b'\0', name).is_some() {
            Err(Box::new(ArgumentError::new(
                interp,
                "bad environment variable name: contains null byte",
            )))
        } else if memchr::memchr(b'=', name).is_some() {
            // MRI accepts names containing '=' on get and should always return
            // `nil` since these names are invalid at the OS level.
            Ok(interp.convert(None::<Value>))
        } else if let Some(value) = self.store.get(name) {
            Ok(interp.convert(value.clone()))
        } else {
            Ok(interp.convert(None::<Value>))
        }
    }

    fn put(
        &mut self,
        interp: &Artichoke,
        name: &[u8],
        value: Option<&[u8]>,
    ) -> Result<(), Box<dyn RubyException>> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() {
            // TODO: This should raise `Errno::EINVAL`.
            Err(Box::new(ArgumentError::new(
                interp,
                "Invalid argument - setenv()",
            )))
        } else if memchr::memchr(b'\0', name).is_some() {
            Err(Box::new(ArgumentError::new(
                interp,
                "bad environment variable name: contains null byte",
            )))
        } else if memchr::memchr(b'=', name).is_some() {
            let mut message = b"Invalid argumen - setenv(".to_vec();
            message.extend(name.to_vec());
            message.push(b')');
            // TODO: This should raise `Errno::EINVAL`.
            Err(Box::new(ArgumentError::new_raw(interp, message)))
        } else if let Some(value) = value {
            if memchr::memchr(b'\0', value).is_some() {
                Err(Box::new(ArgumentError::new(
                    interp,
                    "bad environment variable value: contains null byte",
                )))
            } else {
                self.store.insert(name.to_vec(), value.to_vec());
                Ok(())
            }
        } else {
            self.store.remove(name);
            Ok(())
        }
    }

    fn as_map(
        &self,
        interp: &Artichoke,
    ) -> Result<HashMap<Vec<u8>, Vec<u8>>, Box<dyn RubyException>> {
        let _ = interp;
        Ok(self.store.clone())
    }
}

#[cfg(test)]
mod tests {
    use artichoke_core::value::Value as _;

    use crate::extn::core::env::backend::memory::Memory;
    use crate::extn::core::env::Env;

    #[test]
    fn test_hashmap_backend_set_get() {
        let interp = crate::interpreter().expect("init");
        // given
        let mut backend = Memory::new();
        let env_name = "308a3d98-2f87-46fd-b996-ae471a76b64e";
        let env_value = "value";

        // when
        backend
            .put(&interp, env_name.as_bytes(), Some(env_value.as_bytes()))
            .unwrap();
        let value = backend.get(&interp, env_name.as_bytes());

        // then
        assert_eq!(
            Some(env_value.as_bytes()),
            value.unwrap().try_into::<Option<&[u8]>>().unwrap()
        );
    }

    #[test]
    fn test_hashmap_backend_set_unset() {
        let interp = crate::interpreter().expect("init");
        // given
        let mut backend = Memory::new();
        let env_name = "7a6885c3-0c17-4310-a5e7-ed971cac69b6";
        let env_value = "value";

        // when
        backend
            .put(&interp, env_name.as_bytes(), Some(env_value.as_bytes()))
            .unwrap();
        backend.put(&interp, env_name.as_bytes(), None).unwrap();
        let value = backend.get(&interp, env_name.as_bytes());

        // then
        assert_eq!(None, value.unwrap().try_into::<Option<&[u8]>>().unwrap());
    }

    #[test]
    fn test_hashmap_backend_to_hashmap() {
        let interp = crate::interpreter().expect("init");
        // given
        let mut backend = Memory::new();
        let env1_name = "3ab42e94-9b7f-4e96-b9c7-ba1738c61f89";
        let env1_value = "value1";
        let env2_name = "3e7bf2b3-9517-444b-bda8-7f5dd3b36648";
        let env2_value = "value2";

        // when
        let size_before = backend.as_map(&interp).unwrap().len();
        backend
            .put(&interp, env1_name.as_bytes(), Some(env1_value.as_bytes()))
            .unwrap();
        backend
            .put(&interp, env2_name.as_bytes(), Some(env2_value.as_bytes()))
            .unwrap();
        let data = backend.as_map(&interp).unwrap();
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
