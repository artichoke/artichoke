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

impl Deref for EncodedString {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(inner) => &*inner,
            EncodedString::Binary(inner) => &*inner,
            EncodedString::Utf8(inner) => &*inner,
        }
    }
}

impl DerefMut for EncodedString {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(inner) => &mut *inner,
            EncodedString::Binary(inner) => &mut *inner,
            EncodedString::Utf8(inner) => &mut *inner,
        }
    }
}
