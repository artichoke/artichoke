use std::any::Any;

use crate::extn::prelude::*;

pub mod default;
pub mod rand;

/// Common API for [`Random`](crate::extn::core::random::Random) backends.
pub trait Rand: Any {
    /// Completely fill a buffer with random bytes.
    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]);

    /// Return the value this backend was seeded with.
    fn seed(&self, interp: &Artichoke) -> u64;

    /// Return true if this and `other` would return the same sequence of random
    /// data.
    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn Rand) -> bool;

    /// Return a random `Integer` between 0 and `max` -- `[0, max)`.
    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Int;

    /// Return a random `Float` between 0 and `max` -- `[0, max)`.
    ///
    /// If `max` is `None`, return a random `Float between 0 and 1.0 --
    /// `[0, 1.0)`.
    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Float>) -> Float;
}

#[allow(clippy::missing_safety_doc)]
mod internal {
    downcast!(dyn super::Rand);
}
