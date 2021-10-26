use core::fmt;
use core::iter::FusedIterator;
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
/// [Ruby `ArgumentError` Exception class]: https://ruby-doc.org/core-2.6.3/ArgumentError.html
/// [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodepointsError {
    /// Error returned when calling [`String::codepoints`] on a [`String`] with
    /// [UTF-8 encoding] which is not a valid UTF-8 bytestring.
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
    #[allow(clippy::unused_self)]
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

/// An iterator that yields a `u32` codepoints from a [`String`].
///
/// This struct is created by the [`codepoints`] method on a Spinoso [`String`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use spinoso_string::{CodepointsError, String};
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let s = String::from("hello");
///
/// assert_eq!(s.codepoints()?.collect::<Vec<_>>(), [104, 101, 108, 108, 111]);
///
/// let s = String::utf8(b"abc\xFFxyz".to_vec());
/// assert!(matches!(s.codepoints(), Err(CodepointsError::InvalidUtf8Codepoint)));
///
/// let s = String::binary(b"abc\xFFxyz".to_vec());
/// assert_eq!(s.codepoints()?.collect::<Vec<_>>(), [97, 98, 99, 255, 120, 121, 122]);
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
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
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
                if let Ok(s) = s.buf.to_str() {
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
