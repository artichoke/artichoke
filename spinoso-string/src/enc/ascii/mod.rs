use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::fmt;
use core::ops::Range;
use core::slice::SliceIndex;

use bstr::{ByteSlice, ByteVec};

use crate::codepoints::InvalidCodepointError;
use crate::iter::{Bytes, IntoIter, Iter, IterMut};
use crate::ord::OrdError;

mod eq;
mod impls;
mod inspect;
#[cfg(feature = "std")]
mod io;

pub use inspect::Inspect;

#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsciiString {
    inner: Vec<u8>,
}

// Constructors
impl AsciiString {
    pub const fn new(buf: Vec<u8>) -> Self {
        Self { inner: buf }
    }
}

impl fmt::Debug for AsciiString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsciiString")
            .field("buf", &self.inner.as_bstr())
            .finish()
    }
}

// Raw Accessors
impl AsciiString {
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.inner
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

// Core Iterators
impl AsciiString {
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        Iter::from_slice(&self.inner)
    }

    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut::from_mut_slice(&mut self.inner)
    }

    #[inline]
    #[must_use]
    pub fn bytes(&self) -> Bytes<'_> {
        Bytes::from_slice(&self.inner)
    }

    #[inline]
    #[must_use]
    pub fn into_iter(self) -> IntoIter {
        IntoIter::from_vec(self.inner)
    }
}

// Size and Capacity
impl AsciiString {
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        self.inner.set_len(len);
    }

    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[inline]
    #[must_use]
    pub fn char_len(&self) -> usize {
        self.len()
    }
}

// Memory management
impl AsciiString {
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

// Indexing
impl AsciiString {
    #[inline]
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_char(&self, index: usize) -> Option<&'_ [u8]> {
        self.get(index..=index)
    }

    #[inline]
    #[must_use]
    pub fn get_char_slice(&self, range: Range<usize>) -> Option<&'_ [u8]> {
        let Range { start, end } = range;

        self.inner.get(start..end).or_else(|| self.inner.get(start..))
    }

    #[inline]
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_mut(index)
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked(index)
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked_mut(index)
    }
}

// Pushing and popping bytes, codepoints, and strings.
impl AsciiString {
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    #[inline]
    pub fn try_push_codepoint(&mut self, codepoint: i64) -> Result<(), InvalidCodepointError> {
        if let Ok(byte) = u8::try_from(codepoint) {
            self.push_byte(byte);
            Ok(())
        } else {
            Err(InvalidCodepointError::codepoint_out_of_range(codepoint))
        }
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
    }
}

// Encoding
impl AsciiString {
    #[inline]
    #[must_use]
    pub fn is_ascii_only(&self) -> bool {
        self.inner.is_ascii()
    }

    #[inline]
    #[must_use]
    pub fn is_valid_encoding(&self) -> bool {
        self.is_ascii_only()
    }
}

// Casing
impl AsciiString {
    #[inline]
    pub fn make_capitalized(&mut self) {
        if let Some((head, tail)) = self.inner.split_first_mut() {
            head.make_ascii_uppercase();
            tail.make_ascii_lowercase();
        }
    }

    #[inline]
    pub fn make_lowercase(&mut self) {
        self.inner.make_ascii_lowercase();
    }

    #[inline]
    pub fn make_uppercase(&mut self) {
        self.inner.make_ascii_uppercase();
    }
}

impl AsciiString {
    #[inline]
    #[must_use]
    pub fn chr(&self) -> &[u8] {
        self.inner.get(0..1).unwrap_or_default()
    }

    #[inline]
    pub fn ord(&self) -> Result<u32, OrdError> {
        let byte = self.inner.get(0).copied().ok_or_else(OrdError::empty_string)?;
        Ok(u32::from(byte))
    }

    #[inline]
    #[must_use]
    pub fn ends_with(&self, slice: &[u8]) -> bool {
        self.inner.ends_with(slice)
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.inner.reverse();
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    use quickcheck::quickcheck;

    use super::AsciiString;

    quickcheck! {
        fn fuzz_char_len_utf8_contents_ascii_string(contents: String) -> bool {
            let expected = contents.len();
            let s = AsciiString::new(contents.into_bytes());
            s.char_len() == expected
        }

        fn fuzz_len_utf8_contents_ascii_string(contents: String) -> bool {
            let expected = contents.len();
            let s = AsciiString::new(contents.into_bytes());
            s.len() == expected
        }

        fn fuzz_char_len_binary_contents_ascii_string(contents: Vec<u8>) -> bool {
            let expected = contents.len();
            let s = AsciiString::new(contents);
            s.char_len() == expected
        }

        fn fuzz_len_binary_contents_ascii_string(contents: Vec<u8>) -> bool {
            let expected = contents.len();
            let s = AsciiString::new(contents);
            s.len() == expected
        }
    }

    #[test]
    fn constructs_empty_buffer() {
        let s = AsciiString::new(Vec::new());
        assert_eq!(0, s.len());
    }

    #[test]
    fn casing_ascii_string_empty() {
        let mut s = AsciiString::new(b"".to_vec());

        s.make_capitalized();
        assert_eq!(s, "");

        s.make_lowercase();
        assert_eq!(s, "");

        s.make_uppercase();
        assert_eq!(s, "");
    }

    #[test]
    fn casing_ascii_string_ascii() {
        let lower = AsciiString::new(b"abc".to_vec());
        let mid_upper = AsciiString::new(b"aBc".to_vec());
        let upper = AsciiString::new(b"ABC".to_vec());
        let long = AsciiString::new(b"aBC, 123, ABC, baby you and me girl".to_vec());

        let capitalize: fn(&AsciiString) -> AsciiString = |value: &AsciiString| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&AsciiString) -> AsciiString = |value: &AsciiString| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&AsciiString) -> AsciiString = |value: &AsciiString| {
            let mut value = value.clone();
            value.make_uppercase();
            value
        };

        assert_eq!(capitalize(&lower), "Abc");
        assert_eq!(capitalize(&mid_upper), "Abc");
        assert_eq!(capitalize(&upper), "Abc");
        assert_eq!(capitalize(&long), "Abc, 123, abc, baby you and me girl");

        assert_eq!(lowercase(&lower), "abc");
        assert_eq!(lowercase(&mid_upper), "abc");
        assert_eq!(lowercase(&upper), "abc");
        assert_eq!(lowercase(&long), "abc, 123, abc, baby you and me girl");

        assert_eq!(uppercase(&lower), "ABC");
        assert_eq!(uppercase(&mid_upper), "ABC");
        assert_eq!(uppercase(&upper), "ABC");
        assert_eq!(uppercase(&long), "ABC, 123, ABC, BABY YOU AND ME GIRL");
    }

    #[test]
    fn casing_ascii_string_utf8() {
        let sharp_s = AsciiString::from("ß");
        let tomorrow = AsciiString::from("αύριο");
        let year = AsciiString::from("έτος");
        // two-byte characters
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L198-L200
        let two_byte_chars = AsciiString::from("𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆");
        // Changes length when case changes
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L226-L232
        let varying_length = AsciiString::from("zȺȾ");
        let rtl = AsciiString::from("مرحبا الخرشوف");

        let capitalize: fn(&AsciiString) -> AsciiString = |value: &AsciiString| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&AsciiString) -> AsciiString = |value: &AsciiString| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&AsciiString) -> AsciiString = |value: &AsciiString| {
            let mut value = value.clone();
            value.make_uppercase();
            value
        };

        assert_eq!(capitalize(&sharp_s), "ß");
        assert_eq!(capitalize(&tomorrow), "αύριο");
        assert_eq!(capitalize(&year), "έτος");
        assert_eq!(
            capitalize(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(capitalize(&varying_length), "ZȺȾ");
        assert_eq!(capitalize(&rtl), "مرحبا الخرشوف");

        assert_eq!(lowercase(&sharp_s), "ß");
        assert_eq!(lowercase(&tomorrow), "αύριο");
        assert_eq!(lowercase(&year), "έτος");
        assert_eq!(
            lowercase(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(lowercase(&varying_length), "zȺȾ");
        assert_eq!(lowercase(&rtl), "مرحبا الخرشوف");

        assert_eq!(uppercase(&sharp_s), "ß");
        assert_eq!(uppercase(&tomorrow), "αύριο");
        assert_eq!(uppercase(&year), "έτος");
        assert_eq!(
            uppercase(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(uppercase(&varying_length), "ZȺȾ");
        assert_eq!(uppercase(&rtl), "مرحبا الخرشوف");
    }

    #[test]
    fn casing_ascii_string_invalid_utf8() {
        let mut s = AsciiString::new(b"\xFF\xFE".to_vec());

        s.make_capitalized();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_lowercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_uppercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);
    }

    #[test]
    fn casing_ascii_string_unicode_replacement_character() {
        let mut s = AsciiString::from("�");

        s.make_capitalized();
        assert_eq!(s, "�");

        s.make_lowercase();
        assert_eq!(s, "�");

        s.make_uppercase();
        assert_eq!(s, "�");
    }
}
