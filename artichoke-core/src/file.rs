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
    /// When required, a `File` can mutate the interpreter arbitrarily and many
    /// of these APIs, like [`Eval::eval`](crate::eval::Eval::eval) and
    /// [`LoadSources::def_rb_source_file`](crate::load::LoadSources::def_rb_source_file),
    /// are fallible.
    ///
    /// Implementators should propagate these errors back to the interpreter if
    /// they cannot locally recover.
    fn require(interp: &mut Self::Artichoke) -> Result<(), ArtichokeError>;
}
