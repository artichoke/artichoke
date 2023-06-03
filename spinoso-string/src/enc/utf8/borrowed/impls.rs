use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::array::TryFromSliceError;
use core::fmt;
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;
use core::str;

use super::Utf8Str;

impl<'a> Default for &'a Utf8Str {
    #[inline]
    fn default() -> Self {
        Utf8Str::empty()
    }
}

impl fmt::Debug for Utf8Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use bstr::ByteSlice;

        f.debug_struct("Utf8Str")
            .field("bytes", &self.as_bytes().as_bstr())
            .finish()
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for Utf8Str {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.as_bytes(), index)
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for Utf8Str {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(self.as_bytes_mut(), index)
    }
}

impl AsRef<[u8]> for Utf8Str {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for Utf8Str {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl AsRef<Utf8Str> for Utf8Str {
    #[inline]
    fn as_ref(&self) -> &Utf8Str {
        self
    }
}

impl AsMut<Utf8Str> for Utf8Str {
    #[inline]
    fn as_mut(&mut self) -> &mut Utf8Str {
        self
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for &'a Utf8Str {
    #[inline]
    fn from(content: &'a [u8; N]) -> Self {
        Utf8Str::new(content)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for &'a mut Utf8Str {
    #[inline]
    fn from(content: &'a mut [u8; N]) -> Self {
        Utf8Str::new_mut(content)
    }
}

impl<'a> From<&'a [u8]> for &'a Utf8Str {
    #[inline]
    fn from(content: &'a [u8]) -> Self {
        Utf8Str::new(content)
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut Utf8Str {
    #[inline]
    fn from(content: &'a mut [u8]) -> Self {
        Utf8Str::new_mut(content)
    }
}

impl<'a> From<&'a str> for &'a Utf8Str {
    #[inline]
    fn from(s: &'a str) -> Self {
        Utf8Str::new(s)
    }
}

impl<'a> From<&'a Utf8Str> for &'a [u8] {
    #[inline]
    fn from(content: &'a Utf8Str) -> Self {
        content.as_bytes()
    }
}

impl<'a> From<&'a mut Utf8Str> for &'a mut [u8] {
    #[inline]
    fn from(content: &'a mut Utf8Str) -> Self {
        content.as_bytes_mut()
    }
}

impl<'a, const N: usize> TryFrom<&'a Utf8Str> for &'a [u8; N] {
    type Error = &'a Utf8Str;

    #[inline]
    fn try_from(content: &'a Utf8Str) -> Result<Self, Self::Error> {
        if let Ok(arr) = content.as_bytes().try_into() {
            Ok(arr)
        } else {
            Err(content)
        }
    }
}

impl<'a, const N: usize> TryFrom<&'a mut Utf8Str> for &'a mut [u8; N] {
    type Error = TryFromSliceError;

    #[inline]
    fn try_from(content: &'a mut Utf8Str) -> Result<Self, Self::Error> {
        content.as_bytes_mut().try_into()
    }
}

impl<'a> From<&'a Utf8Str> for Cow<'a, [u8]> {
    #[inline]
    fn from(content: &'a Utf8Str) -> Self {
        Cow::Borrowed(content.as_bytes())
    }
}

impl<'a> TryFrom<&'a Utf8Str> for &'a str {
    type Error = &'a Utf8Str;

    #[inline]
    fn try_from(s: &'a Utf8Str) -> Result<Self, Self::Error> {
        if s.is_valid_encoding() {
            let slice = s.as_bytes();
            // SAFETY: `is_valid_encoding` only returns true if the byte
            // content of the `&Utf8Str` is valid UTF-8.
            let s = unsafe { str::from_utf8_unchecked(slice) };
            Ok(s)
        } else {
            Err(s)
        }
    }
}

impl<'a> TryFrom<&'a Utf8Str> for Cow<'a, str> {
    type Error = &'a Utf8Str;

    #[inline]
    fn try_from(content: &'a Utf8Str) -> Result<Self, Self::Error> {
        let s = content.try_into()?;
        Ok(Cow::Borrowed(s))
    }
}

impl<'a> From<&'a Utf8Str> for Cow<'a, Utf8Str> {
    #[inline]
    fn from(content: &'a Utf8Str) -> Self {
        Cow::Borrowed(content)
    }
}

impl From<Box<[u8]>> for Box<Utf8Str> {
    #[inline]
    fn from(s: Box<[u8]>) -> Self {
        Utf8Str::from_boxed_bytes(s)
    }
}

impl From<Box<Utf8Str>> for Box<[u8]> {
    #[inline]
    fn from(s: Box<Utf8Str>) -> Self {
        Utf8Str::into_boxed_bytes(s)
    }
}
