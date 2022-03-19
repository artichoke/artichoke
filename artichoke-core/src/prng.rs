//! Interpreter global pseudorandom number generator.

/// Interpreter global pseudorandom number generator (PRNG).
///
/// Implementations of this trait may be used as the backing pseudorandom number
/// generator for [`Random::DEFAULT`].
///
/// The PRNG used to implement this trait is **not** required to be
/// cryptographically secure. For example, MRI's PRNG is a variant of Mersenne
/// Twister.
///
/// [`Random::DEFAULT`]: https://ruby-doc.org/core-2.6.3/Random.html#DEFAULT
pub trait Prng {
    /// Concrete type for errors when retrieving the pseudorandom number
    /// generator.
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
