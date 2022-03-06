use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::fmt;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::SliceIndex;
#[cfg(feature = "std")]
use std::io;

use crate::String;

impl fmt::Write for String {
    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        let mut buf = alloc::string::String::new();
        buf.write_fmt(args)?;
        self.push_str(buf.as_str());
        Ok(())
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push_char(c);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl io::Write for String {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner.write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.inner.write_fmt(fmt)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.inner.write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl Extend<u8> for String {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl<'a> Extend<&'a u8> for String {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl<'a> Extend<&'a mut u8> for String {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a mut u8>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl FromIterator<u8> for String {
    #[inline]
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut s = String::new();
        s.extend(iter.into_iter());
        s
    }
}

impl<'a> FromIterator<&'a u8> for String {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        let buf = iter.into_iter().copied().collect::<Vec<_>>();
        String::utf8(buf)
    }
}

impl<'a> FromIterator<&'a mut u8> for String {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a mut u8>>(iter: I) -> Self {
        let buf = iter.into_iter().map(|&mut b| b).collect::<Vec<_>>();
        String::utf8(buf)
    }
}

impl FromIterator<char> for String {
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let s = iter.into_iter().collect::<alloc::string::String>();
        String::from(s)
    }
}

impl<'a> FromIterator<&'a char> for String {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        iter.into_iter().copied().collect::<String>()
    }
}

impl<'a> FromIterator<&'a mut char> for String {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a mut char>>(iter: I) -> Self {
        iter.into_iter().map(|&mut ch| ch).collect::<String>()
    }
}

impl From<Vec<u8>> for String {
    #[inline]
    fn from(content: Vec<u8>) -> Self {
        Self::utf8(content)
    }
}

impl<'a> From<&'a [u8]> for String {
    #[inline]
    fn from(content: &'a [u8]) -> Self {
        Self::utf8(content.to_vec())
    }
}

impl<'a> From<&'a mut [u8]> for String {
    #[inline]
    fn from(content: &'a mut [u8]) -> Self {
        Self::utf8(content.to_vec())
    }
}

impl<'a> From<Cow<'a, [u8]>> for String {
    #[inline]
    fn from(content: Cow<'a, [u8]>) -> Self {
        Self::utf8(content.into_owned())
    }
}

impl From<alloc::string::String> for String {
    #[inline]
    fn from(s: alloc::string::String) -> Self {
        Self::utf8(s.into_bytes())
    }
}

impl From<&str> for String {
    #[inline]
    fn from(s: &str) -> Self {
        Self::utf8(s.as_bytes().to_vec())
    }
}

impl From<String> for Vec<u8> {
    #[inline]
    fn from(s: String) -> Self {
        s.inner.into_vec()
    }
}

impl AsRef<[u8]> for String {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl AsMut<[u8]> for String {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl AsRef<Vec<u8>> for String {
    #[inline]
    fn as_ref(&self) -> &Vec<u8> {
        self.inner.as_ref()
    }
}

impl AsMut<Vec<u8>> for String {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<u8> {
        self.inner.as_mut()
    }
}

impl Deref for String {
    type Target = [u8];

    #[allow(clippy::explicit_deref_methods)]
    #[inline]
    fn deref(&self) -> &[u8] {
        self.inner.deref()
    }
}

impl DerefMut for String {
    #[allow(clippy::explicit_deref_methods)]
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.inner.deref_mut()
    }
}

impl Borrow<[u8]> for String {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.inner.borrow()
    }
}

impl BorrowMut<[u8]> for String {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.inner.borrow_mut()
    }
}

impl Borrow<Vec<u8>> for String {
    #[inline]
    fn borrow(&self) -> &Vec<u8> {
        self.inner.borrow()
    }
}

impl BorrowMut<Vec<u8>> for String {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Vec<u8> {
        self.inner.borrow_mut()
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for String {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.inner.as_slice(), index)
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for String {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(self.inner.as_mut_slice(), index)
    }
}
