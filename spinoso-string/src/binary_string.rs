use alloc::vec::Vec;
use bstr::{ByteSlice, BStr};

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

// Size and Capacity
impl BinaryString {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
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
        assert_eq!(inner, s.len());
    }
}
