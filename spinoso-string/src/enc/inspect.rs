use core::iter::FusedIterator;

use super::{ascii, binary, utf8};

#[derive(Default, Debug, Clone)]
#[must_use = "this `Inspect` is an `Iterator`, which should be consumed if constructed"]
pub struct Inspect<'a>(State<'a>);

impl<'a> From<&'a ascii::AsciiString> for Inspect<'a> {
    #[inline]
    fn from(value: &'a ascii::AsciiString) -> Self {
        Self(State::Ascii(value.into()))
    }
}

impl<'a> From<&'a binary::BinaryString> for Inspect<'a> {
    #[inline]
    fn from(value: &'a binary::BinaryString) -> Self {
        Self(State::Binary(value.into()))
    }
}

impl<'a> From<&'a utf8::Utf8String> for Inspect<'a> {
    #[inline]
    fn from(value: &'a utf8::Utf8String) -> Self {
        Self(State::Utf8(value.into()))
    }
}

#[derive(Debug, Clone)]
enum State<'a> {
    Ascii(ascii::Inspect<'a>),
    Binary(binary::Inspect<'a>),
    Utf8(utf8::Inspect<'a>),
}

impl<'a> Default for State<'a> {
    /// Construct a `State` that will render debug output for the empty slice.
    ///
    /// This constructor produces inspect contents like `""`.
    #[inline]
    fn default() -> Self {
        Self::Ascii(Default::default())
    }
}

impl<'a> Iterator for Inspect<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            State::Ascii(iter) => iter.next(),
            State::Binary(iter) => iter.next(),
            State::Utf8(iter) => iter.next(),
        }
    }
}

impl<'a> FusedIterator for Inspect<'a> {}
