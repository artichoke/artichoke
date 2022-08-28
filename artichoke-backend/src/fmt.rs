//! Utilities for interfacing [`std::fmt`] with Artichoke's exception types.

use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::core::{ClassRegistry, TryConvertMut};
use crate::error::{Error, RubyException};
use crate::extn::core::exception::Fatal;
use crate::sys;
use crate::Artichoke;

/// Error type which converts a [`fmt::Error`] into an Artichoke [`Error`].
///
/// This error type can also be used to convert generic [`fmt::Error`] into an
/// [`Error`], such as when formatting integers with [`write!`].
///
/// This  error type wraps a [`fmt::Error`].
///
/// # Examples
///
/// ```
/// use std::fmt::Write;
/// # use artichoke_backend::Error;;
/// # use artichoke_backend::fmt::WriteError;
/// fn task() -> Result<String, Error> {
///     let mut buf = String::new();
///     write!(&mut buf, "success!").map_err(WriteError::from)?;
///     Ok(buf)
/// }
/// # task().unwrap();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct WriteError(fmt::Error);

impl From<fmt::Error> for WriteError {
    fn from(err: fmt::Error) -> Self {
        Self(err)
    }
}

impl From<WriteError> for fmt::Error {
    fn from(err: WriteError) -> Self {
        err.into_inner()
    }
}

impl WriteError {
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> fmt::Error {
        self.0
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unable to write message into destination")
    }
}

impl error::Error for WriteError {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

impl RubyException for WriteError {
    #[inline]
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"Unable to write message into destination")
    }

    #[inline]
    fn name(&self) -> Cow<'_, str> {
        "fatal".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.message()).ok()?;
        let value = interp.new_instance::<Fatal>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<WriteError> for Error {
    #[inline]
    fn from(exception: WriteError) -> Self {
        let err: Box<dyn RubyException> = Box::new(exception);
        Self::from(err)
    }
}
