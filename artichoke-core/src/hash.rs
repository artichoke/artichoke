//! Build hashers and hash values.

use core::hash::BuildHasher;

/// A trait for retrieving an interpreter-global [`BuildHasher`].
///
/// The [`BuildHasher`] associated with the interpreter is for creating instances
/// of [`Hasher`]. A `BuildHasher` is typically used (e.g., by `HashMap`) to
/// create [`Hasher`]s for each key such that they are hashed independently of
/// one another, since [`Hasher`]s contain state.
///
/// By associating one [`BuildHasher`] with the interpreter, identical Ruby
/// objects should hash identically, even if the interpreter's [`BuildHasher`]
/// includes randomness.
///
/// [`Hasher`]: core::hash::Hasher
pub trait Hash {
    /// Concrete error type for errors encountered when retrieving the
    /// interpreter's global [`BuildHasher`].
    type Error;

    /// Concrete [`BuildHasher`] type which is global to the interpreter.
    type GlobalBuildHasher: BuildHasher;

    /// Retrieve the interpreter's global [`BuildHasher`].
    ///
    /// # Errors
    ///
    /// If the [`BuildHasher`] is inaccessible, an error is returned.
    fn global_build_hasher(&mut self) -> Result<&Self::GlobalBuildHasher, Self::Error>;
}
