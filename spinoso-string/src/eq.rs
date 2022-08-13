use alloc::vec::Vec;

use crate::String;

impl PartialEq<Vec<u8>> for String {
    fn eq(&self, other: &Vec<u8>) -> bool {
        **self == **other
    }
}

impl PartialEq<String> for Vec<u8> {
    fn eq(&self, other: &String) -> bool {
        **self == **other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for String {
    fn eq(&self, other: &[u8; N]) -> bool {
        **self == *other
    }
}

impl<const N: usize> PartialEq<String> for [u8; N] {
    fn eq(&self, other: &String) -> bool {
        *self == **other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for String {
    fn eq(&self, other: &&[u8; N]) -> bool {
        **self == **other
    }
}

impl<const N: usize> PartialEq<String> for &[u8; N] {
    fn eq(&self, other: &String) -> bool {
        **self == **other
    }
}

impl PartialEq<[u8]> for String {
    fn eq(&self, other: &[u8]) -> bool {
        **self == *other
    }
}

impl PartialEq<String> for [u8] {
    fn eq(&self, other: &String) -> bool {
        *self == **other
    }
}

impl PartialEq<&[u8]> for String {
    fn eq(&self, other: &&[u8]) -> bool {
        **self == **other
    }
}

impl PartialEq<String> for &[u8] {
    fn eq(&self, other: &String) -> bool {
        **self == **other
    }
}

impl PartialEq<alloc::string::String> for String {
    fn eq(&self, other: &alloc::string::String) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<String> for alloc::string::String {
    fn eq(&self, other: &String) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        *self.as_bytes() == **other
    }
}

impl PartialEq<&str> for String {
    fn eq(&self, other: &&str) -> bool {
        **self == *other.as_bytes()
    }
}

impl PartialEq<String> for &str {
    fn eq(&self, other: &String) -> bool {
        *self.as_bytes() == **other
    }
}
