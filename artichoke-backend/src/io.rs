use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;

use crate::class_registry::ClassRegistry;
use crate::core::{ConvertMut, Io};
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception;
use crate::ffi::InterpreterExtractError;
use crate::state::output::Output;
use crate::sys;
use crate::Artichoke;

impl Io for Artichoke {
    type Error = Exception;

    /// Writes the given bytes to the interpreter stdout stream.
    ///
    /// This implementation delegates to the underlying output strategy.
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    fn print<T: AsRef<[u8]>>(&mut self, message: T) -> Result<(), Self::Error> {
        let state = self.state.as_mut().ok_or(InterpreterExtractError)?;
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
        let state = self.state.as_mut().ok_or(InterpreterExtractError)?;
        state.output.write_stdout(message.as_ref())?;
        state.output.write_stdout(b"\n")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct IOError(io::Error);

impl From<io::Error> for IOError {
    fn from(err: io::Error) -> Self {
        Self(err)
    }
}

impl From<io::Error> for Exception {
    fn from(err: io::Error) -> Self {
        Self::from(IOError::from(err))
    }
}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IOError: {}", self.0)
    }
}

impl error::Error for IOError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

impl RubyException for IOError {
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
        let message = interp.convert_mut(self.message());
        let value = interp
            .new_instance::<exception::IOError>(&[message])
            .ok()
            .flatten()?;
        Some(value.inner())
    }
}

impl From<IOError> for Exception {
    fn from(exception: IOError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<IOError>> for Exception {
    fn from(exception: Box<IOError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<IOError> for Box<dyn RubyException> {
    fn from(exception: IOError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<IOError>> for Box<dyn RubyException> {
    fn from(exception: Box<IOError>) -> Box<dyn RubyException> {
        exception
    }
}
