//! Utilities for working with Ruby `String`s.
//!
//! In Ruby, `String` is a `Vec<u8>` with an optional encoding. `String`s
//! default to UTF-8 encoding but this does not require the byte vector to
//! contain valid UTF-8.
//!
//! Artichoke aims to support ASCII, UTF-8, maybe UTF-8, and binary encodings.

use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::core::{ClassRegistry, TryConvertMut};
use crate::error::{Error, RubyException};
use crate::extn::core::exception::Fatal;
use crate::sys;
use crate::Artichoke;

/// Error type for [`format_unicode_debug_into`].
///
/// This error type can also be used to convert generic [`fmt::Error`] into an
/// [`Error`], such as when formatting integers with [`write!`].
///
/// This  error type wraps a [`fmt::Error`].
///
/// [`Error`]: crate::error::Error
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
