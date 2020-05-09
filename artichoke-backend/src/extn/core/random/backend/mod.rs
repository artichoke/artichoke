use std::fmt;

use crate::extn::prelude::*;

pub mod default;
pub mod rand;

/// Common API for [`Random`](crate::extn::core::random::Random) backends.
pub trait RandType {
    /// Return a `dyn Debug` representation of this `Random`.
    fn as_debug(&self) -> &dyn fmt::Debug;

    /// Completely fill a buffer with random bytes.
    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) -> Result<(), Exception>;

    /// Return the value this backend was seeded with.
    fn seed(&self, interp: &Artichoke) -> Result<u64, Exception>;

    /// Return true if this and `other` would return the same sequence of random
    /// data.
    fn internal_state(&self, interp: &Artichoke) -> Result<InternalState, Exception>;

    /// Return a random `Integer` between 0 and `max` -- `[0, max)`.
    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Result<Int, Exception>;

    /// Return a random `Float` between 0 and `max` -- `[0, max)`.
    ///
    /// If `max` is `None`, return a random `Float between 0 and 1.0 --
    /// `[0, 1.0)`.
    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Fp>) -> Result<Fp, Exception>;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum InternalState {
    Rand { seed: u64 },
}
