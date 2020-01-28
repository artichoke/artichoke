use crate::extn::core::random::backend::RandType;
use crate::extn::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Default;

impl RandType for Default {
    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) {
        let mut borrow = interp.0.borrow_mut();
        borrow.prng.bytes(buf);
    }

    fn seed(&self, interp: &Artichoke) -> u64 {
        let borrow = interp.0.borrow();
        borrow.prng.seed()
    }

    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn RandType) -> bool {
        let borrow = interp.0.borrow();
        borrow.prng.has_same_internal_state(other)
    }

    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Int {
        let mut borrow = interp.0.borrow_mut();
        borrow.prng.rand_int(max)
    }

    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Float>) -> Float {
        let mut borrow = interp.0.borrow_mut();
        borrow.prng.rand_float(max)
    }
}
