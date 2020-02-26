use std::borrow::Cow;
use std::collections::HashMap;

use crate::extn::core::env::backend::EnvType;
use crate::extn::prelude::*;
use crate::ffi;

#[derive(Debug, Default, Clone, Copy)]
pub struct System;

impl System {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl EnvType for System {
    fn get<'a>(
        &'a self,
        interp: &Artichoke,
        name: &[u8],
    ) -> Result<Option<Cow<'a, [u8]>>, Exception> {
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
        if memchr::memchr(b'\0', name).is_some() {
            return Err(Exception::from(ArgumentError::new(
                interp,
                "bad environment variable name: contains null byte",
            )));
        }
        if memchr::memchr(b'=', name).is_some() {
            // MRI accepts names containing '=' on get and should always return
            // `nil` since these names are invalid at the OS level.
            Ok(None)
        } else {
            let name = ffi::bytes_to_os_str(name)?;
            if let Some(value) = std::env::var_os(name) {
                let value = ffi::os_str_to_bytes(value.as_os_str())?;
                Ok(Some(value.to_vec().into()))
            } else {
                Ok(None)
            }
        }
    }

    fn put(
        &mut self,
        interp: &Artichoke,
        name: &[u8],
        value: Option<&[u8]>,
    ) -> Result<(), Exception> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() {
            // TODO: This should raise `Errno::EINVAL`.
            return Err(Exception::from(ArgumentError::new(
                interp,
                "Invalid argument - setenv()",
            )));
        }
        if memchr::memchr(b'\0', name).is_some() {
            return Err(Exception::from(ArgumentError::new(
                interp,
                "bad environment variable name: contains null byte",
            )));
        }
        if memchr::memchr(b'=', name).is_some() {
            let mut message = b"Invalid argumen - setenv(".to_vec();
            message.extend(name.to_vec());
            message.push(b')');
            // TODO: This should raise `Errno::EINVAL`.
            return Err(Exception::from(ArgumentError::new_raw(interp, message)));
        }
        if let Some(value) = value {
            if memchr::memchr(b'\0', value).is_some() {
                return Err(Exception::from(ArgumentError::new(
                    interp,
                    "bad environment variable value: contains null byte",
                )));
            }
            let name = ffi::bytes_to_os_str(name)?;
            let value = ffi::bytes_to_os_str(value)?;
            std::env::set_var(name, value);
            Ok(())
        } else {
            let name = ffi::bytes_to_os_str(name)?;
            std::env::remove_var(name);
            Ok(())
        }
    }

    fn as_map(&self, interp: &Artichoke) -> Result<HashMap<Vec<u8>, Vec<u8>>, Exception> {
        let _ = interp;
        let mut map = HashMap::default();
        for (name, value) in std::env::vars_os() {
            let name = ffi::os_str_to_bytes(name.as_os_str())?;
            let value = ffi::os_str_to_bytes(value.as_os_str())?;
            map.insert(name.to_vec(), value.to_vec());
        }
        Ok(map)
    }
}
