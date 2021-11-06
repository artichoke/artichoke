//! Build hashers and hash values

/// A trait for creating instances of RandomState
pub trait BuildHasher<B: std::hash::BuildHasher> {
    /// Concrete error type for errors encountered when outputting hash errors.
    type Error;
    /// Build a RandomState hasher
    fn build_hasher(&mut self) -> Result<&B, Self::Error>;
}
