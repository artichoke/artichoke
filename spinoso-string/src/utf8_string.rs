use alloc::vec::Vec;
use bstr::{ByteSlice, BStr};
use crate::encoded_accessors::EncodedAccessors;

#[derive(Default, Clone)]
pub struct Utf8String {
    inner: Vec<u8>
}

// Constructors
impl Utf8String {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { inner: buf }
    }
}

// Debug
impl Utf8String {
    pub fn as_bstr(&self) -> &BStr {
        self.inner.as_bstr()
    }
}

// Raw
impl EncodedAccessors for Utf8String {
    fn as_vec(&self) -> &Vec<u8> {
        &self.inner
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

// Size and Capacity
impl Utf8String {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn set_len(&mut self, len: usize) {
        self.inner.set_len(len);
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

// Migration functions
// TODO: Remove these. If it compiles, we've migrated successfully
impl Utf8String {
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
