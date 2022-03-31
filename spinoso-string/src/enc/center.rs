use core::fmt;

/// Error returned when calling [`AsciiString::center`] with an empty padding
/// byte string.
///
/// [`AsciiString::center`]: crate::enc::AsciiString::center
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroWidthPaddingError {
    _private: (),
}

impl ZeroWidthPaddingError {
    pub const EXCEPTION_TYPE: &'static str = "ArgumentError";

    /// Create a new zero width padding `CenterError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::ZeroWidthPaddingError;
    ///
    /// const ERR: CenterError = ZeroWidthPaddingError::new();
    /// assert_eq!(ERR.message(), "zero width padding");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the exception message associated with this center error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::ZeroWidthPaddingError;
    /// let err = ZeroWidthPaddingError::new();
    /// assert_eq!(err.message(), "zero width padding");
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn message(self) -> &'static str {
        "zero width padding"
    }
}

impl fmt::Display for ZeroWidthPaddingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ZeroWidthPaddingError {}
