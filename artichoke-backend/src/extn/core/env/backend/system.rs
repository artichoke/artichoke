use bstr::ByteSlice;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env as systemenv;
use std::fmt;

use crate::extn::core::env::backend::{EnvArgumentError, EnvType};
use crate::extn::prelude::*;
use crate::ffi;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct System;

impl System {
    /// Constructs a new, default ENV `System` backend.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl EnvType for System {
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
            let name = ffi::bytes_to_os_str(name)?;
            if let Some(value) = systemenv::var_os(name) {
                let value = ffi::os_string_to_bytes(value)?;
                Ok(Some(value.into()))
            } else {
                Ok(None)
            }
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
            let name = ffi::bytes_to_os_str(name)?;
            let value = ffi::bytes_to_os_str(value)?;
            systemenv::set_var(name, value);
            Ok(())
        } else {
            let name = ffi::bytes_to_os_str(name)?;
            systemenv::remove_var(name);
            Ok(())
        }
    }

    fn to_map(&self) -> Result<HashMap<Vec<u8>, Vec<u8>>, Exception> {
        let mut map = HashMap::default();
        for (name, value) in systemenv::vars_os() {
            let name = ffi::os_string_to_bytes(name)?;
            let value = ffi::os_string_to_bytes(value)?;
            map.insert(name, value);
        }
        Ok(map)
    }
}
