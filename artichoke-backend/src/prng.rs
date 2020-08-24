use crate::core::Prng;
use crate::error::Error;
use crate::extn::core::random::backend::InternalState;
use crate::ffi::InterpreterExtractError;
use crate::types::{Fp, Int};
use crate::Artichoke;

impl Prng for Artichoke {
    type Error = Error;
    type InternalState = InternalState;
    type Int = Int;
    type Float = Fp;

    fn prng_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let state = self.state.as_mut().ok_or(InterpreterExtractError::new())?;
        state.prng.bytes(buf);
        Ok(())
    }

    fn prng_seed(&self) -> Result<u64, Self::Error> {
        let state = self.state.as_ref().ok_or(InterpreterExtractError::new())?;
        let seed = state.prng.seed();
        Ok(seed)
    }

    fn prng_reseed(&mut self, seed: Option<u64>) -> Result<(), Self::Error> {
        let state = self.state.as_mut().ok_or(InterpreterExtractError::new())?;
        state.prng.reseed(seed);
        Ok(())
    }

    fn prng_internal_state(&self) -> Result<Self::InternalState, Self::Error> {
        let state = self.state.as_ref().ok_or(InterpreterExtractError::new())?;
        let internal_state = state.prng.internal_state();
        Ok(internal_state)
    }

    fn rand_int(&mut self, max: Self::Int) -> Result<Self::Int, Self::Error> {
        let state = self.state.as_mut().ok_or(InterpreterExtractError::new())?;
        let next = state.prng.rand_int(max);
        Ok(next)
    }

    fn rand_float(&mut self, max: Option<Self::Float>) -> Result<Self::Float, Self::Error> {
        let state = self.state.as_mut().ok_or(InterpreterExtractError::new())?;
        let next = state.prng.rand_float(max);
        Ok(next)
    }
}
