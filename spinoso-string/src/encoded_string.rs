use alloc::vec::Vec;
use bstr::{BStr};
use core::slice::SliceIndex;

use crate::encoding::Encoding;
use crate::ascii_string::AsciiString;
use crate::binary_string::BinaryString;
use crate::utf8_string::Utf8String;
use crate::iter::{IntoIter, Iter, IterMut, Bytes};


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
}

// Migration functions
// TODO: Remove these. If it compiles, we've migrated successfully
impl EncodedString {
    pub fn buf(&self) -> &Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.buf(),
            EncodedString::Binary(n) => n.buf(),
            EncodedString::Utf8(n) => n.buf(),
        }
    }

    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        match self {
            EncodedString::Ascii(n) => n.buf_mut(),
            EncodedString::Binary(n) => n.buf_mut(),
            EncodedString::Utf8(n) => n.buf_mut(),
        }
    }
}

