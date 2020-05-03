use crate::core::Prng;
use crate::exception::Exception;
use crate::extn::core::random::backend::InternalState;
use crate::ffi::InterpreterExtractError;
use crate::types::{Fp, Int};
use crate::Artichoke;

impl Prng for Artichoke {
    type Error = Exception;
    type InternalState = InternalState;
    type Int = Int;
    type Float = Fp;

    fn prng_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.state.ok_or(InterpreterExtractError)?.prng.bytes(buf);
        Ok(())
    }

    fn prng_seed(&self) -> Result<u64, Self::Error> {
        let seed = self.state.ok_or(InterpreterExtractError)?.prng.seed();
        Ok(seed)
    }

    fn prng_reseed(&mut self, seed: Option<u64>) -> Result<(), Self::Error> {
        self.state.ok_or(InterpreterExtractError)?.prng.reseed(seed);
        Ok(())
    }

    fn prng_internal_state(&self) -> Result<Self::InternalState, Self::Error> {
        let internal_state = self
            .state
            .ok_or(InterpreterExtractError)?
            .prng
            .internal_state();
        Ok(internal_state)
    }

    fn rand_int(&mut self, max: Self::Int) -> Result<Self::Int, Self::Error> {
        let next = self
            .state
            .ok_or(InterpreterExtractError)?
            .prng
            .rand_int(max);
        Ok(next)
    }

    fn rand_float(&mut self, max: Option<Self::Float>) -> Result<Self::Float, Self::Error> {
        let next = self
            .state
            .ok_or(InterpreterExtractError)?
            .prng
            .rand_float(max);
        Ok(next)
    }
}
