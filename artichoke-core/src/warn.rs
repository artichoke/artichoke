//! Emit warnings during interpreter execution.

/// Emit warnings during interpreter execution to stderr.
///
/// Some functionality required to be compliant with ruby/spec is deprecated or
/// invalid behavior and ruby/spec expects a warning to be emitted to `$stderr`
/// using the [`Warning`][warningmod] module from the standard library.
///
/// [warningmod]: https://ruby-doc.org/core-3.1.2/Warning.html#method-i-warn
pub trait Warn {
    /// Concrete error type for errors encountered when outputting warnings.
    type Error;

    /// Emit a warning message using `Warning#warn`.
    ///
    /// This method appends newlines to message if necessary.
    ///
    /// # Errors
    ///
    /// Interpreters should issue warnings by calling the `warn` method on the
    /// `Warning` module.
    ///
    /// If an exception is raised on the interpreter, then an error is returned.
    fn warn(&mut self, message: &[u8]) -> Result<(), Self::Error>;
}
