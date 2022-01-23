use alloc::vec::Vec;

#[derive(Default, Clone)]
pub struct AsciiString(Vec<u8>);

impl AsciiString {
    pub fn new(buf: Vec<u8>) -> Self {
        Self(buf)
    }

    pub fn as_bstr(&self) -> Vec<u8> {
        self.as_bstr()
    }
}
