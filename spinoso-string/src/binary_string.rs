use alloc::vec::Vec;

#[derive(Default, Clone)]
pub struct BinaryString(Vec<u8>);

impl BinaryString {
    pub fn new(buf: Vec<u8>) -> Self {
        Self(buf)
    }

    pub fn as_bstr(&self) -> Vec<u8> {
        self.as_bstr()
    }
}
