use std::any::Any;

use crate::extn::core::exception::RubyException;
use crate::types::{Float, Int};
use crate::Artichoke;

pub mod default;
pub mod rand;

/// Common API for [`Random`](crate::extn::core::random::Random) backends.
pub trait Rand: Any {
    /// Completely fill a buffer with random bytes.
    fn bytes(&mut self, interp: &Artichoke, buf: &mut [u8]) -> Result<(), Box<dyn RubyException>>;

    /// Return the value this backend was seeded with.
    fn seed(&self, interp: &Artichoke) -> Result<u64, Box<dyn RubyException>>;

    /// Return true if this and `other` would return the same sequence of random
    /// data.
    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn Rand) -> bool;

    /// Return a random `Integer` between 0 and `max` -- `[0, max)`.
    fn rand_int(&mut self, interp: &Artichoke, max: Int) -> Result<Int, Box<dyn RubyException>>;

    /// Return a random `Float` between 0 and `max` -- `[0, max)`.
    ///
    /// If `max` is `None`, return a random `Float between 0 and 1.0 --
    /// `[0, 1.0)`.
    fn rand_float(
        &mut self,
        interp: &Artichoke,
        max: Option<Float>,
    ) -> Result<Float, Box<dyn RubyException>>;
}

#[allow(clippy::missing_safety_doc)]
mod internal {
    downcast!(dyn super::Rand);
}
