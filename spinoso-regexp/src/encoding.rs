//! Parse encoding parameter to `Regexp#initialize` and `Regexp::compile`.

use bstr::ByteSlice;
use core::convert::TryFrom;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::mem;
use std::error;

use crate::Flags;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidEncodingError {
    _private: (),
}

impl InvalidEncodingError {
    /// Constructs a new, default `InvalidEncodingError`.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl fmt::Display for InvalidEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid Regexp encoding")
    }
}

impl error::Error for InvalidEncodingError {}

/// The encoding of a Regexp literal.
///
/// Regexps are assumed to use the source encoding but literals may override
/// the encoding with a Regexp modifier.
///
/// See [`Regexp encoding][regexp-encoding].
///
/// [regexp-encoding]: https://ruby-doc.org/core-2.6.3/Regexp.html#class-Regexp-label-Encoding
#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
pub enum Encoding {
    Fixed,
    No,
    None,
}

impl Default for Encoding {
    fn default() -> Self {
        Self::None
    }
}

impl Hash for Encoding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let discriminant = mem::discriminant(self);
        discriminant.hash(state);
    }
}

impl PartialEq for Encoding {
    fn eq(&self, other: &Self) -> bool {
        use Encoding::{Fixed, No, None};

        matches!((self, other), (No | None, No | None) | (Fixed, Fixed))
    }
}

impl Eq for Encoding {}

impl TryFrom<Flags> for Encoding {
    type Error = InvalidEncodingError;

    fn try_from(mut flags: Flags) -> Result<Self, Self::Error> {
        flags.set(Flags::ALL_REGEXP_OPTS, false);
        if flags.intersects(Flags::FIXEDENCODING) {
            Ok(Self::Fixed)
        } else if flags.intersects(Flags::NOENCODING) {
            Ok(Encoding::No)
        } else if flags.is_empty() {
            Ok(Encoding::new())
        } else {
            Err(InvalidEncodingError::new())
        }
    }
}

impl TryFrom<u8> for Encoding {
    type Error = InvalidEncodingError;

    fn try_from(flags: u8) -> Result<Self, Self::Error> {
        let flags = Flags::from_bits(flags).ok_or_else(InvalidEncodingError::new)?;
        Self::try_from(flags)
    }
}

impl TryFrom<i64> for Encoding {
    type Error = InvalidEncodingError;

    fn try_from(flags: i64) -> Result<Self, Self::Error> {
        let [byte, _, _, _, _, _, _, _] = flags.to_le_bytes();
        Self::try_from(byte)
    }
}

impl TryFrom<&str> for Encoding {
    type Error = InvalidEncodingError;

    fn try_from(encoding: &str) -> Result<Self, Self::Error> {
        if encoding.contains('u') && encoding.contains('n') {
            return Err(InvalidEncodingError::new());
        }
        let mut enc = None;
        for flag in encoding.bytes() {
            match flag {
                b'u' | b's' | b'e' if enc.is_none() => enc = Some(Encoding::Fixed),
                b'n' if enc.is_none() => enc = Some(Encoding::No),
                b'i' | b'm' | b'x' | b'o' => continue,
                _ => return Err(InvalidEncodingError::new()),
            }
        }
        Ok(enc.unwrap_or_default())
    }
}

impl TryFrom<&[u8]> for Encoding {
    type Error = InvalidEncodingError;

    fn try_from(encoding: &[u8]) -> Result<Self, Self::Error> {
        if encoding.find_byte(b'u').is_some() && encoding.find_byte(b'n').is_some() {
            return Err(InvalidEncodingError::new());
        }
        let mut enc = None;
        for &flag in encoding {
            match flag {
                b'u' | b's' | b'e' if enc.is_none() => enc = Some(Encoding::Fixed),
                b'n' if enc.is_none() => enc = Some(Encoding::No),
                b'i' | b'm' | b'x' | b'o' => continue,
                _ => return Err(InvalidEncodingError::new()),
            }
        }
        Ok(enc.unwrap_or_default())
    }
}

impl TryFrom<String> for Encoding {
    type Error = InvalidEncodingError;

    fn try_from(encoding: String) -> Result<Self, Self::Error> {
        Self::try_from(encoding.as_str())
    }
}

impl TryFrom<Vec<u8>> for Encoding {
    type Error = InvalidEncodingError;

    fn try_from(encoding: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(encoding.as_slice())
    }
}

impl From<Encoding> for Flags {
    /// Convert an `Encoding` to its bit flag representation.
    fn from(encoding: Encoding) -> Self {
        encoding.flags()
    }
}

impl From<&Encoding> for Flags {
    /// Convert an `Encoding` to its bit flag representation.
    fn from(encoding: &Encoding) -> Self {
        encoding.flags()
    }
}

impl From<Encoding> for u8 {
    /// Convert an `Encoding` to its bit representation.
    fn from(encoding: Encoding) -> Self {
        encoding.into_bits()
    }
}

impl From<&Encoding> for u8 {
    /// Convert an `Encoding` to its bit representation.
    fn from(encoding: &Encoding) -> Self {
        encoding.into_bits()
    }
}

impl From<Encoding> for i64 {
    /// Convert an `Encoding` to its widened bit representation.
    fn from(encoding: Encoding) -> Self {
        encoding.into_bits().into()
    }
}

impl From<&Encoding> for i64 {
    /// Convert an `Encoding` to its widened bit representation.
    fn from(encoding: &Encoding) -> Self {
        encoding.into_bits().into()
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.modifier_string())
    }
}

impl Encoding {
    /// Construct a new [`None`] encoding.
    ///
    /// [`None`]: Self::None
    #[must_use]
    pub const fn new() -> Self {
        Self::None
    }

    /// Convert an `Encoding` to its bit flag representation.
    ///
    /// Alias for the corresponding `Into<Flags>` implementation.
    #[must_use]
    pub const fn flags(self) -> Flags {
        match self {
            Encoding::Fixed => Flags::FIXEDENCODING,
            Encoding::No => Flags::NOENCODING,
            Encoding::None => Flags::empty(),
        }
    }

    /// Convert an `Encoding` to its bit representation.
    ///
    /// Alias for the corresponding `Into<u8>` implementation.
    #[must_use]
    pub const fn into_bits(self) -> u8 {
        self.flags().bits()
    }

    /// Serialize the encoding flags to a string suitable for a `Regexp` display
    /// or debug implementation.
    ///
    /// See also [`Regexp#inspect`][regexp-inspect].
    ///
    /// [regexp-inspect]: https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-inspect
    #[must_use]
    pub const fn modifier_string(self) -> &'static str {
        match self {
            Self::Fixed | Self::None => "",
            Self::No => "n",
        }
    }
}
