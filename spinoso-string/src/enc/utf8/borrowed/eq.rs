use alloc::string::String;
use alloc::vec::Vec;

use bstr::BStr;

use super::Utf8Str;

impl Eq for Utf8Str {}

impl PartialEq<Utf8Str> for Utf8Str {
    fn eq(&self, other: &Utf8Str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<'a> PartialEq<&'a Utf8Str> for Utf8Str {
    fn eq(&self, other: &&'a Utf8Str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl_partial_eq!(Utf8Str, Vec<u8>);
impl_partial_eq!(Utf8Str, &'a Vec<u8>);
impl_partial_eq!(Utf8Str, [u8]);
impl_partial_eq!(Utf8Str, &'a [u8]);
impl_partial_eq!(Utf8Str, &'a mut [u8]);
impl_partial_eq!(Utf8Str, BStr);
impl_partial_eq!(Utf8Str, &'a BStr);
impl_partial_eq!(Utf8Str, &'a mut BStr);
impl_partial_eq!(Utf8Str, String);
impl_partial_eq!(Utf8Str, &'a String);
impl_partial_eq!(Utf8Str, str);
impl_partial_eq!(Utf8Str, &'a str);
impl_partial_eq!(Utf8Str, &'a mut str);
impl_partial_eq_array!(Utf8Str, [u8; N]);
impl_partial_eq_array!(Utf8Str, &'a [u8; N]);
impl_partial_eq_array!(Utf8Str, &'a mut [u8; N]);
