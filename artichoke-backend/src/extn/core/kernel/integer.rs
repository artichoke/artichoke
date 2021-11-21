use std::error;
use std::fmt;
use std::iter::Iterator;
use std::num::NonZeroU32;
use std::str::{self, FromStr};

use bstr::ByteSlice;

use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
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
    /// `radix` must be non-zero and between 2 and 36 inclusive; otherwise
    /// [`None`] is returned.
    #[must_use]
    pub fn new(radix: u32) -> Option<Self> {
        let radix = NonZeroU32::new(radix)?;
        if (2..=36).contains(&radix.get()) {
            Some(Self(radix))
        } else {
            None
        }
    }

    /// Construct a new `Radix` without checking the value.
    ///
    /// # Safety
    ///
    /// The given radix must not be zero. The given radix must be between 2 and
    /// 36 inclusive.
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
            let num = implicitly_convert_to_int(self, value)?;
            let radix = if let Ok(radix) = u32::try_from(num) {
                radix
            } else {
                let num = num
                    .checked_neg()
                    .ok_or_else(|| ArgumentError::with_message("invalid radix"))?;
                match u32::try_from(num) {
                    // See https://github.com/ruby/ruby/blob/v2_6_3/bignum.c#L4106-L4110
                    Ok(1) => return Ok(Some(Radix::default())),
                    Ok(radix) => radix,
                    Err(_) => {
                        let mut message = String::from("invalid radix ");
                        itoa::fmt(&mut message, num).map_err(WriteError::from)?;
                        return Err(ArgumentError::from(message).into());
                    }
                }
            };
            match Radix::new(radix) {
                Some(radix) => Ok(Some(radix)),
                // a zero radix means `Integer` should fall back to string parsing
                // of numeric literal prefixes.
                None if radix == 0 => Ok(None),
                None => {
                    let mut message = String::from("invalid radix ");
                    itoa::fmt(&mut message, radix).map_err(WriteError::from)?;
                    Err(ArgumentError::from(message).into())
                }
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

        // Safety:
        //
        // There is no use of an `Artichoke` in this module, which means a
        // garbage collection of `value` cannot be triggered.
        if let Ok(arg) = unsafe { implicitly_convert_to_string(self, value) } {
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
            [b'0', b'b' | b'B', ..] => {
                digits.drain(..2);
                Radix::new(2)
            }
            [b'0', b'o' | b'O', ..] => {
                digits.drain(..2);
                Radix::new(8)
            }
            [b'0', b'd' | b'D', ..] => {
                digits.drain(..2);
                Radix::new(10)
            }
            [b'0', b'x' | b'X', ..] => {
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

pub fn method(arg: IntegerString<'_>, radix: Option<Radix>) -> Result<i64, Error> {
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
            if let Some('+' | '-') = prev {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                format_unicode_debug_into(&mut message, arg.into())?;
                message.push('"');
                return Err(ArgumentError::from(message).into());
            }
            prev = Some(current);
            continue;
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
        (Some(x), Some(y)) if x == y => i64::from_str_radix(src.as_str(), x.as_u32()).ok(),
        (None, None) => i64::from_str(src.as_str()).ok(),
        (Some(x), None) | (None, Some(x)) if (2..=36).contains(&x.as_u32()) => {
            i64::from_str_radix(src.as_str(), x.as_u32()).ok()
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

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::{method as integer, Radix};
    use crate::test::prelude::*;

    #[test]
    fn radix_new_validates_radix_is_nonzero() {
        let radix = Radix::new(0);
        assert!(radix.is_none());
    }

    #[test]
    fn radix_new_parses_valid_radixes() {
        for r in 2..=36 {
            let radix = Radix::new(r);
            assert!(radix.is_some());
        }
    }

    #[test]
    fn radix_new_rejects_too_large_radixes() {
        let radix = Radix::new(12000);
        assert!(radix.is_none());
    }

    #[test]
    fn no_digits_with_base_prefix() {
        let result = integer("0x".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0x""#.as_bytes().as_bstr()
        );

        let result = integer("0b".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0b""#.as_bytes().as_bstr()
        );

        let result = integer("0o".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0o""#.as_bytes().as_bstr()
        );

        let result = integer("o".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "o""#.as_bytes().as_bstr()
        );

        let result = integer("0X".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0X""#.as_bytes().as_bstr()
        );

        let result = integer("0B".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0B""#.as_bytes().as_bstr()
        );

        let result = integer("0O".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0O""#.as_bytes().as_bstr()
        );

        let result = integer("O".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            r#"invalid value for Integer(): "O""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn no_digits_with_invalid_base_prefix() {
        let result = integer("0z".into(), None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0z""#.as_bytes().as_bstr()
        );

        let result = integer("0z".into(), Radix::new(12));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0z""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn nil_radix_parses_to_none() {
        let mut interp = interpreter().unwrap();
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(None);
        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn zero_radix_parses_to_none() {
        let mut interp = interpreter().unwrap();
        let radix = interp.convert(0);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        let result = result.unwrap();
        assert!(
            result.is_none(),
            "0 radix should parse to None and fallback to literal prefix parsing"
        );
    }

    #[test]
    fn negative_one_radix_parses_to_none() {
        let mut interp = interpreter().unwrap();
        let expected = Radix::new(10).unwrap();
        let radix = interp.convert(-1);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        let result = result.unwrap();
        assert_eq!(result, Some(expected), "-1 radix should parse to base 10");
    }

    #[test]
    fn one_radix_has_parse_failure() {
        let mut interp = interpreter().unwrap();
        let radix = interp.convert(1);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 1".as_bstr()
        );
    }

    #[test]
    fn invalid_radix_has_parse_failure() {
        let mut interp = interpreter().unwrap();
        let radix = interp.convert(12000);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 12000".as_bstr()
        );
    }

    #[test]
    fn invalid_negative_radix_has_parse_failure() {
        let mut interp = interpreter().unwrap();
        let radix = interp.convert(-12000);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert!(result.is_err());
        // ```ruby
        // irb(main):003:0> Integer("123", -12000)
        // (irb):3:in `Integer': invalid radix 12000 (ArgumentError)
        // from (irb):3:in `<main>'
        // from C:/Ruby30-x64/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
        // from C:/Ruby30-x64/bin/irb.cmd:31:in `load'
        // from C:/Ruby30-x64/bin/irb.cmd:31:in `<main>'
        // ```
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 12000".as_bstr()
        );
    }

    #[test]
    fn positive_radix_in_valid_range_is_parsed() {
        let mut interp = interpreter().unwrap();
        for r in 2_i32..=36_i32 {
            let radix = interp.convert(r);
            let expected = Radix::new(r.try_into().unwrap()).unwrap();
            let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
            let result = result.unwrap();
            assert_eq!(result, Some(expected), "expected {} to parse to Radix({})", r, r);
        }
    }

    #[test]
    fn negative_radix_in_valid_range_is_parsed() {
        let mut interp = interpreter().unwrap();
        for r in 2_i32..=36_i32 {
            let radix = interp.convert(-r);
            let expected = Radix::new(r.try_into().unwrap()).unwrap();
            let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
            let result = result.unwrap();
            assert_eq!(result, Some(expected), "expected -{} to parse to Radix({})", r, r);
        }
    }

    #[test]
    fn int_max_min_do_not_panic() {
        let mut interp = interpreter().unwrap();
        let radix = interp.convert(i64::MAX);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert!(result.is_err());

        let radix = interp.convert(i64::MIN);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert!(result.is_err());
    }
}
