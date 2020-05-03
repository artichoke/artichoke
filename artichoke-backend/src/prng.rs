use crate::core::Prng;
use crate::exception::Exception;
use crate::extn::core::random::backend::InternalState;
use crate::types::{Fp, Int};
use crate::Artichoke;

impl Prng for Artichoke {
    type Error = Exception;
    type InternalState = InternalState;
    type Int = Int;
    type Float = Fp;

    fn prng_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut borrow = self.0.borrow_mut();
        borrow.prng.bytes(buf);
        Ok(())
    }

    fn prng_seed(&self) -> Result<u64, Self::Error> {
        let borrow = self.0.borrow();
        Ok(borrow.prng.seed())
    }

    fn prng_reseed(&mut self, seed: Option<u64>) -> Result<(), Self::Error> {
        self.0.borrow_mut().prng.reseed(seed);
        Ok(())
    }

    fn prng_internal_state(&self) -> Result<Self::InternalState, Self::Error> {
        let borrow = self.0.borrow();
        Ok(borrow.prng.internal_state())
    }

    fn rand_int(&mut self, max: Self::Int) -> Result<Self::Int, Self::Error> {
        let mut borrow = self.0.borrow_mut();
        Ok(borrow.prng.rand_int(max))
    }

    fn rand_float(&mut self, max: Option<Self::Float>) -> Result<Self::Float, Self::Error> {
        let mut borrow = self.0.borrow_mut();
        Ok(borrow.prng.rand_float(max))
    }
}
