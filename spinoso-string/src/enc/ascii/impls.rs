use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

use super::AsciiString;
use crate::buf::Buf;

impl Extend<u8> for AsciiString {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter());
    }
}

impl<'a> Extend<&'a u8> for AsciiString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().copied());
    }
}

impl<'a> Extend<&'a mut u8> for AsciiString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a mut u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().map(|byte| *byte));
    }
}

impl From<Buf> for AsciiString {
    #[inline]
    fn from(content: Buf) -> Self {
        Self::new(content)
    }
}

impl From<Vec<u8>> for AsciiString {
    #[inline]
    fn from(content: Vec<u8>) -> Self {
        let buf = content.into();
        Self::new(buf)
    }
}

impl<const N: usize> From<[u8; N]> for AsciiString {
    #[inline]
    fn from(content: [u8; N]) -> Self {
        let buf = content.to_vec();
        Self::new(buf.into())
    }
}

impl<const N: usize> From<&[u8; N]> for AsciiString {
    #[inline]
    fn from(content: &[u8; N]) -> Self {
        let buf = content.to_vec();
        Self::new(buf.into())
    }
}

impl<'a> From<&'a [u8]> for AsciiString {
    #[inline]
    fn from(content: &'a [u8]) -> Self {
        let buf = content.to_vec();
        Self::new(buf.into())
    }
}

impl<'a> From<&'a mut [u8]> for AsciiString {
    #[inline]
    fn from(content: &'a mut [u8]) -> Self {
        let buf = content.to_vec();
        Self::new(buf.into())
    }
}

impl<'a> From<Cow<'a, [u8]>> for AsciiString {
    #[inline]
    fn from(content: Cow<'a, [u8]>) -> Self {
        let buf = content.into_owned();
        Self::new(buf.into())
    }
}

impl From<String> for AsciiString {
    #[inline]
    fn from(s: String) -> Self {
        let buf = s.into_bytes();
        Self::new(buf.into())
    }
}

impl From<&str> for AsciiString {
    #[inline]
    fn from(s: &str) -> Self {
        let buf = s.as_bytes().to_vec();
        Self::new(buf.into())
    }
}

impl From<AsciiString> for Buf {
    #[inline]
    fn from(s: AsciiString) -> Self {
        s.into_buf()
    }
}

impl AsRef<[u8]> for AsciiString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl AsMut<[u8]> for AsciiString {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }
}

impl Deref for AsciiString {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.inner
    }
}

impl DerefMut for AsciiString {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}
