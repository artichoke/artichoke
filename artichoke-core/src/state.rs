//! Capture stdout and stderr from an interpreter.

/// Interpreters that implement [`OutputCapture`] expose methods for redirecting
/// stdout and stderr to an internal `String` buffer and retrieving it later.
pub trait OutputCapture {
    /// Clear any existing output capture and capture any new IO to stdout and
    /// stderr from the Artichoke VM.
    ///
    /// At a minimum, this method should capture `Kernel#puts` and
    /// `Kernel#print`.
    fn begin_capturing_output(&mut self);

    /// Return any existing captured output or empty string if output capturing
    /// is not enabled. Clear the existing capture buffer.
    fn get_and_clear_captured_output(&mut self) -> String;
}
