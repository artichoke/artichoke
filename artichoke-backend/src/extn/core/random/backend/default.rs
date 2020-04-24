use std::fmt;

use crate::extn::core::random::backend::{InternalState, RandType};
use crate::extn::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Default;

impl RandType for Default {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) {
        let mut borrow = interp.0.borrow_mut();
        borrow.prng.bytes(buf);
    }

    fn seed(&self, interp: &Artichoke) -> u64 {
        let borrow = interp.0.borrow();
        borrow.prng.seed()
    }

    fn internal_state(&self, interp: &Artichoke) -> InternalState {
        let borrow = interp.0.borrow();
        borrow.prng.internal_state()
    }

    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Int {
        let mut borrow = interp.0.borrow_mut();
        borrow.prng.rand_int(max)
    }

    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Fp>) -> Fp {
        let mut borrow = interp.0.borrow_mut();
        borrow.prng.rand_float(max)
    }
}
