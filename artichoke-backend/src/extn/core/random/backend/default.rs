use crate::extn::core::random::backend;
use crate::extn::prelude::*;

#[must_use]
pub fn new() -> Box<dyn backend::Rand> {
    Box::new(Default::default())
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Default;

impl backend::Rand for Default {
    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) -> Result<(), Exception> {
        let prng = interp.state_mut().prng_mut();
        prng.inner_mut().bytes(interp, buf)?;
        Ok(())
    }

    fn seed(&self, interp: &Artichoke) -> Result<u64, Exception> {
        let prng = interp.state().prng();
        let seed = prng.inner().seed(interp)?;
        Ok(seed)
    }

    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn backend::Rand) -> bool {
        let prng = interp.state().prng();
        prng.inner().has_same_internal_state(interp, other)
    }

    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Result<Int, Exception> {
        let prng = interp.state_mut().prng_mut();
        let rand = prng.inner_mut().rand_int(interp, max)?;
        Ok(rand)
    }

    fn rand_float(
        &mut self,
        interp: &mut Artichoke,
        max: Option<Float>,
    ) -> Result<Float, Exception> {
        let prng = interp.state_mut().prng_mut();
        let rand = prng.inner_mut().rand_float(interp, max)?;
        Ok(rand)
    }
}
