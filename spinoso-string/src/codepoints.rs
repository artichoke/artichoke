use core::fmt::{self, Write};
use core::iter::FusedIterator;
use core::mem;
use core::str::Chars;

use bstr::{ByteSlice, Bytes};

use crate::{Encoding, String};

/// Error returned when failing to construct a [`Codepoints`] iterator/
///
/// This error is returned from [`String::codepoints`]. See its documentation
/// for more detail.
///
/// This error corresponds to the [Ruby `ArgumentError` Exception class].
///
/// When the **std** feature of `spinoso-string` is enabled, this struct
/// implements [`std::error::Error`].
///
/// [Ruby `ArgumentError` Exception class]: https://ruby-doc.org/core-3.1.2/ArgumentError.html
/// [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodepointsError {
    /// Error returned when calling [`String::codepoints`] on a [`String`] with
    /// [UTF-8 encoding] which is not a valid UTF-8 byte string.
    ///
    /// [UTF-8 encoding]: crate::Encoding::Utf8
    InvalidUtf8Codepoint,
}

impl CodepointsError {
    pub const EXCEPTION_TYPE: &'static str = "ArgumentError";

    /// Create a new invalid UTF-8 codepoint `CodepointsError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::CodepointsError;
    ///
    /// const ERR: CodepointsError = CodepointsError::invalid_utf8_codepoint();
    /// assert_eq!(ERR.message(), "invalid byte sequence in UTF-8");
    /// ```
    #[inline]
    #[must_use]
    pub const fn invalid_utf8_codepoint() -> Self {
        Self::InvalidUtf8Codepoint
    }

    /// Retrieve the exception message associated with this center error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::CodepointsError;
    /// let err = CodepointsError::invalid_utf8_codepoint();
    /// assert_eq!(err.message(), "invalid byte sequence in UTF-8");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        "invalid byte sequence in UTF-8"
    }
}

impl fmt::Display for CodepointsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let CodepointsError::InvalidUtf8Codepoint = self;
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CodepointsError {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CodePointRangeError {
    InvalidUtf8Codepoint(u32),
    OutOfRange(i64),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidCodepointError(CodePointRangeError);

impl InvalidCodepointError {
    pub const EXCEPTION_TYPE: &'static str = "RangeError";

    #[inline]
    #[must_use]
    pub const fn invalid_utf8_codepoint(codepoint: u32) -> Self {
        Self(CodePointRangeError::InvalidUtf8Codepoint(codepoint))
    }

    #[inline]
    #[must_use]
    pub const fn codepoint_out_of_range(codepoint: i64) -> Self {
        Self(CodePointRangeError::OutOfRange(codepoint))
    }

    #[inline]
    #[must_use]
    pub const fn is_invalid_utf8(self) -> bool {
        matches!(self.0, CodePointRangeError::InvalidUtf8Codepoint(_))
    }

    #[inline]
    #[must_use]
    pub const fn is_out_of_range(self) -> bool {
        matches!(self.0, CodePointRangeError::OutOfRange(_))
    }

    #[inline]
    #[must_use]
    pub fn message(self) -> alloc::string::String {
        // The longest error message is 27 bytes + a hex-encoded codepoint
        // formatted as `0x...`.
        const MESSAGE_MAX_LENGTH: usize = 27 + 2 + mem::size_of::<u32>() * 2;
        let mut s = alloc::string::String::with_capacity(MESSAGE_MAX_LENGTH);
        // In practice, the errors from `write!` below are safe to ignore
        // because the `core::fmt::Write` impl for `String` will never panic
        // and these `String`s will never approach `isize::MAX` bytes.
        //
        // See the `core::fmt::Display` impl for `InvalidCodepointError`.
        let _ = write!(s, "{self}");
        s
    }
}

impl fmt::Display for InvalidCodepointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            CodePointRangeError::InvalidUtf8Codepoint(codepoint) => {
                write!(f, "invalid codepoint {codepoint:X} in UTF-8")
            }
            CodePointRangeError::OutOfRange(codepoint) => write!(f, "{codepoint} out of char range"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidCodepointError {}

/// An iterator that yields a `u32` codepoints from a [`String`].
///
/// This struct is created by the [`codepoints`] method on a Spinoso [`String`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use spinoso_string::{CodepointsError, String};
/// # fn example() -> Result<(), CodepointsError> {
/// let s = String::from("hello");
///
/// assert_eq!(
///     s.codepoints()?.collect::<Vec<_>>(),
///     [104, 101, 108, 108, 111]
/// );
///
/// let s = String::utf8(b"abc\xFFxyz".to_vec());
/// assert!(matches!(
///     s.codepoints(),
///     Err(CodepointsError::InvalidUtf8Codepoint)
/// ));
///
/// let s = String::binary(b"abc\xFFxyz".to_vec());
/// assert_eq!(
///     s.codepoints()?.collect::<Vec<_>>(),
///     [97, 98, 99, 255, 120, 121, 122]
/// );
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// This iterator is [encoding-aware]. [Conventionally UTF-8] strings are
/// iterated by UTF-8 byte sequences.
///
/// ```
/// use spinoso_string::String;
/// # fn example() -> Result<(), spinoso_string::CodepointsError> {
/// let s = String::from("💎");
///
/// assert_eq!(s.codepoints()?.collect::<Vec<_>>(), [u32::from('💎')]);
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// [`codepoints`]: crate::String::codepoints
/// [encoding-aware]: crate::Encoding
/// [Conventionally UTF-8]: crate::Encoding::Utf8
#[derive(Debug, Default, Clone)]
pub struct Codepoints<'a>(State<'a>);

impl<'a> TryFrom<&'a String> for Codepoints<'a> {
    type Error = CodepointsError;

    #[inline]
    fn try_from(s: &'a String) -> Result<Self, Self::Error> {
        let state = match s.encoding() {
            Encoding::Utf8 => {
                if let Ok(s) = s.inner.as_slice().to_str() {
                    State::Utf8(s.chars())
                } else {
                    return Err(CodepointsError::invalid_utf8_codepoint());
                }
            }
            Encoding::Ascii => {
                let iter = s.as_slice().bytes();
                State::Ascii(iter)
            }
            Encoding::Binary => {
                let iter = s.as_slice().bytes();
                State::Binary(iter)
            }
        };
        Ok(Self(state))
    }
}

impl<'a> Iterator for Codepoints<'a> {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> FusedIterator for Codepoints<'a> {}

#[derive(Debug, Clone)]
enum State<'a> {
    Utf8(Chars<'a>),
    Ascii(Bytes<'a>),
    Binary(Bytes<'a>),
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self::Utf8("".chars())
    }
}

impl<'a> Iterator for State<'a> {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Ascii(iter) | Self::Binary(iter) => iter.next().map(u32::from),
            Self::Utf8(iter) => iter.next().map(u32::from),
        }
    }
}

impl<'a> FusedIterator for State<'a> {}
