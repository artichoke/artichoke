use core::iter::FusedIterator;

use super::{
    ascii::{self, AsciiString},
    binary::{self, BinaryString},
    utf8::{self, Utf8String},
};
use crate::CodepointsError;

#[derive(Default, Debug, Clone)]
#[must_use = "this `Codepoint` is an `Iterator`, which should be consumed if constructed"]
pub struct Codepoints<'a>(State<'a>);

impl<'a> From<&'a AsciiString> for Codepoints<'a> {
    fn from(value: &'a AsciiString) -> Self {
        Self(State::Ascii(ascii::Codepoints::new(value)))
    }
}

impl<'a> From<&'a BinaryString> for Codepoints<'a> {
    fn from(value: &'a BinaryString) -> Self {
        Self(State::Binary(binary::Codepoints::new(value)))
    }
}
impl<'a> TryFrom<&'a Utf8String> for Codepoints<'a> {
    type Error = CodepointsError;

    fn try_from(s: &'a Utf8String) -> Result<Self, Self::Error> {
        utf8::Codepoints::try_from(s.as_utf8_str()).map(|v| Self(State::Utf8(v)))
    }
}

#[derive(Debug, Clone)]
enum State<'a> {
    Ascii(ascii::Codepoints<'a>),
    Binary(binary::Codepoints<'a>),
    Utf8(utf8::Codepoints<'a>),
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self::Ascii(ascii::Codepoints::default())
    }
}

impl<'a> Iterator for Codepoints<'a> {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self(State::Ascii(iter)) => iter.next(),
            Self(State::Binary(iter)) => iter.next(),
            Self(State::Utf8(iter)) => iter.next().map(u32::from),
        }
    }
}

impl<'a> FusedIterator for Codepoints<'a> {}
