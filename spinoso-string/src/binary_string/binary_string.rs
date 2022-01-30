use alloc::vec::Vec;
use bstr::{ByteSlice, BStr};
use crate::iter::{IntoIter, Iter, IterMut, Bytes};

#[derive(Default, Clone)]
pub struct BinaryString {
    inner: Vec<u8>
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

// Migration functions
// TODO: Remove these. If it compiles, we've migrated successfully
impl BinaryString {
    pub fn buf(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_string::BinaryString;
    use alloc::vec::Vec;

    #[test]
    fn constructs_empty_buffer() {
        let s = BinaryString::new(Vec::new());
        assert_eq!(0, s.len());
    }
}
