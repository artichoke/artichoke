use bstr::ByteSlice;
use std::convert::TryFrom;
use std::iter::Iterator;
use std::num::NonZeroU32;
use std::str::{self, FromStr};

use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radix(NonZeroU32);

impl Radix {
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
                    if radix.get() >= 2 && radix.get() <= 36 {
                        return Ok(Some(Radix(radix)));
                    }
                }
                let mut message = String::from("invalid radix ");
                string::format_int_into(&mut message, radix)?;
                Err(Exception::from(ArgumentError::new(self, message)))
            } else {
                Err(Exception::from(ArgumentError::new(self, "invalid radix")))
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
            TypeError::new(self, message)
        })?;
        if let Some(converted) = IntegerString::from_slice(arg) {
            Ok(converted)
        } else {
            let mut message = String::from(r#"invalid value for Integer(): ""#);
            string::format_unicode_debug_into(&mut message, arg)?;
            message.push('"');
            Err(ArgumentError::new(self, message).into())
        }
    }
}

impl<'a> Into<&'a [u8]> for IntegerString<'a> {
    #[inline]
    fn into(self) -> &'a [u8] {
        self.as_bytes()
    }
}

pub fn method<'a>(
    interp: &mut Artichoke,
    arg: IntegerString<'a>,
    radix: Option<Radix>,
) -> Result<Int, Exception> {
    #[derive(Debug, Clone, Copy)]
    enum Sign {
        Pos,
        Neg,
    }
    #[derive(Debug, Clone)]
    enum ParseState {
        Initial,
        Sign(Sign),
        Accumulate(Sign, String),
    }

    let mut state = ParseState::Initial;
    let mut chars = arg
        .inner()
        .chars()
        .skip_while(|c| c.is_whitespace())
        .peekable();
    let mut prev = None::<char>;

    while let Some(current) = chars.next() {
        // Ignore an embedded underscore (`_`).
        if current == '_' {
            let valid_prev = prev
                .map(|prev| prev.is_numeric() || prev.is_alphabetic())
                .unwrap_or_default();
            let next = chars.peek();
            let valid_next = next
                .map(|next| next.is_numeric() || next.is_alphabetic())
                .unwrap_or_default();
            if valid_prev && valid_next {
                prev = Some(current);
                continue;
            }
        }
        if current.is_whitespace() {
            if let Some('+') | Some('-') = prev {
                return Err(Exception::from(invalid_value_err(interp, arg)?));
            } else {
                prev = Some(current);

                continue;
            }
        }

        state = match current {
            '+' => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Exception::from(invalid_value_err(interp, arg)?));
                }
                ParseState::Sign(Sign::Pos)
            }
            '-' => {
                if let ParseState::Sign(_) | ParseState::Accumulate(_, _) = state {
                    return Err(Exception::from(invalid_value_err(interp, arg)?));
                }
                ParseState::Sign(Sign::Neg)
            }
            digit => match state {
                ParseState::Initial => {
                    let mut digits = String::new();
                    digits.push(digit);
                    ParseState::Accumulate(Sign::Pos, digits)
                }
                ParseState::Sign(sign) => {
                    let mut digits = String::new();
                    digits.push(digit);
                    ParseState::Accumulate(sign, digits)
                }
                ParseState::Accumulate(sign, mut digits) => {
                    digits.push(digit);
                    ParseState::Accumulate(sign, digits)
                }
            },
        };
        prev = Some(current);
    }

    let (candidate, parsed_radix) = if let ParseState::Accumulate(sign, mut digits) = state {
        let parsed_radix = match digits.chars().take(2).collect::<String>().as_str() {
            "0b" | "0B" => {
                digits.drain(..2);
                Some(Radix(unsafe { NonZeroU32::new_unchecked(2) }))
            }
            "0o" | "0O" => {
                digits.drain(..2);
                Some(Radix(unsafe { NonZeroU32::new_unchecked(8) }))
            }
            "0d" | "0D" => {
                digits.drain(..2);
                Some(Radix(unsafe { NonZeroU32::new_unchecked(10) }))
            }
            "0x" | "0X" => {
                digits.drain(..2);
                Some(Radix(unsafe { NonZeroU32::new_unchecked(16) }))
            }
            prefix => {
                let mut chars = prefix.chars();
                let first = chars.next();
                let next = chars.next();
                if let Some(next) = next {
                    if !next.is_numeric() && !next.is_alphabetic() {
                        return Err(Exception::from(invalid_value_err(interp, arg)?));
                    } else if let Some('0') = first {
                        digits.drain(..1);
                        Some(Radix(unsafe { NonZeroU32::new_unchecked(8) }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };

        if let Sign::Neg = sign {
            digits.insert(0, '-');
        }
        (digits, parsed_radix)
    } else {
        return Err(Exception::from(invalid_value_err(interp, arg)?));
    };

    match (radix, parsed_radix) {
        (x, y) if x == y => {
            let integer = if let Some(radix) = x {
                Int::from_str_radix(candidate.as_str(), radix.as_u32())
            } else {
                Int::from_str(candidate.as_str())
            };
            if let Ok(integer) = integer {
                Ok(integer)
            } else {
                Err(Exception::from(invalid_value_err(interp, arg.as_bytes())?))
            }
        }
        (Some(radix), None) | (None, Some(radix)) => {
            if radix.as_u32() >= 2 && radix.as_u32() <= 36 {
                if let Ok(integer) = Int::from_str_radix(candidate.as_str(), radix.as_u32()) {
                    Ok(integer)
                } else {
                    Err(Exception::from(invalid_value_err(interp, arg)?))
                }
            } else {
                Err(Exception::from(ArgumentError::new(interp, "invalid radix")))
            }
        }
        _ => Err(Exception::from(ArgumentError::new(interp, "invalid radix"))),
    }
}

#[inline]
fn invalid_value_err<'a, T: Into<&'a [u8]>>(
    interp: &Artichoke,
    arg: T,
) -> Result<ArgumentError, Exception> {
    let mut message = String::from(r#"invalid value for Integer(): ""#);
    string::format_unicode_debug_into(&mut message, arg.into())?;
    message.push('"');
    Ok(ArgumentError::new(interp, message))
}
