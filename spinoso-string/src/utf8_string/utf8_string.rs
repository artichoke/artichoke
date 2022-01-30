use alloc::vec::Vec;
use core::slice::SliceIndex;

use bstr::{BStr, ByteSlice, ByteVec};

use crate::codepoints::InvalidCodepointError;
use crate::iter::{Bytes, IntoIter, Iter, IterMut};
use crate::ord::OrdError;

#[derive(Default, Clone)]
pub struct Utf8String {
    inner: Vec<u8>,
}

// Constructors
impl Utf8String {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { inner: buf }
    }
}

// Debug
impl Utf8String {
    pub fn as_bstr(&self) -> &BStr {
        self.inner.as_bstr()
    }
}

// Raw
impl Utf8String {
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.inner
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

// Core Iterators
impl Utf8String {
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.inner.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut(self.inner.iter_mut())
    }

    pub fn bytes(&self) -> Bytes<'_> {
        Bytes(self.inner.iter())
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter(self.inner.into_iter())
    }
}

// Size and Capacity
impl Utf8String {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.inner.set_len(len);
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    pub fn char_len(&self) -> usize {
        let mut bytes = self.as_slice();
        let tail = if let Some(idx) = bytes.find_non_ascii_byte() {
            idx
        } else {
            return bytes.len();
        };
        // Safety:
        //
        // If `ByteSlice::find_non_ascii_byte` returns `Some(_)`, the index is
        // guaranteed to be a valid index within `bytes`.
        bytes = unsafe { bytes.get_unchecked(tail..) };
        if simdutf8::basic::from_utf8(bytes).is_ok() {
            return tail + bytecount::num_chars(bytes);
        }
        let mut char_len = tail;
        for chunk in bytes.utf8_chunks() {
            char_len += bytecount::num_chars(chunk.valid().as_bytes());
            char_len += chunk.invalid().len();
        }
        char_len
    }
}

// Memory management
impl Utf8String {
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

// Indexing
impl Utf8String {
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get(index)
    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_mut(index)
    }

    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked_mut(index)
    }
}

// Pushing and popping bytes, codepoints, and strings.
impl Utf8String {
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    pub fn try_push_codepoint(&mut self, codepoint: i64) -> Result<(), InvalidCodepointError> {
        let codepoint = if let Ok(codepoint) = u32::try_from(codepoint) {
            codepoint
        } else {
            return Err(InvalidCodepointError::codepoint_out_of_range(codepoint));
        };
        if let Ok(ch) = char::try_from(codepoint) {
            self.push_char(ch);
            Ok(())
        } else {
            Err(InvalidCodepointError::invalid_utf8_codepoint(codepoint))
        }
    }

    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
    }

    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
    }
}

impl Utf8String {
    pub fn is_ascii_only(&self) -> bool {
        self.inner.is_ascii()
    }
}

// Casing
impl Utf8String {
    pub fn make_capitalized(&mut self) {
        // This allocation assumes that in the common case, capitalizing
        // and lower-casing `char`s do not change the length of the
        // `String`.
        let mut replacement = Vec::with_capacity(self.len());
        let mut bytes = self.inner.as_slice();

        match bstr::decode_utf8(bytes) {
            (Some(ch), size) => {
                // Converting a UTF-8 character to uppercase may yield
                // multiple codepoints.
                for ch in ch.to_uppercase() {
                    replacement.push_char(ch);
                }
                bytes = &bytes[size..];
            }
            (None, size) if size == 0 => return,
            (None, size) => {
                let (substring, remainder) = bytes.split_at(size);
                replacement.extend_from_slice(substring);
                bytes = remainder;
            }
        }

        while !bytes.is_empty() {
            let (ch, size) = bstr::decode_utf8(bytes);
            if let Some(ch) = ch {
                // Converting a UTF-8 character to lowercase may yield
                // multiple codepoints.
                for ch in ch.to_lowercase() {
                    replacement.push_char(ch);
                }
                bytes = &bytes[size..];
            } else {
                let (substring, remainder) = bytes.split_at(size);
                replacement.extend_from_slice(substring);
                bytes = remainder;
            }
        }
        self.inner = replacement;
    }

    pub fn make_lowercase(&mut self) {
        // This allocation assumes that in the common case, lower-casing
        // `char`s do not change the length of the `String`.
        let mut replacement = Vec::with_capacity(self.len());
        let mut bytes = self.inner.as_slice();

        while !bytes.is_empty() {
            let (ch, size) = bstr::decode_utf8(bytes);
            if let Some(ch) = ch {
                // Converting a UTF-8 character to lowercase may yield
                // multiple codepoints.
                for ch in ch.to_lowercase() {
                    replacement.push_char(ch);
                }
                bytes = &bytes[size..];
            } else {
                let (substring, remainder) = bytes.split_at(size);
                replacement.extend_from_slice(substring);
                bytes = remainder;
            }
        }
        self.inner = replacement;
    }

    pub fn make_uppercase(&mut self) {
        // This allocation assumes that in the common case, upper-casing
        // `char`s do not change the length of the `String`.
        let mut replacement = Vec::with_capacity(self.len());
        let mut bytes = self.inner.as_slice();

        while !bytes.is_empty() {
            let (ch, size) = bstr::decode_utf8(bytes);
            if let Some(ch) = ch {
                // Converting a UTF-8 character to lowercase may yield
                // multiple codepoints.
                for ch in ch.to_uppercase() {
                    replacement.push_char(ch);
                }
                bytes = &bytes[size..];
            } else {
                let (substring, remainder) = bytes.split_at(size);
                replacement.extend_from_slice(substring);
                bytes = remainder;
            }
        }
        self.inner = replacement;
    }
}

impl Utf8String {
    pub fn chr(&self) -> &[u8] {
        match bstr::decode_utf8(self.inner.as_slice()) {
            (Some(_), size) => &self.inner[..size],
            (None, 0) => &[],
            (None, _) => &self.inner[..1],
        }
    }

    #[inline]
    pub fn ord(&self) -> Result<u32, OrdError> {
        let (ch, size) = bstr::decode_utf8(self.inner.as_slice());
        match ch {
            // All `char`s are valid `u32`s
            // https://github.com/rust-lang/rust/blob/1.48.0/library/core/src/char/convert.rs#L12-L20
            Some(ch) => Ok(u32::from(ch)),
            None if size == 0 => Err(OrdError::empty_string()),
            None => Err(OrdError::invalid_utf8_byte_sequence()),
        }
    }
}

// Migration functions
// TODO: Remove these. If it compiles, we've migrated successfully
impl Utf8String {
    pub fn buf(&self) -> &Vec<u8> {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::binary_string::BinaryString;

    #[test]
    fn constructs_empty_buffer() {
        let s = BinaryString::new(Vec::new());
        assert_eq!(0, s.len());
    }
}
