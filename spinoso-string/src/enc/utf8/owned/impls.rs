use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt;
use core::ops::{Deref, DerefMut};

use super::{Utf8Str, Utf8String};
use crate::Buf;

impl Default for Utf8String {
    #[inline]
    fn default() -> Self {
        Utf8String::empty()
    }
}

impl fmt::Debug for Utf8String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use bstr::ByteSlice;

        f.debug_struct("Utf8String")
            .field("buf", &self.as_bytes().as_bstr())
            .finish()
    }
}

impl Clone for Utf8String {
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.clear();
        let bytes = source.as_bytes();
        self.extend_from_slice(bytes);
    }
}

impl Deref for Utf8String {
    type Target = Utf8Str;

    fn deref(&self) -> &Self::Target {
        self.as_utf8_str()
    }
}

impl DerefMut for Utf8String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_utf8_str()
    }
}

impl Borrow<[u8]> for Utf8String {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<Utf8Str> for Utf8String {
    fn borrow(&self) -> &Utf8Str {
        self.as_utf8_str()
    }
}

impl ToOwned for Utf8Str {
    type Owned = Utf8String;

    fn to_owned(&self) -> Self::Owned {
        Self::Owned::from(self)
    }

    fn clone_into(&self, target: &mut Self::Owned) {
        target.clear();
        target.extend(self.as_bytes());
    }
}

impl AsRef<[u8]> for Utf8String {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for Utf8String {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl AsRef<Utf8Str> for Utf8String {
    #[inline]
    fn as_ref(&self) -> &Utf8Str {
        self.as_utf8_str()
    }
}

impl AsMut<Utf8Str> for Utf8String {
    #[inline]
    fn as_mut(&mut self) -> &mut Utf8Str {
        self.as_mut_utf8_str()
    }
}

impl Extend<u8> for Utf8String {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl<'a> Extend<&'a u8> for Utf8String {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl From<Buf> for Utf8String {
    #[inline]
    fn from(content: Buf) -> Self {
        Self::new(content)
    }
}

impl From<Vec<u8>> for Utf8String {
    #[inline]
    fn from(content: Vec<u8>) -> Self {
        Self::new(content.into())
    }
}

impl<const N: usize> From<[u8; N]> for Utf8String {
    #[inline]
    fn from(content: [u8; N]) -> Self {
        Self::new(content.into())
    }
}

impl<const N: usize> From<&[u8; N]> for Utf8String {
    #[inline]
    fn from(content: &[u8; N]) -> Self {
        Self::new(content.into())
    }
}

impl From<&[u8]> for Utf8String {
    #[inline]
    fn from(content: &[u8]) -> Self {
        Self::new(content.into())
    }
}

impl From<&mut [u8]> for Utf8String {
    #[inline]
    fn from(content: &mut [u8]) -> Self {
        Self::new(content.into())
    }
}

impl<'a> From<Cow<'a, [u8]>> for Utf8String {
    #[inline]
    fn from(content: Cow<'a, [u8]>) -> Self {
        Self::new(content.into())
    }
}

impl From<String> for Utf8String {
    #[inline]
    fn from(content: String) -> Self {
        Self::new(content.into())
    }
}

impl From<&str> for Utf8String {
    #[inline]
    fn from(content: &str) -> Self {
        Self::new(content.into())
    }
}

impl<'a> From<Cow<'a, str>> for Utf8String {
    #[inline]
    fn from(content: Cow<'a, str>) -> Self {
        Self::new(content.into())
    }
}

impl From<&Utf8Str> for Utf8String {
    #[inline]
    fn from(content: &Utf8Str) -> Self {
        Self::new(content.as_bytes().into())
    }
}

impl<'a> From<Cow<'a, Utf8Str>> for Utf8String {
    #[inline]
    fn from(content: Cow<'a, Utf8Str>) -> Self {
        content.into_owned()
    }
}

impl From<Utf8String> for Buf {
    #[inline]
    fn from(content: Utf8String) -> Self {
        content.into_buf()
    }
}

impl From<Utf8String> for Vec<u8> {
    #[inline]
    fn from(content: Utf8String) -> Self {
        content.into_buf().into_inner()
    }
}

impl<const N: usize> TryFrom<Utf8String> for [u8; N] {
    type Error = Utf8String;

    #[inline]
    fn try_from(content: Utf8String) -> Result<Self, Self::Error> {
        match content.into_buf().into_inner().try_into() {
            Ok(array) => Ok(array),
            Err(vec) => Err(vec.into()),
        }
    }
}

impl<'a> From<Utf8String> for Cow<'a, [u8]> {
    #[inline]
    fn from(content: Utf8String) -> Self {
        let buf = content.into();
        Cow::Owned(buf)
    }
}

impl TryFrom<Utf8String> for String {
    type Error = Utf8String;

    #[inline]
    fn try_from(s: Utf8String) -> Result<Self, Self::Error> {
        if s.is_valid_encoding() {
            let vec = s.into();
            // SAFETY: `is_valid_encoding` only returns true if the byte
            // content of the `Utf8String` is valid UTF-8.
            let s = unsafe { String::from_utf8_unchecked(vec) };
            Ok(s)
        } else {
            Err(s)
        }
    }
}

impl<'a> TryFrom<Utf8String> for Cow<'a, str> {
    type Error = Utf8String;

    #[inline]
    fn try_from(content: Utf8String) -> Result<Self, Self::Error> {
        let s = content.try_into()?;
        Ok(Cow::Owned(s))
    }
}

impl<'a> From<Utf8String> for Cow<'a, Utf8Str> {
    #[inline]
    fn from(content: Utf8String) -> Self {
        Cow::Owned(content)
    }
}
