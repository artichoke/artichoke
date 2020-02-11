use std::error;
use std::fmt;
use std::io;

use crate::exception::{Exception, RubyException};
use crate::extn::core::exception;
use crate::sys;
use crate::{Artichoke, ConvertMut};

#[derive(Debug)]
pub struct IOError {
    inner: Option<io::Error>,
    message: String,
}

impl From<io::Error> for IOError {
    fn from(err: io::Error) -> Self {
        let message = err.to_string();
        Self {
            inner: Some(err),
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IOError: {}", self.message)
    }
}

impl error::Error for IOError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        if let Some(ref err) = self.inner {
            Some(err)
        } else {
            None
        }
    }
}

impl RubyException for IOError {
    fn box_clone(&self) -> Box<dyn RubyException> {
        Box::new(Self {
            inner: None,
            message: self.message.clone(),
        })
    }

    fn message(&self) -> &[u8] {
        self.message.as_bytes()
    }

    fn name(&self) -> String {
        String::from("IOError")
    }

    fn vm_backtrace(&self, interp: &Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = if let Some(ref err) = self.inner {
            interp.convert_mut(err.to_string())
        } else {
            interp.convert_mut(self.message())
        };
        let borrow = interp.0.borrow();
        let spec = borrow.class_spec::<exception::IOError>()?;
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
