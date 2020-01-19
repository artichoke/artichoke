//! Emit warnings during VM execution.

/// Interpreters that implement [`Warn`] expose methods for emitting warnings
/// during execution.
///
/// Some functionality required to be compliant with ruby-spec is deprecated or
/// invalid behavior and ruby-spec expects a warning to be emitted to `$stderr`
/// using the
/// [`Warning`](https://ruby-doc.org/core-2.6.3/Warning.html#method-i-warn)
/// module from the standard library.
pub trait Warn {
    /// Concrete error type for errors encountered when outputting warnings.
    type Error: std::error::Error;

    /// Emit a warning message using `Warning#warn`.
    ///
    /// This method appends newlines to message if necessary.
    fn warn(&self, message: &[u8]) -> Result<(), Self::Error>;
}
