use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

use crate::extn::prelude::*;

pub mod backend;
mod boxing;
pub mod mruby;
pub mod trampoline;

use backend::memory::Memory;
use backend::system::System;
use backend::EnvType;

pub struct Environ(Box<dyn EnvType>);

impl fmt::Debug for Environ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environ")
            .field("backend", self.0.as_debug())
            .finish()
    }
}

impl Environ {
    #[must_use]
    pub fn new_system_env() -> Self {
        Self(Box::new(System::new()))
    }

    #[must_use]
    pub fn new_memory_env() -> Self {
        Self(Box::new(Memory::new()))
    }

    #[must_use]
    pub fn initialize() -> Self {
        #[cfg(feature = "core-env-system")]
        let environ = Self::new_system_env();
        #[cfg(not(feature = "core-env-system"))]
        let environ = Self::new_memory_env();

        environ
    }

    pub fn get(&self, name: &[u8]) -> Result<Option<Cow<'_, [u8]>>, Error> {
        self.0.get(name)
    }

    pub fn put(&mut self, name: &[u8], value: Option<&[u8]>) -> Result<(), Error> {
        self.0.put(name, value)?;
        Ok(())
    }

    pub fn to_map(&self) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
        self.0.to_map()
    }
}
