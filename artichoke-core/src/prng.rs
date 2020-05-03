//! Interpreter global psuedorandom number generator.

use std::error;

/// Interpreter global psuedorandom number generator.
///
/// Implementors of this trait back the `Random::DEFAULT` PRNG.
pub trait Prng {
    /// Concrete type for PRNG errors.
    type Error: error::Error;

    /// Cocnrete type representing the internal state of all PRNG backends.
    type InternalState;

    /// Conrete type for integer input and output.
    type Int;

    /// Concrete type for floating point input and output.
    type Float;

    /// Completely fill a buffer with random bytes.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    ///
    /// If the PRNG encounters an error, an error is returned.
    fn prng_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Return the value this PRNG was seeded with.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    fn prng_seed(&self) -> Result<u64, Self::Error>;

    /// Reseed the PRNG with a new seed.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    fn prng_reseed(&mut self, seed: Option<u64>) -> Result<(), Self::Error>;

    /// Return true if this and `other` would return the same sequence of random
    /// data.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    fn prng_internal_state(&self) -> Result<Self::InternalState, Self::Error>;

    /// Return a random `Integer` between 0 and `max` -- `[0, max)`.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    ///
    /// If the PRNG encounters an error, an error is returned.
    fn rand_int(&mut self, max: Self::Int) -> Result<Self::Int, Self::Error>;

    /// Return a random `Float` between 0 and `max` -- `[0, max)`.
    ///
    /// If `max` is `None`, return a random `Float between 0 and 1.0 --
    /// `[0, 1.0)`.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    ///
    /// If the PRNG encounters an error, an error is returned.
    fn rand_float(&mut self, max: Option<Self::Float>) -> Result<Self::Float, Self::Error>;
}
