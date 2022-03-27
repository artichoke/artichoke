//! Conversions between [platform strings] and bytes.
//!
//! [platform strings]: OsString

use core::fmt;
use std::error;
use std::ffi::{OsStr, OsString};

use bstr::{ByteSlice, ByteVec, Utf8Error};

mod impls;

/// Convert a byte slice to a platform-specific [`OsStr`].
///
/// Unsupported platforms fallback to converting through [`str`].
///
/// # Examples
///
/// ```
/// # use std::ffi::OsStr;
/// # use artichoke_backend::platform_string::bytes_to_os_str;
/// let bytes: &[u8] = b"/etc/passwd";
/// assert_eq!(bytes_to_os_str(bytes), Ok(OsStr::new("/etc/passwd")));
/// ```
///
/// # Errors
///
/// On unix-like platforms, this function is infallible.
///
/// On Windows, if the given byte slice does not contain valid UTF-8, an error
/// is returned.
#[inline]
pub fn bytes_to_os_str(value: &[u8]) -> Result<&OsStr, ConvertBytesError> {
    let platform_string = value.to_os_str()?;
    Ok(platform_string)
}

/// Convert a platform-specific [`OsStr`] to a byte slice.
///
/// Unsupported platforms fallback to converting through [`str`].
///
/// # Examples
///
/// ```
/// # use std::ffi::OsStr;
/// # use artichoke_backend::platform_string::os_str_to_bytes;
/// let platform_string: &OsStr = OsStr::new("/etc/passwd");
/// assert_eq!(os_str_to_bytes(platform_string), Ok(&b"/etc/passwd"[..]));
/// ```
///
/// # Errors
///
/// On unix-like platforms, this function is infallible.
///
/// On Windows, if the given byte slice does not contain valid UTF-8, an error
/// is returned.
#[inline]
pub fn os_str_to_bytes(value: &OsStr) -> Result<&[u8], ConvertBytesError> {
    <[u8]>::from_os_str(value).ok_or_else(ConvertBytesError::new)
}

/// Convert a platform-specific [`OsString`] to a byte vec.
///
/// Unsupported platforms fallback to converting through [`String`].
///
/// # Examples
///
/// ```
/// # use std::ffi::OsString;
/// # use artichoke_backend::platform_string::os_string_to_bytes;
/// let platform_string: OsString = OsString::from("/etc/passwd");
/// assert_eq!(os_string_to_bytes(platform_string), Ok(b"/etc/passwd".to_vec()));
/// ```
///
/// # Errors
///
/// On unix-like platforms, this function is infallible.
///
/// On Windows, if the given byte slice does not contain valid UTF-8, an error
/// is returned.
#[inline]
pub fn os_string_to_bytes(value: OsString) -> Result<Vec<u8>, ConvertBytesError> {
    let bytes = Vec::from_os_string(value)?;
    Ok(bytes)
}

/// Error returned when a [platform string] cannot be converted to a byte
/// vector or a byte vector cannot be converted to a [platform string].
///
/// On unix-like platforms, this error is never returned. On Windows platforms,
/// platform strings can only be converted to byte vectors (and conversely byte
/// vectors to platform strings) when they contain valid UTF-8 contents.
///
/// This error is returned by [`bytes_to_os_str`], [`os_string_to_bytes`] and
/// [`os_str_to_bytes`]. See their documentation for more details.
///
/// [platform string]: OsString
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConvertBytesError {
    _private: (),
}

impl ConvertBytesError {
    /// Constructs a new, default `ConvertBytesError`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use artichoke_backend::platform_string::ConvertBytesError;
    /// const ERR: ConvertBytesError = ConvertBytesError::new();
    /// assert_eq!(ERR.message(), "Could not convert between bytes and platform string");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the exception message associated with this convert bytes error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use artichoke_backend::platform_string::ConvertBytesError;
    /// let err = ConvertBytesError::new();
    /// assert_eq!(err.message(), "Could not convert between bytes and platform string");
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn message(self) -> &'static str {
        "Could not convert between bytes and platform string"
    }
}

impl fmt::Display for ConvertBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl error::Error for ConvertBytesError {}

impl From<Utf8Error> for ConvertBytesError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        let _ = err;
        Self { _private: () }
    }
}

impl From<OsString> for ConvertBytesError {
    #[inline]
    fn from(err: OsString) -> Self {
        drop(err);
        Self { _private: () }
    }
}
