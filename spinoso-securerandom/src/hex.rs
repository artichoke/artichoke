//! Functions for encoding sequences of bytes into base 16 hex encoding.
//!
//! [Base 16 encoding] is an encoding scheme that uses a 16 character ASCII
//! alphabet for encoding arbitrary octets.
//!
//! This module offers encoders that:
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
//! spinoso_securerandom::hex::encode_into(data, &mut buf);
//! assert_eq!(buf, "4172746963686f6b652052756279");
//! ```
//!
//! This module also exposes an iterator:
//!
//! ```
//! # use spinoso_securerandom::hex::Hex;
//! let data = "Artichoke Ruby";
//! let iter = Hex::from(data);
//! assert_eq!(iter.collect::<String>(), "4172746963686f6b652052756279");
//! ```
//!
//! [Base 16 encoding]: https://tools.ietf.org/html/rfc4648#section-8
//! [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

use core::fmt;
use core::iter::FusedIterator;
use core::slice;
use std::io;

/// Encode arbitrary octets as base16. Returns a [`String`].
///
/// This function allocates a [`String`] and delegates to [`encode_into`].
///
/// # Examples
///
/// ```
/// let data = b"Artichoke Ruby";
/// let buf = spinoso_securerandom::hex::encode(data);
/// assert_eq!(buf, "4172746963686f6b652052756279");
/// ```
#[inline]
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
/// spinoso_securerandom::hex::encode_into(data, &mut buf);
/// assert_eq!(buf, "4172746963686f6b652052756279");
/// ```
#[inline]
pub fn encode_into<T: AsRef<[u8]>>(data: T, buf: &mut String) {
    let data = data.as_ref();
    let iter = Hex::from(data);
    buf.reserve(iter.clone().count());
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
/// spinoso_securerandom::hex::format_into(data, &mut buf);
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
/// spinoso_securerandom::hex::write_into(data, &mut buf);
/// assert_eq!(buf, b"4172746963686f6b652052756279".to_vec());
/// ```
///
/// # Errors
///
/// If the destination returns an error, that error is returned.
#[inline]
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
/// # use spinoso_securerandom::hex::Hex;
/// let data = "Artichoke Ruby";
/// let iter = Hex::from(data);
/// assert_eq!(iter.collect::<String>(), "4172746963686f6b652052756279");
/// ```
#[derive(Debug, Clone)]
pub struct Hex<'a> {
    iter: slice::Iter<'a, u8>,
    next: Option<u8>,
}

impl<'a> Hex<'a> {
    /// Returns the number of remaining hex encoded characters in the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::hex::Hex;
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
        if self.next.is_some() {
            let remaining_bytes = self.iter.as_slice().len();
            // Every byte expands to two hexadecimal ASCII `char`s.
            let remaining_bytes_encoded_len = remaining_bytes.saturating_mul(2);
            // Add one for the dangling char from the `EncodedByte` iterator.
            remaining_bytes_encoded_len.saturating_add(1)
        } else {
            let remaining_bytes = self.iter.as_slice().len();
            // Every byte expands to two hexadecimal ASCII `char`s.
            // the only data remaining is unencoded bytes in the slice.
            remaining_bytes.saturating_mul(2)
        }
    }

    /// Returns `true` if the iterator will yield no more hex encoded characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::hex::Hex;
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
        self.iter.as_slice().is_empty() && self.next.is_none()
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
            next: None,
        }
    }
}

impl<'a> Iterator for Hex<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.next.take() {
            return Some(char::from(byte));
        }
        let byte = self.iter.next().copied()?;
        let mut encoded = EncodedByte::from(byte);
        if let Some([current, next]) = encoded.next() {
            self.next = Some(next);
            return Some(char::from(current));
        }
        None
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
        if let Some([_, last]) = EncodedByte::from(byte).last() {
            Some(char::from(last))
        } else {
            None
        }
    }
}

impl<'a> FusedIterator for Hex<'a> {}

impl<'a> ExactSizeIterator for Hex<'a> {}

#[derive(Debug, Clone)]
#[must_use = "this `EncodedByte` is an `Iterator`, which should be consumed if constructed"]
struct EncodedByte(Option<[u8; 2]>);

impl From<u8> for EncodedByte {
    /// Map from a `u8` to a hex encoded string literal.
    ///
    /// For example, `00`, `20` or `ff`.
    #[allow(clippy::too_many_lines)]
    fn from(value: u8) -> Self {
        let encoded = match value {
            0 => *b"00",
            1 => *b"01",
            2 => *b"02",
            3 => *b"03",
            4 => *b"04",
            5 => *b"05",
            6 => *b"06",
            7 => *b"07",
            8 => *b"08",
            9 => *b"09",
            10 => *b"0a",
            11 => *b"0b",
            12 => *b"0c",
            13 => *b"0d",
            14 => *b"0e",
            15 => *b"0f",
            16 => *b"10",
            17 => *b"11",
            18 => *b"12",
            19 => *b"13",
            20 => *b"14",
            21 => *b"15",
            22 => *b"16",
            23 => *b"17",
            24 => *b"18",
            25 => *b"19",
            26 => *b"1a",
            27 => *b"1b",
            28 => *b"1c",
            29 => *b"1d",
            30 => *b"1e",
            31 => *b"1f",
            32 => *b"20",
            33 => *b"21",
            34 => *b"22",
            35 => *b"23",
            36 => *b"24",
            37 => *b"25",
            38 => *b"26",
            39 => *b"27",
            40 => *b"28",
            41 => *b"29",
            42 => *b"2a",
            43 => *b"2b",
            44 => *b"2c",
            45 => *b"2d",
            46 => *b"2e",
            47 => *b"2f",
            48 => *b"30",
            49 => *b"31",
            50 => *b"32",
            51 => *b"33",
            52 => *b"34",
            53 => *b"35",
            54 => *b"36",
            55 => *b"37",
            56 => *b"38",
            57 => *b"39",
            58 => *b"3a",
            59 => *b"3b",
            60 => *b"3c",
            61 => *b"3d",
            62 => *b"3e",
            63 => *b"3f",
            64 => *b"40",
            65 => *b"41",
            66 => *b"42",
            67 => *b"43",
            68 => *b"44",
            69 => *b"45",
            70 => *b"46",
            71 => *b"47",
            72 => *b"48",
            73 => *b"49",
            74 => *b"4a",
            75 => *b"4b",
            76 => *b"4c",
            77 => *b"4d",
            78 => *b"4e",
            79 => *b"4f",
            80 => *b"50",
            81 => *b"51",
            82 => *b"52",
            83 => *b"53",
            84 => *b"54",
            85 => *b"55",
            86 => *b"56",
            87 => *b"57",
            88 => *b"58",
            89 => *b"59",
            90 => *b"5a",
            91 => *b"5b",
            92 => *b"5c",
            93 => *b"5d",
            94 => *b"5e",
            95 => *b"5f",
            96 => *b"60",
            97 => *b"61",
            98 => *b"62",
            99 => *b"63",
            100 => *b"64",
            101 => *b"65",
            102 => *b"66",
            103 => *b"67",
            104 => *b"68",
            105 => *b"69",
            106 => *b"6a",
            107 => *b"6b",
            108 => *b"6c",
            109 => *b"6d",
            110 => *b"6e",
            111 => *b"6f",
            112 => *b"70",
            113 => *b"71",
            114 => *b"72",
            115 => *b"73",
            116 => *b"74",
            117 => *b"75",
            118 => *b"76",
            119 => *b"77",
            120 => *b"78",
            121 => *b"79",
            122 => *b"7a",
            123 => *b"7b",
            124 => *b"7c",
            125 => *b"7d",
            126 => *b"7e",
            127 => *b"7f",
            128 => *b"80",
            129 => *b"81",
            130 => *b"82",
            131 => *b"83",
            132 => *b"84",
            133 => *b"85",
            134 => *b"86",
            135 => *b"87",
            136 => *b"88",
            137 => *b"89",
            138 => *b"8a",
            139 => *b"8b",
            140 => *b"8c",
            141 => *b"8d",
            142 => *b"8e",
            143 => *b"8f",
            144 => *b"90",
            145 => *b"91",
            146 => *b"92",
            147 => *b"93",
            148 => *b"94",
            149 => *b"95",
            150 => *b"96",
            151 => *b"97",
            152 => *b"98",
            153 => *b"99",
            154 => *b"9a",
            155 => *b"9b",
            156 => *b"9c",
            157 => *b"9d",
            158 => *b"9e",
            159 => *b"9f",
            160 => *b"a0",
            161 => *b"a1",
            162 => *b"a2",
            163 => *b"a3",
            164 => *b"a4",
            165 => *b"a5",
            166 => *b"a6",
            167 => *b"a7",
            168 => *b"a8",
            169 => *b"a9",
            170 => *b"aa",
            171 => *b"ab",
            172 => *b"ac",
            173 => *b"ad",
            174 => *b"ae",
            175 => *b"af",
            176 => *b"b0",
            177 => *b"b1",
            178 => *b"b2",
            179 => *b"b3",
            180 => *b"b4",
            181 => *b"b5",
            182 => *b"b6",
            183 => *b"b7",
            184 => *b"b8",
            185 => *b"b9",
            186 => *b"ba",
            187 => *b"bb",
            188 => *b"bc",
            189 => *b"bd",
            190 => *b"be",
            191 => *b"bf",
            192 => *b"c0",
            193 => *b"c1",
            194 => *b"c2",
            195 => *b"c3",
            196 => *b"c4",
            197 => *b"c5",
            198 => *b"c6",
            199 => *b"c7",
            200 => *b"c8",
            201 => *b"c9",
            202 => *b"ca",
            203 => *b"cb",
            204 => *b"cc",
            205 => *b"cd",
            206 => *b"ce",
            207 => *b"cf",
            208 => *b"d0",
            209 => *b"d1",
            210 => *b"d2",
            211 => *b"d3",
            212 => *b"d4",
            213 => *b"d5",
            214 => *b"d6",
            215 => *b"d7",
            216 => *b"d8",
            217 => *b"d9",
            218 => *b"da",
            219 => *b"db",
            220 => *b"dc",
            221 => *b"dd",
            222 => *b"de",
            223 => *b"df",
            224 => *b"e0",
            225 => *b"e1",
            226 => *b"e2",
            227 => *b"e3",
            228 => *b"e4",
            229 => *b"e5",
            230 => *b"e6",
            231 => *b"e7",
            232 => *b"e8",
            233 => *b"e9",
            234 => *b"ea",
            235 => *b"eb",
            236 => *b"ec",
            237 => *b"ed",
            238 => *b"ee",
            239 => *b"ef",
            240 => *b"f0",
            241 => *b"f1",
            242 => *b"f2",
            243 => *b"f3",
            244 => *b"f4",
            245 => *b"f5",
            246 => *b"f6",
            247 => *b"f7",
            248 => *b"f8",
            249 => *b"f9",
            250 => *b"fa",
            251 => *b"fb",
            252 => *b"fc",
            253 => *b"fd",
            254 => *b"fe",
            255 => *b"ff",
        };
        Self(Some(encoded))
    }
}

impl Iterator for EncodedByte {
    type Item = [u8; 2];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n == 0 {
            self.0.take()
        } else {
            None
        }
    }

    #[inline]
    fn count(self) -> usize {
        if self.0.is_some() {
            1
        } else {
            0
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = if self.0.is_some() { 1 } else { 0 };
        (size, Some(size))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        // For a size == 1 iterator, `next` and `last` are the same operation.
        self.next()
    }
}

impl DoubleEndedIterator for EncodedByte {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // For a size == 1 iterator, `next` and `next_back` are the same
        // operation.
        self.next()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        // For a size == 1 iterator, `nth` and `nth_back` are the same
        // operation.
        self.nth(n)
    }
}

impl FusedIterator for EncodedByte {}

impl ExactSizeIterator for EncodedByte {}

#[cfg(test)]
mod tests {
    use super::{encode, encode_into, format_into, write_into, EncodedByte, Hex};

    #[test]
    fn literal_exhaustive() {
        for byte in 0..=255 {
            let mut lit = EncodedByte::from(byte);
            let [left, right] = lit.next().unwrap();
            let top = byte >> 4;
            match top {
                0x0 => assert_eq!(left, b'0'),
                0x1 => assert_eq!(left, b'1'),
                0x2 => assert_eq!(left, b'2'),
                0x3 => assert_eq!(left, b'3'),
                0x4 => assert_eq!(left, b'4'),
                0x5 => assert_eq!(left, b'5'),
                0x6 => assert_eq!(left, b'6'),
                0x7 => assert_eq!(left, b'7'),
                0x8 => assert_eq!(left, b'8'),
                0x9 => assert_eq!(left, b'9'),
                0xA => assert_eq!(left, b'a'),
                0xB => assert_eq!(left, b'b'),
                0xC => assert_eq!(left, b'c'),
                0xD => assert_eq!(left, b'd'),
                0xE => assert_eq!(left, b'e'),
                0xF => assert_eq!(left, b'f'),
                tuple => panic!("unknown top 16th: {}, from byte: {}", tuple, byte),
            }
            let bottom = byte & 0xF;
            match bottom {
                0x0 => assert_eq!(right, b'0'),
                0x1 => assert_eq!(right, b'1'),
                0x2 => assert_eq!(right, b'2'),
                0x3 => assert_eq!(right, b'3'),
                0x4 => assert_eq!(right, b'4'),
                0x5 => assert_eq!(right, b'5'),
                0x6 => assert_eq!(right, b'6'),
                0x7 => assert_eq!(right, b'7'),
                0x8 => assert_eq!(right, b'8'),
                0x9 => assert_eq!(right, b'9'),
                0xA => assert_eq!(right, b'a'),
                0xB => assert_eq!(right, b'b'),
                0xC => assert_eq!(right, b'c'),
                0xD => assert_eq!(right, b'd'),
                0xE => assert_eq!(right, b'e'),
                0xF => assert_eq!(right, b'f'),
                tuple => panic!("unknown bottom 16th: {}, from byte: {}", tuple, byte),
            }
            assert!(lit.next().is_none());
        }
    }

    // https://tools.ietf.org/html/rfc4648#section-10
    #[test]
    fn rfc4648_test_vectors() {
        // BASE16("") = ""
        assert_eq!(encode(""), "");
        assert_eq!(Hex::from("").collect::<String>(), "");
        let mut s = String::new();
        encode_into("", &mut s);
        assert_eq!(s, "");
        assert_eq!(s.capacity(), 0);
        let mut fmt = String::new();
        format_into("", &mut fmt).unwrap();
        assert_eq!(fmt, "");
        let mut write = Vec::new();
        write_into("", &mut write).unwrap();
        assert_eq!(write, b"".to_vec());

        // BASE16("f") = "66"
        assert_eq!(encode("f"), "66");
        assert_eq!(Hex::from("f").collect::<String>(), "66");
        let mut s = String::new();
        encode_into("f", &mut s);
        assert_eq!(s, "66");
        assert!(s.capacity() >= 2);
        let mut fmt = String::new();
        format_into("f", &mut fmt).unwrap();
        assert_eq!(fmt, "66");
        let mut write = Vec::new();
        write_into("f", &mut write).unwrap();
        assert_eq!(write, b"66".to_vec());

        // BASE16("fo") = "666F"
        assert_eq!(encode("fo"), "666f");
        assert_eq!(Hex::from("fo").collect::<String>(), "666f");
        let mut s = String::new();
        encode_into("fo", &mut s);
        assert_eq!(s, "666f");
        assert!(s.capacity() >= 4);
        let mut fmt = String::new();
        format_into("fo", &mut fmt).unwrap();
        assert_eq!(fmt, "666f");
        let mut write = Vec::new();
        write_into("fo", &mut write).unwrap();
        assert_eq!(write, b"666f".to_vec());

        // BASE16("foo") = "666F6F"
        assert_eq!(encode("foo"), "666f6f");
        assert_eq!(Hex::from("foo").collect::<String>(), "666f6f");
        let mut s = String::new();
        encode_into("foo", &mut s);
        assert_eq!(s, "666f6f");
        assert!(s.capacity() >= 6);
        let mut fmt = String::new();
        format_into("foo", &mut fmt).unwrap();
        assert_eq!(fmt, "666f6f");
        let mut write = Vec::new();
        write_into("foo", &mut write).unwrap();
        assert_eq!(write, b"666f6f".to_vec());

        // BASE16("foob") = "666F6F62"
        assert_eq!(encode("foob"), "666f6f62");
        assert_eq!(Hex::from("foob").collect::<String>(), "666f6f62");
        let mut s = String::new();
        encode_into("foob", &mut s);
        assert_eq!(s, "666f6f62");
        assert!(s.capacity() >= 8);
        let mut fmt = String::new();
        format_into("foob", &mut fmt).unwrap();
        assert_eq!(fmt, "666f6f62");
        let mut write = Vec::new();
        write_into("foob", &mut write).unwrap();
        assert_eq!(write, b"666f6f62".to_vec());

        // BASE16("fooba") = "666F6F6261"
        assert_eq!(encode("fooba"), "666f6f6261");
        assert_eq!(Hex::from("fooba").collect::<String>(), "666f6f6261");
        let mut s = String::new();
        encode_into("fooba", &mut s);
        assert_eq!(s, "666f6f6261");
        assert!(s.capacity() >= 10);
        let mut fmt = String::new();
        format_into("fooba", &mut fmt).unwrap();
        assert_eq!(fmt, "666f6f6261");
        let mut write = Vec::new();
        write_into("fooba", &mut write).unwrap();
        assert_eq!(write, b"666f6f6261".to_vec());

        // BASE16("foobar") = "666F6F626172"
        assert_eq!(encode("foobar"), "666f6f626172");
        assert_eq!(Hex::from("foobar").collect::<String>(), "666f6f626172");
        let mut s = String::new();
        encode_into("foobar", &mut s);
        assert_eq!(s, "666f6f626172");
        assert!(s.capacity() >= 12);
        let mut fmt = String::new();
        format_into("foobar", &mut fmt).unwrap();
        assert_eq!(fmt, "666f6f626172");
        let mut write = Vec::new();
        write_into("foobar", &mut write).unwrap();
        assert_eq!(write, b"666f6f626172".to_vec());
    }
}
