use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::iter::Iterator;
use std::num::NonZeroU32;
use std::str::{self, FromStr};

use bstr::ByteSlice;

use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radix(NonZeroU32);

impl Default for Radix {
    fn default() -> Self {
        // Safety:
        // Constant `10` is non-zero.
        unsafe { Self::new_unchecked(10) }
    }
}

impl From<Radix> for u32 {
    fn from(radix: Radix) -> Self {
        radix.as_u32()
    }
}

impl Radix {
    /// Construct a new `Radix`.
    ///
    /// `radix` must be non-zero otherwise `None` is returned.
    #[must_use]
    pub fn new(radix: u32) -> Option<Self> {
        NonZeroU32::new(radix).map(Self)
    }

    /// Construct a new `Radix` without checking the value.
    ///
    /// # Safety
    ///
    /// The radix must not be zero.
    #[must_use]
    pub unsafe fn new_unchecked(radix: u32) -> Self {
        Self(NonZeroU32::new_unchecked(radix))
    }

    /// Extract the `Radix` as the underlying `u32`.
    #[inline]
    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.0.get()
    }
}

impl TryConvertMut<Option<Value>, Option<Radix>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<Value>) -> Result<Option<Radix>, Self::Error> {
        if let Some(value) = value {
            let num = value.implicitly_convert_to_int(self)?;
            let radix = u32::try_from(num).map_err(|_| ArgumentError::with_message("invalid radix"))?;
            if (2..=36).contains(&radix) {
                Ok(Radix::new(radix))
            } else {
                let mut message = String::from("invalid radix ");
                itoa::fmt(&mut message, radix).map_err(WriteError::from)?;
                Err(ArgumentError::from(message).into())
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct IntegerString<'a>(&'a str);

impl<'a> From<&'a str> for IntegerString<'a> {
    fn from(to_parse: &'a str) -> Self {
        Self(to_parse)
    }
}

impl<'a> TryFrom<&'a [u8]> for IntegerString<'a> {
    type Error = Utf8Error;

    fn try_from(to_parse: &'a [u8]) -> Result<Self, Self::Error> {
        if to_parse.find_byte(b'\0').is_some() {
            return Err(Utf8Error::NulByte);
        }
        let to_parse = str::from_utf8(to_parse)?;
        Ok(to_parse.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utf8Error {
    NulByte,
    InvalidUtf8(str::Utf8Error),
}

impl error::Error for Utf8Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::NulByte => None,
            Self::InvalidUtf8(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NulByte => f.write_str("String contained forbidden NUL byte"),
            Self::InvalidUtf8(_) => f.write_str("String contained invalid UTF-8 bytes"),
        }
    }
}

impl From<str::Utf8Error> for Utf8Error {
    fn from(err: str::Utf8Error) -> Self {
        Self::InvalidUtf8(err)
    }
}

impl<'a> IntegerString<'a> {
    /// Constructs a new, empty `IntegerString`.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self("")
    }

    #[must_use]
    pub fn from_slice(arg: &'a [u8]) -> Option<Self> {
        arg.try_into().ok()
    }

    #[inline]
    #[must_use]
    pub fn inner(self) -> &'a str {
        self.0
    }

    #[inline]
    #[must_use]
    pub fn as_bytes(self) -> &'a [u8] {
        self.0.as_bytes()
    }
}

impl<'a> TryConvertMut<&'a mut Value, IntegerString<'a>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &'a mut Value) -> Result<IntegerString<'a>, Self::Error> {
        let mut message = String::from("can't convert ");
        message.push_str(self.inspect_type_name_for_value(*value));
        message.push_str(" into Integer");

        if let Ok(arg) = value.implicitly_convert_to_string(self) {
            if let Some(converted) = IntegerString::from_slice(arg) {
                Ok(converted)
            } else {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                format_unicode_debug_into(&mut message, arg)?;
                message.push('"');
                Err(ArgumentError::from(message).into())
            }
        } else {
            Err(TypeError::from(message).into())
        }
    }
}

impl<'a> From<IntegerString<'a>> for &'a [u8] {
    #[inline]
    fn from(string: IntegerString<'a>) -> &'a [u8] {
        string.as_bytes()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Sign {
    Positive,
    Negative,
}

impl Sign {
    const fn new() -> Self {
        Self::Positive
    }
}

impl Default for Sign {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum ParseState<'a> {
    Initial(IntegerString<'a>),
    Sign(IntegerString<'a>, Sign),
    Accumulate(IntegerString<'a>, Sign, String),
}

impl<'a> ParseState<'a> {
    #[inline]
    fn new(arg: IntegerString<'a>) -> Self {
        Self::Initial(arg)
    }

    fn set_sign(self, sign: Sign) -> Result<Self, Error> {
        match self {
            Self::Sign(arg, _) | Self::Accumulate(arg, _, _) => {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                format_unicode_debug_into(&mut message, arg.into())?;
                message.push('"');
                Err(ArgumentError::from(message).into())
            }
            Self::Initial(arg) => Ok(ParseState::Sign(arg, sign)),
        }
    }

    fn collect_digit(self, digit: char) -> Self {
        match self {
            Self::Initial(arg) => {
                let mut digits = String::new();
                digits.push(digit);
                Self::Accumulate(arg, Sign::new(), digits)
            }
            Self::Sign(arg, sign) => {
                let mut digits = String::new();
                digits.push(digit);
                Self::Accumulate(arg, sign, digits)
            }
            Self::Accumulate(arg, sign, mut digits) => {
                digits.push(digit);
                Self::Accumulate(arg, sign, digits)
            }
        }
    }

    fn parse(self) -> Result<(String, Option<Radix>), Error> {
        let (arg, sign, mut digits) = match self {
            Self::Accumulate(arg, sign, digits) => (arg, sign, digits),
            Self::Initial(arg) | Self::Sign(arg, _) => {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                format_unicode_debug_into(&mut message, arg.into())?;
                message.push('"');
                return Err(ArgumentError::from(message).into());
            }
        };
        let radix = match digits.as_bytes() {
            [b'0', b'b', ..] | [b'0', b'B', ..] => {
                digits.drain(..2);
                Radix::new(2)
            }
            [b'0', b'o', ..] | [b'0', b'O', ..] => {
                digits.drain(..2);
                Radix::new(8)
            }
            [b'0', b'd', ..] | [b'0', b'D', ..] => {
                digits.drain(..2);
                Radix::new(10)
            }
            [b'0', b'x', ..] | [b'0', b'X', ..] => {
                digits.drain(..2);
                Radix::new(16)
            }
            [x, y, ..] => {
                let first = char::from(*x);
                let next = char::from(*y);
                if !next.is_numeric() && !next.is_alphabetic() {
                    let mut message = String::from(r#"invalid value for Integer(): ""#);
                    format_unicode_debug_into(&mut message, arg.into())?;
                    message.push('"');
                    return Err(ArgumentError::from(message).into());
                } else if '0' == first {
                    Radix::new(8)
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Sign::Negative = sign {
            digits.insert(0, '-');
        }
        Ok((digits, radix))
    }
}

pub fn method(arg: IntegerString<'_>, radix: Option<Radix>) -> Result<Int, Error> {
    let mut state = ParseState::new(arg);
    let mut chars = arg.inner().chars().skip_while(|c| c.is_whitespace()).peekable();
    let mut prev = None::<char>;

    while let Some(current) = chars.next() {
        // Ignore an embedded underscore (`_`).
        if current == '_' {
            let valid_prev = prev.map_or(false, |prev| prev.is_numeric() || prev.is_alphabetic());
            let next = chars.peek();
            let valid_next = next.map_or(false, |next| next.is_numeric() || next.is_alphabetic());
            if valid_prev && valid_next {
                prev = Some(current);
                continue;
            }
        }
        if current.is_whitespace() {
            if let Some('+') | Some('-') = prev {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                format_unicode_debug_into(&mut message, arg.into())?;
                message.push('"');
                return Err(ArgumentError::from(message).into());
            } else {
                prev = Some(current);
                continue;
            }
        }

        state = match current {
            '+' => state.set_sign(Sign::Positive)?,
            '-' => state.set_sign(Sign::Negative)?,
            digit => state.collect_digit(digit),
        };
        prev = Some(current);
    }

    let (src, src_radix) = state.parse()?;

    let parsed_int = match (radix, src_radix) {
        (Some(x), Some(y)) if x == y => Int::from_str_radix(src.as_str(), x.as_u32()).ok(),
        (None, None) => Int::from_str(src.as_str()).ok(),
        (Some(x), None) | (None, Some(x)) if (2..=36).contains(&x.as_u32()) => {
            Int::from_str_radix(src.as_str(), x.as_u32()).ok()
        }
        _ => None,
    };
    if let Some(parsed_int) = parsed_int {
        Ok(parsed_int)
    } else {
        let mut message = String::from(r#"invalid value for Integer(): ""#);
        format_unicode_debug_into(&mut message, arg.into())?;
        message.push('"');
        Err(ArgumentError::from(message).into())
    }
}
