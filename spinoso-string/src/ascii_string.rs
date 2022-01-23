use alloc::vec::Vec;
use bstr::{ByteSlice, BStr};

#[derive(Default, Clone)]
pub struct AsciiString(Vec<u8>);

// Constructors
impl AsciiString {
    pub fn new(buf: Vec<u8>) -> Self {
        Self(buf)
    }
}

// Debug
impl AsciiString {
    pub fn as_bstr(&self) -> &BStr {
        self.0.as_bstr()
    }
}

// Size and Capacity
impl AsciiString {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }
}

// Migration functions
// TODO: Remove these. If it compiles, we've migrated successfully
impl AsciiString {
    pub fn buf(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
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
