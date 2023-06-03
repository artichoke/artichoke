use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::TryReserveError;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
#[cfg(feature = "std")]
use core::fmt::Arguments;
use core::ops::{Deref, DerefMut, RangeBounds};
#[cfg(feature = "std")]
use std::io::{self, IoSlice, Write};

use bstr::ByteVec;
use raw_parts::RawParts;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Buf {
    inner: Vec<u8>,
}

impl From<Vec<u8>> for Buf {
    #[inline]
    fn from(vec: Vec<u8>) -> Self {
        Self { inner: vec }
    }
}

impl<'a> From<&'a [u8]> for Buf {
    #[inline]
    fn from(s: &'a [u8]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<'a> From<&'a mut [u8]> for Buf {
    #[inline]
    fn from(s: &'a mut [u8]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<const N: usize> From<[u8; N]> for Buf {
    #[inline]
    fn from(s: [u8; N]) -> Self {
        let vec = Vec::from(s);
        Self::from(vec)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for Buf {
    #[inline]
    fn from(s: &'a [u8; N]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for Buf {
    #[inline]
    fn from(s: &'a mut [u8; N]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<'a> From<Cow<'a, [u8]>> for Buf {
    #[inline]
    fn from(s: Cow<'a, [u8]>) -> Self {
        let vec = s.into_owned();
        Self::from(vec)
    }
}

impl From<String> for Buf {
    #[inline]
    fn from(s: String) -> Self {
        let vec = s.into_bytes();
        Self::from(vec)
    }
}

impl<'a> From<&'a str> for Buf {
    #[inline]
    fn from(s: &'a str) -> Self {
        let vec = s.as_bytes().to_vec();
        Self::from(vec)
    }
}

impl<'a> From<&'a mut str> for Buf {
    #[inline]
    fn from(s: &'a mut str) -> Self {
        let vec = s.as_bytes().to_vec();
        Self::from(vec)
    }
}

impl<'a> From<Cow<'a, str>> for Buf {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        let vec = s.into_owned().into_bytes();
        Self::from(vec)
    }
}

impl From<Buf> for Vec<u8> {
    #[inline]
    fn from(buf: Buf) -> Self {
        buf.inner
    }
}

impl<const N: usize> TryFrom<Buf> for [u8; N] {
    type Error = Buf;

    #[inline]
    fn try_from(buf: Buf) -> Result<Self, Self::Error> {
        match buf.into_inner().try_into() {
            Ok(array) => Ok(array),
            Err(vec) => Err(vec.into()),
        }
    }
}

impl<'a> From<Buf> for Cow<'a, [u8]> {
    #[inline]
    fn from(buf: Buf) -> Self {
        Cow::Owned(buf.into())
    }
}

impl AsRef<[u8]> for Buf {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl AsMut<[u8]> for Buf {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl Borrow<[u8]> for Buf {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl BorrowMut<[u8]> for Buf {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl Deref for Buf {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Buf {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FromIterator<u8> for Buf {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        let inner = iter.into_iter().collect();
        Self { inner }
    }
}

impl Extend<u8> for Buf {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter());
    }
}

impl<'a> Extend<&'a u8> for Buf {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().copied());
    }
}

macro_rules! impl_partial_eq {
    ($lhs:ty, $rhs:ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                let other: &[u8] = other.as_ref();
                PartialEq::eq(self.as_slice(), other)
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                let this: &[u8] = self.as_ref();
                PartialEq::eq(this, other.as_slice())
            }
        }
    };
}

macro_rules! impl_partial_eq_array {
    ($lhs:ty, $rhs:ty) => {
        impl<'a, 'b, const N: usize> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                let other: &[u8] = other.as_ref();
                PartialEq::eq(self.as_slice(), other)
            }
        }

        impl<'a, 'b, const N: usize> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                let this: &[u8] = self.as_ref();
                PartialEq::eq(this, other.as_slice())
            }
        }
    };
}

impl_partial_eq!(Buf, Vec<u8>);
impl_partial_eq!(Buf, &'a Vec<u8>);
impl_partial_eq!(Buf, [u8]);
impl_partial_eq!(Buf, &'a [u8]);
impl_partial_eq!(Buf, String);
impl_partial_eq!(Buf, &'a String);
impl_partial_eq!(Buf, str);
impl_partial_eq!(Buf, &'a str);
impl_partial_eq_array!(Buf, [u8; N]);
impl_partial_eq_array!(Buf, &'a [u8; N]);

impl IntoIterator for Buf {
    type Item = u8;
    type IntoIter = IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inner().into_iter()
    }
}

impl<'a> IntoIterator for &'a Buf {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Buf {
    type Item = &'a mut u8;
    type IntoIter = IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Buf {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = Vec::with_capacity(capacity);
        Self { inner }
    }

    #[inline]
    #[must_use]
    pub unsafe fn from_raw_parts(raw_parts: RawParts<u8>) -> Self {
        let inner = raw_parts.into_vec();
        Self { inner }
    }

    #[inline]
    #[must_use]
    pub fn into_raw_parts(self) -> RawParts<u8> {
        RawParts::from_vec(self.inner)
    }

    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    #[inline]
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.inner.into_boxed_slice()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> u8 {
        self.inner.swap_remove(index)
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: u8) {
        self.inner.insert(index, element);
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> u8 {
        self.inner.remove(index)
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&u8) -> bool,
    {
        self.inner.retain(f);
    }

    #[inline]
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut u8) -> bool,
    {
        self.inner.retain_mut(f);
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut u8) -> K,
        K: PartialEq<K>,
    {
        self.inner.dedup_by_key(key);
    }

    #[inline]
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut u8, &mut u8) -> bool,
    {
        self.inner.dedup_by(same_bucket);
    }

    #[inline]
    pub fn push(&mut self, value: u8) {
        self.inner.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        self.inner.pop()
    }

    #[inline]
    pub fn append(&mut self, other: &mut Buf) {
        self.inner.append(&mut other.inner);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn split_off(&mut self, at: usize) -> Self {
        let split = self.inner.split_off(at);
        Self::from(split)
    }

    #[inline]
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> u8,
    {
        self.inner.resize_with(new_len, f);
    }

    #[inline]
    #[must_use]
    pub fn leak<'a>(self) -> &'a mut [u8] {
        self.inner.leak()
    }
}

impl Buf
where
    u8: Clone,
{
    #[inline]
    pub fn resize(&mut self, new_len: usize, value: u8) {
        self.inner.resize(new_len, value);
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
    }

    #[inline]
    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize>,
    {
        self.inner.extend_from_within(src);
    }
}

impl Buf
where
    u8: PartialEq<u8>,
{
    #[inline]
    pub fn dedup(&mut self) {
        self.inner.dedup();
    }
}

impl Buf {
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

// `bstr::ByteVec` impls
impl Buf {
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
    }

    #[inline]
    pub fn push_str<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.inner.push_str(bytes);
    }
}

#[cfg(feature = "std")]
impl Write for Buf {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.inner.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner.write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> io::Result<()> {
        self.inner.write_fmt(fmt)
    }
}
