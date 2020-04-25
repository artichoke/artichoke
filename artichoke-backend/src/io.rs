use std::error;
use std::fmt;
use std::io;

use crate::class_registry::ClassRegistry;
use crate::core::{ConvertMut, Io};
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception;
use crate::state::output::Output;
use crate::sys;
use crate::Artichoke;

impl Io for Artichoke {
    type Error = IOError;

    /// Writes the given bytes to the interpreter stdout stream.
    ///
    /// This implementation delegates to the underlying output strategy.
    ///
    /// # Errors
    ///
    /// If the output stream encounters an error, an error is returned.
    fn print<T: AsRef<[u8]>>(&mut self, message: T) -> Result<(), Self::Error> {
        self.0.borrow_mut().output.write_stdout(message.as_ref())?;
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
        self.0.borrow_mut().output.write_stdout(message.as_ref())?;
        self.0.borrow_mut().output.write_stdout(b"\n")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct IOError {
    inner: io::Error,
    message: String,
}

impl From<io::Error> for IOError {
    fn from(err: io::Error) -> Self {
        let message = err.to_string();
        Self {
            inner: err,
            message,
        }
    }
}

impl From<io::Error> for Exception {
    fn from(err: io::Error) -> Self {
        Self::from(IOError::from(err))
    }
}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IOError: {}", self.message)
    }
}

impl error::Error for IOError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl RubyException for IOError {
    fn message(&self) -> &[u8] {
        self.message.as_bytes()
    }

    fn name(&self) -> String {
        String::from("IOError")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.inner.to_string());
        let spec = interp.class_spec::<exception::IOError>()?;
        let value = spec.new_instance(interp, &[message])?;
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

#[allow(clippy::use_self)]
impl From<IOError> for Box<dyn RubyException> {
    fn from(exception: IOError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<IOError>> for Box<dyn RubyException> {
    fn from(exception: Box<IOError>) -> Box<dyn RubyException> {
        exception
    }
}
