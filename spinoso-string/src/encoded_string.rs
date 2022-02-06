use alloc::vec::Vec;
use core::ops::Range;
use core::slice::SliceIndex;

use bstr::BStr;

use crate::ascii_string::AsciiString;
use crate::binary_string::BinaryString;
use crate::codepoints::InvalidCodepointError;
use crate::encoding::Encoding;
use crate::iter::{Bytes, IntoIter, Iter, IterMut};
use crate::ord::OrdError;
use crate::utf8_string::Utf8String;

pub enum EncodedString {
    Ascii(AsciiString),
    Binary(BinaryString),
    Utf8(Utf8String),
}

// Constructors
impl EncodedString {
    pub fn new(buf: Vec<u8>, encoding: Encoding) -> Self {
        match encoding {
            Encoding::Ascii => Self::Ascii(AsciiString::new(buf)),
            Encoding::Binary => Self::Binary(BinaryString::new(buf)),
            Encoding::Utf8 => Self::Utf8(Utf8String::new(buf)),
        }
    }
}

impl EncodedString {
    pub fn encoding(&self) -> Encoding {
        match self {
            EncodedString::Ascii(_) => Encoding::Ascii,
            EncodedString::Binary(_) => Encoding::Binary,
            EncodedString::Utf8(_) => Encoding::Utf8,
        }
    }
}

// Defer to Encoded Implementation
impl EncodedString {
    // TODO:
    //   Maybe we don't expost bstr, but rely on formatting instead?
    pub fn as_bstr(&self) -> &BStr {
        match self {
            EncodedString::Ascii(n) => n.as_bstr(),
            EncodedString::Binary(n) => n.as_bstr(),
            EncodedString::Utf8(n) => n.as_bstr(),
        }
    }

    pub fn as_vec(&self) -> &Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.as_vec(),
            EncodedString::Binary(n) => n.as_vec(),
            EncodedString::Utf8(n) => n.as_vec(),
        }
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.as_mut_vec(),
            EncodedString::Binary(n) => n.as_mut_vec(),
            EncodedString::Utf8(n) => n.as_mut_vec(),
        }
    }

    pub fn into_vec(self) -> Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.into_vec(),
            EncodedString::Binary(n) => n.into_vec(),
            EncodedString::Utf8(n) => n.into_vec(),
        }
    }

    pub fn into_iter(self) -> IntoIter {
        match self {
            EncodedString::Ascii(n) => n.into_iter(),
            EncodedString::Binary(n) => n.into_iter(),
            EncodedString::Utf8(n) => n.into_iter(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(n) => n.as_slice(),
            EncodedString::Binary(n) => n.as_slice(),
            EncodedString::Utf8(n) => n.as_slice(),
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(n) => n.as_mut_slice(),
            EncodedString::Binary(n) => n.as_mut_slice(),
            EncodedString::Utf8(n) => n.as_mut_slice(),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        match self {
            EncodedString::Ascii(n) => n.as_ptr(),
            EncodedString::Binary(n) => n.as_ptr(),
            EncodedString::Utf8(n) => n.as_ptr(),
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        match self {
            EncodedString::Ascii(n) => n.as_mut_ptr(),
            EncodedString::Binary(n) => n.as_mut_ptr(),
            EncodedString::Utf8(n) => n.as_mut_ptr(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            EncodedString::Ascii(n) => n.len(),
            EncodedString::Binary(n) => n.len(),
            EncodedString::Utf8(n) => n.len(),
        }
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        match self {
            EncodedString::Ascii(n) => n.set_len(len),
            EncodedString::Binary(n) => n.set_len(len),
            EncodedString::Utf8(n) => n.set_len(len),
        }
    }

    pub fn capacity(&self) -> usize {
        match self {
            EncodedString::Ascii(n) => n.capacity(),
            EncodedString::Binary(n) => n.capacity(),
            EncodedString::Utf8(n) => n.capacity(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            EncodedString::Ascii(n) => n.clear(),
            EncodedString::Binary(n) => n.clear(),
            EncodedString::Utf8(n) => n.clear(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            EncodedString::Ascii(n) => n.is_empty(),
            EncodedString::Binary(n) => n.is_empty(),
            EncodedString::Utf8(n) => n.is_empty(),
        }
    }

    pub fn truncate(&mut self, len: usize) {
        match self {
            EncodedString::Ascii(n) => n.truncate(len),
            EncodedString::Binary(n) => n.truncate(len),
            EncodedString::Utf8(n) => n.truncate(len),
        };
    }

    pub fn char_len(&self) -> usize {
        match self {
            EncodedString::Ascii(n) => n.char_len(),
            EncodedString::Binary(n) => n.char_len(),
            EncodedString::Utf8(n) => n.char_len(),
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        match self {
            EncodedString::Ascii(n) => n.iter(),
            EncodedString::Binary(n) => n.iter(),
            EncodedString::Utf8(n) => n.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        match self {
            EncodedString::Ascii(n) => n.iter_mut(),
            EncodedString::Binary(n) => n.iter_mut(),
            EncodedString::Utf8(n) => n.iter_mut(),
        }
    }

    pub fn bytes(&self) -> Bytes<'_> {
        match self {
            EncodedString::Ascii(n) => n.bytes(),
            EncodedString::Binary(n) => n.bytes(),
            EncodedString::Utf8(n) => n.bytes(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        match self {
            EncodedString::Ascii(n) => n.reserve(additional),
            EncodedString::Binary(n) => n.reserve(additional),
            EncodedString::Utf8(n) => n.reserve(additional),
        }
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        match self {
            EncodedString::Ascii(n) => n.try_reserve(additional),
            EncodedString::Binary(n) => n.try_reserve(additional),
            EncodedString::Utf8(n) => n.try_reserve(additional),
        }
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        match self {
            EncodedString::Ascii(n) => n.reserve_exact(additional),
            EncodedString::Binary(n) => n.reserve_exact(additional),
            EncodedString::Utf8(n) => n.reserve_exact(additional),
        }
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        match self {
            EncodedString::Ascii(n) => n.try_reserve_exact(additional),
            EncodedString::Binary(n) => n.try_reserve_exact(additional),
            EncodedString::Utf8(n) => n.try_reserve_exact(additional),
        }
    }

    pub fn shrink_to_fit(&mut self) {
        match self {
            EncodedString::Ascii(n) => n.shrink_to_fit(),
            EncodedString::Binary(n) => n.shrink_to_fit(),
            EncodedString::Utf8(n) => n.shrink_to_fit(),
        }
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        match self {
            EncodedString::Ascii(n) => n.shrink_to(min_capacity),
            EncodedString::Binary(n) => n.shrink_to(min_capacity),
            EncodedString::Utf8(n) => n.shrink_to(min_capacity),
        }
    }

    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(n) => n.get(index),
            EncodedString::Binary(n) => n.get(index),
            EncodedString::Utf8(n) => n.get(index),
        }
    }

    pub fn get_char(&self, index: usize) -> Option<&'_ [u8]> {
        match self {
            EncodedString::Ascii(n) => n.get_char(index),
            EncodedString::Binary(n) => n.get_char(index),
            EncodedString::Utf8(n) => n.get_char(index),
        }
    }

    pub fn get_char_slice(&self, range: Range<usize>) -> Option<&'_ [u8]> {
        match self {
            EncodedString::Ascii(n) => n.get_char_slice(range),
            EncodedString::Binary(n) => n.get_char_slice(range),
            EncodedString::Utf8(n) => n.get_char_slice(range),
        }
    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(n) => n.get_mut(index),
            EncodedString::Binary(n) => n.get_mut(index),
            EncodedString::Utf8(n) => n.get_mut(index),
        }
    }

    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(n) => n.get_unchecked(index),
            EncodedString::Binary(n) => n.get_unchecked(index),
            EncodedString::Utf8(n) => n.get_unchecked(index),
        }
    }

    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(n) => n.get_unchecked_mut(index),
            EncodedString::Binary(n) => n.get_unchecked_mut(index),
            EncodedString::Utf8(n) => n.get_unchecked_mut(index),
        }
    }

    pub fn push_byte(&mut self, byte: u8) {
        match self {
            EncodedString::Ascii(n) => n.push_byte(byte),
            EncodedString::Binary(n) => n.push_byte(byte),
            EncodedString::Utf8(n) => n.push_byte(byte),
        }
    }

    pub fn try_push_codepoint(&mut self, codepoint: i64) -> Result<(), InvalidCodepointError> {
        match self {
            EncodedString::Ascii(n) => n.try_push_codepoint(codepoint),
            EncodedString::Binary(n) => n.try_push_codepoint(codepoint),
            EncodedString::Utf8(n) => n.try_push_codepoint(codepoint),
        }
    }

    pub fn push_char(&mut self, ch: char) {
        match self {
            EncodedString::Ascii(n) => n.push_char(ch),
            EncodedString::Binary(n) => n.push_char(ch),
            EncodedString::Utf8(n) => n.push_char(ch),
        }
    }

    pub fn push_str(&mut self, s: &str) {
        match self {
            EncodedString::Ascii(n) => n.push_str(s),
            EncodedString::Binary(n) => n.push_str(s),
            EncodedString::Utf8(n) => n.push_str(s),
        }
    }

    pub fn extend_from_slice(&mut self, other: &[u8]) {
        match self {
            EncodedString::Ascii(n) => n.extend_from_slice(other),
            EncodedString::Binary(n) => n.extend_from_slice(other),
            EncodedString::Utf8(n) => n.extend_from_slice(other),
        }
    }

    pub fn is_ascii_only(&self) -> bool {
        match self {
            EncodedString::Ascii(n) => n.is_ascii_only(),
            EncodedString::Binary(n) => n.is_ascii_only(),
            EncodedString::Utf8(n) => n.is_ascii_only(),
        }
    }

    pub fn is_valid_encoding(&self) -> bool {
        match self {
            EncodedString::Ascii(n) => n.is_valid_encoding(),
            EncodedString::Binary(n) => n.is_valid_encoding(),
            EncodedString::Utf8(n) => n.is_valid_encoding(),
        }
    }

    pub fn make_capitalized(&mut self) {
        match self {
            EncodedString::Ascii(n) => n.make_capitalized(),
            EncodedString::Binary(n) => n.make_capitalized(),
            EncodedString::Utf8(n) => n.make_capitalized(),
        }
    }

    pub fn make_uppercase(&mut self) {
        match self {
            EncodedString::Ascii(n) => n.make_uppercase(),
            EncodedString::Binary(n) => n.make_uppercase(),
            EncodedString::Utf8(n) => n.make_uppercase(),
        }
    }
    pub fn make_lowercase(&mut self) {
        match self {
            EncodedString::Ascii(n) => n.make_lowercase(),
            EncodedString::Binary(n) => n.make_lowercase(),
            EncodedString::Utf8(n) => n.make_lowercase(),
        }
    }

    pub fn chr(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(n) => n.chr(),
            EncodedString::Binary(n) => n.chr(),
            EncodedString::Utf8(n) => n.chr(),
        }
    }

    pub fn ord(&self) -> Result<u32, OrdError> {
        match self {
            EncodedString::Ascii(n) => n.ord(),
            EncodedString::Binary(n) => n.ord(),
            EncodedString::Utf8(n) => n.ord(),
        }
    }

    pub fn ends_with(&self, slice: &[u8]) -> bool {
        match self {
            EncodedString::Ascii(n) => n.ends_with(slice),
            EncodedString::Binary(n) => n.ends_with(slice),
            EncodedString::Utf8(n) => n.ends_with(slice),
        }
    }
}
