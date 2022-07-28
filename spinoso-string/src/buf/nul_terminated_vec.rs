use alloc::boxed::Box;
use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::borrow::Borrow;
#[cfg(feature = "std")]
use core::fmt::Arguments;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut, RangeBounds};
#[cfg(feature = "std")]
use std::io::{self, IoSlice, Write};

use bstr::ByteVec;
use raw_parts::RawParts;

fn ensure_nul_terminated(vec: &mut Vec<u8>) {
    const NUL_BYTE: u8 = 0;

    let spare_capacity = vec.spare_capacity_mut();
    // If the vec has spare capacity, set the first byte to NUL.
    if let Some(next) = spare_capacity.get_mut(0) {
        next.write(NUL_BYTE);
        return;
    }
    // Else `vec.len == vec.capacity`, so reserve an extra byte.
    vec.reserve_exact(1);
    let spare_capacity = vec.spare_capacity_mut();
    let next = spare_capacity.get_mut(0).expect("Vec should have spare capacity");
    next.write(NUL_BYTE);
}

#[repr(transparent)]
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Buf {
    inner: Vec<u8>,
}

impl Clone for Buf {
    fn clone(&self) -> Self {
        let mut vec = self.inner.clone();
        ensure_nul_terminated(&mut vec);
        Self { inner: vec }
    }
}

impl From<Vec<u8>> for Buf {
    #[inline]
    fn from(mut vec: Vec<u8>) -> Self {
        ensure_nul_terminated(&mut vec);
        Self { inner: vec }
    }
}

impl From<Buf> for Vec<u8> {
    #[inline]
    fn from(buf: Buf) -> Self {
        buf.inner
    }
}

impl Deref for Buf {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl DerefMut for Buf {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: the mutable reference given out is a slice, NOT the underlying
        // `Vec`, so the allocation cannot change size.
        &mut *self.inner
    }
}

impl Borrow<[u8]> for Buf {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.inner
    }
}

impl FromIterator<u8> for Buf {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        let mut inner = iter.into_iter().collect();
        ensure_nul_terminated(&mut inner);
        Self { inner }
    }
}

impl Extend<u8> for Buf {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter());
        ensure_nul_terminated(&mut self.inner);
    }
}

impl Buf {
    #[inline]
    pub fn new() -> Self {
        let mut inner = Vec::with_capacity(1);
        ensure_nul_terminated(&mut inner);
        Self { inner }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.checked_add(1).expect("capacity overflow");
        let mut inner = Vec::with_capacity(capacity);
        ensure_nul_terminated(&mut inner);
        Self { inner }
    }

    #[inline]
    pub unsafe fn from_raw_parts(raw_parts: RawParts<u8>) -> Self {
        let mut inner = RawParts::into_vec(raw_parts);
        // SAFETY: callers may have written into the spare capacity of the `Vec`
        // so we must ensure the NUL termination byte is still present.
        ensure_nul_terminated(&mut inner);
        Self { inner }
    }

    #[inline]
    pub fn into_raw_parts(self) -> RawParts<u8> {
        RawParts::from_vec(self.inner)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)?;
        ensure_nul_terminated(&mut self.inner);
        Ok(())
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)?;
        ensure_nul_terminated(&mut self.inner);
        Ok(())
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.inner.into_boxed_slice()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> u8 {
        let removed = self.inner.swap_remove(index);
        ensure_nul_terminated(&mut self.inner);
        removed
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: u8) {
        self.inner.insert(index, element);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> u8 {
        let removed = self.inner.remove(index);
        ensure_nul_terminated(&mut self.inner);
        removed
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&u8) -> bool,
    {
        self.inner.retain(f);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut u8) -> bool,
    {
        self.inner.retain_mut(f);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut u8) -> K,
        K: PartialEq<K>,
    {
        self.inner.dedup_by_key(key);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut u8, &mut u8) -> bool,
    {
        self.inner.dedup_by(same_bucket);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn push(&mut self, value: u8) {
        self.inner.push(value);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        let popped = self.inner.pop();
        ensure_nul_terminated(&mut self.inner);
        popped
    }

    #[inline]
    pub fn append(&mut self, other: &mut Buf) {
        self.inner.append(&mut other.inner);
        ensure_nul_terminated(&mut self.inner);
        ensure_nul_terminated(&mut other.inner);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn split_off(&mut self, at: usize) -> Buf {
        let mut split = self.inner.split_off(at);
        ensure_nul_terminated(&mut self.inner);
        ensure_nul_terminated(&mut split);
        Self { inner: split }
    }

    #[inline]
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> u8,
    {
        self.inner.resize_with(new_len, f);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn leak<'a>(self) -> &'a mut [u8] {
        self.inner.leak()
    }

    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        self.inner.spare_capacity_mut()
    }
}

impl Buf
where
    u8: Clone,
{
    #[inline]
    pub fn resize(&mut self, new_len: usize, value: u8) {
        self.inner.resize(new_len, value);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize>,
    {
        self.inner.extend_from_within(src);
        ensure_nul_terminated(&mut self.inner);
    }
}

impl Buf
where
    u8: PartialEq<u8>,
{
    #[inline]
    pub fn dedup(&mut self) {
        self.inner.dedup();
        ensure_nul_terminated(&mut self.inner);
    }
}

impl Buf {
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

// bstr bytevec impls
impl Buf {
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
        ensure_nul_terminated(&mut self.inner);
    }

    #[inline]
    pub fn push_str<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.inner.push_str(bytes);
        ensure_nul_terminated(&mut self.inner);
    }
}

#[cfg(feature = "std")]
impl Write for Buf {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = self.inner.write(buf);
        ensure_nul_terminated(&mut self.inner);
        result
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        let result = self.inner.flush();
        ensure_nul_terminated(&mut self.inner);
        result
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let result = self.inner.write_vectored(bufs);
        ensure_nul_terminated(&mut self.inner);
        result
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let result = self.inner.write_all(buf);
        ensure_nul_terminated(&mut self.inner);
        result
    }

    #[inline]
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> io::Result<()> {
        let result = self.inner.write_fmt(fmt);
        ensure_nul_terminated(&mut self.inner);
        result
    }
}
