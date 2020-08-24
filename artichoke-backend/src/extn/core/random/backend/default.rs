use std::fmt;

use crate::extn::core::random::backend::{InternalState, RandType};
use crate::extn::prelude::*;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Default {
    _private: (),
}

impl Default {
    /// Constructs a new, default `Default` Random backend.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl RandType for Default {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) -> Result<(), Error> {
        interp.prng_fill_bytes(buf)
    }

    fn seed(&self, interp: &Artichoke) -> Result<u64, Error> {
        interp.prng_seed()
    }

    fn internal_state(&self, interp: &Artichoke) -> Result<InternalState, Error> {
        interp.prng_internal_state()
    }

    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Result<Int, Error> {
        interp.rand_int(max)
    }

    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Fp>) -> Result<Fp, Error> {
        interp.rand_float(max)
    }
}
