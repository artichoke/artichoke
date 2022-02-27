mod eq;
mod impls;

use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Range;
use core::slice::SliceIndex;

use bstr::{ByteSlice, ByteVec};

use crate::codepoints::InvalidCodepointError;
use crate::iter::{Bytes, IntoIter, Iter, IterMut};
use crate::ord::OrdError;

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Clone)]
pub struct Utf8String {
    inner: Vec<u8>,
}

impl Hash for Utf8String {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_slice().hash(hasher);
    }
}

impl PartialEq for Utf8String {
    fn eq(&self, other: &Self) -> bool {
        *self.inner.as_slice() == *other.inner.as_slice()
    }
}

impl Eq for Utf8String {}

// Constructors
impl Utf8String {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { inner: buf }
    }
}

impl fmt::Debug for Utf8String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Utf8String")
            .field("buf", &self.inner.as_bstr())
            .finish()
    }
}

// Raw
impl Utf8String {
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
impl Utf8String {
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.inner.iter())
    }

    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut(self.inner.iter_mut())
    }

    #[inline]
    #[must_use]
    pub fn bytes(&self) -> Bytes<'_> {
        Bytes(self.inner.iter())
    }

    #[inline]
    #[must_use]
    pub fn into_iter(self) -> IntoIter {
        IntoIter(self.inner.into_iter())
    }
}

// Size and Capacity
impl Utf8String {
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
impl Utf8String {
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
        // Fast path rejection for indexes beyond bytesize, which is
        // cheap to retrieve.
        if index >= self.len() {
            return None;
        }
        // Fast path for trying to treat the conventionally UTF-8 string
        // as entirely ASCII.
        //
        // If the string is either all ASCII or all ASCII for a prefix
        // of the string that contains the range we wish to slice,
        // fallback to byte slicing as in the ASCII and binary fast path.
        let consumed = match self.inner.find_non_ascii_byte() {
            None => return self.inner.get(index..=index),
            Some(idx) if idx > index => return self.inner.get(index..=index),
            Some(idx) => idx,
        };
        let mut slice = &self.inner[consumed..];
        // Count of "characters" remaining until the `index`th character.
        let mut remaining = index - consumed;
        // This loop will terminate when either:
        //
        // - It counts `index` number of characters.
        // - It consumes the entire slice when scanning for the
        //   `index`th character.
        //
        // The loop will advance by at least one byte every iteration.
        loop {
            match bstr::decode_utf8(slice) {
                // If we've run out of slice while trying to find the
                // `index`th character, the lookup fails and we return `nil`.
                (_, 0) => return None,

                // The next two arms mean we've reached the `index`th
                // character. Either return the next valid UTF-8
                // character byte slice or, if the next bytes are an
                // invalid UTF-8 sequence, the next byte.
                (Some(_), size) if remaining == 0 => return Some(&slice[..size]),
                // Size is guaranteed to be positive per the first arm
                // which means this slice operation will not panic.
                (None, _) if remaining == 0 => return Some(&slice[..1]),

                // We found a single UTF-8 encoded characterk keep track
                // of the count and advance the substring to continue
                // decoding.
                (Some(_), size) => {
                    slice = &slice[size..];
                    remaining -= 1;
                }

                // The next two arms handle the case where we have
                // encountered an invalid UTF-8 byte sequence.
                //
                // In this case, `decode_utf8` will return slices whose
                // length is `1..=3`. The length of this slice is the
                // number of "characters" we can advance the loop by.
                //
                // If the invalid UTF-8 sequence contains more bytes
                // than we have remaining to get to the `index`th char,
                // then the target character is inside the invalid UTF-8
                // sequence.
                (None, size) if remaining < size => return Some(&slice[remaining..=remaining]),
                // If there are more characters remaining than the number
                // of bytes yielded in the invalid UTF-8 byte sequence,
                // count `size` bytes and advance the slice to continue
                // decoding.
                (None, size) => {
                    slice = &slice[size..];
                    remaining -= size;
                }
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn get_char_slice(&self, range: Range<usize>) -> Option<&'_ [u8]> {
        let Range { start, end } = range;
        // Fast path for trying to treat the conventionally UTF-8 string
        // as entirely ASCII.
        //
        // If the string is either all ASCII or all ASCII for the subset
        // of the string we wish to slice, fallback to byte slicing as in
        // the ASCII and binary fast path.
        //
        // Perform the same saturate-to-end slicing mechanism if `end`
        // is beyond the character length of the string.
        let consumed = match self.inner.find_non_ascii_byte() {
            // The entire string is ASCII, so byte indexing <=> char
            // indexing.
            None => return self.inner.get(start..end).or_else(|| self.inner.get(start..)),
            // The whole substring we are interested in is ASCII, so
            // byte indexing is still valid.
            Some(non_ascii_byte_offset) if non_ascii_byte_offset > end => return self.get(start..end),
            // We turn non-ASCII somewhere inside before the substring
            // we're interested in, so consume that much.
            Some(non_ascii_byte_offset) if non_ascii_byte_offset <= start => non_ascii_byte_offset,
            // This means we turn non-ASCII somewhere inside the substring.
            // Consume up to start.
            Some(_) => start,
        };
        // Scan for the beginning of the slice
        let mut slice = &self.inner[consumed..];
        // Count of "characters" remaining until the `start`th character.
        let mut remaining = start - consumed;
        if remaining > 0 {
            // This loop will terminate when either:
            //
            // - It counts `start` number of characters.
            // - It consumes the entire slice when scanning for the
            //   `start`th character.
            //
            // The loop will advance by at least one byte every iteration.
            slice = loop {
                match bstr::decode_utf8(slice) {
                    // If we've run out of slice while trying to find the
                    // `start`th character, the lookup fails and we return `nil`.
                    (_, 0) => return None,

                    // We found a single UTF-8 encoded character. keep track
                    // of the count and advance the substring to continue
                    // decoding.
                    //
                    // If there's only one more to go, advance and stop the
                    // loop.
                    (Some(_), size) if remaining == 1 => break &slice[size..],
                    // Otherwise, keep track of the character we observed and
                    // advance the slice to continue decoding.
                    (Some(_), size) => {
                        slice = &slice[size..];
                        remaining -= 1;
                    }

                    // The next two arms handle the case where we have
                    // encountered an invalid UTF-8 byte sequence.
                    //
                    // In this case, `decode_utf8` will return slices whose
                    // length is `1..=3`. The length of this slice is the
                    // number of "characters" we can advance the loop by.
                    //
                    // If the invalid UTF-8 sequence contains more bytes
                    // than we have remaining to get to the `start`th char,
                    // then we can break the loop directly.
                    (None, size) if remaining <= size => break &slice[remaining..],
                    // If there are more characters remaining than the number
                    // of bytes yielded in the invalid UTF-8 byte sequence,
                    // count `size` bytes and advance the slice to continue
                    // decoding.
                    (None, size) => {
                        slice = &slice[size..];
                        remaining -= size;
                    }
                }
            }
        };

        // Scan the slice for the span of characters we want to return.
        remaining = end - start;
        // We know `remaining` is not zero because we fast-pathed that
        // case above.
        debug_assert!(remaining > 0);

        // keep track of the start of the substring from the `start`th
        // character.
        let substr = slice;

        // This loop will terminate when either:
        //
        // - It counts the next `start - end` number of characters.
        // - It consumes the entire slice when scanning for the `end`th
        //   character.
        //
        // The loop will advance by at least one byte every iteration.
        loop {
            match bstr::decode_utf8(slice) {
                // If we've run out of slice while trying to find the `end`th
                // character, saturate the slice to the end of the string.
                (_, 0) => return Some(substr),

                // We found a single UTF-8 encoded character. keep track
                // of the count and advance the substring to continue
                // decoding.
                //
                // If there's only one more to go, advance and stop the
                // loop.
                (Some(_), size) if remaining == 1 => {
                    // Push `endth` more positive because this match has
                    // the effect of shrinking `slice`.
                    let endth = substr.len() - slice.len() + size;
                    return Some(&substr[..endth]);
                }
                // Otherwise, keep track of the character we observed and
                // advance the slice to continue decoding.
                (Some(_), size) => {
                    slice = &slice[size..];
                    remaining -= 1;
                }

                // The next two arms handle the case where we have
                // encountered an invalid UTF-8 byte sequence.
                //
                // In this case, `decode_utf8` will return slices whose
                // length is `1..=3`. The length of this slice is the
                // number of "characters" we can advance the loop by.
                //
                // If the invalid UTF-8 sequence contains more bytes
                // than we have remaining to get to the `end`th char,
                // then we can break the loop directly.
                (None, size) if remaining <= size => {
                    // For an explanation of this arithmetic:
                    // If we're trying to slice:
                    //
                    // ```
                    // s = "a\xF0\x9F\x87"
                    // s[0, 2]
                    // ```
                    //
                    // By the time we get to this branch in this loop:
                    //
                    // ```
                    // substr = "a\xF0\x9F\x87"
                    // slice = "\xF0\x9F\x87"
                    // remaining = 1
                    // ```
                    //
                    // We want to compute `endth == 2`:
                    //
                    //    2   =      4       -      3      +     1
                    let endth = substr.len() - slice.len() + remaining;
                    return Some(&substr[..endth]);
                }
                // If there are more characters remaining than the number
                // of bytes yielded in the invalid UTF-8 byte sequence,
                // count `size` bytes and advance the slice to continue
                // decoding.
                (None, size) => {
                    slice = &slice[size..];
                    remaining -= size;
                }
            }
        }
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
impl Utf8String {
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    #[inline]
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
impl Utf8String {
    #[inline]
    #[must_use]
    pub fn is_ascii_only(&self) -> bool {
        self.inner.is_ascii()
    }

    #[inline]
    #[must_use]
    pub fn is_valid_encoding(&self) -> bool {
        if self.is_ascii_only() {
            return true;
        }

        simdutf8::basic::from_utf8(&self.inner).is_ok()
    }
}

// Casing
impl Utf8String {
    #[inline]
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

    #[inline]
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

    #[inline]
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
    #[inline]
    #[must_use]
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

    #[inline]
    #[must_use]
    pub fn ends_with(&self, slice: &[u8]) -> bool {
        self.inner.ends_with(slice)
    }
}

#[allow(clippy::invisible_characters)]
#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;
    use core::str;

    use bstr::ByteSlice;
    use quickcheck::quickcheck;

    use super::Utf8String;

    const REPLACEMENT_CHARACTER_BYTES: [u8; 3] = [239, 191, 189];

    quickcheck! {
        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_char_len_utf8_contents_utf8_string(contents: String) -> bool {
            let expected = contents.chars().count();
            let s = Utf8String::new(contents.into_bytes());
            s.char_len() == expected
        }

        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_len_utf8_contents_utf8_string(contents: String) -> bool {
            let expected = contents.len();
            let s = Utf8String::new(contents.into_bytes());
            s.len() == expected
        }

        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_char_len_binary_contents_utf8_string(contents: Vec<u8>) -> bool {
            if let Ok(utf8_contents) = str::from_utf8(&contents) {
                let expected = utf8_contents.chars().count();
                let s = Utf8String::new(contents);
                s.char_len() == expected
            } else {
                let expected_at_most = contents.len();
                let s = Utf8String::new(contents);
                s.char_len() <= expected_at_most
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_len_binary_contents_utf8_string(contents: Vec<u8>) -> bool {
            let expected = contents.len();
            let s = Utf8String::new(contents);
            s.len() == expected
        }
    }

    #[test]
    fn constructs_empty_buffer() {
        let s = Utf8String::new(Vec::new());
        assert_eq!(0, s.len());
    }

    #[test]
    fn char_len_empty() {
        let s = Utf8String::from("");
        assert_eq!(s.char_len(), 0);
    }

    #[test]
    fn char_len_ascii() {
        let s = Utf8String::from("Artichoke Ruby");
        assert_eq!(s.char_len(), 14);
    }

    #[test]
    fn char_len_emoji() {
        let s = Utf8String::from("💎");
        assert_eq!(s.char_len(), 1);
        let s = Utf8String::from("💎🦀🎉");
        assert_eq!(s.char_len(), 3);
        let s = Utf8String::from("a💎b🦀c🎉d");
        assert_eq!(s.char_len(), 7);
        // with invalid UTF-8 bytes
        let s = Utf8String::new(b"a\xF0\x9F\x92\x8E\xFFabc".to_vec());
        assert_eq!(s.char_len(), 6);
    }

    #[test]
    fn char_len_unicode_replacement_character() {
        let s = Utf8String::from("�");
        assert_eq!(s.char_len(), 1);
        let s = Utf8String::from("���");
        assert_eq!(s.char_len(), 3);
        let s = Utf8String::from("a�b�c�d");
        assert_eq!(s.char_len(), 7);
        let s = Utf8String::from("�💎b🦀c🎉�");
        assert_eq!(s.char_len(), 7);
        // with invalid UFF-8 bytes
        let s = Utf8String::new(b"\xEF\xBF\xBD\xF0\x9F\x92\x8E\xFF\xEF\xBF\xBDab".to_vec());
        assert_eq!(s.char_len(), 6);
        let s = Utf8String::new(REPLACEMENT_CHARACTER_BYTES.to_vec());
        assert_eq!(s.char_len(), 1);
    }

    #[test]
    fn char_len_nul_byte() {
        let s = Utf8String::from(b"\x00".as_bytes());
        assert_eq!(s.char_len(), 1);
        let s = Utf8String::from(b"abc\x00".as_bytes());
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from(b"abc\x00xyz".as_bytes());
        assert_eq!(s.char_len(), 7);
    }

    #[test]
    fn char_len_invalid_utf8_byte_sequences() {
        let s = Utf8String::from(b"\x00\x00\xD8\x00".as_bytes());
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from(b"\xFF\xFE".as_bytes());
        assert_eq!(s.char_len(), 2);
    }

    #[test]
    fn char_len_binary() {
        let bytes = &[
            0xB3, 0x7E, 0x39, 0x70, 0x8E, 0xFD, 0xBB, 0x75, 0x62, 0x77, 0xE7, 0xDF, 0x6F, 0xF2, 0x76, 0x27, 0x81,
            0x9A, 0x3A, 0x9D, 0xED, 0x6B, 0x4F, 0xAE, 0xC4, 0xE7, 0xA1, 0x66, 0x11, 0xF1, 0x08, 0x1C,
        ];
        let s = Utf8String::from(bytes.as_bytes());
        assert_eq!(s.char_len(), 32);
        // Mixed binary and ASCII
        let bytes = &[
            b'?', b'!', b'a', b'b', b'c', 0xFD, 0xBB, 0x75, 0x62, 0x77, 0xE7, 0xDF, 0x6F, 0xF2, 0x76, 0x27, 0x81,
            0x9A, 0x3A, 0x9D, 0xED, 0x6B, 0x4F, 0xAE, 0xC4, 0xE7, 0xA1, 0x66, 0x11, 0xF1, 0x08, 0x1C,
        ];
        let s = Utf8String::from(bytes.as_bytes());
        assert_eq!(s.char_len(), 32);
    }

    #[test]
    fn char_len_mixed_ascii_emoji_invalid_bytes() {
        // ```
        // [2.6.3] > s = "🦀abc💎\xff"
        // => "🦀abc💎\xFF"
        // [2.6.3] > s.length
        // => 6
        // [2.6.3] > puts s.bytes.map{|b| "\\x#{b.to_s(16).upcase}"}.join
        // \xF0\x9F\xA6\x80\x61\x62\x63\xF0\x9F\x92\x8E\xFF
        // ```
        let s = Utf8String::from(b"\xF0\x9F\xA6\x80\x61\x62\x63\xF0\x9F\x92\x8E\xFF".as_bytes());
        assert_eq!(s.char_len(), 6);
    }

    #[test]
    fn char_len_utf8() {
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L147-L157
        let s = Utf8String::from("Ω≈ç√∫˜µ≤≥÷");
        assert_eq!(s.char_len(), 10);
        let s = Utf8String::from("åß∂ƒ©˙∆˚¬…æ");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("œ∑´®†¥¨ˆøπ“‘");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("¡™£¢∞§¶•ªº–≠");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("¸˛Ç◊ı˜Â¯˘¿");
        assert_eq!(s.char_len(), 10);
        let s = Utf8String::from("ÅÍÎÏ˝ÓÔÒÚÆ☃");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("Œ„´‰ˇÁ¨ˆØ∏”’");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("`⁄€‹›ﬁﬂ‡°·‚—±");
        assert_eq!(s.char_len(), 13);
        let s = Utf8String::from("⅛⅜⅝⅞");
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from("ЁЂЃЄЅІЇЈЉЊЋЌЍЎЏАБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдежзийклмнопрстуфхцчшщъыьэюя");
        assert_eq!(s.char_len(), 79);
    }

    #[test]
    fn char_len_vmware_super_string() {
        // A super string recommended by VMware Inc. Globalization Team: can
        // effectively cause rendering issues or character-length issues to
        // validate product globalization readiness.
        //
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L202-L224
        let s = Utf8String::from("表ポあA鷗ŒéＢ逍Üßªąñ丂㐀𠀀");
        assert_eq!(s.char_len(), 17);
    }

    #[test]
    fn char_len_two_byte_chars() {
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L188-L196
        let s = Utf8String::from("田中さんにあげて下さい");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("パーティーへ行かないか");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("和製漢語");
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from("部落格");
        assert_eq!(s.char_len(), 3);
        let s = Utf8String::from("사회과학원 어학연구소");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("찦차를 타고 온 펲시맨과 쑛다리 똠방각하");
        assert_eq!(s.char_len(), 22);
        let s = Utf8String::from("社會科學院語學研究所");
        assert_eq!(s.char_len(), 10);
        let s = Utf8String::from("울란바토르");
        assert_eq!(s.char_len(), 5);
        let s = Utf8String::from("𠜎𠜱𠝹𠱓𠱸𠲖𠳏");
        assert_eq!(s.char_len(), 7);
    }

    #[test]
    fn char_len_space_chars() {
        // Whitespace: all the characters with category Zs, Zl, or Zp (in Unicode
        // version 8.0.0), plus U+0009 (HT), U+000B (VT), U+000C (FF), U+0085 (NEL),
        // and U+200B (ZERO WIDTH SPACE), which are in the C categories but are often
        // treated as whitespace in some contexts.
        // This file unfortunately cannot express strings containing
        // U+0000, U+000A, or U+000D (NUL, LF, CR).
        // The next line may appear to be blank or mojibake in some viewers.
        // The next line may be flagged for "trailing whitespace" in some viewers.
        //
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L131
        let bytes = "	              ​    　
";
        let s = Utf8String::from(bytes);
        assert_eq!(s.char_len(), 25);
    }

    #[test]
    fn casing_utf8_string_empty() {
        let mut s = Utf8String::new(b"".to_vec());

        s.make_capitalized();
        assert_eq!(s, "");

        s.make_lowercase();
        assert_eq!(s, "");

        s.make_uppercase();
        assert_eq!(s, "");
    }

    #[test]
    fn casing_utf8_string_ascii() {
        let lower = Utf8String::new(b"abc".to_vec());
        let mid_upper = Utf8String::new(b"aBc".to_vec());
        let upper = Utf8String::new(b"ABC".to_vec());
        let long = Utf8String::new(b"aBC, 123, ABC, baby you and me girl".to_vec());

        let capitalize: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
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
    fn casing_utf8_string_utf8() {
        // Capitalization of ß (SS) differs from MRI:
        //
        // ```console
        // [2.6.3] > "ß".capitalize
        // => "Ss"
        // ```
        let sharp_s = Utf8String::from("ß");
        let tomorrow = Utf8String::from("αύριο");
        let year = Utf8String::from("έτος");
        // two-byte characters
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L198-L200
        let two_byte_chars = Utf8String::from("𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆");
        // Changes length when case changes
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L226-L232
        let varying_length = Utf8String::from("zȺȾ");
        // There doesn't appear to be any RTL scripts that have cases, but might aswell make sure
        let rtl = Utf8String::from("مرحبا الخرشوف");

        let capitalize: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_uppercase();
            value
        };

        assert_eq!(capitalize(&sharp_s), "SS");
        assert_eq!(capitalize(&tomorrow), "Αύριο");
        assert_eq!(capitalize(&year), "Έτος");
        assert_eq!(
            capitalize(&two_byte_chars),
            "𐐜 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐑁𐐲𐑉𐑅𐐻/𐑅𐐯𐐿𐐲𐑌𐐼 𐐺𐐳𐐿 𐐺𐐴 𐑄 𐑉𐐨𐐾𐐯𐑌𐐻𐑅 𐐱𐑂 𐑄 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐐷𐐮𐐭𐑌𐐮𐑂𐐲𐑉𐑅𐐮𐐻𐐮"
        );
        assert_eq!(capitalize(&varying_length), "Zⱥⱦ");
        assert_eq!(capitalize(&rtl), "مرحبا الخرشوف");

        assert_eq!(lowercase(&sharp_s), "ß");
        assert_eq!(lowercase(&tomorrow), "αύριο");
        assert_eq!(lowercase(&year), "έτος");
        assert_eq!(
            lowercase(&two_byte_chars),
            "𐑄 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐑁𐐲𐑉𐑅𐐻/𐑅𐐯𐐿𐐲𐑌𐐼 𐐺𐐳𐐿 𐐺𐐴 𐑄 𐑉𐐨𐐾𐐯𐑌𐐻𐑅 𐐱𐑂 𐑄 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐐷𐐮𐐭𐑌𐐮𐑂𐐲𐑉𐑅𐐮𐐻𐐮"
        );
        assert_eq!(lowercase(&varying_length), "zⱥⱦ");
        assert_eq!(lowercase(&rtl), "مرحبا الخرشوف");

        assert_eq!(uppercase(&sharp_s), "SS");
        assert_eq!(uppercase(&tomorrow), "ΑΎΡΙΟ");
        assert_eq!(uppercase(&year), "ΈΤΟΣ");
        assert_eq!(
            uppercase(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐉𐐚 𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(uppercase(&varying_length), "ZȺȾ");
        assert_eq!(uppercase(&rtl), "مرحبا الخرشوف");
    }

    #[test]
    fn casing_utf8_string_invalid_utf8() {
        let mut s = Utf8String::new(b"\xFF\xFE".to_vec());

        s.make_capitalized();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_lowercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_uppercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);
    }

    #[test]
    fn casing_utf8_string_unicode_replacement_character() {
        let mut s = Utf8String::from("�");

        s.make_capitalized();
        assert_eq!(s, "�");

        s.make_lowercase();
        assert_eq!(s, "�");

        s.make_uppercase();
        assert_eq!(s, "�");
    }

    #[test]
    fn chr_does_not_return_more_than_one_byte_for_invalid_utf8() {
        // ```ruby
        // [3.0.1] > "\xF0\x9F\x87".chr
        // => "\xF0"
        // ```
        //
        // Per `bstr`:
        //
        // The bytes `\xF0\x9F\x87` could lead to a valid UTF-8 sequence, but 3 of them
        // on their own are invalid. Only one replacement codepoint is substituted,
        // which demonstrates the "substitution of maximal subparts" strategy.
        let s = Utf8String::new(b"\xF0\x9F\x87".to_vec());
        assert_eq!(s.chr(), b"\xF0");
    }
}
