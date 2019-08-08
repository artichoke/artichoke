use std::env;
use std::ffi::OsString;

pub trait EnvBackend {
    fn get_value(env_name: &str) -> Option<String>;
    fn set_value(env_name: &str, env_value: &Option<String>);
}

#[derive(Debug)]
pub struct EnvStdBackend;

impl EnvBackend for EnvStdBackend {
    fn get_value(env_name: &str) -> Option<String> {
        if let Some(value) = env::var_os(env_name) {
            Some(String::from(value.to_str().unwrap()))
        } else {
            None
        }
    }

    fn set_value(env_name: &str, env_value: &Option<String>) {
        match env_value {
            Some(string) => env::set_var(OsString::from(env_name), OsString::from(string)),
            None => env::remove_var(OsString::from(env_name)),
        }
    }
}
