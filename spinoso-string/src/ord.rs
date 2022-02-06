#[cfg(feature = "std")]
use std::fmt;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrdError {
    /// The first character in a [conventionally UTF-8] `String` is an invalid
    /// UTF-8 byte sequence.
    ///
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    InvalidUtf8ByteSequence,
    /// The given `String` is empty and has no first character.
    EmptyString,
}

impl OrdError {
    /// `OrdError` corresponds to an [`ArgumentError`] Ruby exception.
    ///
    /// [`ArgumentError`]: https://ruby-doc.org/core-2.6.3/ArgumentError.html
    pub const EXCEPTION_TYPE: &'static str = "ArgumentError";

    /// Construct a new `OrdError` for an invalid UTF-8 byte sequence.
    ///
    /// Only [conventionally UTF-8] `String`s can generate this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::{OrdError, String};
    ///
    /// let s = String::utf8(b"\xFFabc".to_vec());
    /// assert_eq!(s.ord(), Err(OrdError::invalid_utf8_byte_sequence()));
    ///
    /// let s = String::binary(b"\xFFabc".to_vec());
    /// assert_eq!(s.ord(), Ok(0xFF));
    /// ```
    ///
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    #[inline]
    #[must_use]
    pub const fn invalid_utf8_byte_sequence() -> Self {
        Self::InvalidUtf8ByteSequence
    }

    /// Construct a new `OrdError` for an empty `String`.
    ///
    /// Empty `String`s have no first character. Empty `String`s with any
    /// encoding return this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::{OrdError, String};
    ///
    /// let s = String::utf8(b"\xFFabc".to_vec());
    /// assert_eq!(s.ord(), Err(OrdError::invalid_utf8_byte_sequence()));
    /// ```
    #[inline]
    #[must_use]
    pub const fn empty_string() -> Self {
        Self::EmptyString
    }

    /// Error message for this `OrdError`.
    ///
    /// This message is suitable for generating an [`ArgumentError`] exception
    /// from this `OrdError`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::OrdError;
    ///
    /// assert_eq!(OrdError::invalid_utf8_byte_sequence().message(), "invalid byte sequence in UTF-8");
    /// assert_eq!(OrdError::empty_string().message(), "empty string");
    /// ```
    ///
    /// [`ArgumentError`]: https://ruby-doc.org/core-2.6.3/ArgumentError.html
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidUtf8ByteSequence => "invalid byte sequence in UTF-8",
            Self::EmptyString => "empty string",
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for OrdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OrdError {}
