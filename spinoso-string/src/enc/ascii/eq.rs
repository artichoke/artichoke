use alloc::string::String;
use alloc::vec::Vec;

use super::AsciiString;

impl PartialEq<Vec<u8>> for AsciiString {
    fn eq(&self, other: &Vec<u8>) -> bool {
        **self == **other
    }
}

impl PartialEq<AsciiString> for Vec<u8> {
    fn eq(&self, other: &AsciiString) -> bool {
        **self == **other
    }
}

impl PartialEq<[u8]> for AsciiString {
    fn eq(&self, other: &[u8]) -> bool {
        **self == *other
    }
}

impl PartialEq<AsciiString> for [u8] {
    fn eq(&self, other: &AsciiString) -> bool {
        *self == **other
    }
}

impl PartialEq<&[u8]> for AsciiString {
    fn eq(&self, other: &&[u8]) -> bool {
        **self == **other
    }
}

impl PartialEq<AsciiString> for &[u8] {
    fn eq(&self, other: &AsciiString) -> bool {
        **self == **other
    }
}

impl PartialEq<String> for AsciiString {
    fn eq(&self, other: &String) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<AsciiString> for String {
    fn eq(&self, other: &AsciiString) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<str> for AsciiString {
    fn eq(&self, other: &str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<AsciiString> for str {
    fn eq(&self, other: &AsciiString) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<&str> for AsciiString {
    fn eq(&self, other: &&str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<AsciiString> for &str {
    fn eq(&self, other: &AsciiString) -> bool {
        *self.as_bytes() == **other
    }
}
