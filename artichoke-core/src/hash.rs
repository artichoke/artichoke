//! Build hashers and hash values

/// Hashing functions such as building hashers
pub trait Hash {
    /// Concrete error type for errors encountered when outputting hash errors.
    type Error;

    /// Concrete build hasher type.
    type BuildHasher: core::hash::BuildHasher;

    /// Build a Hasher
    fn build_hasher(&mut self) -> Result<&Self::BuildHasher, Self::Error>;
}
