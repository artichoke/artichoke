use crate::extn::core::random::backend;
use crate::extn::prelude::*;

#[must_use]
pub fn new() -> Box<dyn backend::Rand> {
    Box::new(Default::default())
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Default;

impl backend::Rand for Default {
    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) {
        let prng = interp.state_mut().prng_mut();
        prng.bytes(buf);
    }

    fn seed(&self, interp: &Artichoke) -> u64 {
        let prng = interp.state().prng();
        prng.seed()
    }

    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn backend::Rand) -> bool {
        let prng = interp.state().prng();
        prng.has_same_internal_state(other)
    }

    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Int {
        let prng = interp.state_mut().prng_mut();
        prng.rand_int(max)
    }

    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Float>) -> Float {
        let prng = interp.state_mut().prng_mut();
        prng.rand_float(max)
    }
}
