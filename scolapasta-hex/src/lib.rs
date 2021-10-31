#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::option_if_let_else)]
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
//! - Allocate and return a [`String`]: [`encode`].
//! - Encode into an already allocated [`String`]: [`encode_into`].
//! - Encode into a [`fmt::Write`]: [`format_into`].
//! - Encode into a [`io::Write`]: [`write_into`].
//!
//! # Examples
//!
//! ```
//! let data = b"Artichoke Ruby";
//! let mut buf = String::new();
//! # #[cfg(feature = "alloc")]
//! scolapasta_hex::encode_into(data, &mut buf);
//! # #[cfg(not(feature = "alloc"))]
//! # buf.push_str("4172746963686f6b652052756279");
//! assert_eq!(buf, "4172746963686f6b652052756279");
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
//! [Base 16 encoding]: https://tools.ietf.org/html/rfc4648#section-8
//! [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

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
use alloc::string::String;
use core::fmt;
use core::iter::FusedIterator;
use core::slice;
use core::str::Chars;
#[cfg(feature = "std")]
use std::io;

/// Encode arbitrary octets as base16. Returns a [`String`].
///
/// This function allocates a [`String`] and delegates to [`encode_into`].
///
/// # Examples
///
/// ```
/// let data = b"Artichoke Ruby";
/// let buf = scolapasta_hex::encode(data);
/// assert_eq!(buf, "4172746963686f6b652052756279");
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    let mut buf = String::new();
    encode_into(data.as_ref(), &mut buf);
    buf
}

/// Encode arbitrary octets as base16 into the given [`String`].
///
/// This function writes encoded octets into the given `String`. This function
/// will allocate at most once.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// # use alloc::string::String;
/// let data = b"Artichoke Ruby";
/// let mut buf = String::new();
/// scolapasta_hex::encode_into(data, &mut buf);
/// assert_eq!(buf, "4172746963686f6b652052756279");
/// ```
#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn encode_into<T: AsRef<[u8]>>(data: T, buf: &mut String) {
    let data = data.as_ref();
    let iter = Hex::from(data);
    buf.reserve(iter.len());
    for ch in iter {
        buf.push(ch);
    }
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
    #[allow(clippy::too_many_lines)]
    const fn hex_escape(value: u8) -> &'static str {
        match value {
            0 => "00",
            1 => "01",
            2 => "02",
            3 => "03",
            4 => "04",
            5 => "05",
            6 => "06",
            7 => "07",
            8 => "08",
            9 => "09",
            10 => "0a",
            11 => "0b",
            12 => "0c",
            13 => "0d",
            14 => "0e",
            15 => "0f",
            16 => "10",
            17 => "11",
            18 => "12",
            19 => "13",
            20 => "14",
            21 => "15",
            22 => "16",
            23 => "17",
            24 => "18",
            25 => "19",
            26 => "1a",
            27 => "1b",
            28 => "1c",
            29 => "1d",
            30 => "1e",
            31 => "1f",
            32 => "20",
            33 => "21",
            34 => "22",
            35 => "23",
            36 => "24",
            37 => "25",
            38 => "26",
            39 => "27",
            40 => "28",
            41 => "29",
            42 => "2a",
            43 => "2b",
            44 => "2c",
            45 => "2d",
            46 => "2e",
            47 => "2f",
            48 => "30",
            49 => "31",
            50 => "32",
            51 => "33",
            52 => "34",
            53 => "35",
            54 => "36",
            55 => "37",
            56 => "38",
            57 => "39",
            58 => "3a",
            59 => "3b",
            60 => "3c",
            61 => "3d",
            62 => "3e",
            63 => "3f",
            64 => "40",
            65 => "41",
            66 => "42",
            67 => "43",
            68 => "44",
            69 => "45",
            70 => "46",
            71 => "47",
            72 => "48",
            73 => "49",
            74 => "4a",
            75 => "4b",
            76 => "4c",
            77 => "4d",
            78 => "4e",
            79 => "4f",
            80 => "50",
            81 => "51",
            82 => "52",
            83 => "53",
            84 => "54",
            85 => "55",
            86 => "56",
            87 => "57",
            88 => "58",
            89 => "59",
            90 => "5a",
            91 => "5b",
            92 => "5c",
            93 => "5d",
            94 => "5e",
            95 => "5f",
            96 => "60",
            97 => "61",
            98 => "62",
            99 => "63",
            100 => "64",
            101 => "65",
            102 => "66",
            103 => "67",
            104 => "68",
            105 => "69",
            106 => "6a",
            107 => "6b",
            108 => "6c",
            109 => "6d",
            110 => "6e",
            111 => "6f",
            112 => "70",
            113 => "71",
            114 => "72",
            115 => "73",
            116 => "74",
            117 => "75",
            118 => "76",
            119 => "77",
            120 => "78",
            121 => "79",
            122 => "7a",
            123 => "7b",
            124 => "7c",
            125 => "7d",
            126 => "7e",
            127 => "7f",
            128 => "80",
            129 => "81",
            130 => "82",
            131 => "83",
            132 => "84",
            133 => "85",
            134 => "86",
            135 => "87",
            136 => "88",
            137 => "89",
            138 => "8a",
            139 => "8b",
            140 => "8c",
            141 => "8d",
            142 => "8e",
            143 => "8f",
            144 => "90",
            145 => "91",
            146 => "92",
            147 => "93",
            148 => "94",
            149 => "95",
            150 => "96",
            151 => "97",
            152 => "98",
            153 => "99",
            154 => "9a",
            155 => "9b",
            156 => "9c",
            157 => "9d",
            158 => "9e",
            159 => "9f",
            160 => "a0",
            161 => "a1",
            162 => "a2",
            163 => "a3",
            164 => "a4",
            165 => "a5",
            166 => "a6",
            167 => "a7",
            168 => "a8",
            169 => "a9",
            170 => "aa",
            171 => "ab",
            172 => "ac",
            173 => "ad",
            174 => "ae",
            175 => "af",
            176 => "b0",
            177 => "b1",
            178 => "b2",
            179 => "b3",
            180 => "b4",
            181 => "b5",
            182 => "b6",
            183 => "b7",
            184 => "b8",
            185 => "b9",
            186 => "ba",
            187 => "bb",
            188 => "bc",
            189 => "bd",
            190 => "be",
            191 => "bf",
            192 => "c0",
            193 => "c1",
            194 => "c2",
            195 => "c3",
            196 => "c4",
            197 => "c5",
            198 => "c6",
            199 => "c7",
            200 => "c8",
            201 => "c9",
            202 => "ca",
            203 => "cb",
            204 => "cc",
            205 => "cd",
            206 => "ce",
            207 => "cf",
            208 => "d0",
            209 => "d1",
            210 => "d2",
            211 => "d3",
            212 => "d4",
            213 => "d5",
            214 => "d6",
            215 => "d7",
            216 => "d8",
            217 => "d9",
            218 => "da",
            219 => "db",
            220 => "dc",
            221 => "dd",
            222 => "de",
            223 => "df",
            224 => "e0",
            225 => "e1",
            226 => "e2",
            227 => "e3",
            228 => "e4",
            229 => "e5",
            230 => "e6",
            231 => "e7",
            232 => "e8",
            233 => "e9",
            234 => "ea",
            235 => "eb",
            236 => "ec",
            237 => "ed",
            238 => "ee",
            239 => "ef",
            240 => "f0",
            241 => "f1",
            242 => "f2",
            243 => "f3",
            244 => "f4",
            245 => "f5",
            246 => "f6",
            247 => "f7",
            248 => "f8",
            249 => "f9",
            250 => "fa",
            251 => "fb",
            252 => "fc",
            253 => "fd",
            254 => "fe",
            255 => "ff",
        }
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
                0x0 => assert_eq!(left, '0'),
                0x1 => assert_eq!(left, '1'),
                0x2 => assert_eq!(left, '2'),
                0x3 => assert_eq!(left, '3'),
                0x4 => assert_eq!(left, '4'),
                0x5 => assert_eq!(left, '5'),
                0x6 => assert_eq!(left, '6'),
                0x7 => assert_eq!(left, '7'),
                0x8 => assert_eq!(left, '8'),
                0x9 => assert_eq!(left, '9'),
                0xA => assert_eq!(left, 'a'),
                0xB => assert_eq!(left, 'b'),
                0xC => assert_eq!(left, 'c'),
                0xD => assert_eq!(left, 'd'),
                0xE => assert_eq!(left, 'e'),
                0xF => assert_eq!(left, 'f'),
                tuple => panic!("unknown top 16th: {}, from byte: {}", tuple, byte),
            }

            let right = lit.next().unwrap();
            let bottom = byte & 0xF;
            match bottom {
                0x0 => assert_eq!(right, '0'),
                0x1 => assert_eq!(right, '1'),
                0x2 => assert_eq!(right, '2'),
                0x3 => assert_eq!(right, '3'),
                0x4 => assert_eq!(right, '4'),
                0x5 => assert_eq!(right, '5'),
                0x6 => assert_eq!(right, '6'),
                0x7 => assert_eq!(right, '7'),
                0x8 => assert_eq!(right, '8'),
                0x9 => assert_eq!(right, '9'),
                0xA => assert_eq!(right, 'a'),
                0xB => assert_eq!(right, 'b'),
                0xC => assert_eq!(right, 'c'),
                0xD => assert_eq!(right, 'd'),
                0xE => assert_eq!(right, 'e'),
                0xF => assert_eq!(right, 'f'),
                tuple => panic!("unknown bottom 16th: {}, from byte: {}", tuple, byte),
            }
            assert!(lit.next().is_none());
        }
    }

    #[cfg(feature = "alloc")]
    mod alloc {
        use alloc::string::String;

        use crate::{encode, encode_into, format_into, Hex};

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_encode() {
            // BASE16("") = ""
            assert_eq!(encode(""), "");

            // BASE16("f") = "66"
            assert_eq!(encode("f"), "66");

            // BASE16("fo") = "666F"
            assert_eq!(encode("fo"), "666f");

            // BASE16("foo") = "666F6F"
            assert_eq!(encode("foo"), "666f6f");

            // BASE16("foob") = "666F6F62"
            assert_eq!(encode("foob"), "666f6f62");

            // BASE16("fooba") = "666F6F6261"
            assert_eq!(encode("fooba"), "666f6f6261");

            // BASE16("foobar") = "666F6F626172"
            assert_eq!(encode("foobar"), "666f6f626172");
        }

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_hex_iter() {
            // BASE16("") = ""
            assert_eq!(Hex::from("").collect::<String>(), "");

            // BASE16("f") = "66"
            assert_eq!(Hex::from("f").collect::<String>(), "66");

            // BASE16("fo") = "666F"
            assert_eq!(Hex::from("fo").collect::<String>(), "666f");

            // BASE16("foo") = "666F6F"
            assert_eq!(Hex::from("foo").collect::<String>(), "666f6f");

            // BASE16("foob") = "666F6F62"
            assert_eq!(Hex::from("foob").collect::<String>(), "666f6f62");

            // BASE16("fooba") = "666F6F6261"
            assert_eq!(Hex::from("fooba").collect::<String>(), "666f6f6261");

            // BASE16("foobar") = "666F6F626172"
            assert_eq!(Hex::from("foobar").collect::<String>(), "666f6f626172");
        }

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_encode_into_string() {
            // BASE16("") = ""
            let mut s = String::new();
            encode_into("", &mut s);
            assert_eq!(s, "");
            assert_eq!(s.capacity(), 0);

            // BASE16("f") = "66"
            let mut s = String::new();
            encode_into("f", &mut s);
            assert_eq!(s, "66");
            assert!(s.capacity() >= 2);

            // BASE16("fo") = "666F"
            let mut s = String::new();
            encode_into("fo", &mut s);
            assert_eq!(s, "666f");
            assert!(s.capacity() >= 4);

            // BASE16("foo") = "666F6F"
            let mut s = String::new();
            encode_into("foo", &mut s);
            assert_eq!(s, "666f6f");
            assert!(s.capacity() >= 6);

            // BASE16("foob") = "666F6F62"
            let mut s = String::new();
            encode_into("foob", &mut s);
            assert_eq!(s, "666f6f62");
            assert!(s.capacity() >= 8);

            // BASE16("fooba") = "666F6F6261"
            let mut s = String::new();
            encode_into("fooba", &mut s);
            assert_eq!(s, "666f6f6261");
            assert!(s.capacity() >= 10);

            // BASE16("foobar") = "666F6F626172"
            let mut s = String::new();
            encode_into("foobar", &mut s);
            assert_eq!(s, "666f6f626172");
            assert!(s.capacity() >= 12);
        }

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn rfc4648_test_vectors_format_into() {
            // BASE16("") = ""
            let mut fmt = String::new();
            format_into("", &mut fmt).unwrap();
            assert_eq!(fmt, "");

            // BASE16("f") = "66"
            let mut fmt = String::new();
            format_into("f", &mut fmt).unwrap();
            assert_eq!(fmt, "66");

            // BASE16("fo") = "666F"
            let mut fmt = String::new();
            format_into("fo", &mut fmt).unwrap();
            assert_eq!(fmt, "666f");

            // BASE16("foo") = "666F6F"
            let mut fmt = String::new();
            format_into("foo", &mut fmt).unwrap();
            assert_eq!(fmt, "666f6f");

            // BASE16("foob") = "666F6F62"
            let mut fmt = String::new();
            format_into("foob", &mut fmt).unwrap();
            assert_eq!(fmt, "666f6f62");

            // BASE16("fooba") = "666F6F6261"
            let mut fmt = String::new();
            format_into("fooba", &mut fmt).unwrap();
            assert_eq!(fmt, "666f6f6261");

            // BASE16("foobar") = "666F6F626172"
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
            // BASE16("") = ""
            let mut write = Vec::new();
            write_into("", &mut write).unwrap();
            assert_eq!(write, b"".to_vec());

            // BASE16("f") = "66"
            let mut write = Vec::new();
            write_into("f", &mut write).unwrap();
            assert_eq!(write, b"66".to_vec());

            // BASE16("fo") = "666F"
            let mut write = Vec::new();
            write_into("fo", &mut write).unwrap();
            assert_eq!(write, b"666f".to_vec());

            // BASE16("foo") = "666F6F"
            let mut write = Vec::new();
            write_into("foo", &mut write).unwrap();
            assert_eq!(write, b"666f6f".to_vec());

            // BASE16("foob") = "666F6F62"
            let mut write = Vec::new();
            write_into("foob", &mut write).unwrap();
            assert_eq!(write, b"666f6f62".to_vec());

            // BASE16("fooba") = "666F6F6261"
            let mut write = Vec::new();
            write_into("fooba", &mut write).unwrap();
            assert_eq!(write, b"666f6f6261".to_vec());

            // BASE16("foobar") = "666F6F626172"
            let mut write = Vec::new();
            write_into("foobar", &mut write).unwrap();
            assert_eq!(write, b"666f6f626172".to_vec());
        }
    }
}
