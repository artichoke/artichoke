use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

use crate::extn::core::env::Error;

pub trait EnvBackend {
    fn new() -> Self;
    fn get_value(&self, name: &str) -> Result<Option<String>, Error>;
    fn set_value(&self, name: &str, value: Option<&str>) -> Result<(), Error>;
    fn as_map(&self) -> HashMap<String, String>;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EnvStdBackend;

impl EnvBackend for EnvStdBackend {
    fn new() -> Self {
        Self {}
    }

    fn get_value(&self, name: &str) -> Result<Option<String>, Error> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() || name.contains('=') {
            return Err(Error::Os(name.to_owned()));
        }
        if name.contains('\0') {
            return Err(Error::NameContainsNullByte);
        }
        if let Some(value) = env::var_os(name) {
            // todo convert the OsString to bytes
            value
                .to_str()
                .ok_or(Error::Fatal)
                .map(String::from)
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn set_value(&self, name: &str, value: Option<&str>) -> Result<(), Error> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() || name.contains('=') {
            return Err(Error::Os(name.to_owned()));
        }
        if name.contains('\0') {
            return Err(Error::NameContainsNullByte);
        }

        match value {
            Some(value) if value.contains('\0') => Err(Error::ValueContainsNullByte),
            Some(value) => {
                env::set_var(name, value);
                Ok(())
            }
            None => {
                env::remove_var(name);
                Ok(())
            }
        }
    }

    fn as_map(&self) -> HashMap<String, String> {
        env::vars()
            .map(move |(var_name, var_value)| (var_name, var_value))
            .collect()
    }
}

#[derive(Debug)]
pub struct EnvStorage {
    data: RwLock<HashMap<String, String>>,
}

#[allow(dead_code)]
impl EnvStorage {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn put(&self, env_name: &str, env_value: &str) {
        self.data
            .write()
            .unwrap()
            .insert(env_name.to_string(), env_value.to_string());
    }

    pub fn get(&self, env_name: &str) -> Option<String> {
        match self.data.read().unwrap().get(env_name) {
            Some(string_reference) => Some(string_reference.clone()),
            None => None,
        }
    }

    pub fn delete(&self, env_name: &str) {
        self.data.write().unwrap().remove(env_name);
    }

    pub fn get_hash(&self) -> HashMap<String, String> {
        self.data.read().unwrap().clone()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EnvHashMapBackend {
    env_storage: EnvStorage,
}

impl EnvBackend for EnvHashMapBackend {
    fn new() -> Self {
        Self {
            env_storage: EnvStorage::new(),
        }
    }

    fn get_value(&self, name: &str) -> Result<Option<String>, Error> {
        let guard = self.env_storage.data.read().map_err(|_| Error::Fatal)?;
        Ok(guard.get(name).cloned())
    }

    fn set_value(&self, name: &str, value: Option<&str>) -> Result<(), Error> {
        let mut guard = self.env_storage.data.write().map_err(|_| Error::Fatal)?;
        if let Some(value) = value {
            guard.insert(name.to_owned(), value.to_owned());
        } else {
            guard.remove(name);
        }
        Ok(())
    }

    fn as_map(&self) -> HashMap<String, String> {
        self.env_storage.data.read().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{EnvBackend, EnvHashMapBackend};

    #[test]
    fn test_hashmap_backend_set_get() {
        // given
        let backend = EnvHashMapBackend::new();
        let env_name = "308a3d98-2f87-46fd-b996-ae471a76b64e";
        let env_value = "value".to_string();

        // when
        backend.set_value(env_name, Some(&env_value)).unwrap();
        let value = backend.get_value(env_name);

        // then
        assert_eq!(Ok(Some(env_value)), value);
    }

    #[test]
    fn test_hashmap_backend_set_unset() {
        // given
        let backend = EnvHashMapBackend::new();
        let env_name = "7a6885c3-0c17-4310-a5e7-ed971cac69b6";
        let env_value = "value".to_string();

        // when
        backend.set_value(env_name, Some(&env_value)).unwrap();
        backend.set_value(env_name, None).unwrap();
        let value = backend.get_value(env_name);

        // then
        assert_eq!(Ok(None), value);
    }

    #[test]
    fn test_hashmap_backend_to_hashmap() {
        // given
        let backend = EnvHashMapBackend::new();
        let env1_name = "3ab42e94-9b7f-4e96-b9c7-ba1738c61f89";
        let env1_value = "value1".to_string();
        let env2_name = "3e7bf2b3-9517-444b-bda8-7f5dd3b36648";
        let env2_value = "value2".to_string();

        // when
        let size_before = backend.as_map().len();
        backend.set_value(env1_name, Some(&env1_value)).unwrap();
        backend.set_value(env2_name, Some(&env2_value)).unwrap();
        let data = backend.as_map();
        let size_after = data.len();

        // then
        assert_eq!(2, size_after - size_before);
        let value1 = data.get(env1_name);
        let value2 = data.get(env2_name);
        assert!(value1.is_some());
        assert!(value2.is_some());
        assert_eq!(env1_value, value1.unwrap().as_str());
        assert_eq!(env2_value, value2.unwrap().as_str());
    }
}
