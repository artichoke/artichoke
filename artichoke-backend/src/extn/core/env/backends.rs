use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::sync::RwLock;

pub trait EnvBackend {
    fn get_value(env_name: &str) -> Option<String>;
    fn set_value(env_name: &str, env_value: Option<&String>);
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EnvStdBackend;

impl EnvBackend for EnvStdBackend {
    fn get_value(env_name: &str) -> Option<String> {
        if let Some(value) = env::var_os(env_name) {
            Some(String::from(value.to_str().unwrap()))
        } else {
            None
        }
    }

    fn set_value(env_name: &str, env_value: Option<&String>) {
        match env_value {
            Some(string) => env::set_var(OsString::from(env_name), OsString::from(string)),
            None => env::remove_var(OsString::from(env_name)),
        }
    }
}

#[derive(Debug)]
pub struct EnvStorage {
    data: RwLock<HashMap<String, String>>,
}

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
}

mod hashmap_storage {
    lazy_static! {
        pub static ref ENV_STORAGE: super::EnvStorage = super::EnvStorage::new();
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EnvHashMapBackend;

impl EnvBackend for EnvHashMapBackend {
    fn get_value(env_name: &str) -> Option<String> {
        hashmap_storage::ENV_STORAGE.get(env_name)
    }

    fn set_value(env_name: &str, env_value: Option<&String>) {
        match env_value {
            Some(value) => hashmap_storage::ENV_STORAGE.put(env_name, value),
            None => hashmap_storage::ENV_STORAGE.delete(env_name),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{EnvBackend, EnvHashMapBackend};

    #[test]
    fn test_hashmap_backend_set_get() {
        // given
        let env_name = "308a3d98-2f87-46fd-b996-ae471a76b64e";
        let env_value = "value".to_string();

        // when
        EnvHashMapBackend::set_value(env_name, Some(&env_value));
        let value = EnvHashMapBackend::get_value(env_name);

        // then
        assert!(value.is_some());
        assert_eq!(env_value, value.unwrap());
    }

    #[test]
    fn test_hashmap_backend_set_unset() {
        // given
        let env_name = "7a6885c3-0c17-4310-a5e7-ed971cac69b6";
        let env_value = "value".to_string();

        // when
        EnvHashMapBackend::set_value(env_name, Some(&env_value));
        EnvHashMapBackend::set_value(env_name, None);
        let value = EnvHashMapBackend::get_value(env_name);

        // then
        assert!(value.is_none());
    }
}
