use alloc::vec::Vec;

use crate::encoding::Encoding;
use crate::ascii_string::AsciiString;
use crate::binary_string::BinaryString;
use crate::utf8_string::Utf8String;

pub enum EncodedString {
    Ascii(AsciiString),
    Binary(BinaryString),
    Utf8(Utf8String),
}

impl EncodedString {
    pub fn new(buf: Vec<u8>, encoding: Encoding) -> Self {
        match encoding {
            Encoding::Ascii => Self::Ascii(AsciiString::new(buf)),
            Encoding::Binary => Self::Binary(BinaryString::new(buf)),
            Encoding::Utf8 => Self::Utf8(Utf8String::new(buf)),
        }
    }
}

// functions where EncodedString can make the decision
impl EncodedString {
    pub fn encoding(&self) -> Encoding {
        match self {
            EncodedString::Ascii(_) => Encoding::Ascii,
            EncodedString::Binary(_) => Encoding::Binary,
            EncodedString::Utf8(_) => Encoding::Utf8,
        }
    }
}

// Functions whre the instance defines the method
impl EncodedString {
    pub fn as_bstr(&self) -> Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.as_bstr(),
            EncodedString::Binary(n) => n.as_bstr(),
            EncodedString::Utf8(n) => n.as_bstr(),
        }
    }
}
