use alloc::collections::TryReserveError;
use alloc::vec::Vec;

use scolapasta_strbuf::Buf;

use super::Utf8Str;
use crate::chars::ConventionallyUtf8;
use crate::codepoints::InvalidCodepointError;
use crate::iter::IntoIter;

mod eq;
mod impls;
#[cfg(feature = "std")]
mod io;

#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord)]
pub struct Utf8String {
    inner: Buf,
}

// Constructors
impl Utf8String {
    #[inline]
    pub const fn new(buf: Buf) -> Self {
        Self { inner: buf }
    }

    #[inline]
    pub fn empty() -> Self {
        Self { inner: Buf::new() }
    }
}

// Raw
impl Utf8String {
    #[inline]
    #[must_use]
    pub(crate) fn into_buf(self) -> Buf {
        self.inner
    }

    #[inline]
    #[must_use]
    pub fn as_utf8_str(&self) -> &Utf8Str {
        Utf8Str::from_bytes(self.inner.as_slice())
    }

    #[inline]
    #[must_use]
    pub fn as_mut_utf8_str(&mut self) -> &mut Utf8Str {
        Utf8Str::from_bytes_mut(self.inner.as_mut_slice())
    }
}

// Core Iterators
impl Utf8String {
    #[inline]
    #[must_use]
    pub fn into_iter(self) -> IntoIter {
        IntoIter::from_vec(self.inner.into_inner())
    }
}

// Size and Capacity
impl Utf8String {
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
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
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
    pub fn try_push_int(&mut self, int: i64) -> Result<(), InvalidCodepointError> {
        self.try_push_codepoint(int)
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

// Casing
impl Utf8String {
    // TODO: Use roe for case changing operations. UTF-8 case changing needs to
    //       be parameterized on the case folding strategy to account for e.g.
    //       Turkic or ASCII-only modes
    #[inline]
    pub fn make_capitalized(&mut self) {
        use bstr::ByteVec;

        // This allocation assumes that in the common case, capitalizing
        // and lower-casing `char`s do not change the length of the
        // `String`.
        //
        // Use a `Vec` here instead of a `Buf` to ensure at most one alloc
        // fix-up happens instead of alloc fix-ups being O(chars).
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
        self.inner = replacement.into();
    }

    #[inline]
    pub fn make_lowercase(&mut self) {
        use bstr::ByteVec;

        // This allocation assumes that in the common case, lower-casing
        // `char`s do not change the length of the `String`.
        //
        // Use a `Vec` here instead of a `Buf` to ensure at most one alloc
        // fix-up happens instead of alloc fix-ups being O(chars).
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
        self.inner = replacement.into();
    }

    #[inline]
    pub fn make_uppercase(&mut self) {
        use bstr::ByteVec;

        // This allocation assumes that in the common case, upper-casing
        // `char`s do not change the length of the `String`.
        //
        // Use a `Vec` here instead of a `Buf` to ensure at most one alloc
        // fix-up happens instead of alloc fix-ups being O(chars).
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
        self.inner = replacement.into();
    }
}

// Reversing
impl Utf8String {
    #[inline]
    pub fn reverse(&mut self) {
        // Fast path when all characters are one byte wide.
        if self.is_ascii_only() {
            self.inner.reverse();
            return;
        }
        // FIXME: this allocation can go away if `ConventionallyUtf8` impls
        // `DoubleEndedIterator`.
        let chars = ConventionallyUtf8::from(&self.inner[..]).collect::<Vec<_>>();
        // Use a `Vec` here instead of a `Buf` to ensure at most one alloc
        // fix-up happens instead of alloc fix-ups being O(chars).
        let mut replacement = Vec::with_capacity(self.inner.len());
        for &bytes in chars.iter().rev() {
            replacement.extend_from_slice(bytes);
        }
        self.inner = replacement.into();
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::Utf8String;

    #[test]
    fn reverse_ascii() {
        let mut s = Utf8String::from("1234");
        s.reverse();
        assert_eq!(s, "4321")
    }

    #[test]
    fn reverse_ascii_with_invalid_utf8() {
        let mut s = Utf8String::from(b"1234\xFF\xFE");
        s.reverse();
        assert_eq!(s, b"\xFE\xFF4321".as_bstr())
    }

    #[test]
    fn reverse_multibyte() {
        // ```console
        // [3.2.2] > "怎么样".reverse
        // => "样么怎"
        // ```
        let mut s = Utf8String::from("怎么样");
        s.reverse();
        assert_eq!(s, "样么怎")
    }

    #[test]
    fn reverse_multibyte_with_invalid_utf8() {
        // ```console
        // [3.2.2] > "怎么样\xFF\xFE".reverse
        // => => "\xFE\xFF样么怎"
        // ```
        let mut s = Utf8String::from("怎么样");
        s.extend_from_slice(b"\xFF\xFE");
        s.reverse();

        let mut expected = b"\xFE\xFF".to_vec();
        expected.extend_from_slice("样么怎".as_bytes());
        assert_eq!(s, expected.as_bstr())
    }

    #[test]
    fn reverse_replacement_char_with_invalid_utf8_prefix() {
        // the Unicode replacement char has the following byte contents:
        //
        // ```console
        // [3.2.2] > puts "�".b.inspect
        // "\xEF\xBF\xBD"
        // ```
        //
        // `\xF0\x9F\x87` is a valid UTF-8 prefix for a 4 byte sequence but is
        // not itself a valid byte sequence. We expect these 3 bytes to be
        // treated as 3 characters.
        let mut s = Utf8String::from(b"abc\xF0\x9F\x87def\xEF\xBF\xBD");
        s.reverse();
        assert_eq!(s, b"\xEF\xBF\xBDfed\x87\x9F\xF0cba".as_bstr());
    }
}
