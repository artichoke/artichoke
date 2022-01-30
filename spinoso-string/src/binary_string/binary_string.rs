use alloc::vec::Vec;
use core::slice::SliceIndex;

use bstr::{BStr, ByteSlice, ByteVec};

use crate::codepoints::InvalidCodepointError;
use crate::iter::{Bytes, IntoIter, Iter, IterMut};
use crate::ord::OrdError;

#[derive(Default, Clone)]
pub struct BinaryString {
    inner: Vec<u8>,
}

// Constructors
impl BinaryString {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { inner: buf }
    }
}

// Debug
impl BinaryString {
    pub fn as_bstr(&self) -> &BStr {
        self.inner.as_bstr()
    }
}

// Raw
impl BinaryString {
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.inner
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

// Core Iterators
impl BinaryString {
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.inner.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut(self.inner.iter_mut())
    }

    pub fn bytes(&self) -> Bytes<'_> {
        Bytes(self.inner.iter())
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter(self.inner.into_iter())
    }
}

// Size and Capacity
impl BinaryString {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.inner.set_len(len);
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    pub fn char_len(&self) -> usize {
        self.len()
    }
}

// Memory management
impl BinaryString {
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

// Indexing
impl BinaryString {
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get(index)
    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_mut(index)
    }

    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked_mut(index)
    }
}

// Pushing and popping bytes, codepoints, and strings.
impl BinaryString {
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    pub fn try_push_codepoint(&mut self, codepoint: i64) -> Result<(), InvalidCodepointError> {
        if let Ok(byte) = u8::try_from(codepoint) {
            self.push_byte(byte);
            Ok(())
        } else {
            Err(InvalidCodepointError::codepoint_out_of_range(codepoint))
        }
    }

    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
    }

    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
    }
}

// Encoding
impl BinaryString {
    pub fn is_ascii_only(&self) -> bool {
        self.inner.is_ascii()
    }
}

// Casing
impl BinaryString {
    pub fn make_capitalized(&mut self) {
        if let Some((head, tail)) = self.inner.split_first_mut() {
            head.make_ascii_uppercase();
            tail.make_ascii_lowercase();
        }
    }

    pub fn make_lowercase(&mut self) {
        self.inner.make_ascii_lowercase();
    }

    pub fn make_uppercase(&mut self) {
        self.inner.make_ascii_uppercase();
    }
}

impl BinaryString {
    pub fn chr(&self) -> &[u8] {
        self.inner.get(0..1).unwrap_or_default()
    }

    pub fn ord(&self) -> Result<u32, OrdError> {
        let byte = self.inner.get(0).copied().ok_or_else(OrdError::empty_string)?;
        Ok(u32::from(byte))
    }
}

// Migration functions
// TODO: Remove these. If it compiles, we've migrated successfully
impl BinaryString {
    pub fn buf(&self) -> &Vec<u8> {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::binary_string::BinaryString;

    #[test]
    fn constructs_empty_buffer() {
        let s = BinaryString::new(Vec::new());
        assert_eq!(0, s.len());
    }
}
