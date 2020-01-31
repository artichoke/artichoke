//! File-backed Rust extensions for the Artichoke VM.

use crate::ArtichokeError;

/// Types that implement `File` can be loaded into an interpreter and modify
/// the VM when `require`d.
pub trait File {
    /// Concrete type for interpreter.
    type Artichoke;

    /// Called when the filename mapped to this type is required by the VM.
    ///
    /// This function can mutate interpreter state, such as defining classes and
    /// modules. This function is equivalent to the "init" methods of
    /// C-implemented Rubygems.
    ///
    /// # Errors
    ///
    /// If a fallible API on the interpreter returns an error, implementors
    /// should return an error. Example fallible APIs that might be called on
    /// require include [`Eval::eval`](crate::eval::Eval::eval) and
    /// [`LoadSources::def_rb_source_file`](crate::load::LoadSources::def_rb_source_file).
    fn require(interp: &mut Self::Artichoke) -> Result<(), ArtichokeError>;
}
