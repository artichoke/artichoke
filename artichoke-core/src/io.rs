//! I/O read and write APIs.

/// Make I/O external to the interpreter.
pub trait Io {
    /// Concrete error type for errors encountered when reading and writing.
    type Error;

    /// Writes the given bytes to the interpreter stdout stream.
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    fn print<T: AsRef<[u8]>>(&mut self, message: T) -> Result<(), Self::Error>;

    /// Writes the given bytes to the interpreter stdout stream followed by a
    /// newline.
    ///
    /// This default implementation uses two calls to [`Io::print`].
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    fn puts<T: AsRef<[u8]>>(&mut self, message: T) -> Result<(), Self::Error> {
        self.print(message.as_ref())?;
        self.print("\n")?;
        Ok(())
    }
}
