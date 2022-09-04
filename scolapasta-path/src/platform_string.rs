//! Conversions between [platform strings] and bytes.
//!
//! [platform strings]: OsString

use core::fmt;
use std::error;
use std::ffi::{OsStr, OsString};

#[cfg(not(any(unix, windows, target_os = "wasi")))]
mod default;
#[cfg(any(unix, target_os = "wasi"))]
mod unix_wasi;
#[cfg(windows)]
mod windows;

#[cfg(not(any(unix, windows, target_os = "wasi")))]
use default as imp;
#[cfg(any(unix, target_os = "wasi"))]
use unix_wasi as imp;
#[cfg(windows)]
use windows as imp;

/// Convert a byte slice to a platform-specific [`OsStr`].
///
/// Unsupported platforms fallback to converting through [`str`].
///
/// # Examples
///
/// ```
/// # use std::ffi::OsStr;
/// # use scolapasta_path::bytes_to_os_str;
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
pub fn bytes_to_os_str(bytes: &[u8]) -> Result<&OsStr, ConvertBytesError> {
    imp::bytes_to_os_str(bytes)
}

/// Convert a platform-specific [`OsStr`] to a byte slice.
///
/// Unsupported platforms fallback to converting through [`str`].
///
/// # Examples
///
/// ```
/// # use std::ffi::OsStr;
/// # use scolapasta_path::os_str_to_bytes;
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
pub fn os_str_to_bytes(os_str: &OsStr) -> Result<&[u8], ConvertBytesError> {
    imp::os_str_to_bytes(os_str)
}

/// Convert a platform-specific [`OsString`] to a byte vec.
///
/// Unsupported platforms fallback to converting through [`String`].
///
/// # Examples
///
/// ```
/// # use std::ffi::OsString;
/// # use scolapasta_path::os_string_to_bytes;
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
pub fn os_string_to_bytes(os_string: OsString) -> Result<Vec<u8>, ConvertBytesError> {
    imp::os_string_to_bytes(os_string)
}

/// Error returned when a [platform string] cannot be converted to bytes or
/// bytes cannot be converted to a [platform string].
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
    /// # use scolapasta_path::ConvertBytesError;
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
    /// # use scolapasta_path::ConvertBytesError;
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
