use alloc::vec::Vec;

#[derive(Default, Clone)]
pub struct Utf8String(Vec<u8>);

impl Utf8String {
    pub fn new(buf: Vec<u8>) -> Self {
        Self(buf)
    }

    pub fn as_bstr(&self) -> Vec<u8> {
        self.as_bstr()
    }
}
