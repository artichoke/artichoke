//! File-backed Rust extensions for the Artichoke VM.

/// Rust extension hook that can be required.
///
/// `File`s are mounted in the interpreter file system and can modify interpreter
/// state when they are loaded.
pub trait File {
    /// Concrete type for interpreter.
    type Artichoke;

    /// Concrete error type for eval functions.
    type Error;

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
    fn require(interp: &mut Self::Artichoke) -> Result<(), Self::Error>;
}
