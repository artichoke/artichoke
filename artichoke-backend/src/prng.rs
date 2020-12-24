use crate::core::Prng;
use crate::extn::core::random::Random;
use crate::ffi::InterpreterExtractError;
use crate::{Artichoke, Error};

impl Prng for Artichoke {
    type Error = Error;
    type Prng = Random;

    fn prng(&self) -> Result<&Self::Prng, Self::Error> {
        let state = self.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
        Ok(&state.prng)
    }

    fn prng_mut(&mut self) -> Result<&mut Self::Prng, Self::Error> {
        let state = self.state.as_mut().ok_or_else(InterpreterExtractError::new)?;
        Ok(&mut state.prng)
    }
}
