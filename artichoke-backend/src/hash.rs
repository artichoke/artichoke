use crate::core::BuildHasher;
use crate::error::Error;
use std::collections::hash_map::RandomState;
use crate::ffi::InterpreterExtractError;
use crate::Artichoke;

impl BuildHasher for Artichoke {
    type Error = Error;

    fn build_hasher(&mut self) -> Result<RandomState, Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        Ok(state.hash_builder.clone())
    }
}
