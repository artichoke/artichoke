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

use crate::exception::{Exception, RubyException};
use crate::extn::core::exception::Fatal;
use crate::sys;
use crate::{Artichoke, ConvertMut};

/// Write a UTF-8 representation of a (potentially) binary `String`.
///
/// This function encodes a bytes slice into a UTF-8 valid representation by
/// writing invalid sequences as `\xXX` escape codes.
///
/// This function uses `char::escape_debug` which means UTF-8 valid characters
/// like `\n` and `\t` are also escaped.
///
/// # Examples
///
/// ```
/// # use artichoke_backend::string::escape_unicode;
/// let mut message = String::from("cannot load such file -- ");
/// let filename = b"oh-no-\xFF";
/// escape_unicode(&mut message, &filename[..]);
/// assert_eq!(r"cannot load such file -- oh-no-\xFF", message);
/// ```
pub fn escape_unicode<T>(f: &mut T, string: &[u8]) -> Result<(), WriteError>
where
    T: fmt::Write,
{
    let buf = bstr::B(string);
    for (start, end, ch) in buf.char_indices() {
        if ch == '\u{FFFD}' {
            for byte in buf[start..end].as_bytes() {
                write!(f, r"\x{:X}", byte)?;
            }
        } else {
            write!(f, "{}", ch.escape_debug())?;
        }
    }
    Ok(())
}

pub fn format_int_into<T, I>(f: &mut T, value: I) -> Result<(), WriteError>
where
    T: fmt::Write,
    I: itoa::Integer,
{
    itoa::fmt(f, value)?;
    Ok(())
}

/// Error type for [`escape_unicode`].
///
/// This error type wraps a [`fmt::Error`].
#[derive(Debug, Clone)]
pub struct WriteError(fmt::Error);

impl WriteError {
    #[must_use]
    pub fn into_inner(self) -> fmt::Error {
        self.0
    }
}

impl From<fmt::Error> for WriteError {
    fn from(err: fmt::Error) -> Self {
        Self(err)
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to write escaped Unicode into destination")
    }
}

impl error::Error for WriteError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

impl RubyException for WriteError {
    fn box_clone(&self) -> Box<dyn RubyException> {
        Box::new(self.clone())
    }

    fn message(&self) -> &[u8] {
        &b"Unable to escape Unicode message"[..]
    }

    fn name(&self) -> String {
        String::from("fatal")
    }

    fn vm_backtrace(&self, interp: &Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let borrow = interp.0.borrow();
        let spec = borrow.class_spec::<Fatal>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<WriteError> for Exception {
    fn from(exception: WriteError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<WriteError>> for Exception {
    fn from(exception: Box<WriteError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<WriteError> for Box<dyn RubyException> {
    fn from(exception: WriteError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<WriteError>> for Box<dyn RubyException> {
    fn from(exception: Box<WriteError>) -> Box<dyn RubyException> {
        exception
    }
}

#[cfg(test)]
mod tests {
    use super::escape_unicode;

    #[test]
    fn invalid_utf8() {
        let mut buf = String::new();
        escape_unicode(&mut buf, &b"abc\xFF"[..]).unwrap();
        assert_eq!(r"abc\xFF", buf.as_str());
    }

    #[test]
    fn ascii() {
        let mut buf = String::new();
        escape_unicode(&mut buf, &b"abc"[..]).unwrap();
        assert_eq!(r"abc", buf.as_str());
    }

    #[test]
    fn emoji() {
        let mut buf = String::new();
        escape_unicode(&mut buf, "Ruby 💎".as_bytes()).unwrap();
        assert_eq!(r"Ruby 💎", buf.as_str());
    }

    #[test]
    fn escaped() {
        let mut buf = String::new();
        escape_unicode(&mut buf, b"\n").unwrap();
        assert_eq!(r"\n", buf.as_str());
    }
}
