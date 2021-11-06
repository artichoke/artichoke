use crate::core::Hash;
use crate::error::Error;
use std::collections::hash_map::RandomState;
use crate::ffi::InterpreterExtractError;
use crate::Artichoke;

impl Hash for Artichoke {
    type Error = Error;
    type BuildHasher = RandomState;

    fn build_hasher(&mut self) -> Result<&Self::BuildHasher, Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        Ok(&state.hash_builder)
    }
}
