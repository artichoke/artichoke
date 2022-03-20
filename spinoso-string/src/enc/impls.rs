use alloc::vec::Vec;
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut};

use super::EncodedString;

impl Extend<u8> for EncodedString {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        match self {
            EncodedString::Ascii(inner) => inner.extend(iter),
            EncodedString::Binary(inner) => inner.extend(iter),
            EncodedString::Utf8(inner) => inner.extend(iter),
        }
    }
}

impl<'a> Extend<&'a u8> for EncodedString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        match self {
            EncodedString::Ascii(inner) => inner.extend(iter),
            EncodedString::Binary(inner) => inner.extend(iter),
            EncodedString::Utf8(inner) => inner.extend(iter),
        }
    }
}

impl<'a> Extend<&'a mut u8> for EncodedString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a mut u8>>(&mut self, iter: I) {
        match self {
            EncodedString::Ascii(inner) => inner.extend(iter),
            EncodedString::Binary(inner) => inner.extend(iter),
            EncodedString::Utf8(inner) => inner.extend(iter),
        }
    }
}

impl AsRef<[u8]> for EncodedString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(inner) => inner.as_ref(),
            EncodedString::Binary(inner) => inner.as_ref(),
            EncodedString::Utf8(inner) => inner.as_ref(),
        }
    }
}

impl AsMut<[u8]> for EncodedString {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(inner) => inner.as_mut(),
            EncodedString::Binary(inner) => inner.as_mut(),
            EncodedString::Utf8(inner) => inner.as_mut(),
        }
    }
}

impl AsRef<Vec<u8>> for EncodedString {
    #[inline]
    fn as_ref(&self) -> &Vec<u8> {
        match self {
            EncodedString::Ascii(inner) => inner.as_ref(),
            EncodedString::Binary(inner) => inner.as_ref(),
            EncodedString::Utf8(inner) => inner.as_ref(),
        }
    }
}

impl AsMut<Vec<u8>> for EncodedString {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<u8> {
        match self {
            EncodedString::Ascii(inner) => inner.as_mut(),
            EncodedString::Binary(inner) => inner.as_mut(),
            EncodedString::Utf8(inner) => inner.as_mut(),
        }
    }
}

impl Deref for EncodedString {
    type Target = [u8];

    #[allow(clippy::explicit_deref_methods)]
    #[inline]
    fn deref(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(inner) => inner.deref(),
            EncodedString::Binary(inner) => inner.deref(),
            EncodedString::Utf8(inner) => inner.deref(),
        }
    }
}

impl DerefMut for EncodedString {
    #[allow(clippy::explicit_deref_methods)]
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(inner) => inner.deref_mut(),
            EncodedString::Binary(inner) => inner.deref_mut(),
            EncodedString::Utf8(inner) => inner.deref_mut(),
        }
    }
}

impl Borrow<[u8]> for EncodedString {
    #[inline]
    fn borrow(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(inner) => inner.borrow(),
            EncodedString::Binary(inner) => inner.borrow(),
            EncodedString::Utf8(inner) => inner.borrow(),
        }
    }
}
