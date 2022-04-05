use alloc::string::String;
use alloc::vec::Vec;

use super::Utf8String;

impl PartialEq<Vec<u8>> for Utf8String {
    fn eq(&self, other: &Vec<u8>) -> bool {
        **self == **other
    }
}

impl PartialEq<Utf8String> for Vec<u8> {
    fn eq(&self, other: &Utf8String) -> bool {
        **self == **other
    }
}

impl PartialEq<[u8]> for Utf8String {
    fn eq(&self, other: &[u8]) -> bool {
        **self == *other
    }
}

impl PartialEq<Utf8String> for [u8] {
    fn eq(&self, other: &Utf8String) -> bool {
        *self == **other
    }
}

impl PartialEq<&[u8]> for Utf8String {
    fn eq(&self, other: &&[u8]) -> bool {
        **self == **other
    }
}

impl PartialEq<Utf8String> for &[u8] {
    fn eq(&self, other: &Utf8String) -> bool {
        **self == **other
    }
}

impl PartialEq<String> for Utf8String {
    fn eq(&self, other: &String) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<Utf8String> for String {
    fn eq(&self, other: &Utf8String) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<str> for Utf8String {
    fn eq(&self, other: &str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<Utf8String> for str {
    fn eq(&self, other: &Utf8String) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<&str> for Utf8String {
    fn eq(&self, other: &&str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<Utf8String> for &str {
    fn eq(&self, other: &Utf8String) -> bool {
        *self.as_bytes() == **other
    }
}
