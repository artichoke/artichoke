//! Interpreter global pseudorandom number generator.

/// Interpreter global pseudorandom number generator.
///
/// Implementors of this trait back the `Random::DEFAULT` PRNG.
pub trait Prng {
    /// Concrete type for PRNG errors.
    type Error;

    /// Concrete type for the interpreter pseudorandom number generator.
    type Prng;

    /// Return a shared reference to the interpreter pseudorandom number
    /// generator.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    fn prng(&self) -> Result<&Self::Prng, Self::Error>;

    /// Return a mutable reference to the interpreter pseudorandom number
    /// generator.
    ///
    /// # Errors
    ///
    /// If the PRNG is inaccessible, an error is returned.
    fn prng_mut(&mut self) -> Result<&mut Self::Prng, Self::Error>;
}
