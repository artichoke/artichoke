//! I/O read and write APIs.

/// Perform I/O external to the interpreter.
pub trait Io {
    /// Concrete error type for errors encountered when reading and writing.
    type Error;

    /// Writes the given bytes to the interpreter stdout stream.
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    fn print(&mut self, message: &[u8]) -> Result<(), Self::Error>;

    /// Writes the given bytes to the interpreter stdout stream followed by a
    /// newline.
    ///
    /// The default implementation uses two calls to [`print`].
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    ///
    /// [`print`]: Self::print
    fn puts(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        self.print(message)?;
        self.print(b"\n")?;
        Ok(())
    }
}
