use std::fmt::Write as _;
use std::iter::Iterator;
use std::num::NonZeroU32;
use std::str;

use bstr::ByteSlice;

use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radix(NonZeroU32);

impl Default for Radix {
    fn default() -> Self {
        // SAFETY: Constant `10` is non-zero and between 2 and 36.
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
                        let mut message = String::new();
                        write!(&mut message, "invalid radix {}", num).map_err(WriteError::from)?;
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
                    let mut message = String::new();
                    write!(&mut message, "invalid radix {}", radix).map_err(WriteError::from)?;
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
pub struct IntegerString<'a>(&'a [u8]);

impl<'a> TryFrom<&'a [u8]> for IntegerString<'a> {
    type Error = Error;

    fn try_from(to_parse: &'a [u8]) -> Result<Self, Self::Error> {
        if !to_parse.is_ascii() {
            return Err(int_to_argument_error(to_parse));
        }
        if to_parse.find_byte(b'\0').is_some() {
            return Err(int_to_argument_error(to_parse));
        }
        Ok(Self(to_parse))
    }
}

impl<'a> TryFrom<&'a str> for IntegerString<'a> {
    type Error = Error;

    fn try_from(to_parse: &'a str) -> Result<Self, Self::Error> {
        to_parse.as_bytes().try_into()
    }
}

fn int_to_argument_error(arg: &[u8]) -> Error {
    let mut message = String::from(r#"invalid value for Integer(): ""#);
    if let Err(err) = format_unicode_debug_into(&mut message, arg) {
        return err.into();
    }
    message.push('"');
    ArgumentError::from(message).into()
}

impl<'a> IntegerString<'a> {
    /// Constructs a new, empty `IntegerString`.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(b"")
    }

    #[must_use]
    pub fn from_slice(arg: &'a [u8]) -> Option<Self> {
        arg.try_into().ok()
    }

    #[inline]
    #[must_use]
    pub fn as_bytes(self) -> &'a [u8] {
        self.0
    }

    #[must_use]
    pub fn to_error(self) -> Error {
        int_to_argument_error(self.0)
    }
}

impl<'a> TryConvertMut<&'a mut Value, IntegerString<'a>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &'a mut Value) -> Result<IntegerString<'a>, Self::Error> {
        let mut message = String::from("can't convert ");
        message.push_str(self.inspect_type_name_for_value(*value));
        message.push_str(" into Integer");

        // SAFETY: There is no use of an `Artichoke` in this module, which means
        // a garbage collection of `value` cannot be triggered.
        if let Ok(arg) = unsafe { implicitly_convert_to_string(self, value) } {
            if let Some(converted) = IntegerString::from_slice(arg) {
                Ok(converted)
            } else {
                Err(int_to_argument_error(arg))
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
    Accumulate(IntegerString<'a>, String),
}

impl<'a> ParseState<'a> {
    #[inline]
    fn new(arg: IntegerString<'a>) -> Self {
        Self::Initial(arg)
    }

    fn set_sign(self, sign: Sign) -> Result<Self, Error> {
        match self {
            Self::Sign(arg, _) | Self::Accumulate(arg, _) => Err(arg.to_error()),
            Self::Initial(arg) => Ok(ParseState::Sign(arg, sign)),
        }
    }

    fn collect_digit(self, digit: u8) -> Self {
        match self {
            Self::Initial(arg) => {
                let mut digits = String::new();
                digits.push(char::from(digit));
                Self::Accumulate(arg, digits)
            }
            Self::Sign(arg, sign) => {
                let mut digits = String::new();
                if let Sign::Negative = sign {
                    digits.push('-');
                }
                digits.push(char::from(digit));
                Self::Accumulate(arg, digits)
            }
            Self::Accumulate(arg, mut digits) => {
                digits.push(char::from(digit));
                Self::Accumulate(arg, digits)
            }
        }
    }

    fn parse(self) -> Result<String, Error> {
        match self {
            Self::Accumulate(_, digits) => Ok(digits),
            Self::Initial(arg) | Self::Sign(arg, _) => {
                let mut message = String::from(r#"invalid value for Integer(): ""#);
                format_unicode_debug_into(&mut message, arg.into())?;
                message.push('"');
                Err(ArgumentError::from(message).into())
            }
        }
    }
}

const fn radix_table() -> [u32; 256] {
    let mut table = [255; 256];
    let mut idx = 0_usize;
    loop {
        if idx >= table.len() {
            return table;
        }
        let byte = idx as u8;
        if byte >= b'0' && byte <= b'9' {
            table[idx] = (byte - b'0' + 1) as u32;
        } else if byte >= b'A' && byte <= b'Z' {
            table[idx] = (byte - b'A' + 11) as u32;
        } else if byte >= b'a' && byte <= b'z' {
            table[idx] = (byte - b'a' + 11) as u32;
        }
        idx += 1;
    }
}

pub fn method(arg: IntegerString<'_>, radix: Option<Radix>) -> Result<i64, Error> {
    const RADIX_TABLE: [u32; 256] = radix_table();

    let mut state = ParseState::new(arg);
    let mut chars = arg
        .as_bytes()
        .iter()
        .copied()
        .skip_while(|b| b.is_ascii_whitespace())
        .peekable();

    match chars.peek() {
        Some(b'+') => {
            state = state.set_sign(Sign::Positive)?;
            chars.next();
        }
        Some(b'-') => {
            state = state.set_sign(Sign::Negative)?;
            chars.next();
        }
        Some(_) => {}
        None => return Err(arg.to_error()),
    }
    let radix = match chars.peek() {
        // https://github.com/ruby/ruby/blob/v3_1_2/bignum.c#L4094-L4115
        Some(b'0') => {
            chars.next();
            match chars.peek() {
                Some(b'b' | b'B') if matches!(radix, None) || matches!(radix, Some(radix) if radix.as_u32() == 2) => {
                    chars.next();
                    2
                }
                Some(b'o' | b'O') if matches!(radix, None) || matches!(radix, Some(radix) if radix.as_u32() == 8) => {
                    chars.next();
                    8
                }
                Some(b'd' | b'D') if matches!(radix, None) || matches!(radix, Some(radix) if radix.as_u32() == 10) => {
                    chars.next();
                    10
                }
                Some(b'x' | b'X') if matches!(radix, None) || matches!(radix, Some(radix) if radix.as_u32() == 16) => {
                    chars.next();
                    16
                }
                Some(b'b' | b'B' | b'o' | b'O' | b'd' | b'D' | b'x' | b'X') => return Err(arg.to_error()),
                Some(_) | None => 8,
            }
        }
        Some(_) => radix.map_or(10, Radix::as_u32),
        None => return Err(arg.to_error()),
    };
    // Squeeze leading zeros.
    loop {
        if chars.next_if_eq(&b'0').is_some() {
            if chars.next_if_eq(&b'_').is_some() {
                match chars.peek() {
                    None | Some(b'_') => return Err(arg.to_error()),
                    Some(_) => {}
                }
            }
        } else if let Some(b'_') = chars.peek() {
            return Err(arg.to_error());
        } else {
            break;
        }
    }

    loop {
        match chars.next() {
            Some(b'_') => match chars.peek() {
                None | Some(b'_') => return Err(arg.to_error()),
                Some(_) => {}
            },
            Some(b) if RADIX_TABLE[usize::from(b)] <= radix => {
                state = state.collect_digit(b);
            }
            Some(_) => return Err(arg.to_error()),
            None => break,
        }
    }

    let s = state.parse()?;

    if let Ok(int) = i64::from_str_radix(&*s, radix) {
        Ok(int)
    } else {
        Err(arg.to_error())
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::{method as integer, IntegerString, Radix};
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
    fn squeeze_leading_zeros() {
        let result = integer("0x0000000000000011".try_into().unwrap(), Radix::new(16));
        assert_eq!(result.unwrap(), 17);
        let result = integer("0x00_00000000000011".try_into().unwrap(), Radix::new(16));
        assert_eq!(result.unwrap(), 17);
        let result = integer("0x0_0_0_11".try_into().unwrap(), Radix::new(16));
        assert_eq!(result.unwrap(), 17);
        let result = integer("0x00000_15".try_into().unwrap(), Radix::new(16));
        assert_eq!(result.unwrap(), 21);

        let result = integer("0x___11".try_into().unwrap(), Radix::new(16));
        result.unwrap_err();
        let result = integer("0x0___11".try_into().unwrap(), Radix::new(16));
        result.unwrap_err();
        let result = integer("0x_0__11".try_into().unwrap(), Radix::new(16));
        result.unwrap_err();
        let result = integer("0x_00__11".try_into().unwrap(), Radix::new(16));
        result.unwrap_err();
    }

    #[test]
    fn no_digits_with_base_prefix() {
        let result = integer("0x".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0x""#.as_bytes().as_bstr()
        );

        let result = integer("0b".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0b""#.as_bytes().as_bstr()
        );

        let result = integer("0o".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0o""#.as_bytes().as_bstr()
        );

        let result = integer("o".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "o""#.as_bytes().as_bstr()
        );

        let result = integer("0X".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0X""#.as_bytes().as_bstr()
        );

        let result = integer("0B".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0B""#.as_bytes().as_bstr()
        );

        let result = integer("0O".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0O""#.as_bytes().as_bstr()
        );

        let result = integer("O".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message(),
            r#"invalid value for Integer(): "O""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn no_digits_with_invalid_base_prefix() {
        let result = integer("0z".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0z""#.as_bytes().as_bstr()
        );

        let result = integer("0z".try_into().unwrap(), Radix::new(12));
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0z""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn leading_underscore_is_err() {
        let result = integer("0x_0000001234567".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0x_0000001234567""#.as_bytes().as_bstr()
        );

        let result = integer("0_x0000001234567".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0_x0000001234567""#.as_bytes().as_bstr()
        );

        let result = integer("___0x0000001234567".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "___0x0000001234567""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn double_underscore_is_err() {
        let result = integer("0x111__11".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0x111__11""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn trailing_underscore_is_err() {
        let result = integer("0x111_11_".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0x111_11_""#.as_bytes().as_bstr()
        );
        let result = integer("0x00000_".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "0x00000_""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn all_spaces_is_err() {
        let result = integer("    ".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "    ""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn empty_is_err() {
        let result = integer("".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): """#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn nul_byte_is_err() {
        IntegerString::try_from("\0").unwrap_err();
        IntegerString::try_from("123\0").unwrap_err();
        IntegerString::try_from("123\0456").unwrap_err();
    }

    #[test]
    fn more_than_one_sign_is_err() {
        let result = integer("++12".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "++12""#.as_bytes().as_bstr()
        );

        let result = integer("+-12".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "+-12""#.as_bytes().as_bstr()
        );

        let result = integer("-+12".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "-+12""#.as_bytes().as_bstr()
        );

        let result = integer("--12".try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "--12""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn emoji_is_err() {
        IntegerString::try_from("🕐").unwrap_err();
    }

    #[test]
    #[should_panic]
    fn invalid_utf8_is_err() {
        let result = integer(b"\xFF"[..].try_into().unwrap(), None);
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            r#"invalid value for Integer(): "🕐""#.as_bytes().as_bstr()
        );
    }

    #[test]
    fn nil_radix_parses_to_none() {
        let mut interp = interpreter();
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(None);
        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn zero_radix_parses_to_none() {
        let mut interp = interpreter();
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
        let mut interp = interpreter();
        let expected = Radix::new(10).unwrap();
        let radix = interp.convert(-1);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        let result = result.unwrap();
        assert_eq!(result, Some(expected), "-1 radix should parse to base 10");
    }

    #[test]
    fn one_radix_has_parse_failure() {
        let mut interp = interpreter();
        let radix = interp.convert(1);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 1".as_bstr()
        );
    }

    #[test]
    fn invalid_radix_has_parse_failure() {
        let mut interp = interpreter();
        let radix = interp.convert(12000);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 12000".as_bstr()
        );
    }

    #[test]
    fn invalid_negative_radix_has_parse_failure() {
        let mut interp = interpreter();
        let radix = interp.convert(-12000);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
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
        let mut interp = interpreter();
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
        let mut interp = interpreter();
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
        let mut interp = interpreter();
        let radix = interp.convert(i64::MAX);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        result.unwrap_err();

        let radix = interp.convert(i64::MIN);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        result.unwrap_err();
    }
}
