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
//! # #[cfg(feature = "alloc")]
//! # extern crate alloc;
//! # #[cfg(feature = "alloc")]
//! # use alloc::collections::TryReserveError;
//! # #[cfg(feature = "alloc")]
//! # fn example() -> Result<(), TryReserveError> {
//! let data = b"Artichoke Ruby";
//! let mut buf = String::new();
//! scolapasta_hex::try_encode_into(data, &mut buf)?;
//! assert_eq!(buf, "4172746963686f6b652052756279");
//! # Ok(())
//! # }
//! # #[cfg(feature = "alloc")]
//! # example().unwrap()
//! ```
//!
//! This module also exposes an iterator:
//!
//! ```
//! use scolapasta_hex::Hex;
//!
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
#[cfg(all(doctest, alloc))]
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
    buf.extend(iter);
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
/// use scolapasta_hex::Hex;
///
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
    /// use scolapasta_hex::Hex;
    ///
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
    /// use scolapasta_hex::Hex;
    ///
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

impl<'a, const N: usize> From<&'a [u8; N]> for Hex<'a> {
    #[inline]
    fn from(data: &'a [u8; N]) -> Self {
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
mod tests;
