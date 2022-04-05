use alloc::string::String;
use alloc::vec::Vec;

use super::BinaryString;

impl PartialEq<Vec<u8>> for BinaryString {
    fn eq(&self, other: &Vec<u8>) -> bool {
        **self == **other
    }
}

impl PartialEq<BinaryString> for Vec<u8> {
    fn eq(&self, other: &BinaryString) -> bool {
        **self == **other
    }
}

impl PartialEq<[u8]> for BinaryString {
    fn eq(&self, other: &[u8]) -> bool {
        **self == *other
    }
}

impl PartialEq<BinaryString> for [u8] {
    fn eq(&self, other: &BinaryString) -> bool {
        *self == **other
    }
}

impl PartialEq<&[u8]> for BinaryString {
    fn eq(&self, other: &&[u8]) -> bool {
        **self == **other
    }
}

impl PartialEq<BinaryString> for &[u8] {
    fn eq(&self, other: &BinaryString) -> bool {
        **self == **other
    }
}

impl PartialEq<String> for BinaryString {
    fn eq(&self, other: &String) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<BinaryString> for String {
    fn eq(&self, other: &BinaryString) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<str> for BinaryString {
    fn eq(&self, other: &str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<BinaryString> for str {
    fn eq(&self, other: &BinaryString) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<&str> for BinaryString {
    fn eq(&self, other: &&str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<BinaryString> for &str {
    fn eq(&self, other: &BinaryString) -> bool {
        *self.as_bytes() == **other
    }
}
