//use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};

use super::EncodedString;

impl Extend<u8> for EncodedString {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        match self {
            EncodedString::Ascii(n) => n.extend(iter),
            EncodedString::Binary(n) => n.extend(iter),
            EncodedString::Utf8(n) => n.extend(iter),
        }
    }
}

impl<'a> Extend<&'a u8> for EncodedString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        match self {
            EncodedString::Ascii(n) => n.extend(iter),
            EncodedString::Binary(n) => n.extend(iter),
            EncodedString::Utf8(n) => n.extend(iter),
        }
    }
}

impl<'a> Extend<&'a mut u8> for EncodedString {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a mut u8>>(&mut self, iter: I) {
        match self {
            EncodedString::Ascii(n) => n.extend(iter),
            EncodedString::Binary(n) => n.extend(iter),
            EncodedString::Utf8(n) => n.extend(iter),
        }
    }
}

impl AsRef<[u8]> for EncodedString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(n) => n.as_ref(),
            EncodedString::Binary(n) => n.as_ref(),
            EncodedString::Utf8(n) => n.as_ref(),
        }
    }
}

impl AsMut<[u8]> for EncodedString {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(n) => n.as_mut(),
            EncodedString::Binary(n) => n.as_mut(),
            EncodedString::Utf8(n) => n.as_mut(),
        }
    }
}

impl AsRef<Vec<u8>> for EncodedString {
    #[inline]
    fn as_ref(&self) -> &Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.as_ref(),
            EncodedString::Binary(n) => n.as_ref(),
            EncodedString::Utf8(n) => n.as_ref(),
        }
    }
}

impl AsMut<Vec<u8>> for EncodedString {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.as_mut(),
            EncodedString::Binary(n) => n.as_mut(),
            EncodedString::Utf8(n) => n.as_mut(),
        }
    }
}

impl Deref for EncodedString {
    type Target = [u8];

    #[allow(clippy::explicit_deref_methods)]
    #[inline]
    fn deref(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(n) => n.deref(),
            EncodedString::Binary(n) => n.deref(),
            EncodedString::Utf8(n) => n.deref(),
        }
    }
}

impl DerefMut for EncodedString {
    #[allow(clippy::explicit_deref_methods)]
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(n) => n.deref_mut(),
            EncodedString::Binary(n) => n.deref_mut(),
            EncodedString::Utf8(n) => n.deref_mut(),
        }
    }
}

impl Borrow<[u8]> for EncodedString {
    #[inline]
    fn borrow(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(n) => n.borrow(),
            EncodedString::Binary(n) => n.borrow(),
            EncodedString::Utf8(n) => n.borrow(),
        }
    }
}

impl BorrowMut<[u8]> for EncodedString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(n) => n.borrow_mut(),
            EncodedString::Binary(n) => n.borrow_mut(),
            EncodedString::Utf8(n) => n.borrow_mut(),
        }
    }
}

impl Borrow<Vec<u8>> for EncodedString {
    #[inline]
    fn borrow(&self) -> &Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.borrow(),
            EncodedString::Binary(n) => n.borrow(),
            EncodedString::Utf8(n) => n.borrow(),
        }
    }
}

impl BorrowMut<Vec<u8>> for EncodedString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.borrow_mut(),
            EncodedString::Binary(n) => n.borrow_mut(),
            EncodedString::Utf8(n) => n.borrow_mut(),
        }
    }
}
