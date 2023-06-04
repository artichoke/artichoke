use alloc::boxed::Box;
use core::ops::Range;
use core::slice::SliceIndex;

use bstr::ByteSlice;

use crate::iter::{Bytes, Iter, IterMut};
use crate::ord::OrdError;

mod eq;
mod impls;

#[repr(transparent)]
pub struct Utf8Str {
    bytes: [u8],
}

impl Utf8Str {
    #[inline]
    #[must_use]
    pub fn new<B: ?Sized + AsRef<[u8]>>(bytes: &B) -> &Utf8Str {
        Utf8Str::from_bytes(bytes.as_ref())
    }

    #[inline]
    #[must_use]
    pub(crate) fn new_mut<B: ?Sized + AsMut<[u8]>>(bytes: &mut B) -> &mut Utf8Str {
        Utf8Str::from_bytes_mut(bytes.as_mut())
    }

    #[inline]
    #[must_use]
    pub const fn empty() -> &'static Utf8Str {
        Utf8Str::from_bytes(b"")
    }

    #[inline]
    #[must_use]
    pub const fn from_bytes(slice: &[u8]) -> &Utf8Str {
        // SAFETY: `Utf8Str` is a `repr(transparent)` wrapper around `[u8]`.
        unsafe {
            let ptr: *const [u8] = slice;
            let ptr = ptr as *const Utf8Str;
            &*ptr
        }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes_mut(slice: &mut [u8]) -> &mut Utf8Str {
        // SAFETY: `Utf8Str` is a `repr(transparent)` wrapper around `[u8]`.
        unsafe {
            let ptr: *mut [u8] = slice;
            let ptr = ptr as *mut Utf8Str;
            &mut *ptr
        }
    }

    #[inline]
    pub fn from_boxed_bytes(slice: Box<[u8]>) -> Box<Utf8Str> {
        // SAFETY: `Utf8Str` is a `repr(transparent)` wrapper around `[u8]`.
        unsafe { Box::from_raw(Box::into_raw(slice) as _) }
    }

    #[inline]
    pub fn into_boxed_bytes(slice: Box<Utf8Str>) -> Box<[u8]> {
        // SAFETY: `Utf8Str` is a `repr(transparent)` wrapper around `[u8]`.
        unsafe { Box::from_raw(Box::into_raw(slice) as _) }
    }

    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    #[must_use]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

// Raw
impl Utf8Str {
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }

    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
}

// Core Iterators
impl Utf8Str {
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        Iter::from_slice(self.as_bytes())
    }

    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut::from_mut_slice(self.as_bytes_mut())
    }

    #[inline]
    #[must_use]
    pub fn bytes(&self) -> Bytes<'_> {
        Bytes::from_slice(self.as_bytes())
    }
}

// Size and Capacity
impl Utf8Str {
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }
}

// Character-oriented APIs
impl Utf8Str {
    #[must_use]
    pub fn chr(&self) -> &Utf8Str {
        let slice = self.as_bytes();
        let prefix = match bstr::decode_utf8(slice) {
            (Some(_), size) => size,
            (None, 0) => return Utf8Str::empty(),
            (None, _) => 1,
        };
        // SAFETY: the UTF-8 decode above guarantees the prefix len is a valid
        // slice index.
        let s = unsafe { self.get_unchecked(..prefix) };
        Utf8Str::from_bytes(s)
    }

    pub fn ord(&self) -> Result<u32, OrdError> {
        let (ch, size) = bstr::decode_utf8(self.as_bytes());
        match ch {
            // All `char`s are valid `u32`s
            Some(ch) => Ok(u32::from(ch)),
            None if size == 0 => Err(OrdError::empty_string()),
            None => Err(OrdError::invalid_utf8_byte_sequence()),
        }
    }

    #[must_use]
    pub fn char_len(&self) -> usize {
        let bytes = self.as_bytes();

        let tail = if let Some(idx) = bytes.find_non_ascii_byte() {
            idx
        } else {
            // The entire string is ASCII bytes, so fastpath return the slice
            // length.
            return bytes.len();
        };

        // SAFETY: `ByteSlice::find_non_ascii_byte` guarantees that the index is
        // in range for slicing if `Some(_)` is returned.
        let bytes = unsafe { bytes.get_unchecked(tail..) };

        // if the tail is valid UTF-8, use a fast path by delegating to SIMD
        // `bytecount` crate.
        if simdutf8::basic::from_utf8(bytes).is_ok() {
            return tail + bytecount::num_chars(bytes);
        }

        // Else fallback to decoding UTF-8 in chunks using `bstr`.
        let mut char_len = tail;
        for chunk in bytes.utf8_chunks() {
            char_len += bytecount::num_chars(chunk.valid().as_bytes());
            char_len += chunk.invalid().len();
        }
        char_len
    }

    #[must_use]
    pub fn get_char(&self, index: usize) -> Option<&Utf8Str> {
        // Fast path rejection for indexes beyond bytesize, which is cheap to
        // retrieve.
        if index >= self.len() {
            return None;
        }

        let slice = self.as_bytes();
        // Fast path for trying to treat the conventionally UTF-8 string as
        // entirely ASCII.
        //
        // If the string is either all ASCII or all ASCII for a prefix of the
        // string that contains the range we wish to slice, use byte slicing
        // like `AsciiStr` and `BinaryStr` do.
        let consumed = match slice.find_non_ascii_byte() {
            // The string is entirely ASCII, so we can always use byte slicing
            // to mean char slicing.
            None => {
                let s = slice.get(index..=index)?;
                return Some(Utf8Str::from_bytes(s));
            }
            // The first non-ASCII character occurs beyond the index we wish to
            // retrieve, so we can use byte slicing to mean char slicing.
            Some(idx) if idx > index => {
                let s = slice.get(index..=index)?;
                return Some(Utf8Str::from_bytes(s));
            }
            // The first `idx` characters of the `Utf8Str` end at the `idx` byte
            // position.
            Some(idx) => idx,
        };

        // Discard the ASCII prefix and begin a forward search with a character-
        // at-a-time decode.
        //
        // SAFETY: `find_non_ascii_byte` guarantees that when `Some(idx)` is
        // returned, `idx` is a valid position in the slice.
        let mut slice = unsafe { slice.get_unchecked(consumed..) };
        // Count of "characters" remaining until the `index`th character.
        let mut remaining = index - consumed;

        // This loop will terminate when either:
        //
        // - It counts `index` number of characters.
        // - It consumes the entire slice when scanning for the `index`th
        //   character.
        //
        // The loop will advance by at least one byte every iteration.
        loop {
            match bstr::decode_utf8(slice) {
                // `decode_utf8` only returns a 0 size when the slice is empty.
                //
                // If we've run out of slice while trying to find the `index`th
                // character, the lookup fails and we return `nil`.
                (_, 0) => return None,

                // The next two arms mean we've reached the `index`th character.
                // Either return the next valid UTF-8 character byte slice or,
                // if the next bytes are an invalid UTF-8 sequence, the next byte.
                (Some(_), size) if remaining == 0 => {
                    // SAFETY: `decode_utf8` guarantees that the number of bytes
                    // returned on a successful decode can be used to slice into
                    // the given slice.
                    let s = unsafe { slice.get_unchecked(..size) };
                    return Some(Utf8Str::from_bytes(s));
                }
                (None, _) if remaining == 0 => {
                    // SAFETY: `decode_utf8` guarantees unsuccessful decodes
                    // consume 0..=3 bytes and size is guaranteed to be non-zero
                    // per the first match arm.
                    let s = unsafe { slice.get_unchecked(..1) };
                    return Some(Utf8Str::from_bytes(s));
                }

                // We found a single UTF-8 encoded character keep track of the
                // count and advance the substring to continue decoding.
                (Some(_), size) => {
                    // SAFETY: `decode_utf8` guarantees that at least `size`
                    // bytes exist in the slice.
                    slice = unsafe { slice.get_unchecked(size..) };
                    remaining -= 1;
                }

                // The next two arms handle the case where we have encountered
                // an invalid UTF-8 byte sequence.
                //
                // In this case, `decode_utf8` will return slices whose length
                // is `1..=3`. The length of this slice is the number of
                // "characters" we can advance the loop by.
                //
                // If the invalid UTF-8 sequence contains more bytes than we
                // have remaining to get to the `index`th char, then the target
                // character is inside the invalid UTF-8 sequence.
                (None, size) if remaining < size => {
                    // SAFETY: `decode_utf8` guarantees that at least `size`
                    // bytes exist in the slice and we check that `remaining` is
                    // less than `size`.
                    let s = unsafe { slice.get_unchecked(remaining..=remaining) };
                    return Some(Utf8Str::from_bytes(s));
                }
                // If there are more characters remaining than the number of
                // bytes yielded in the invalid UTF-8 byte sequence, count
                // `size` bytes and advance the slice to continue decoding.
                (None, size) => {
                    // SAFETY: `decode_utf8` guarantees that at least `size`
                    // bytes exist in the slice.
                    slice = unsafe { slice.get_unchecked(size..) };
                    remaining -= size;
                }
            }
        }
    }

    #[must_use]
    pub fn get_char_slice(&self, range: Range<usize>) -> Option<&Utf8Str> {
        let Range { start, end } = range;

        // Fast path the lookup if the end of the range is before the start.
        if end < start {
            // Yes, these types of ranges are allowed and they return `""`.
            //
            // ```
            // [3.0.1] > "aaa"[1..0]
            // => ""
            // [3.0.1] > "aaa"[2..0]
            // => ""
            // [3.0.1] > "aaa"[2..1]
            // => ""
            // [3.0.1] > "💎🦀😅"[2..1]
            // => ""
            // [3.0.1] > "💎🦀😅"[3..0]
            // => ""
            // ```
            //
            // but only if `start` is within the string.
            //
            // ```
            // [3.0.1] > "aaa"[10..4]
            // => nil
            // [3.0.1] > "aaa"[10..0]
            // => nil
            // [3.0.1] > "💎🦀😅"[10..4]
            // => nil
            // [3.0.1] > "💎🦀😅"[10..0]
            // => nil
            // [3.0.1] > "💎🦀😅"[6..0]
            // => nil
            // [3.0.1] > "💎🦀😅"[4..0]
            // => nil
            // ```
            //
            // attempt to short-circuit with a cheap length retrieval
            if start > self.len() || start > self.char_len() {
                return None;
            }
            return Some(Utf8Str::empty());
        }

        // If the start of the range is beyond the character count of the
        // string, the whole lookup must fail.
        //
        // Slice lookups where the start is just beyond the last character index
        // always return an empty slice.
        //
        // ```
        // [3.0.1] > "aaa"[10, 0]
        // => nil
        // [3.0.1] > "aaa"[10, 7]
        // => nil
        // [3.0.1] > "aaa"[3, 7]
        // => ""
        // [3.0.1] > "🦀💎"[2, 0]
        // => ""
        // [3.0.1] > "🦀💎"[3, 1]
        // => nil
        // [3.0.1] > "🦀💎"[2, 1]
        // => ""
        // ```
        //
        // Fast path rejection for indexes beyond bytesize, which is cheap to
        // retrieve.
        if start > self.len() {
            return None;
        }
        match self.char_len() {
            char_length if start > char_length => return None,
            char_length if start == char_length => return Some(Utf8Str::empty()),
            _ => {}
        }

        // The span is guaranteed to at least partially overlap now.
        match end - start {
            // Empty substrings are present in all strings, even empty ones.
            //
            // ```
            // [3.0.1] > "aaa"[""]
            // => ""
            // [3.0.1] > ""[""]
            // => ""
            // [3.0.1] > ""[0, 0]
            // => ""
            // [3.0.1] > "aaa"[0, 0]
            // => ""
            // [3.0.1] > "aaa"[2, 0]
            // => ""
            // [3.0.1] > "🦀💎"[1, 0]
            // => ""
            // [3.0.1] > "🦀💎"[2, 0]
            // => ""
            // ```
            0 => return Some(Utf8Str::empty()),
            // Delegate to the specialized single char lookup, which allows the
            // remainder of this routine to fall back to the general case of
            // multi-character spans.
            //
            // ```
            // [3.0.1] > "abc"[2, 1]
            // => "c"
            // [3.0.1] > "🦀💎"[1, 1]
            // => "💎"
            // ```
            1 => return self.get_char(start),
            _ => {}
        }

        let slice = self.as_bytes();

        // Fast path for trying to treat the conventionally UTF-8 string
        // as entirely ASCII.
        //
        // If the string is either all ASCII or all ASCII for the subset
        // of the string we wish to slice, fallback to byte slicing as in
        // the ASCII and binary fast path.
        //
        // Perform the same saturate-to-end slicing mechanism if `end`
        // is beyond the character length of the string.
        let consumed = match slice.find_non_ascii_byte() {
            // The entire string is ASCII, so byte indexing <=> char
            // indexing.
            None => {
                let s = slice.get(start..end).or_else(|| slice.get(start..))?;
                return Some(Utf8Str::from_bytes(s));
            }
            // The whole substring we are interested in is ASCII, so
            // byte indexing is still valid.
            Some(non_ascii_byte_offset) if non_ascii_byte_offset > end => {
                let s = self.get(start..end)?;
                return Some(Utf8Str::from_bytes(s));
            }
            // We turn non-ASCII somewhere inside before the substring
            // we're interested in, so consume that much.
            Some(non_ascii_byte_offset) if non_ascii_byte_offset <= start => non_ascii_byte_offset,
            // This means we turn non-ASCII somewhere inside the substring.
            // Consume up to start.
            Some(_) => start,
        };

        // Scan for the beginning of the slice
        let mut slice = &slice[consumed..];
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
            loop {
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
                    (Some(_), size) if remaining == 1 => {
                        slice = &slice[size..];
                        break;
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
                    // than we have remaining to get to the `start`th char,
                    // then we can break the loop directly.
                    (None, size) if remaining <= size => {
                        slice = &slice[remaining..];
                        break;
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
                (_, 0) => return Some(Utf8Str::from_bytes(substr)),

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
                    let s = &substr[..endth];
                    return Some(Utf8Str::from_bytes(s));
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
                    let s = &substr[..endth];
                    return Some(Utf8Str::from_bytes(s));
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
}

// Indexing
impl Utf8Str {
    #[inline]
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.as_bytes().get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.as_bytes_mut().get_mut(index)
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.as_bytes().get_unchecked(index)
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.as_bytes_mut().get_unchecked_mut(index)
    }
}

// Encoding
impl Utf8Str {
    #[must_use]
    pub fn is_ascii_only(&self) -> bool {
        self.as_bytes().is_ascii()
    }

    #[must_use]
    pub fn is_valid_encoding(&self) -> bool {
        if self.is_ascii_only() {
            return true;
        }

        simdutf8::basic::from_utf8(self.as_bytes()).is_ok()
    }
}

// Slicing routines
impl Utf8Str {
    #[inline]
    #[must_use]
    pub fn starts_with(&self, slice: &[u8]) -> bool {
        self.as_bytes().starts_with(slice)
    }

    #[inline]
    #[must_use]
    pub fn ends_with(&self, slice: &[u8]) -> bool {
        self.as_bytes().ends_with(slice)
    }
}

// Searching routines
impl Utf8Str {
    #[must_use]
    pub fn index(&self, needle: &[u8], offset: usize) -> Option<usize> {
        // Decode needle
        // Needle containing any invalid UTF-8 should never match in MRI
        //
        // ```console
        // [3.2.2] > s = "abc"
        // => "abc"
        // [3.2.2] > s.encoding
        // => #<Encoding:UTF-8>
        // [3.2.2] > s.index "\xFF"
        // => nil
        // [3.2.2] > s = "\xFF\xFE"
        // => "\xFF\xFE"
        // [3.2.2] > s.encoding
        // => #<Encoding:UTF-8>
        // [3.2.2] > s.index "\xFF"
        // => nil
        // [3.2.2] > s.index "\xFF".b
        // (irb):14:in `index': incompatible character encodings: UTF-8 and ASCII-8BIT (Encoding::CompatibilityError)
        //         from (irb):14:in `<main>'
        //         from /usr/local/var/rbenv/versions/3.2.2/lib/ruby/gems/3.2.0/gems/irb-1.6.2/exe/irb:11:in `<top (required)>'
        //         from /usr/local/var/rbenv/versions/3.2.2/bin/irb:25:in `load'
        //         from /usr/local/var/rbenv/versions/3.2.2/bin/irb:25:in `<main>'
        // ```
        if !Utf8Str::from_bytes(needle).is_valid_encoding() {
            return None;
        }

        let prefix = self.get_char_slice(0..offset)?;
        let tail = &self[prefix.len()..];
        let index = tail.as_bytes().find(needle)?;

        let s = Utf8Str::from_bytes(&tail[..index]);
        Some(offset + s.char_len())
    }

    #[must_use]
    pub fn rindex(&self, needle: &[u8], offset: usize) -> Option<usize> {
        // Decode needle
        // Needle containing any invalid UTF-8 should never match in MRI
        //
        // ```console
        // [3.2.2] > s = "abc"
        // => "abc"
        // [3.2.2] > s.encoding
        // => #<Encoding:UTF-8>
        // [3.2.2] > s.rindex "\xFF"
        // => nil
        // [3.2.2] > s = "\xFF\xFE"
        // => "\xFF\xFE"
        // [3.2.2] > s.encoding
        // => #<Encoding:UTF-8>
        // [3.2.2] > s.rindex "\xFF"
        // => nil
        // [3.2.2] > s.rindex "\xFF".b
        // (irb):7:in `rindex': incompatible character encodings: UTF-8 and ASCII-8BIT (Encoding::CompatibilityError)
        //         from (irb):7:in `<main>'
        //         from /usr/local/var/rbenv/versions/3.2.2/lib/ruby/gems/3.2.0/gems/irb-1.6.2/exe/irb:11:in `<top (required)>'
        //         from /usr/local/var/rbenv/versions/3.2.2/bin/irb:25:in `load'
        //         from /usr/local/var/rbenv/versions/3.2.2/bin/irb:25:in `<main>'
        // ```
        if !needle.is_utf8() {
            return None;
        }

        let endpoint = offset.saturating_add(1);
        let buf = self.get_char_slice(0..endpoint).unwrap_or(self);
        let index = buf.as_bytes().rfind(needle)?;
        let s = Utf8Str::from_bytes(&buf[..index]);
        Some(s.char_len())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use core::fmt::Write;

    use super::Utf8Str;

    #[test]
    fn empty_is_empty() {
        let s = Utf8Str::empty();
        assert_eq!(s.len(), 0);
        assert_eq!(s.as_bytes(), &[]);
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(Utf8Str::empty(), <&Utf8Str>::default());
    }

    #[test]
    fn debug_is_not_empty() {
        let s = Utf8Str::empty();
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(!buf.is_empty());

        let s = Utf8Str::new("abc");
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(!buf.is_empty());
        assert!(buf.contains(r#""abc""#));

        let s = Utf8Str::new("🦀💎");
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(!buf.is_empty());

        let s = Utf8Str::new(b"\xFF\xFE");
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(!buf.is_empty());
    }

    #[test]
    fn debug_contains_readable_byte_contents() {
        let s = Utf8Str::empty();
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(buf.contains(r#""""#));

        let s = Utf8Str::new("abc");
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(buf.contains(r#""abc""#));

        let s = Utf8Str::new("🦀💎");
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(buf.contains(r#""🦀💎""#));

        let s = Utf8Str::new(b"\xFF\xFE");
        let mut buf = String::new();
        write!(&mut buf, "{s:?}").unwrap();
        assert!(buf.contains(r#""\xFF\xFE""#));
    }

    #[test]
    #[allow(clippy::no_effect_underscore_binding)]
    fn slice_indexing_is_byte_slicing() {
        let s = Utf8Str::new("a🦀b💎c");
        // individual bytes can be copied out of the string ref.
        for idx in 0..s.len() {
            let _byte: u8 = s[idx];
        }

        // slicing in the middle of multi-byte UTF-8 characters is fine.
        for idx in 0..s.len() {
            let _span: &[u8] = &s[idx..=idx];
        }
        for idx in 0..s.len() - 1 {
            let _span: &[u8] = &s[idx..idx + 2];
        }
    }

    #[test]
    fn mut_slice_indexing_is_mut_byte_slicing() {
        let mut data = "a🦀b💎c".as_bytes().to_vec();
        let s = Utf8Str::new_mut(&mut data);
        // individual bytes can be copied out of the string ref.
        for idx in 0..s.len() {
            let cell: &mut u8 = &mut s[idx];
            *cell = b'!';
        }
        assert_eq!(s, Utf8Str::new("!!!!!!!!!!!"));

        // slicing in the middle of multi-byte UTF-8 characters is fine.
        let s = Utf8Str::new_mut(&mut data);
        for idx in 0..s.len() {
            let span: &mut [u8] = &mut s[idx..=idx];
            span.copy_from_slice(b"%");
        }
        assert_eq!(s, Utf8Str::new("%%%%%%%%%%%"));

        let s = Utf8Str::new_mut(&mut data);
        for idx in 0..s.len() - 1 {
            let span: &mut [u8] = &mut s[idx..idx + 2];
            span.copy_from_slice(b"^&");
        }
        assert_eq!(s, Utf8Str::new("^^^^^^^^^^&"));
    }
}
