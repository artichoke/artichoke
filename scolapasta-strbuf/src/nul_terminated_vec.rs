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

/// Ensure the given `Vec` can be used safely by C code as a string buffer.
///
/// mruby C code assumes that all string buffers it allocates have at least one
/// extra byte trailing the requested capacity AND that said byte is the NUL
/// byte (`b'\0'` or `0`).
///
/// This function MUST be called by all APIs which may modify the inner `Vec`.
///
/// This function produces a stronger guarantee than that provided by mruby: the
/// first AND last bytes of the spare capacity trailing the `Vec` will be the
/// NUL byte.
fn ensure_nul_terminated(vec: &mut Vec<u8>) -> Result<(), TryReserveError> {
    const NUL_BYTE: u8 = 0;

    let spare_capacity = vec.spare_capacity_mut();
    // If the vec has spare capacity, set the first and last bytes to NUL.
    // See:
    //
    // - https://github.com/artichoke/artichoke/pull/1976#discussion_r932782264
    // - https://github.com/artichoke/artichoke/blob/16c869a9ad29acfe143bfcc011917ef442ccac54/artichoke-backend/vendor/mruby/src/string.c#L36-L38
    match spare_capacity {
        [] => {}
        [next] => {
            next.write(NUL_BYTE);
            return Ok(());
        }
        [head, .., tail] => {
            head.write(NUL_BYTE);
            tail.write(NUL_BYTE);
            return Ok(());
        }
    }
    // Else `vec.len == vec.capacity`, so reserve an extra byte.
    vec.try_reserve_exact(1)?;
    let spare_capacity = vec.spare_capacity_mut();
    match spare_capacity {
        [] => unreachable!("Vec should have spare capacity"),
        [next] => {
            next.write(NUL_BYTE);
        }
        [head, .., tail] => {
            head.write(NUL_BYTE);
            tail.write(NUL_BYTE);
        }
    }
    Ok(())
}

#[repr(transparent)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Buf {
    inner: Vec<u8>,
}

impl Default for Buf {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Buf {
    #[inline]
    fn clone(&self) -> Self {
        let vec = self.inner.clone();
        Self::from(vec)
    }
}

impl From<Vec<u8>> for Buf {
    #[inline]
    fn from(mut vec: Vec<u8>) -> Self {
        ensure_nul_terminated(&mut vec).expect("alloc failure");
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
        // SAFETY: the mutable reference given out is a slice, NOT the
        // underlying `Vec`, so the allocation cannot change size.
        &mut self.inner
    }
}

impl FromIterator<u8> for Buf {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        let inner = iter.into_iter().collect::<Vec<u8>>();
        Self::from(inner)
    }
}

impl Extend<u8> for Buf {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter());
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }
}

impl Buf {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let inner = Vec::with_capacity(1);
        Self::from(inner)
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.checked_add(1).expect("capacity overflow");
        let inner = Vec::with_capacity(capacity);
        Self::from(inner)
    }

    #[inline]
    #[must_use]
    pub unsafe fn from_raw_parts(raw_parts: RawParts<u8>) -> Self {
        let inner = raw_parts.into_vec();
        Self::from(inner)
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
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        let additional = additional.checked_add(1).unwrap_or(additional);
        self.inner.try_reserve(additional)?;
        ensure_nul_terminated(&mut self.inner)?;
        Ok(())
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        let additional = additional.checked_add(1).unwrap_or(additional);
        self.inner.try_reserve_exact(additional)?;
        ensure_nul_terminated(&mut self.inner)?;
        Ok(())
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.inner.into_boxed_slice()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    #[inline]
    #[must_use]
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
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> u8 {
        let removed = self.inner.swap_remove(index);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        removed
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: u8) {
        self.inner.insert(index, element);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> u8 {
        let removed = self.inner.remove(index);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        removed
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&u8) -> bool,
    {
        self.inner.retain(f);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut u8) -> bool,
    {
        self.inner.retain_mut(f);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut u8) -> K,
        K: PartialEq<K>,
    {
        self.inner.dedup_by_key(key);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut u8, &mut u8) -> bool,
    {
        self.inner.dedup_by(same_bucket);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn push(&mut self, value: u8) {
        self.inner.push(value);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        let popped = self.inner.pop();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        popped
    }

    #[inline]
    pub fn append(&mut self, other: &mut Buf) {
        self.inner.append(&mut other.inner);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        ensure_nul_terminated(&mut other.inner).expect("alloc failure");
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
    pub fn split_off(&mut self, at: usize) -> Self {
        let split = self.inner.split_off(at);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        Self::from(split)
    }

    #[inline]
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> u8,
    {
        self.inner.resize_with(new_len, f);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize>,
    {
        self.inner.extend_from_within(src);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }
}

impl Buf
where
    u8: PartialEq<u8>,
{
    #[inline]
    pub fn dedup(&mut self) {
        self.inner.dedup();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    #[inline]
    pub fn push_str<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.inner.push_str(bytes);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }
}

#[cfg(feature = "std")]
impl Write for Buf {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = self.inner.write(buf);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        let result = self.inner.flush();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let result = self.inner.write_vectored(bufs);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let result = self.inner.write_all(buf);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }

    #[inline]
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> io::Result<()> {
        let result = self.inner.write_fmt(fmt);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    use quickcheck::quickcheck;
    use raw_parts::RawParts;

    use super::{ensure_nul_terminated, Buf};

    #[must_use]
    fn is_nul_terminated(bytes: &mut Vec<u8>) -> bool {
        let spare_capacity = bytes.spare_capacity_mut();
        if spare_capacity.is_empty() {
            return false;
        }

        let first = unsafe { spare_capacity.first().unwrap().assume_init() };
        if first != 0 {
            return false;
        }

        let last = unsafe { spare_capacity.last().unwrap().assume_init() };
        if last != 0 {
            return false;
        }
        true
    }

    #[test]
    fn test_ensure_nul_terminated_default() {
        let buf = Buf::default();
        let mut bytes = buf.into_inner();
        assert!(is_nul_terminated(&mut bytes));
    }

    #[test]
    fn test_ensure_nul_terminated_new() {
        let buf = Buf::new();
        let mut bytes = buf.into_inner();
        assert!(is_nul_terminated(&mut bytes));
    }

    #[test]
    fn test_ensure_nul_terminated_with_capacity() {
        let capacities = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
        for capa in capacities {
            let buf = Buf::with_capacity(capa);
            let mut bytes = buf.into_inner();
            assert!(is_nul_terminated(&mut bytes), "failed for capacity {capa}");
        }
    }

    quickcheck! {
        fn test_ensure_nul_terminated(bytes: Vec<u8>) -> bool {
            let mut bytes = bytes;
            ensure_nul_terminated(&mut bytes).unwrap();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_after_shrink(bytes: Vec<u8>) -> bool {
            let mut bytes = bytes;
            bytes.shrink_to_fit();
            ensure_nul_terminated(&mut bytes).unwrap();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_vec(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_buf(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            let mut bytes = Vec::from(buf);
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_after_clone(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            let buf = buf.clone();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_iterator(bytes: Vec<u8>) -> bool {
            let buf = Buf::from_iter(bytes.into_iter());
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_collect(bytes: Vec<u8>) -> bool {
            let buf = bytes.into_iter().collect::<Buf>();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_after_extend(bytes: Vec<u8>, extend: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.extend(extend.into_iter());
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_raw_parts(bytes: Vec<u8>) -> bool {
            let raw_parts = RawParts::from_vec(bytes);
            let buf = unsafe { Buf::from_raw_parts(raw_parts) };
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_raw_parts_round_trip(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            let raw_parts = buf.into_raw_parts();
            let buf = unsafe { Buf::from_raw_parts(raw_parts) };
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_reserve(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.reserve(reserve);
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_reserve_exact(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.reserve_exact(reserve);
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_try_reserve(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.try_reserve(reserve).unwrap();
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_try_reserve_exact(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.try_reserve_exact(reserve).unwrap();
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_shrink_to_fit(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.shrink_to_fit();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_shrink_to(bytes: Vec<u8>, shrink_to: usize) -> bool {
            let mut buf = Buf::from(bytes);
            buf.shrink_to(shrink_to);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_truncate(bytes: Vec<u8>, truncate_to: usize) -> bool {
            let mut buf = Buf::from(bytes);
            buf.truncate(truncate_to);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_set_len(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            unsafe {
                buf.set_len(0);
            }
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_swap_remove_first(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.swap_remove(0);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_swap_remove_last(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.swap_remove(buf.len() - 1);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_swap_remove_interior(bytes: Vec<u8>) -> bool {
            if bytes.len() < 2 {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.swap_remove(buf.len() - 2);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_first(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.insert(0, u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_past_end(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.insert(buf.len(), u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_last(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.insert(buf.len() - 1, u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_interior(bytes: Vec<u8>) -> bool {
            if bytes.len() < 2 {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.insert(buf.len() - 2, u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_remove_first(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.remove(0);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_remove_last(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.remove(buf.len() - 1);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_remove_interior(bytes: Vec<u8>) -> bool {
            if bytes.len() < 2 {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.remove(buf.len() - 2);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_all(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.retain(|_| true);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_none(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.retain(|_| false);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_some(bytes: Vec<u8>) -> bool {
            let mut idx = 0_usize;
            let mut buf = Buf::from(bytes);
            buf.retain(|_| {
                idx += 1;
                idx % 2 == 0
            });
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_mut_all(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.retain_mut(|_| true);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_mut_none(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.retain_mut(|_| false);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_mut_some(bytes: Vec<u8>) -> bool {
            let mut idx = 0_usize;
            let mut buf = Buf::from(bytes);
            buf.retain_mut(|_| {
                idx += 1;
                idx % 2 == 0
            });
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_dedup_by_key(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.dedup_by_key(|byte| *byte);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_dedup_by(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.dedup_by(|&mut a, &mut b| a == b);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_push(bytes: Vec<u8>, pushed: u8) -> bool {
            let mut buf = Buf::from(bytes);
            buf.push(pushed);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_pop(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.pop();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_append(bytes: Vec<u8>, other: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            let mut other_buf = Buf::from(other);
            buf.append(&mut other_buf);
            let mut bytes = buf.into_inner();
            let mut other = other_buf.into_inner();
            is_nul_terminated(&mut bytes) && is_nul_terminated(&mut other)
        }

        fn test_ensure_nul_terminated_clear(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.clear();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_split_off_before_first(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            let split = buf.split_off(0);
            let mut bytes = buf.into_inner();
            let mut split = split.into_inner();
            is_nul_terminated(&mut bytes) && is_nul_terminated(&mut split)
        }

        fn test_ensure_nul_terminated_split_off_first(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            let split = buf.split_off(1);
            let mut bytes = buf.into_inner();
            let mut split = split.into_inner();
            is_nul_terminated(&mut bytes) && is_nul_terminated(&mut split)
        }

        fn test_ensure_nul_terminated_split_off_past_end(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            let split = buf.split_off(buf.len());
            let mut bytes = buf.into_inner();
            let mut split = split.into_inner();
            is_nul_terminated(&mut bytes) && is_nul_terminated(&mut split)
        }

        fn test_ensure_nul_terminated_split_off_last(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            let split = buf.split_off(buf.len() - 1);
            let mut bytes = buf.into_inner();
            let mut split = split.into_inner();
            is_nul_terminated(&mut bytes) && is_nul_terminated(&mut split)
        }

        fn test_ensure_nul_terminated_split_off_interior(bytes: Vec<u8>) -> bool {
            if bytes.len() < 2 {
                return true;
            }
            let mut buf = Buf::from(bytes);
            let split = buf.split_off(buf.len() - 2);
            let mut bytes = buf.into_inner();
            let mut split = split.into_inner();
            is_nul_terminated(&mut bytes) && is_nul_terminated(&mut split)
        }

        fn test_ensure_nul_terminated_resize_with(bytes: Vec<u8>) -> bool {
            let lengths = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for len in lengths {
                let mut buf = Buf::from(bytes.clone());
                buf.resize_with(len, || u8::MAX);
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_resize(bytes: Vec<u8>) -> bool {
            let lengths = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for len in lengths {
                let mut buf = Buf::from(bytes.clone());
                buf.resize(len, u8::MAX);
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_extend_from_slice(bytes: Vec<u8>, other: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.extend_from_slice(&other);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_extend_from_within_prefix(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.extend_from_within(0..0);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_extend_from_within_suffix(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.extend_from_within(buf.len()..buf.len());
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_extend_from_within_all(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.extend_from_within(0..buf.len());
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_extend_from_within_subset(bytes: Vec<u8>) -> bool {
            if bytes.len() < 3 {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.extend_from_within(1..buf.len() - 2);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_dedup(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.dedup();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_push_byte(bytes: Vec<u8>, pushed: u8) -> bool {
            let mut buf = Buf::from(bytes);
            buf.push_byte(pushed);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_push_char(bytes: Vec<u8>, pushed: char) -> bool {
            let mut buf = Buf::from(bytes);
            buf.push_char(pushed);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_push_str(bytes: Vec<u8>, pushed: String) -> bool {
            let mut buf = Buf::from(bytes);
            buf.push_str(pushed);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write(bytes: Vec<u8>, data: Vec<u8>) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            buf.write(&data).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_flush(bytes: Vec<u8>, data: Vec<u8>) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            buf.write_all(&data).unwrap();
            buf.flush().unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write_vectored(bytes: Vec<u8>, data1: Vec<u8>, data2: Vec<u8>) -> bool {
            use std::io::{IoSlice, Write};

            let mut buf = Buf::from(bytes);
            buf.write_vectored(&[IoSlice::new(&data1), IoSlice::new(&data2)]).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write_all(bytes: Vec<u8>, data: Vec<u8>) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            buf.write_all(&data).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write_fmt(bytes: Vec<u8>, data: String) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            buf.write_fmt(format_args!("{}", data)).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }
    }
}
