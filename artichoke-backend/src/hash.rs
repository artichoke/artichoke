use std::collections::hash_map::RandomState;

use crate::core::Hash;
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::Artichoke;

impl Hash for Artichoke {
    type Error = Error;
    type GlobalBuildHasher = RandomState;

    fn global_build_hasher(&mut self) -> Result<&Self::GlobalBuildHasher, Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        Ok(&state.hash_builder)
    }
}
