use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::Range;
use core::slice::SliceIndex;

use ascii::AsciiString;
use binary::BinaryString;
use utf8::Utf8String;

use crate::codepoints::InvalidCodepointError;
use crate::encoding::Encoding;
use crate::iter::{Bytes, IntoIter, Iter, IterMut};
use crate::ord::OrdError;

mod ascii;
mod binary;
mod impls;
mod inspect;
#[cfg(feature = "std")]
mod io;
mod utf8;

pub use inspect::Inspect;

#[derive(Clone)]
pub enum EncodedString {
    Ascii(AsciiString),
    Binary(BinaryString),
    Utf8(Utf8String),
}

impl Default for EncodedString {
    fn default() -> Self {
        Self::utf8(Vec::new())
    }
}

impl Hash for EncodedString {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // A `EncodedString`'s hash only depends on its byte contents.
        //
        // ```
        // [3.0.2] > s = "abc"
        // => "abc"
        // [3.0.2] > t = s.dup.force_encoding(Encoding::ASCII)
        // => "abc"
        // [3.0.2] > s.hash
        // => 3398383793005079442
        // [3.0.2] > t.hash
        // => 3398383793005079442
        // ```
        self.as_slice().hash(hasher);
    }
}

impl PartialEq for EncodedString {
    fn eq(&self, other: &Self) -> bool {
        // Equality only depends on each `EncodedString`'s byte contents.
        //
        // ```
        // [3.0.2] > s = "abc"
        // => "abc"
        // [3.0.2] > t = s.dup.force_encoding(Encoding::ASCII)
        // => "abc"
        // [3.0.2] > s == t
        // => true
        // ```
        *self.as_slice() == *other.as_slice()
    }
}

impl Eq for EncodedString {}

impl PartialOrd for EncodedString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Ord for EncodedString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

// This impl of `Borrow<[u8]>` is permissible due to the manual implementations
// of `PartialEq`, `Hash`, and `Ord` above which only rely on the byte slice
// contents in the underlying typed strings.
//
// Per the docs in `std`:
//
// > In particular `Eq`, `Ord` and `Hash` must be equivalent for borrowed and
// > owned values: `x.borrow() == y.borrow()` should give the same result as
// > `x == y`.
impl Borrow<[u8]> for EncodedString {
    #[inline]
    fn borrow(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(inner) => inner.borrow(),
            EncodedString::Binary(inner) => inner.borrow(),
            EncodedString::Utf8(inner) => inner.borrow(),
        }
    }
}

// Constructors
impl EncodedString {
    #[inline]
    #[must_use]
    pub const fn new(buf: Vec<u8>, encoding: Encoding) -> Self {
        match encoding {
            Encoding::Ascii => Self::ascii(buf),
            Encoding::Binary => Self::binary(buf),
            Encoding::Utf8 => Self::utf8(buf),
        }
    }

    #[inline]
    #[must_use]
    pub const fn ascii(buf: Vec<u8>) -> Self {
        Self::Ascii(AsciiString::new(buf))
    }

    #[inline]
    #[must_use]
    pub const fn binary(buf: Vec<u8>) -> Self {
        Self::Binary(BinaryString::new(buf))
    }

    #[inline]
    #[must_use]
    pub const fn utf8(buf: Vec<u8>) -> Self {
        Self::Utf8(Utf8String::new(buf))
    }
}

impl EncodedString {
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        match self {
            EncodedString::Ascii(inner) => inner.into_vec(),
            EncodedString::Binary(inner) => inner.into_vec(),
            EncodedString::Utf8(inner) => inner.into_vec(),
        }
    }

    #[inline]
    #[must_use]
    pub fn into_iter(self) -> IntoIter {
        match self {
            EncodedString::Ascii(inner) => inner.into_iter(),
            EncodedString::Binary(inner) => inner.into_iter(),
            EncodedString::Utf8(inner) => inner.into_iter(),
        }
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(inner) => inner.as_slice(),
            EncodedString::Binary(inner) => inner.as_slice(),
            EncodedString::Utf8(inner) => inner.as_slice(),
        }
    }

    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            EncodedString::Ascii(inner) => inner.as_mut_slice(),
            EncodedString::Binary(inner) => inner.as_mut_slice(),
            EncodedString::Utf8(inner) => inner.as_mut_slice(),
        }
    }

    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        match self {
            EncodedString::Ascii(inner) => inner.as_ptr(),
            EncodedString::Binary(inner) => inner.as_ptr(),
            EncodedString::Utf8(inner) => inner.as_ptr(),
        }
    }

    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        match self {
            EncodedString::Ascii(inner) => inner.as_mut_ptr(),
            EncodedString::Binary(inner) => inner.as_mut_ptr(),
            EncodedString::Utf8(inner) => inner.as_mut_ptr(),
        }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            EncodedString::Ascii(inner) => inner.len(),
            EncodedString::Binary(inner) => inner.len(),
            EncodedString::Utf8(inner) => inner.len(),
        }
    }

    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        match self {
            EncodedString::Ascii(inner) => inner.set_len(len),
            EncodedString::Binary(inner) => inner.set_len(len),
            EncodedString::Utf8(inner) => inner.set_len(len),
        }
    }

    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        match self {
            EncodedString::Ascii(inner) => inner.capacity(),
            EncodedString::Binary(inner) => inner.capacity(),
            EncodedString::Utf8(inner) => inner.capacity(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        match self {
            EncodedString::Ascii(inner) => inner.clear(),
            EncodedString::Binary(inner) => inner.clear(),
            EncodedString::Utf8(inner) => inner.clear(),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            EncodedString::Ascii(inner) => inner.is_empty(),
            EncodedString::Binary(inner) => inner.is_empty(),
            EncodedString::Utf8(inner) => inner.is_empty(),
        }
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        match self {
            EncodedString::Ascii(inner) => inner.truncate(len),
            EncodedString::Binary(inner) => inner.truncate(len),
            EncodedString::Utf8(inner) => inner.truncate(len),
        };
    }

    #[inline]
    #[must_use]
    pub fn char_len(&self) -> usize {
        match self {
            EncodedString::Ascii(inner) => inner.char_len(),
            EncodedString::Binary(inner) => inner.char_len(),
            EncodedString::Utf8(inner) => inner.char_len(),
        }
    }

    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        match self {
            EncodedString::Ascii(inner) => inner.iter(),
            EncodedString::Binary(inner) => inner.iter(),
            EncodedString::Utf8(inner) => inner.iter(),
        }
    }

    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        match self {
            EncodedString::Ascii(inner) => inner.iter_mut(),
            EncodedString::Binary(inner) => inner.iter_mut(),
            EncodedString::Utf8(inner) => inner.iter_mut(),
        }
    }

    #[inline]
    #[must_use]
    pub fn bytes(&self) -> Bytes<'_> {
        match self {
            EncodedString::Ascii(inner) => inner.bytes(),
            EncodedString::Binary(inner) => inner.bytes(),
            EncodedString::Utf8(inner) => inner.bytes(),
        }
    }

    #[inline]
    pub fn inspect(&self) -> Inspect<'_> {
        match self {
            EncodedString::Ascii(inner) => inner.into(),
            EncodedString::Binary(inner) => inner.into(),
            EncodedString::Utf8(inner) => inner.into(),
        }
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        match self {
            EncodedString::Ascii(inner) => inner.reserve(additional),
            EncodedString::Binary(inner) => inner.reserve(additional),
            EncodedString::Utf8(inner) => inner.reserve(additional),
        }
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        match self {
            EncodedString::Ascii(inner) => inner.try_reserve(additional),
            EncodedString::Binary(inner) => inner.try_reserve(additional),
            EncodedString::Utf8(inner) => inner.try_reserve(additional),
        }
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        match self {
            EncodedString::Ascii(inner) => inner.reserve_exact(additional),
            EncodedString::Binary(inner) => inner.reserve_exact(additional),
            EncodedString::Utf8(inner) => inner.reserve_exact(additional),
        }
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        match self {
            EncodedString::Ascii(inner) => inner.try_reserve_exact(additional),
            EncodedString::Binary(inner) => inner.try_reserve_exact(additional),
            EncodedString::Utf8(inner) => inner.try_reserve_exact(additional),
        }
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        match self {
            EncodedString::Ascii(inner) => inner.shrink_to_fit(),
            EncodedString::Binary(inner) => inner.shrink_to_fit(),
            EncodedString::Utf8(inner) => inner.shrink_to_fit(),
        }
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        match self {
            EncodedString::Ascii(inner) => inner.shrink_to(min_capacity),
            EncodedString::Binary(inner) => inner.shrink_to(min_capacity),
            EncodedString::Utf8(inner) => inner.shrink_to(min_capacity),
        }
    }

    #[inline]
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(inner) => inner.get(index),
            EncodedString::Binary(inner) => inner.get(index),
            EncodedString::Utf8(inner) => inner.get(index),
        }
    }

    #[inline]
    #[must_use]
    pub fn get_char(&self, index: usize) -> Option<&'_ [u8]> {
        match self {
            EncodedString::Ascii(inner) => inner.get_char(index),
            EncodedString::Binary(inner) => inner.get_char(index),
            EncodedString::Utf8(inner) => inner.get_char(index),
        }
    }

    #[inline]
    #[must_use]
    pub fn get_char_slice(&self, range: Range<usize>) -> Option<&'_ [u8]> {
        match self {
            EncodedString::Ascii(inner) => inner.get_char_slice(range),
            EncodedString::Binary(inner) => inner.get_char_slice(range),
            EncodedString::Utf8(inner) => inner.get_char_slice(range),
        }
    }

    #[inline]
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(inner) => inner.get_mut(index),
            EncodedString::Binary(inner) => inner.get_mut(index),
            EncodedString::Utf8(inner) => inner.get_mut(index),
        }
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(inner) => inner.get_unchecked(index),
            EncodedString::Binary(inner) => inner.get_unchecked(index),
            EncodedString::Utf8(inner) => inner.get_unchecked(index),
        }
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        match self {
            EncodedString::Ascii(inner) => inner.get_unchecked_mut(index),
            EncodedString::Binary(inner) => inner.get_unchecked_mut(index),
            EncodedString::Utf8(inner) => inner.get_unchecked_mut(index),
        }
    }

    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        match self {
            EncodedString::Ascii(inner) => inner.push_byte(byte),
            EncodedString::Binary(inner) => inner.push_byte(byte),
            EncodedString::Utf8(inner) => inner.push_byte(byte),
        }
    }

    #[inline]
    pub fn try_push_codepoint(&mut self, codepoint: i64) -> Result<(), InvalidCodepointError> {
        match self {
            EncodedString::Ascii(inner) => inner.try_push_codepoint(codepoint),
            EncodedString::Binary(inner) => inner.try_push_codepoint(codepoint),
            EncodedString::Utf8(inner) => inner.try_push_codepoint(codepoint),
        }
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        match self {
            EncodedString::Ascii(inner) => inner.push_char(ch),
            EncodedString::Binary(inner) => inner.push_char(ch),
            EncodedString::Utf8(inner) => inner.push_char(ch),
        }
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        match self {
            EncodedString::Ascii(inner) => inner.push_str(s),
            EncodedString::Binary(inner) => inner.push_str(s),
            EncodedString::Utf8(inner) => inner.push_str(s),
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        match self {
            EncodedString::Ascii(inner) => inner.extend_from_slice(other),
            EncodedString::Binary(inner) => inner.extend_from_slice(other),
            EncodedString::Utf8(inner) => inner.extend_from_slice(other),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_ascii_only(&self) -> bool {
        match self {
            EncodedString::Ascii(inner) => inner.is_ascii_only(),
            EncodedString::Binary(inner) => inner.is_ascii_only(),
            EncodedString::Utf8(inner) => inner.is_ascii_only(),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_valid_encoding(&self) -> bool {
        match self {
            EncodedString::Ascii(inner) => inner.is_valid_encoding(),
            EncodedString::Binary(inner) => inner.is_valid_encoding(),
            EncodedString::Utf8(inner) => inner.is_valid_encoding(),
        }
    }

    #[inline]
    pub fn make_capitalized(&mut self) {
        match self {
            EncodedString::Ascii(inner) => inner.make_capitalized(),
            EncodedString::Binary(inner) => inner.make_capitalized(),
            EncodedString::Utf8(inner) => inner.make_capitalized(),
        }
    }

    #[inline]
    pub fn make_uppercase(&mut self) {
        match self {
            EncodedString::Ascii(inner) => inner.make_uppercase(),
            EncodedString::Binary(inner) => inner.make_uppercase(),
            EncodedString::Utf8(inner) => inner.make_uppercase(),
        }
    }

    #[inline]
    pub fn make_lowercase(&mut self) {
        match self {
            EncodedString::Ascii(inner) => inner.make_lowercase(),
            EncodedString::Binary(inner) => inner.make_lowercase(),
            EncodedString::Utf8(inner) => inner.make_lowercase(),
        }
    }

    #[inline]
    #[must_use]
    pub fn chr(&self) -> &[u8] {
        match self {
            EncodedString::Ascii(inner) => inner.chr(),
            EncodedString::Binary(inner) => inner.chr(),
            EncodedString::Utf8(inner) => inner.chr(),
        }
    }

    #[inline]
    pub fn ord(&self) -> Result<u32, OrdError> {
        match self {
            EncodedString::Ascii(inner) => inner.ord(),
            EncodedString::Binary(inner) => inner.ord(),
            EncodedString::Utf8(inner) => inner.ord(),
        }
    }

    #[inline]
    #[must_use]
    pub fn ends_with(&self, slice: &[u8]) -> bool {
        match self {
            EncodedString::Ascii(inner) => inner.ends_with(slice),
            EncodedString::Binary(inner) => inner.ends_with(slice),
            EncodedString::Utf8(inner) => inner.ends_with(slice),
        }
    }
}
