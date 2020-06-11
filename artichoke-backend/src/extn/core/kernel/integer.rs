use bstr::ByteSlice;
use std::convert::TryFrom;
use std::iter::Iterator;
use std::num::NonZeroU32;
use std::str::{self, FromStr};

use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radix(NonZeroU32);

impl Default for Radix {
    fn default() -> Self {
        Self(unsafe { NonZeroU32::new_unchecked(10) })
    }
}

impl Radix {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.0.get()
    }
}

impl TryConvertMut<Option<Value>, Option<Radix>> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, value: Option<Value>) -> Result<Option<Radix>, Self::Error> {
        if let Some(value) = value {
            let num = value.implicitly_convert_to_int(self)?;
            if let Ok(radix) = u32::try_from(num) {
                if let Some(radix) = NonZeroU32::new(radix) {
                    if (2..=36).contains(&radix.get()) {
                        return Ok(Some(Radix(radix)));
                    }
                }
                let mut message = String::from("invalid radix ");
                string::format_int_into(&mut message, radix)?;
                Err(ArgumentError::from(message).into())
            } else {
                Err(ArgumentError::from("invalid radix").into())
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct IntegerString<'a>(&'a str);

impl<'a> IntegerString<'a> {
    #[inline]
    #[must_use]
    pub fn new(string: &'a str) -> Self {
        Self(string)
    }

    #[must_use]
    pub fn from_slice(arg: &'a [u8]) -> Option<Self> {
        if arg.find_byte(b'\0').is_some() {
            return None;
        }
        if let Ok(arg) = str::from_utf8(arg) {
            Some(Self::new(arg))
        } else {
            None
        }
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

impl<'a> TryConvertMut<&'a Value, IntegerString<'a>> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, value: &'a Value) -> Result<IntegerString<'a>, Self::Error> {
        let arg = value.implicitly_convert_to_string(self).map_err(|_| {
            let mut message = String::from("can't convert ");
            message.push_str(value.pretty_name(self));
            message.push_str(" into Integer");
            TypeError::from(message)
        })?;
        if let Some(converted) = IntegerString::from_slice(arg) {
            Ok(converted)
        } else {
            let mut message = String::from(r#"invalid value for Integer(): ""#);
            string::format_unicode_debug_into(&mut message, arg)?;
            message.push('"');
            Err(ArgumentError::from(message).into())
        }
    }
}

impl<'a> Into<&'a [u8]> for IntegerString<'a> {
    #[inline]
    fn into(self) -> &'a [u8] {
        self.as_bytes()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Sign {
    Pos,
    Neg,
}

impl Default for Sign {
    #[inline]
    fn default() -> Self {
        Self::Pos
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

    fn set_sign(self, sign: Sign) -> Result<Self, Exception> {
        match self {
            Self::Sign(arg, _) | Self::Accumulate(arg, _, _) => {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                string::format_unicode_debug_into(&mut message, arg.into())?;
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
                Self::Accumulate(arg, Sign::default(), digits)
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

    fn parse(self) -> Result<(String, Option<Radix>), Exception> {
        match self {
            Self::Accumulate(arg, sign, mut digits) => {
                let (mut src, radix) = match digits.as_bytes() {
                    [b'0', b'b', ..] | [b'0', b'B', ..] => {
                        digits.drain(..2);
                        (digits, Some(Radix(unsafe { NonZeroU32::new_unchecked(2) })))
                    }
                    [b'0', b'o', ..] | [b'0', b'O', ..] => {
                        digits.drain(..2);
                        (digits, Some(Radix(unsafe { NonZeroU32::new_unchecked(8) })))
                    }
                    [b'0', b'd', ..] | [b'0', b'D', ..] => {
                        digits.drain(..2);
                        (
                            digits,
                            Some(Radix(unsafe { NonZeroU32::new_unchecked(10) })),
                        )
                    }
                    [b'0', b'x', ..] | [b'0', b'X', ..] => {
                        digits.drain(..2);
                        (
                            digits,
                            Some(Radix(unsafe { NonZeroU32::new_unchecked(16) })),
                        )
                    }
                    [x, y, ..] => {
                        let first = char::from(*x);
                        let next = char::from(*y);
                        if !next.is_numeric() && !next.is_alphabetic() {
                            let mut message = String::from(r#"invalid value for Integer(): ""#);
                            string::format_unicode_debug_into(&mut message, arg.into())?;
                            message.push('"');
                            return Err(ArgumentError::from(message).into());
                        } else if '0' == first {
                            digits.drain(..1);
                            (digits, Some(Radix(unsafe { NonZeroU32::new_unchecked(8) })))
                        } else {
                            (digits, None)
                        }
                    }
                    _ => (digits, None),
                };
                if let Sign::Neg = sign {
                    src.insert(0, '-');
                }
                Ok((src, radix))
            }
            Self::Initial(arg) | Self::Sign(arg, _) => {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                string::format_unicode_debug_into(&mut message, arg.into())?;
                message.push('"');
                Err(ArgumentError::from(message).into())
            }
        }
    }
}

pub fn method(arg: IntegerString<'_>, radix: Option<Radix>) -> Result<Int, Exception> {
    let mut state = ParseState::new(arg);
    let mut chars = arg
        .inner()
        .chars()
        .skip_while(|c| c.is_whitespace())
        .peekable();
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
                string::format_unicode_debug_into(&mut message, arg.into())?;
                message.push('"');
                return Err(ArgumentError::from(message).into());
            } else {
                prev = Some(current);
                continue;
            }
        }

        state = match current {
            '+' => state.set_sign(Sign::Pos)?,
            '-' => state.set_sign(Sign::Neg)?,
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
        string::format_unicode_debug_into(&mut message, arg.into())?;
        message.push('"');
        Err(ArgumentError::from(message).into())
    }
}
