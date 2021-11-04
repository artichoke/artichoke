//! Build hashers and hash values

use std::collections::hash_map::RandomState;

/// A trait for creating instances of RandomState
pub trait BuildHasher {
    /// Concrete error type for errors encountered when outputting hash errors.
    type Error;
    /// Build a RandomState hasher
    fn build_hasher(&mut self) -> Result<RandomState, Self::Error>;
}
