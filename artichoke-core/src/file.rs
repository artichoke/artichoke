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
    fn require(interp: &mut Self::Artichoke) -> Result<(), ArtichokeError>;
}
