//! Utilities for working with Ruby `String`s.
//!
//! In Ruby, `String` is a `Vec<u8>` with an optional encoding. `String`s
//! default to UTF-8 encoding but this does not require the byte vector to
//! contain valid UTF-8.
//!
//! Artichoke aims to support ASCII, UTF-8, maybe UTF-8, and binary encodings.

use bstr::ByteSlice;
use std::error;
use std::fmt;
use std::io;

use crate::class_registry::ClassRegistry;
use crate::core::ConvertMut;
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception::Fatal;
use crate::sys;
use crate::Artichoke;

/// Write a UTF-8 debug representation of a byte slice into the given writer.
///
/// This method encodes a bytes slice into a UTF-8 valid representation by
/// writing invalid sequences as `\xXX` escape codes.
///
/// This method also escapes UTF-8 valid characters like `\n` and `\t`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use artichoke_backend::string::format_unicode_debug_into;
///
/// let mut message = String::from("cannot load such file -- ");
/// let filename = b"utf8-invalid-name-\xFF";
/// format_unicode_debug_into(&mut message, &filename[..]);
/// assert_eq!(r"cannot load such file -- utf8-invalid-name-\xFF", message);
/// ```
///
/// # Errors
///
/// This method only returns an error when the given writer returns an
/// error.
pub fn format_unicode_debug_into<W>(mut f: W, string: &[u8]) -> Result<(), WriteError>
where
    W: fmt::Write,
{
    for (start, end, ch) in string.char_indices() {
        if ch == '\u{FFFD}' {
            if let Some(slice) = string.get(start..end) {
                for byte in slice {
                    write!(f, r"\x{:X}", byte).map_err(WriteError)?;
                }
            }
        } else {
            write!(f, "{}", ch.escape_debug()).map_err(WriteError)?;
        }
    }
    Ok(())
}

pub fn format_int_into<W, I>(f: W, value: I) -> Result<(), WriteError>
where
    W: fmt::Write,
    I: itoa::Integer,
{
    itoa::fmt(f, value).map_err(WriteError)?;
    Ok(())
}

pub fn write_float_into<W, F>(f: W, value: F) -> Result<(), IoWriteError>
where
    W: io::Write,
    F: dtoa::Floating,
{
    // Potentially replace with a `fmt` variant for better ergonomics like
    // `format_into_int` above.
    //
    // See: https://github.com/dtolnay/dtoa/issues/18
    dtoa::write(f, value).map_err(IoWriteError)?;
    Ok(())
}

/// Error type for [`format_unicode_debug_into`] and [`format_int_into`].
///
/// This error type wraps a [`fmt::Error`].
#[derive(Debug, Clone)]
pub struct WriteError(fmt::Error);

impl WriteError {
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> fmt::Error {
        self.0
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to write message into destination")
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
    fn message(&self) -> &[u8] {
        &b"Unable to write message into destination"[..]
    }

    #[inline]
    fn name(&self) -> String {
        String::from("fatal")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let spec = interp.class_spec::<Fatal>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<WriteError> for Exception {
    #[inline]
    fn from(exception: WriteError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<WriteError>> for Exception {
    #[inline]
    fn from(exception: Box<WriteError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<WriteError> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: WriteError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<WriteError>> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: Box<WriteError>) -> Box<dyn RubyException> {
        exception
    }
}

/// Error type for [`write_float_into`].
///
/// This error type wraps an [`io::Error`].
#[derive(Debug)]
pub struct IoWriteError(io::Error);

impl IoWriteError {
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> io::Error {
        self.0
    }
}

impl fmt::Display for IoWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to write message into destination")
    }
}

impl error::Error for IoWriteError {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

impl RubyException for IoWriteError {
    #[inline]
    fn message(&self) -> &[u8] {
        &b"Unable to write message"[..]
    }

    #[inline]
    fn name(&self) -> String {
        String::from("fatal")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let spec = interp.class_spec::<Fatal>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<IoWriteError> for Exception {
    #[inline]
    fn from(exception: IoWriteError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<IoWriteError>> for Exception {
    #[inline]
    fn from(exception: Box<IoWriteError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<IoWriteError> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: IoWriteError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<IoWriteError>> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: Box<IoWriteError>) -> Box<dyn RubyException> {
        exception
    }
}

#[cfg(test)]
mod tests {
    use super::format_unicode_debug_into;

    #[test]
    fn invalid_utf8() {
        let mut buf = String::new();
        format_unicode_debug_into(&mut buf, &b"abc\xFF"[..]).unwrap();
        assert_eq!(r"abc\xFF", buf.as_str());
    }

    #[test]
    fn ascii() {
        let mut buf = String::new();
        format_unicode_debug_into(&mut buf, &b"abc"[..]).unwrap();
        assert_eq!(r"abc", buf.as_str());
    }

    #[test]
    fn emoji() {
        let mut buf = String::new();
        format_unicode_debug_into(&mut buf, "Ruby 💎".as_bytes()).unwrap();
        assert_eq!(r"Ruby 💎", buf.as_str());
    }

    #[test]
    fn escaped() {
        let mut buf = String::new();
        format_unicode_debug_into(&mut buf, b"\n").unwrap();
        assert_eq!(r"\n", buf.as_str());
    }
}
