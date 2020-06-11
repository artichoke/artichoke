//! Parse encoding parameter to `Regexp#initialize` and `Regexp::compile`.

use bstr::ByteSlice;
use std::error;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::extn::core::regexp;
use crate::extn::prelude::*;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidEncodingError;

impl InvalidEncodingError {
    /// Constructs a new, default `InvalidEncodingError`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for InvalidEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Regexp encoding")
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
#[derive(Debug, Clone, Copy)]
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
        self.modifier_string().hash(state);
    }
}

impl PartialEq for Encoding {
    fn eq(&self, other: &Self) -> bool {
        use Encoding::{Fixed, No, None};
        match (self, other) {
            (No, No) | (No, None) | (None, No) | (None, None) | (Fixed, Fixed) => true,
            _ => false,
        }
    }
}

impl Eq for Encoding {}

impl From<Encoding> for Int {
    /// Convert an `Encoding` to its bitflag representation.
    fn from(enc: Encoding) -> Self {
        match enc {
            Encoding::Fixed => regexp::FIXEDENCODING,
            Encoding::No => regexp::NOENCODING,
            Encoding::None => 0,
        }
    }
}

impl From<&Encoding> for Int {
    /// Convert an `Encoding` to its bitflag representation.
    fn from(enc: &Encoding) -> Self {
        Self::from(*enc)
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.modifier_string())
    }
}

impl Encoding {
    /// Convert an `Encoding` to its bitflag representation.
    ///
    /// Alias for the corresponding `Into<Int>` implementation.
    #[must_use]
    pub fn bitflags(self) -> Int {
        self.into()
    }

    /// Serialize the encoding flags to a string suitable for a `Regexp` display
    /// or debug implementation.
    ///
    /// See also [`Regexp#inspect`][regexp-inspect].
    ///
    /// [regexp-inspect]: https://ruby-doc.org/core-2.7.1/Regexp.html#method-i-inspect
    #[must_use]
    pub fn modifier_string(self) -> &'static str {
        match self {
            Self::Fixed | Self::None => "",
            Self::No => "n",
        }
    }
}

impl TryConvertMut<Value, Encoding> for Artichoke {
    type Error = InvalidEncodingError;

    fn try_convert_mut(&mut self, value: Value) -> Result<Encoding, Self::Error> {
        if let Ok(encoding) = value.try_into::<Int>(self) {
            // Only deal with Encoding opts
            let encoding = encoding & !regexp::ALL_REGEXP_OPTS;
            match encoding {
                regexp::FIXEDENCODING => Ok(Encoding::Fixed),
                regexp::NOENCODING => Ok(Encoding::No),
                0 => Ok(Encoding::default()),
                _ => Err(InvalidEncodingError),
            }
        } else if let Ok(encoding) = value.try_into_mut::<&[u8]>(self) {
            if encoding.find_byte(b'u').is_some() && encoding.find_byte(b'n').is_some() {
                return Err(InvalidEncodingError);
            }
            let mut enc = None;
            for &flag in encoding {
                match flag {
                    b'u' | b's' | b'e' if enc.is_none() => enc = Some(Encoding::Fixed),
                    b'n' if enc.is_none() => enc = Some(Encoding::No),
                    b'i' | b'm' | b'x' | b'o' => continue,
                    _ => return Err(InvalidEncodingError),
                }
            }
            Ok(enc.unwrap_or_default())
        } else {
            Ok(Encoding::default())
        }
    }
}
