use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::sync::RwLock;

pub trait EnvBackend {
    fn get_value(env_name: &str) -> Option<String>;
    fn set_value(env_name: &str, env_value: Option<&String>);
    fn convert_to_h() -> HashMap<String, String>;
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

    fn convert_to_h() -> HashMap<String, String> {
        env::vars()
            .map(move |(var_name, var_value)| (var_name, var_value))
            .collect()
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

    pub fn get_hash(&self) -> HashMap<String, String> {
        self.data.read().unwrap().clone()
    }

    pub fn clear(&self) {
        self.data.write().unwrap().clear();
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

    fn convert_to_h() -> HashMap<String, String> {
        hashmap_storage::ENV_STORAGE.get_hash()
    }
}

impl EnvHashMapBackend {
    #[allow(dead_code)]
    pub fn clear() {
        hashmap_storage::ENV_STORAGE.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::{EnvBackend, EnvHashMapBackend};
    use std::sync::Mutex;

    mod tests_mutex {
        lazy_static! {
            pub static ref TEST_MUTEX: super::Mutex<()> = super::Mutex::new(());
        }
    }
    macro_rules! serial_test {
        (fn $name:ident() $body:block) => {
            #[test]
            fn $name() {
                let _guard = tests_mutex::TEST_MUTEX.lock().unwrap();
                $body
            }
        };
    }

    serial_test! {
        fn test_hashmap_backend_set_get() {
            // given
            EnvHashMapBackend::clear();
            let env_name = "308a3d98-2f87-46fd-b996-ae471a76b64e";
            let env_value = "value".to_string();

            // when
            EnvHashMapBackend::set_value(env_name, Some(&env_value));
            let value = EnvHashMapBackend::get_value(env_name);

            // then
            assert!(value.is_some());
            assert_eq!(env_value, value.unwrap());
        }
    }

    serial_test! {
        fn test_hashmap_backend_set_unset() {
            // given
            EnvHashMapBackend::clear();
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

    serial_test! {
        fn test_hashmap_backend_to_hashmap() {
            // given
            EnvHashMapBackend::clear();
            let env1_name = "3ab42e94-9b7f-4e96-b9c7-ba1738c61f89";
            let env1_value = "value1".to_string();
            let env2_name = "3e7bf2b3-9517-444b-bda8-7f5dd3b36648";
            let env2_value = "value2".to_string();

            // when
            let size_before = EnvHashMapBackend::convert_to_h().len();
            EnvHashMapBackend::set_value(env1_name, Some(&env1_value));
            EnvHashMapBackend::set_value(env2_name, Some(&env2_value));
            let data = EnvHashMapBackend::convert_to_h();
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
}
