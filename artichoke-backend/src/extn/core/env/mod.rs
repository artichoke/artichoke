//! ENV is a hash-like accessor for environment variables.
//!
//! This module implements the [`ENV`] singleton object from Ruby Core.
//!
//! In Artichoke, the environment variable store is modeled as a hash map of
//! byte vector keys and values, e.g. `HashMap<Vec<u8>, Vec<u8>>`. Backends are
//! expected to convert their internals to this representation in their public
//! APIs. For this reason, all APIs exposed by ENV backends in this crate are
//! fallible.
//!
//! You can use this object in your application by accessing it directly. As a
//! Core API, it is globally available:
//!
//! ```ruby
//! ENV['PATH']
//! ENV['PS1'] = 'artichoke> '
//! ```
//!
//! [`ENV`]: https://ruby-doc.org/core-3.1.2/ENV.html

use std::borrow::Cow;
use std::collections::HashMap;

use spinoso_env::{ArgumentError as EnvArgumentError, Error as EnvError, InvalidError};

use crate::extn::prelude::*;

pub mod mruby;
pub mod trampoline;

#[cfg(not(feature = "core-env-system"))]
type Backend = spinoso_env::Memory;
#[cfg(feature = "core-env-system")]
type Backend = spinoso_env::System;

#[derive(Default, Debug)]
#[allow(missing_copy_implementations)] // not all backends implement `Copy`
pub struct Environ(Backend);

impl Environ {
    #[must_use]
    pub fn new() -> Self {
        Self(Backend::new())
    }

    pub fn get(&self, name: &[u8]) -> Result<Option<Cow<'_, [u8]>>, Error> {
        let value = self.0.get(name)?;
        Ok(value)
    }

    pub fn put(&mut self, name: &[u8], value: Option<&[u8]>) -> Result<(), Error> {
        self.0.put(name, value)?;
        Ok(())
    }

    pub fn to_map(&self) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
        let map = self.0.to_map()?;
        Ok(map)
    }
}

impl HeapAllocatedData for Environ {
    const RUBY_TYPE: &'static str = "Artichoke::Environ";
}

impl From<EnvArgumentError> for Error {
    fn from(err: EnvArgumentError) -> Self {
        ArgumentError::from(err.message()).into()
    }
}

impl From<InvalidError> for Error {
    fn from(err: InvalidError) -> Self {
        // TODO: This should be an `Errno::EINVAL`.
        SystemCallError::from(err.into_message()).into()
    }
}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Self {
        match err {
            EnvError::Argument(err) => err.into(),
            EnvError::Invalid(err) => err.into(),
        }
    }
}
