use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;

use crate::core::{ClassRegistry, Io, TryConvertMut};
use crate::error::{Error, RubyException};
use crate::extn::core::exception;
use crate::ffi::InterpreterExtractError;
use crate::state::output::Output;
use crate::sys;
use crate::Artichoke;

impl Io for Artichoke {
    type Error = Error;

    /// Writes the given bytes to the interpreter stdout stream.
    ///
    /// This implementation delegates to the underlying output strategy.
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    fn print<T: AsRef<[u8]>>(&mut self, message: T) -> Result<(), Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.output.write_stdout(message.as_ref())?;
        Ok(())
    }

    /// Writes the given bytes to the interpreter stdout stream followed by a
    /// newline.
    ///
    /// This implementation delegates to the underlying output strategy.
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    fn puts<T: AsRef<[u8]>>(&mut self, message: T) -> Result<(), Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.output.write_stdout(message.as_ref())?;
        state.output.write_stdout(b"\n")?;
        Ok(())
    }
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct IoError(io::Error);

impl From<io::Error> for IoError {
    fn from(err: io::Error) -> Self {
        Self(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::from(IoError::from(err))
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IOError: {}", self.0)
    }
}

impl error::Error for IoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

impl RubyException for IoError {
    fn message(&self) -> Cow<'_, [u8]> {
        self.0.to_string().into_bytes().into()
    }

    fn name(&self) -> Cow<'_, str> {
        "IOError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.message()).ok()?;
        let value = interp.new_instance::<exception::IOError>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<IoError> for Error {
    fn from(exception: IoError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<IoError>> for Error {
    fn from(exception: Box<IoError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<IoError> for Box<dyn RubyException> {
    fn from(exception: IoError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<IoError>> for Box<dyn RubyException> {
    fn from(exception: Box<IoError>) -> Box<dyn RubyException> {
        exception
    }
}
