use core::fmt;

// ```
// [2.6.3] > Encoding::UTF_8.names
// => ["UTF-8", "CP65001", "locale", "external", "file system"]
// ```
const UTF8_NAMES: &[&str] = &["UTF-8", "CP65001"];
// ```
// [2.6.3] > Encoding::ASCII.names
// => ["US-ASCII", "ASCII", "ANSI_X3.4-1968", "646"]
// ```
const ASCII_NAMES: &[&str] = &["US-ASCII", "ASCII", "ANSI_X3.4-1968", "646"];
// ```
// [2.6.3] > Encoding::BINARY.names
// => ["ASCII-8BIT", "BINARY"]
// ```
const BINARY_NAMES: &[&str] = &["ASCII-8BIT", "BINARY"];

/// Error returned when failing to deserialize an [`Encoding`].
///
/// This error is returned from [`Encoding::try_from_flag`]. See its
/// documentation for more detail.
///
/// When the **std** feature of `spinoso-string` is enabled, this struct
/// implements [`std::error::Error`].
///
/// # Examples
///
/// ```
/// # use spinoso_string::{Encoding, InvalidEncodingError};
/// assert_eq!(Encoding::try_from_flag(255), Err(InvalidEncodingError::new()));
/// assert_eq!(Encoding::try_from(255), Err(InvalidEncodingError::new()));
/// ```
///
/// [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidEncodingError {
    _private: (),
}

impl InvalidEncodingError {
    /// Construct a new `InvalidEncodingError`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::InvalidEncodingError;
    /// const ERR: InvalidEncodingError = InvalidEncodingError::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl fmt::Display for InvalidEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Could not parse encoding")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidEncodingError {}

/// An Encoding instance represents a character encoding usable in Ruby.
///
/// `spinoso-string` supports three `String` encodings:
///
/// - [UTF-8](Self::Utf8)
/// - [ASCII](Self::Ascii)
/// - [Binary](Self::Binary)
///
/// A `String`'s encoding makes no assertions about the byte content of the
/// `String`'s internal buffer. The `Encoding` associated with a [`String`]
/// modifies how character-oriented APIs behave, for example
/// [`String::char_len`]. A `String` with an UTF-8 encoding is only
/// [conventionally UTF-8] and may contain invalid UTF-8 byte sequences.
///
/// Ruby provides the [`String#encode`] API which can transcode the bytes of a
/// `String` to another encoding. Calling `String#encode` on any of the
/// encodings defined in this enum is a no-op.
///
/// [`String`]: crate::String
/// [`String::char_len`]: crate::String::char_len
/// [UTF-8]: Self::Utf8
/// [conventionally UTF-8]: https://docs.rs/bstr/0.2.*/bstr/#differences-with-standard-strings
/// [`String#encode`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-encode
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Encoding {
    /// Conventionally UTF-8.
    Utf8,
    /// Conventionally ASCII.
    Ascii,
    /// ASCII-8BIT, binary, arbitrary bytes.
    Binary,
}

impl Default for Encoding {
    #[inline]
    fn default() -> Self {
        Self::Utf8
    }
}

impl fmt::Debug for Encoding {
    /// Outputs the value of `Encoding#inspect`.
    ///
    /// Returns a string which represents the encoding for programmers. See
    /// [`Encoding::inspect`].
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.inspect())
    }
}

impl fmt::Display for Encoding {
    /// Outputs the value of `Encoding#to_s`.
    ///
    /// Returns the name of the encoding. See [`Encoding::name`].
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<u8> for Encoding {
    type Error = InvalidEncodingError;

    /// Try to deserialize an `Encoding` from a bitflag.
    ///
    /// See [`Encoding::try_from_flag`].
    #[inline]
    fn try_from(flag: u8) -> Result<Self, InvalidEncodingError> {
        Self::try_from_flag(flag)
    }
}

impl From<Encoding> for u8 {
    /// Serialize an `Encoding` to a bitflag.
    ///
    /// See [`Encoding::to_flag`].
    #[inline]
    fn from(encoding: Encoding) -> Self {
        encoding.to_flag()
    }
}

impl Encoding {
    /// The total number of supported encodings.
    ///
    /// `spinoso-string` supports three encodings:
    ///
    /// - [UTF-8](Self::Utf8)
    /// - [ASCII](Self::Ascii)
    /// - [Binary](Self::Binary)
    pub const NUM_SUPPORTED_ENCODINGS: usize = 3;

    /// Serialize the encoding to a bitflag.
    ///
    /// See [`try_from_flag`] for how to deserialize an encoding.
    ///
    /// This function is used to implement [`From<Encoding>`] for [`u8`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::Encoding;
    /// assert_eq!(Encoding::Utf8.to_flag(), 2);
    /// assert_eq!(Encoding::Ascii.to_flag(), 4);
    /// assert_eq!(Encoding::Binary.to_flag(), 8);
    /// ```
    ///
    /// [`try_from_flag`]: Self::try_from_flag
    /// [`From<Encoding>`]: From
    #[inline]
    #[must_use]
    pub const fn to_flag(self) -> u8 {
        match self {
            Self::Utf8 => 1 << 1,
            Self::Ascii => 1 << 2,
            Self::Binary => 1 << 3,
        }
    }

    /// Deserialize an encoding from a bitflag.
    ///
    /// See [`to_flag`] for how to serialize an encoding.
    ///
    /// This function is used to implement [`TryFrom<u8>`] for `Encoding`.
    ///
    /// # Errors
    ///
    /// If the given flag does not map to any [`Encoding`], an error is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::{Encoding, InvalidEncodingError};
    /// assert_eq!(Encoding::try_from_flag(2), Ok(Encoding::Utf8));
    /// assert_eq!(Encoding::try_from_flag(4), Ok(Encoding::Ascii));
    /// assert_eq!(Encoding::try_from_flag(8), Ok(Encoding::Binary));
    /// assert_eq!(Encoding::try_from_flag(2 | 4), Err(InvalidEncodingError::new()));
    /// assert_eq!(Encoding::try_from_flag(255), Err(InvalidEncodingError::new()));
    /// ```
    ///
    /// [`to_flag`]: Self::to_flag
    /// [`TryFrom<u8>`]: TryFrom
    #[inline]
    pub const fn try_from_flag(flag: u8) -> Result<Self, InvalidEncodingError> {
        match flag {
            x if x == Self::Utf8.to_flag() => Ok(Self::Utf8),
            x if x == Self::Ascii.to_flag() => Ok(Self::Ascii),
            x if x == Self::Binary.to_flag() => Ok(Self::Binary),
            _ => Err(InvalidEncodingError::new()),
        }
    }

    /// Returns a string which represents the encoding for programmers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::Encoding;
    /// assert_eq!(Encoding::Utf8.inspect(), "#<Encoding:UTF-8>");
    /// assert_eq!(Encoding::Ascii.inspect(), "#<Encoding:US-ASCII>");
    /// assert_eq!(Encoding::Binary.inspect(), "#<Encoding:ASCII-8BIT>");
    /// ```
    ///
    /// # Ruby Examples
    ///
    /// ```ruby
    /// Encoding::UTF_8.inspect       #=> "#<Encoding:UTF-8>"
    /// Encoding::ISO_2022_JP.inspect #=> "#<Encoding:ISO-2022-JP (dummy)>"
    /// ```
    #[must_use]
    pub const fn inspect(self) -> &'static str {
        match self {
            // ```
            // [2.6.3] > Encoding::UTF_8.inspect
            // => "#<Encoding:UTF-8>"
            // ```
            Self::Utf8 => "#<Encoding:UTF-8>",
            // ```
            // [2.6.3] > Encoding::ASCII.inspect
            // => "#<Encoding:US-ASCII>"
            // ```
            Self::Ascii => "#<Encoding:US-ASCII>",
            // ```
            // [2.6.3] > Encoding::BINARY.inspect
            // => "#<Encoding:ASCII-8BIT>"
            // ```
            Self::Binary => "#<Encoding:ASCII-8BIT>",
        }
    }

    /// Returns the name of the encoding.
    ///
    /// This function is used to implement [`fmt::Display`] for `Encoding`.
    ///
    /// This function can be used to implement the Ruby functions
    /// `Encoding#name` and `Encoding#to_s`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::Encoding;
    /// assert_eq!(Encoding::Utf8.name(), "UTF-8");
    /// assert_eq!(Encoding::Ascii.name(), "US-ASCII");
    /// assert_eq!(Encoding::Binary.name(), "ASCII-8BIT");
    /// ```
    ///
    /// # Ruby Examples
    ///
    /// ```ruby
    /// Encoding::UTF_8.name      #=> "UTF-8"
    /// ```
    #[inline]
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            // [2.6.3] > Encoding::UTF_8.name
            // => "UTF-8"
            Self::Utf8 => "UTF-8",
            // [2.6.3] > Encoding::ASCII.name
            // => "US-ASCII"
            Self::Ascii => "US-ASCII",
            // [2.6.3] > Encoding::BINARY.name
            // => "ASCII-8BIT"
            Self::Binary => "ASCII-8BIT",
        }
    }

    /// Returns the list of name and aliases of the encoding.
    ///
    /// This function can be used to implement the Ruby function
    /// `Encoding#names`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::Encoding;
    /// assert_eq!(Encoding::Utf8.names(), ["UTF-8", "CP65001"]);
    /// assert_eq!(Encoding::Ascii.names(), ["US-ASCII", "ASCII", "ANSI_X3.4-1968", "646"]);
    /// assert_eq!(Encoding::Binary.names(), ["ASCII-8BIT", "BINARY"]);
    /// ```
    ///
    /// # Ruby Examples
    ///
    /// ```ruby
    /// Encoding::WINDOWS_31J.names  #=> ["Windows-31J", "CP932", "csWindows31J"]
    /// ```
    #[inline]
    #[must_use]
    pub const fn names(self) -> &'static [&'static str] {
        match self {
            Self::Utf8 => UTF8_NAMES,
            Self::Ascii => ASCII_NAMES,
            Self::Binary => BINARY_NAMES,
        }
    }

    /// Returns whether ASCII-compatible or not.
    ///
    /// This function can be used to implement the Ruby function
    /// `Encoding#ascii_compatible?`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::Encoding;
    /// assert!(Encoding::Utf8.is_ascii_compatible());
    /// assert!(Encoding::Ascii.is_ascii_compatible());
    /// assert!(Encoding::Binary.is_ascii_compatible());
    /// ```
    ///
    /// # Ruby Examples
    ///
    /// ```ruby
    /// Encoding::UTF_8.ascii_compatible?     #=> true
    /// Encoding::UTF_16BE.ascii_compatible?  #=> false
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_compatible(self) -> bool {
        matches!(self, Self::Utf8 | Self::Ascii | Self::Binary)
    }

    /// Returns true for dummy encodings.
    ///
    /// A dummy encoding is an encoding for which character handling is not
    /// properly implemented. It is used for stateful encodings.
    ///
    /// This function can be used to implement the Ruby function
    /// `Encoding#dummy?`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::Encoding;
    /// assert!(!Encoding::Utf8.is_dummy());
    /// assert!(!Encoding::Ascii.is_dummy());
    /// assert!(!Encoding::Binary.is_dummy());
    /// ```
    ///
    /// # Ruby Examples
    ///
    /// ```ruby
    /// Encoding::ISO_2022_JP.dummy?       #=> true
    /// Encoding::UTF_8.dummy?             #=> false
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_dummy(self) -> bool {
        !matches!(self, Self::Utf8 | Self::Ascii | Self::Binary)
    }
}
