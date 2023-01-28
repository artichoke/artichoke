#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::manual_let_else)]
#![cfg_attr(test, allow(clippy::non_ascii_literal))]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Functions for encoding sequences of bytes into base 16 hex encoding.
//!
//! [Base 16 encoding] is an encoding scheme that uses a 16 character ASCII
//! alphabet for encoding arbitrary octets.
//!
//! This crate offers encoders that:
//!
//! - Allocate and return a [`String`]: [`try_encode`].
//! - Encode into an already allocated [`String`]: [`try_encode_into`].
//! - Encode into a [`core::fmt::Write`]: [`format_into`].
//! - Encode into a [`std::io::Write`]: [`write_into`].
//!
//! # Examples
//!
//! ```
//! # #![cfg(feature = "alloc")]
//! # extern crate alloc;
//! # use alloc::collections::TryReserveError;
//! # fn example() -> Result<(), TryReserveError> {
//! let data = b"Artichoke Ruby";
//! let mut buf = String::new();
//! scolapasta_hex::try_encode_into(data, &mut buf)?;
//! assert_eq!(buf, "4172746963686f6b652052756279");
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! This module also exposes an iterator:
//!
//! ```
//! # use scolapasta_hex::Hex;
//! let data = "Artichoke Ruby";
//! let iter = Hex::from(data);
//! assert_eq!(iter.collect::<String>(), "4172746963686f6b652052756279");
//! ```
//!
//! # `no_std`
//!
//! This crate is `no_std` compatible when built without the `std` feature. This
//! crate optionally depends on [`alloc`] when the `alloc` feature is enabled.
//!
//! When this crate depends on `alloc`, it exclusively uses fallible allocation
//! APIs. The APIs in this crate will never abort due to allocation failure or
//! capacity overflows. Note that writers given to [`format_into`] and
//! [`write_into`] may have abort on allocation failure behavior.
//!
//! # Crate features
//!
//! All features are enabled by default.
//!
//! - **std** - Enables a dependency on the Rust Standard Library. Activating
//!   this feature enables APIs that require [`std::io::Write`]. Activating this
//!   feature also activates the **alloc** feature.
//! - **alloc** - Enables a dependency on the Rust [`alloc`] crate. Activating
//!   this feature enables APIs that require [`alloc::string::String`].
//!
#![cfg_attr(
    not(feature = "std"),
    doc = "[`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"
)]
#![cfg_attr(
    not(feature = "std"),
    doc = "[`write_into`]: https://artichoke.github.io/artichoke/scolapasta_hex/fn.write_into.html"
)]
//! [Base 16 encoding]: https://tools.ietf.org/html/rfc4648#section-8

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

#[cfg(feature = "alloc")]
extern crate alloc;
// Having access to `String` in tests is convenient to collect `Inspect`
// iterators for whole content comparisons.
#[cfg(any(feature = "std", test, doctest))]
extern crate std;

#[cfg(feature = "alloc")]
use alloc::collections::TryReserveError;
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::fmt;
use core::iter::FusedIterator;
use core::slice;
use core::str::Chars;
#[cfg(feature = "std")]
use std::io;

/// Encode arbitrary octets as base16. Returns a [`String`].
///
/// This function allocates an empty [`String`] and delegates to
/// [`try_encode_into`].
///
/// # Errors
///
/// If the allocated string's capacity overflows, or the allocator reports a
/// failure, then an error is returned.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// # use alloc::collections::TryReserveError;
/// # fn example() -> Result<(), TryReserveError> {
/// let data = b"Artichoke Ruby";
/// let buf = scolapasta_hex::try_encode(data)?;
/// assert_eq!(buf, "4172746963686f6b652052756279");
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn try_encode<T: AsRef<[u8]>>(data: T) -> Result<String, TryReserveError> {
    let mut buf = String::new();
    try_encode_into(data.as_ref(), &mut buf)?;
    Ok(buf)
}

/// Encode arbitrary octets as base16 into the given [`String`].
///
/// This function writes encoded octets into the given `String`. This function
/// will allocate at most once.
///
/// # Errors
///
/// If the given string's capacity overflows, or the allocator reports a
/// failure, then an error is returned.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// # use alloc::collections::TryReserveError;
/// # use alloc::string::String;
/// # fn example() -> Result<(), TryReserveError> {
/// let data = b"Artichoke Ruby";
/// let mut buf = String::new();
/// scolapasta_hex::try_encode_into(data, &mut buf)?;
/// assert_eq!(buf, "4172746963686f6b652052756279");
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn try_encode_into<T: AsRef<[u8]>>(data: T, buf: &mut String) -> Result<(), TryReserveError> {
    let data = data.as_ref();
    let iter = Hex::from(data);
    buf.try_reserve(iter.len())?;
    for ch in iter {
        buf.push(ch);
    }
    Ok(())
}

/// Write hex-encoded octets into the given [`fmt::Write`].
///
/// This function writes UTF-8 encoded octets into the given writer. This
/// function does not allocate, but the given writer may.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// # use alloc::string::String;
/// let data = b"Artichoke Ruby";
/// let mut buf = String::new();
/// scolapasta_hex::format_into(data, &mut buf);
/// assert_eq!(buf, "4172746963686f6b652052756279");
/// ```
///
/// # Errors
///
/// If the formatter returns an error, that error is returned.
#[inline]
pub fn format_into<T, W>(data: T, mut f: W) -> fmt::Result
where
    T: AsRef<[u8]>,
    W: fmt::Write,
{
    let data = data.as_ref();
    let iter = Hex::from(data);
    let mut enc = [0; 4];
    for ch in iter {
        let escaped = ch.encode_utf8(&mut enc);
        f.write_str(escaped)?;
    }
    Ok(())
}

/// Write hex-encoded octets into the given [`io::Write`].
///
/// This function writes UTF-8 encoded octets into the given writer. This
/// function does not allocate, but the given writer may.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// # use alloc::vec::Vec;
/// let data = b"Artichoke Ruby";
/// let mut buf = Vec::new();
/// scolapasta_hex::write_into(data, &mut buf);
/// assert_eq!(buf, b"4172746963686f6b652052756279".to_vec());
/// ```
///
/// # Errors
///
/// If the destination returns an error, that error is returned.
#[inline]
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn write_into<T, W>(data: T, mut dest: W) -> io::Result<()>
where
    T: AsRef<[u8]>,
    W: io::Write,
{
    let data = data.as_ref();
    let iter = Hex::from(data);
    let mut enc = [0; 4];
    for ch in iter {
        let escaped = ch.encode_utf8(&mut enc);
        dest.write_all(escaped.as_bytes())?;
    }
    Ok(())
}

/// An iterator over a byte slice that returns the data as a sequence of hex
/// encoded [`char`]s.
///
/// # Examples
///
/// ```
/// # use scolapasta_hex::Hex;
/// let data = "Artichoke Ruby";
/// let iter = Hex::from(data);
/// assert_eq!(iter.collect::<String>(), "4172746963686f6b652052756279");
/// ```
#[derive(Debug, Clone)]
pub struct Hex<'a> {
    iter: slice::Iter<'a, u8>,
    escaped_byte: Option<EscapedByte>,
}

impl<'a> Hex<'a> {
    /// Returns the number of remaining hex encoded characters in the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_hex::Hex;
    /// let iter = Hex::from("");
    /// assert_eq!(iter.len(), 0);
    ///
    /// let mut iter = Hex::from("a");
    /// assert_eq!(iter.len(), 2);
    /// assert_eq!(iter.next(), Some('6'));
    /// assert_eq!(iter.len(), 1);
    /// assert_eq!(iter.next(), Some('1'));
    /// assert_eq!(iter.len(), 0);
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.len(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        let remaining_bytes = self.iter.as_slice().len();
        // Every byte expands to two hexadecimal ASCII `char`s.
        let remaining_bytes_encoded_len = remaining_bytes.saturating_mul(2);
        if let Some(ref escaped_byte) = self.escaped_byte {
            // Add the dangling char(s) from the `EscapedByte` iterator.
            remaining_bytes_encoded_len.saturating_add(escaped_byte.len())
        } else {
            remaining_bytes_encoded_len
        }
    }

    /// Returns whether the iterator has no more remaining escape codes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_hex::Hex;
    /// let iter = Hex::from("");
    /// assert!(iter.is_empty());
    ///
    /// let mut iter = Hex::from("a");
    /// assert!(!iter.is_empty());
    /// assert_eq!(iter.next(), Some('6'));
    /// assert!(!iter.is_empty());
    /// assert_eq!(iter.next(), Some('1'));
    /// assert!(iter.is_empty());
    /// assert_eq!(iter.next(), None);
    /// assert!(iter.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        if let Some(ref escaped_byte) = self.escaped_byte {
            self.iter.as_slice().is_empty() && escaped_byte.is_empty()
        } else {
            self.iter.as_slice().is_empty()
        }
    }
}

impl<'a> From<&'a str> for Hex<'a> {
    #[inline]
    fn from(data: &'a str) -> Self {
        Self::from(data.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for Hex<'a> {
    #[inline]
    fn from(data: &'a [u8]) -> Self {
        Self {
            iter: data.iter(),
            escaped_byte: None,
        }
    }
}

impl<'a> Iterator for Hex<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut escaped) = self.escaped_byte {
            let next = escaped.next();
            if next.is_some() {
                return next;
            }
        }
        let byte = self.iter.next().copied()?;
        let mut escaped = EscapedByte::from(byte);
        let next = escaped.next()?;
        self.escaped_byte = Some(escaped);
        Some(next)
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len();
        (size, Some(size))
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        let byte = self.iter.last().copied()?;
        let escaped = EscapedByte::from(byte);
        escaped.last()
    }
}

impl<'a> FusedIterator for Hex<'a> {}

impl<'a> ExactSizeIterator for Hex<'a> {}

/// Map from a `u8` to a hex encoded string literal.
///
/// # Examples
///
/// ```
/// assert_eq!(scolapasta_hex::escape_byte(0), "00");
/// assert_eq!(scolapasta_hex::escape_byte(0x20), "20");
/// assert_eq!(scolapasta_hex::escape_byte(255), "ff");
/// ```
#[inline]
#[must_use]
pub const fn escape_byte(byte: u8) -> &'static str {
    EscapedByte::hex_escape(byte)
}

#[derive(Debug, Clone)]
#[must_use = "this `EscapedByte` is an `Iterator`, which should be consumed if constructed"]
struct EscapedByte(Chars<'static>);

impl EscapedByte {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has `'static` lifetime, and so the iterator can continue to be used
    /// while this exists.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Map from a `u8` to a hex encoded string literal.
    ///
    /// For example, `00`, `20` or `ff`.
    const fn hex_escape(value: u8) -> &'static str {
        // Use a lookup table, generated with:
        //
        // ```ruby
        // puts "const TABLE: [&str; 256] = [" + (0x00..0xFF).to_a.map {|b| b.to_s(16).rjust(2, "0").inspect}.join(", ") + "];"
        // ```
        #[rustfmt::skip]
        const TABLE: [&str; 256] = [
            "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e", "0f",
            "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d", "1e", "1f",
            "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c", "2d", "2e", "2f",
            "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b", "3c", "3d", "3e", "3f",
            "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a", "4b", "4c", "4d", "4e", "4f",
            "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5a", "5b", "5c", "5d", "5e", "5f",
            "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6a", "6b", "6c", "6d", "6e", "6f",
            "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7a", "7b", "7c", "7d", "7e", "7f",
            "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f",
            "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f",
            "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af",
            "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf",
            "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf",
            "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df",
            "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef",
            "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff",
        ];

        TABLE[value as usize]
    }
}

impl From<u8> for EscapedByte {
    #[inline]
    fn from(byte: u8) -> Self {
        let escape = Self::hex_escape(byte);
        Self(escape.chars())
    }
}

impl Iterator for EscapedByte {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last()
    }
}

impl DoubleEndedIterator for EscapedByte {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl FusedIterator for EscapedByte {}

#[cfg(test)]
mod tests {
    use crate::EscapedByte;

    #[test]
    fn literal_exhaustive() {
        for byte in 0..=255 {
            let mut lit = EscapedByte::from(byte);
            let left = lit.next().unwrap();
            let top = byte >> 4;
            match top {
                0x0 => assert_eq!(left, '0', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x1 => assert_eq!(left, '1', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x2 => assert_eq!(left, '2', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x3 => assert_eq!(left, '3', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x4 => assert_eq!(left, '4', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x5 => assert_eq!(left, '5', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x6 => assert_eq!(left, '6', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x7 => assert_eq!(left, '7', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x8 => assert_eq!(left, '8', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0x9 => assert_eq!(left, '9', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0xA => assert_eq!(left, 'a', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0xB => assert_eq!(left, 'b', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0xC => assert_eq!(left, 'c', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0xD => assert_eq!(left, 'd', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0xE => assert_eq!(left, 'e', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                0xF => assert_eq!(left, 'f', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
                tuple => panic!("unknown top 16th: {tuple}, from byte: {byte} ({byte:02x})"),
            }

            let right = lit.next().unwrap();
            let bottom = byte & 0xF;
            match bottom {
                0x0 => assert_eq!(right, '0', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x1 => assert_eq!(right, '1', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x2 => assert_eq!(right, '2', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x3 => assert_eq!(right, '3', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x4 => assert_eq!(right, '4', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x5 => assert_eq!(right, '5', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x6 => assert_eq!(right, '6', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x7 => assert_eq!(right, '7', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x8 => assert_eq!(right, '8', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0x9 => assert_eq!(right, '9', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0xA => assert_eq!(right, 'a', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0xB => assert_eq!(right, 'b', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0xC => assert_eq!(right, 'c', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0xD => assert_eq!(right, 'd', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0xE => assert_eq!(right, 'e', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                0xF => assert_eq!(right, 'f', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
                tuple => panic!("unknown bottom 16th: {tuple}, from byte: {byte} ({byte:02x})"),
            }
            assert!(
                lit.next().is_none(),
                "literal must only expand to two ASCII chracters, found 3+"
            );
        }
    }

    #[cfg(feature = "alloc")]
    mod alloc {
        use alloc::string::String;

        use crate::{format_into, try_encode, try_encode_into, Hex};

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_encode() {
            // ```
            // BASE16("") = ""
            // ```
            assert_eq!(try_encode("").unwrap(), "");

            // ```
            // BASE16("f") = "66"
            // ```
            assert_eq!(try_encode("f").unwrap(), "66");

            // ```
            // BASE16("fo") = "666F"
            // ```
            assert_eq!(try_encode("fo").unwrap(), "666f");

            // ```
            // BASE16("foo") = "666F6F"
            // ```
            assert_eq!(try_encode("foo").unwrap(), "666f6f");

            // ```
            // BASE16("foob") = "666F6F62"
            // ```
            assert_eq!(try_encode("foob").unwrap(), "666f6f62");

            // ```
            // BASE16("fooba") = "666F6F6261"
            // ```
            assert_eq!(try_encode("fooba").unwrap(), "666f6f6261");

            // ```
            // BASE16("foobar") = "666F6F626172"
            // ```
            assert_eq!(try_encode("foobar").unwrap(), "666f6f626172");
        }

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_hex_iter() {
            // ```
            // BASE16("") = ""
            // ```
            assert_eq!(Hex::from("").collect::<String>(), "");

            // ```
            // BASE16("f") = "66"
            // ```
            assert_eq!(Hex::from("f").collect::<String>(), "66");

            // ```
            // BASE16("fo") = "666F"
            // ```
            assert_eq!(Hex::from("fo").collect::<String>(), "666f");

            // ```
            // BASE16("foo") = "666F6F"
            // ```
            assert_eq!(Hex::from("foo").collect::<String>(), "666f6f");

            // ```
            // BASE16("foob") = "666F6F62"
            // ```
            assert_eq!(Hex::from("foob").collect::<String>(), "666f6f62");

            // ```
            // BASE16("fooba") = "666F6F6261"
            // ```
            assert_eq!(Hex::from("fooba").collect::<String>(), "666f6f6261");

            // ```
            // BASE16("foobar") = "666F6F626172"
            // ```
            assert_eq!(Hex::from("foobar").collect::<String>(), "666f6f626172");
        }

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_encode_into_string() {
            // ```
            // BASE16("") = ""
            // ```
            let mut s = String::new();
            try_encode_into("", &mut s).unwrap();
            assert_eq!(s, "");
            assert_eq!(s.capacity(), 0);

            // ```
            // BASE16("f") = "66"
            // ```
            let mut s = String::new();
            try_encode_into("f", &mut s).unwrap();
            assert_eq!(s, "66");
            assert!(s.capacity() >= 2);

            // ```
            // BASE16("fo") = "666F"
            // ```
            let mut s = String::new();
            try_encode_into("fo", &mut s).unwrap();
            assert_eq!(s, "666f");
            assert!(s.capacity() >= 4);

            // ```
            // BASE16("foo") = "666F6F"
            // ```
            let mut s = String::new();
            try_encode_into("foo", &mut s).unwrap();
            assert_eq!(s, "666f6f");
            assert!(s.capacity() >= 6);

            // ```
            // BASE16("foob") = "666F6F62"
            // ```
            let mut s = String::new();
            try_encode_into("foob", &mut s).unwrap();
            assert_eq!(s, "666f6f62");
            assert!(s.capacity() >= 8);

            // ```
            // BASE16("fooba") = "666F6F6261"
            // ```
            let mut s = String::new();
            try_encode_into("fooba", &mut s).unwrap();
            assert_eq!(s, "666f6f6261");
            assert!(s.capacity() >= 10);

            // ```
            // BASE16("foobar") = "666F6F626172"
            // ```
            let mut s = String::new();
            try_encode_into("foobar", &mut s).unwrap();
            assert_eq!(s, "666f6f626172");
            assert!(s.capacity() >= 12);
        }

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_format_into() {
            // ```
            // BASE16("") = ""
            // ```
            let mut fmt = String::new();
            format_into("", &mut fmt).unwrap();
            assert_eq!(fmt, "");

            // ```
            // BASE16("f") = "66"
            // ```
            let mut fmt = String::new();
            format_into("f", &mut fmt).unwrap();
            assert_eq!(fmt, "66");

            // ```
            // BASE16("fo") = "666F"
            // ```
            let mut fmt = String::new();
            format_into("fo", &mut fmt).unwrap();
            assert_eq!(fmt, "666f");

            // ```
            // BASE16("foo") = "666F6F"
            // ```
            let mut fmt = String::new();
            format_into("foo", &mut fmt).unwrap();
            assert_eq!(fmt, "666f6f");

            // ```
            // BASE16("foob") = "666F6F62"
            // ```
            let mut fmt = String::new();
            format_into("foob", &mut fmt).unwrap();
            assert_eq!(fmt, "666f6f62");

            // ```
            // BASE16("fooba") = "666F6F6261"
            // ```
            let mut fmt = String::new();
            format_into("fooba", &mut fmt).unwrap();
            assert_eq!(fmt, "666f6f6261");

            // ```
            // BASE16("foobar") = "666F6F626172"
            // ```
            let mut fmt = String::new();
            format_into("foobar", &mut fmt).unwrap();
            assert_eq!(fmt, "666f6f626172");
        }
    }

    #[cfg(feature = "std")]
    mod std {
        use std::vec::Vec;

        use crate::write_into;

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_write_into() {
            // ```
            // BASE16("") = ""
            // ```
            let mut write = Vec::new();
            write_into("", &mut write).unwrap();
            assert_eq!(write, b"".to_vec());

            // ```
            // BASE16("f") = "66"
            // ```
            let mut write = Vec::new();
            write_into("f", &mut write).unwrap();
            assert_eq!(write, b"66".to_vec());

            // ```
            // BASE16("fo") = "666F"
            // ```
            let mut write = Vec::new();
            write_into("fo", &mut write).unwrap();
            assert_eq!(write, b"666f".to_vec());

            // ```
            // BASE16("foo") = "666F6F"
            // ```
            let mut write = Vec::new();
            write_into("foo", &mut write).unwrap();
            assert_eq!(write, b"666f6f".to_vec());

            // ```
            // BASE16("foob") = "666F6F62"
            // ```
            let mut write = Vec::new();
            write_into("foob", &mut write).unwrap();
            assert_eq!(write, b"666f6f62".to_vec());

            // ```
            // BASE16("fooba") = "666F6F6261"
            // ```
            let mut write = Vec::new();
            write_into("fooba", &mut write).unwrap();
            assert_eq!(write, b"666f6f6261".to_vec());

            // ```
            // BASE16("foobar") = "666F6F626172"
            // ```
            let mut write = Vec::new();
            write_into("foobar", &mut write).unwrap();
            assert_eq!(write, b"666f6f626172".to_vec());
        }
    }
}
