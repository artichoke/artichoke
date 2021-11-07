//! Build hashers and hash values

/// Hashing functions such as building hashers
pub trait Hash {
    /// Concrete error type for errors encountered when outputting hash errors.
    type Error;

    /// Concrete build hasher type.
    type BuildHasher: core::hash::BuildHasher;

    /// Build a Hasher
    ///
    /// # Errors
    ///
    /// If the build hasher is inaccessible, an error is returned.
    fn build_hasher(&mut self) -> Result<&Self::BuildHasher, Self::Error>;
}
