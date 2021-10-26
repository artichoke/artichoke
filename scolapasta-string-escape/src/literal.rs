use core::fmt;
use core::iter::FusedIterator;
use core::slice;
use core::str;

/// Returns whether a [`char`] is ASCII and has a literal escape code.
///
/// Control characters in the range `0x00..=0x1F`, `"`, `\` and `DEL` have
/// non-trivial escapes.
///
/// # Examples
///
/// ```
/// # use core::char::REPLACEMENT_CHARACTER;
/// # use scolapasta_string_escape::is_ascii_char_with_escape;
/// assert!(is_ascii_char_with_escape('\x00'));
/// assert!(is_ascii_char_with_escape('"'));
/// assert!(is_ascii_char_with_escape('\\'));
///
/// assert!(!is_ascii_char_with_escape('a'));
/// assert!(!is_ascii_char_with_escape('Z'));
/// assert!(!is_ascii_char_with_escape(';'));
/// assert!(!is_ascii_char_with_escape('💎'));
/// assert!(!is_ascii_char_with_escape(REPLACEMENT_CHARACTER));
/// ```
#[inline]
#[must_use]
pub const fn is_ascii_char_with_escape(ch: char) -> bool {
    if !ch.is_ascii() {
        return false;
    }
    let [ascii_byte, _, _, _] = (ch as u32).to_le_bytes();
    let escape = Literal::debug_escape(ascii_byte);
    escape.len() > 1
}

/// Iterator of Ruby debug escape sequences for a byte.
///
/// This iterator's item type is [`char`].
///
/// Non printable bytes like `0xFF` or `0x0C` are escaped to `\xFF` or `\f`.
///
/// ASCII printable characters are passed through as is unless they are `"` or
/// `\` since these fields are used to delimit strings and escape sequences.
///
/// # Usage notes
///
/// This iterator operates on individual bytes, which makes it unsuitable for
/// debug printing a conventionally UTF-8 bytestring on its own. See
/// [`format_debug_escape_into`] to debug format an entire bytestring.
///
/// # Examples
///
/// Printable ASCII characters are passed through unescaped:
///
/// ```
/// # use scolapasta_string_escape::Literal;
/// let literal = Literal::from(b'a');
/// assert_eq!(literal.collect::<String>(), "a");
///
/// let literal = Literal::from(b';');
/// assert_eq!(literal.collect::<String>(), ";");
/// ```
///
/// `"` and `\` are escaped:
///
/// ```
/// # use scolapasta_string_escape::Literal;
/// let literal = Literal::from(b'"');
/// assert_eq!(literal.collect::<String>(), r#"\""#);
///
/// let literal = Literal::from(b'\\');
/// assert_eq!(literal.collect::<String>(), r"\\");
/// ```
///
/// ASCII control characters are escaped:
///
/// ```
/// # use scolapasta_string_escape::Literal;
/// let literal = Literal::from(b'\0');
/// assert_eq!(literal.collect::<String>(), r"\x00");
///
/// let literal = Literal::from(b'\x0A');
/// assert_eq!(literal.collect::<String>(), r"\n");
///
/// let literal = Literal::from(b'\x0C');
/// assert_eq!(literal.collect::<String>(), r"\f");
///
/// let literal = Literal::from(b'\x7F');
/// assert_eq!(literal.collect::<String>(), r"\x7F");
/// ```
///
/// UTF-8 invalid bytes are escaped:
///
/// ```
/// # use scolapasta_string_escape::Literal;
/// let literal = Literal::from(b'\xFF');
/// assert_eq!(literal.collect::<String>(), r"\xFF");
/// ```
///
/// [`format_debug_escape_into`]: crate::format_debug_escape_into
#[derive(Debug, Clone)]
#[must_use = "this `Literal` is an `Iterator`, which should be consumed if constructed"]
pub struct Literal(slice::Iter<'static, u8>);

impl Default for Literal {
    fn default() -> Self {
        Self::empty()
    }
}

impl Literal {
    /// Create an empty literal iterator.
    ///
    /// The returned `Literal` always yields [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::Literal;
    ///
    /// let mut literal = Literal::empty();
    /// assert_eq!(literal.as_str(), "");
    /// assert_eq!(literal.next(), None);
    /// ```
    pub fn empty() -> Self {
        Literal(b"".iter())
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has `'static` lifetime, and so the iterator can continue to be used
    /// while this exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::Literal;
    /// let mut literal = Literal::from(b'\0');
    ///
    /// assert_eq!(literal.as_str(), r"\x00");
    /// literal.next();
    /// assert_eq!(literal.as_str(), "x00");
    /// literal.next();
    /// literal.next();
    /// assert_eq!(literal.as_str(), "0");
    /// literal.next();
    /// assert_eq!(literal.as_str(), "");
    /// ```
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        str::from_utf8(self.0.as_slice()).unwrap_or_default()
    }

    /// Return the debug escape code for the given byte.
    ///
    /// Debug escapes can be hex escapes (`\xFF`), control character escapes
    /// (`\e`), or escape sequences for debug printing (`\"` or `\\`).
    ///
    /// Printable ASCII characters that do not have escape sequences are passed
    /// through untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::Literal;
    /// assert_eq!(Literal::debug_escape(255), r"\xFF");
    /// assert_eq!(Literal::debug_escape(0x1B), r"\e");
    /// assert_eq!(Literal::debug_escape(b'"'), r#"\""#);
    /// assert_eq!(Literal::debug_escape(b'\\'), r"\\");
    /// assert_eq!(Literal::debug_escape(b'a'), "a");
    /// ```
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub const fn debug_escape(byte: u8) -> &'static str {
        // Some control character bytes escape to non-hex literals:
        //
        // ```console
        // [2.6.3] > :"\x00"
        // => :"\x00"
        // [2.6.3] > :"\x01"
        // => :"\x01"
        // [2.6.3] > :"\x02"
        // => :"\x02"
        // [2.6.3] > :"\x03"
        // => :"\x03"
        // [2.6.3] > :"\x04"
        // => :"\x04"
        // [2.6.3] > :"\x05"
        // => :"\x05"
        // [2.6.3] > :"\x06"
        // => :"\x06"
        // [2.6.3] > :"\x07"
        // => :"\a"
        // [2.6.3] > :"\x08"
        // => :"\b"
        // [2.6.3] > :"\x09"
        // => :"\t"
        // [2.6.3] > :"\x0A"
        // => :"\n"
        // [2.6.3] > :"\x0B"
        // => :"\v"
        // [2.6.3] > :"\x0C"
        // => :"\f"
        // [2.6.3] > :"\x0D"
        // => :"\r"
        // [2.6.3] > :"\x0E"
        // => :"\x0E"
        // [2.6.3] > :"\x0F"
        // => :"\x0F"
        // [2.6.3] > :"\x10"
        // => :"\x10"
        // [2.6.3] > :"\x11"
        // => :"\x11"
        // [2.6.3] > :"\x12"
        // => :"\x12"
        // [2.6.3] > :"\x13"
        // => :"\x13"
        // [2.6.3] > :"\x14"
        // => :"\x14"
        // [2.6.3] > :"\x15"
        // => :"\x15"
        // [2.6.3] > :"\x16"
        // => :"\x16"
        // [2.6.3] > :"\x17"
        // => :"\x17"
        // [2.6.3] > :"\x18"
        // => :"\x18"
        // [2.6.3] > :"\x19"
        // => :"\x19"
        // [2.6.3] > :"\x1A"
        // => :"\x1A"
        // [2.6.3] > :"\x1B"
        // => :"\e"
        // [2.6.3] > :"\x1C"
        // => :"\x1C"
        // [2.6.3] > :"\x1D"
        // => :"\x1D"
        // [2.6.3] > :"\x1E"
        // => :"\x1E"
        // [2.6.3] > :"\x1F"
        // => :"\x1F"
        // [2.6.3] > :"\x20"
        // => :" "
        // ```
        match byte {
            0 => r"\x00",
            1 => r"\x01",
            2 => r"\x02",
            3 => r"\x03",
            4 => r"\x04",
            5 => r"\x05",
            6 => r"\x06",
            7 => r"\a",
            8 => r"\b",
            9 => r"\t",
            10 => r"\n",
            11 => r"\v",
            12 => r"\f",
            13 => r"\r",
            14 => r"\x0E",
            15 => r"\x0F",
            16 => r"\x10",
            17 => r"\x11",
            18 => r"\x12",
            19 => r"\x13",
            20 => r"\x14",
            21 => r"\x15",
            22 => r"\x16",
            23 => r"\x17",
            24 => r"\x18",
            25 => r"\x19",
            26 => r"\x1A",
            27 => r"\e",
            28 => r"\x1C",
            29 => r"\x1D",
            30 => r"\x1E",
            31 => r"\x1F",
            32 => " ",
            33 => "!",
            // [2.6.3] > '"'.ord
            // => 34
            // [2.6.3] > '"'.ord.to_s(16)
            // => "22"
            // [2.6.3] > :"\x22"
            // => :"\""
            34 => r#"\""#,
            35 => "#",
            36 => "$",
            37 => "%",
            38 => "&",
            39 => "'",
            40 => "(",
            41 => ")",
            42 => "*",
            43 => "+",
            44 => ",",
            45 => "-",
            46 => ".",
            47 => "/",
            48 => "0",
            49 => "1",
            50 => "2",
            51 => "3",
            52 => "4",
            53 => "5",
            54 => "6",
            55 => "7",
            56 => "8",
            57 => "9",
            58 => ":",
            59 => ";",
            60 => "<",
            61 => "=",
            62 => ">",
            63 => "?",
            64 => "@",
            65 => "A",
            66 => "B",
            67 => "C",
            68 => "D",
            69 => "E",
            70 => "F",
            71 => "G",
            72 => "H",
            73 => "I",
            74 => "J",
            75 => "K",
            76 => "L",
            77 => "M",
            78 => "N",
            79 => "O",
            80 => "P",
            81 => "Q",
            82 => "R",
            83 => "S",
            84 => "T",
            85 => "U",
            86 => "V",
            87 => "W",
            88 => "X",
            89 => "Y",
            90 => "Z",
            91 => "[",
            // [2.6.3] > '\\'.ord
            // => 92
            // [2.6.3] > '\\'.ord.to_s(16)
            // => "5c"
            // [2.6.3] > :"\x5C"
            // => :"\\"
            92 => r"\\",
            93 => "]",
            94 => "^",
            95 => "_",
            96 => "`",
            97 => "a",
            98 => "b",
            99 => "c",
            100 => "d",
            101 => "e",
            102 => "f",
            103 => "g",
            104 => "h",
            105 => "i",
            106 => "j",
            107 => "k",
            108 => "l",
            109 => "m",
            110 => "n",
            111 => "o",
            112 => "p",
            113 => "q",
            114 => "r",
            115 => "s",
            116 => "t",
            117 => "u",
            118 => "v",
            119 => "w",
            120 => "x",
            121 => "y",
            122 => "z",
            123 => "{",
            124 => "|",
            125 => "}",
            126 => "~",
            127 => r"\x7F",
            128 => r"\x80",
            129 => r"\x81",
            130 => r"\x82",
            131 => r"\x83",
            132 => r"\x84",
            133 => r"\x85",
            134 => r"\x86",
            135 => r"\x87",
            136 => r"\x88",
            137 => r"\x89",
            138 => r"\x8A",
            139 => r"\x8B",
            140 => r"\x8C",
            141 => r"\x8D",
            142 => r"\x8E",
            143 => r"\x8F",
            144 => r"\x90",
            145 => r"\x91",
            146 => r"\x92",
            147 => r"\x93",
            148 => r"\x94",
            149 => r"\x95",
            150 => r"\x96",
            151 => r"\x97",
            152 => r"\x98",
            153 => r"\x99",
            154 => r"\x9A",
            155 => r"\x9B",
            156 => r"\x9C",
            157 => r"\x9D",
            158 => r"\x9E",
            159 => r"\x9F",
            160 => r"\xA0",
            161 => r"\xA1",
            162 => r"\xA2",
            163 => r"\xA3",
            164 => r"\xA4",
            165 => r"\xA5",
            166 => r"\xA6",
            167 => r"\xA7",
            168 => r"\xA8",
            169 => r"\xA9",
            170 => r"\xAA",
            171 => r"\xAB",
            172 => r"\xAC",
            173 => r"\xAD",
            174 => r"\xAE",
            175 => r"\xAF",
            176 => r"\xB0",
            177 => r"\xB1",
            178 => r"\xB2",
            179 => r"\xB3",
            180 => r"\xB4",
            181 => r"\xB5",
            182 => r"\xB6",
            183 => r"\xB7",
            184 => r"\xB8",
            185 => r"\xB9",
            186 => r"\xBA",
            187 => r"\xBB",
            188 => r"\xBC",
            189 => r"\xBD",
            190 => r"\xBE",
            191 => r"\xBF",
            192 => r"\xC0",
            193 => r"\xC1",
            194 => r"\xC2",
            195 => r"\xC3",
            196 => r"\xC4",
            197 => r"\xC5",
            198 => r"\xC6",
            199 => r"\xC7",
            200 => r"\xC8",
            201 => r"\xC9",
            202 => r"\xCA",
            203 => r"\xCB",
            204 => r"\xCC",
            205 => r"\xCD",
            206 => r"\xCE",
            207 => r"\xCF",
            208 => r"\xD0",
            209 => r"\xD1",
            210 => r"\xD2",
            211 => r"\xD3",
            212 => r"\xD4",
            213 => r"\xD5",
            214 => r"\xD6",
            215 => r"\xD7",
            216 => r"\xD8",
            217 => r"\xD9",
            218 => r"\xDA",
            219 => r"\xDB",
            220 => r"\xDC",
            221 => r"\xDD",
            222 => r"\xDE",
            223 => r"\xDF",
            224 => r"\xE0",
            225 => r"\xE1",
            226 => r"\xE2",
            227 => r"\xE3",
            228 => r"\xE4",
            229 => r"\xE5",
            230 => r"\xE6",
            231 => r"\xE7",
            232 => r"\xE8",
            233 => r"\xE9",
            234 => r"\xEA",
            235 => r"\xEB",
            236 => r"\xEC",
            237 => r"\xED",
            238 => r"\xEE",
            239 => r"\xEF",
            240 => r"\xF0",
            241 => r"\xF1",
            242 => r"\xF2",
            243 => r"\xF3",
            244 => r"\xF4",
            245 => r"\xF5",
            246 => r"\xF6",
            247 => r"\xF7",
            248 => r"\xF8",
            249 => r"\xF9",
            250 => r"\xFA",
            251 => r"\xFB",
            252 => r"\xFC",
            253 => r"\xFD",
            254 => r"\xFE",
            255 => r"\xFF",
        }
    }
}

impl From<u8> for Literal {
    /// Map from a `u8` to a String literal of debug escape code.
    ///
    /// Debug escapes can be hex escapes (`\xFF`), control character escapes
    /// (`\e`), or escape sequences for debug printing (`\"` or `\\`).
    ///
    /// Printable ASCII characters that are not escape sequences are passed
    /// through untouched.
    #[inline]
    fn from(byte: u8) -> Self {
        let escape = Self::debug_escape(byte);
        Self(escape.as_bytes().iter())
    }
}

impl Iterator for Literal {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|&byte| byte as char)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|&byte| byte as char)
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
        self.0.last().map(|&byte| byte as char)
    }
}

impl DoubleEndedIterator for Literal {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|&byte| byte as char)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|&byte| byte as char)
    }
}

impl FusedIterator for Literal {}

/// Error that indicates a [`InvalidUtf8ByteSequence`] could not be constructed
/// because the byte sequence contained more than three bytes.
///
/// This crate decodes conventionally UTF-8 binary strings with the
/// "substitution of maximal subparts" strategy, which at most will return
/// invalid byte sequences with length 3.
///
/// This error is fatal and indicates a bug in a library this crate depends on.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteSequenceTooLongError {
    _private: (),
}

impl ByteSequenceTooLongError {
    /// Construct a new `ByteSequenceTooLongError`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::ByteSequenceTooLongError;
    /// const ERR: ByteSequenceTooLongError = ByteSequenceTooLongError::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the error message associated with this byte sequence too long
    /// error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::ByteSequenceTooLongError;
    /// let err = ByteSequenceTooLongError::new();
    /// assert_eq!(err.message(), "Invalid UTF-8 byte literal sequences can be at most three bytes long");
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn message(self) -> &'static str {
        "Invalid UTF-8 byte literal sequences can be at most three bytes long"
    }
}

impl fmt::Display for ByteSequenceTooLongError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MESSAGE: &str = ByteSequenceTooLongError::new().message();
        f.write_str(MESSAGE)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ByteSequenceTooLongError {}

/// Iterator of Ruby debug escape sequences for a contiguous invalid UTF-8 byte
/// sequence.
///
/// This iterator's item type is [`char`].
///
/// Non printable bytes like `0xFF` or `0x0C` are escaped to `\xFF` or `\f`.
///
/// # Usage notes
///
/// This iterator assumes it is constructed with invalid UTF-8 bytes and will
/// always escape all bytes given to it.
///
/// # Examples
///
/// The bytes `\xF0\x9D\x9C` could lead to a valid UTF-8 sequence, but 3 of them
/// on their own are invalid. All of these bytes should be hex escaped.
///
/// ```
/// # use scolapasta_string_escape::InvalidUtf8ByteSequence;
/// let invalid_byte_sequence = &b"\xF0\x9D\x9C"[..];
/// let iter = InvalidUtf8ByteSequence::try_from(invalid_byte_sequence).unwrap();
/// assert_eq!(iter.collect::<String>(), r"\xF0\x9D\x9C");
/// ```
#[derive(Default, Debug, Clone)]
#[must_use = "this `InvalidUtf8ByteSequence` is an `Iterator`, which should be consumed if constructed"]
pub struct InvalidUtf8ByteSequence {
    one: Option<Literal>,
    two: Option<Literal>,
    three: Option<Literal>,
}

impl InvalidUtf8ByteSequence {
    /// Construct a new, empty invalid UTF-8 byte sequence iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::InvalidUtf8ByteSequence;
    /// let iter = InvalidUtf8ByteSequence::new();
    /// assert_eq!(iter.count(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            one: None,
            two: None,
            three: None,
        }
    }

    /// Construct a new, invalid UTF-8 byte sequence iterator with a single
    /// invalid byte.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::InvalidUtf8ByteSequence;
    /// let iter = InvalidUtf8ByteSequence::with_byte(0xFF);
    /// assert_eq!(iter.collect::<String>(), r"\xFF");
    /// ```
    #[inline]
    pub fn with_byte(byte: u8) -> Self {
        Self {
            one: Some(Literal::from(byte)),
            two: None,
            three: None,
        }
    }

    /// Construct a new, invalid UTF-8 byte sequence iterator with two
    /// consecutive invalid bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::InvalidUtf8ByteSequence;
    /// let iter = InvalidUtf8ByteSequence::with_two_bytes(0xE2, 0x98);
    /// assert_eq!(iter.collect::<String>(), r"\xE2\x98");
    /// ```
    #[inline]
    pub fn with_two_bytes(left: u8, right: u8) -> Self {
        Self {
            one: Some(Literal::from(left)),
            two: Some(Literal::from(right)),
            three: None,
        }
    }

    /// Construct a new, invalid UTF-8 byte sequence iterator with three
    /// consecutive invalid bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_string_escape::InvalidUtf8ByteSequence;
    /// let iter = InvalidUtf8ByteSequence::with_three_bytes(0xF0, 0x9D, 0x9C);
    /// assert_eq!(iter.collect::<String>(), r"\xF0\x9D\x9C");
    /// ```
    #[inline]
    pub fn with_three_bytes(left: u8, mid: u8, right: u8) -> Self {
        Self {
            one: Some(Literal::from(left)),
            two: Some(Literal::from(mid)),
            three: Some(Literal::from(right)),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for InvalidUtf8ByteSequence {
    type Error = ByteSequenceTooLongError;

    #[inline]
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        match *bytes {
            [] => Ok(Self::new()),
            [byte] => Ok(Self::with_byte(byte)),
            [left, right] => Ok(Self::with_two_bytes(left, right)),
            [left, mid, right] => Ok(Self::with_three_bytes(left, mid, right)),
            _ => Err(ByteSequenceTooLongError::new()),
        }
    }
}

impl Iterator for InvalidUtf8ByteSequence {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.one
            .as_mut()
            .and_then(Iterator::next)
            .or_else(|| self.two.as_mut().and_then(Iterator::next))
            .or_else(|| self.three.as_mut().and_then(Iterator::next))
    }
}

impl DoubleEndedIterator for InvalidUtf8ByteSequence {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.three
            .as_mut()
            .and_then(DoubleEndedIterator::next_back)
            .or_else(|| self.two.as_mut().and_then(DoubleEndedIterator::next_back))
            .or_else(|| self.one.as_mut().and_then(DoubleEndedIterator::next_back))
    }
}

impl FusedIterator for InvalidUtf8ByteSequence {}

/// Generation:
///
/// ```ruby
/// (0x00..0xFF).each do |ch|
///   puts "let iter = Literal::from(0x#{ch.to_s(16).upcase}_u8);"
///   puts "assert_eq!(iter.collect::<String>(), r#{[ch].pack('c*').inspect});"
/// end
/// ```
#[cfg(test)]
mod tests {
    use super::Literal;
    use alloc::string::String;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn exhaustive() {
        let iter = Literal::from(0x0_u8);
        assert_eq!(iter.collect::<String>(), r"\x00");
        let iter = Literal::from(0x1_u8);
        assert_eq!(iter.collect::<String>(), r"\x01");
        let iter = Literal::from(0x2_u8);
        assert_eq!(iter.collect::<String>(), r"\x02");
        let iter = Literal::from(0x3_u8);
        assert_eq!(iter.collect::<String>(), r"\x03");
        let iter = Literal::from(0x4_u8);
        assert_eq!(iter.collect::<String>(), r"\x04");
        let iter = Literal::from(0x5_u8);
        assert_eq!(iter.collect::<String>(), r"\x05");
        let iter = Literal::from(0x6_u8);
        assert_eq!(iter.collect::<String>(), r"\x06");
        let iter = Literal::from(0x7_u8);
        assert_eq!(iter.collect::<String>(), r"\a");
        let iter = Literal::from(0x8_u8);
        assert_eq!(iter.collect::<String>(), r"\b");
        let iter = Literal::from(0x9_u8);
        assert_eq!(iter.collect::<String>(), r"\t");
        let iter = Literal::from(0xA_u8);
        assert_eq!(iter.collect::<String>(), r"\n");
        let iter = Literal::from(0xB_u8);
        assert_eq!(iter.collect::<String>(), r"\v");
        let iter = Literal::from(0xC_u8);
        assert_eq!(iter.collect::<String>(), r"\f");
        let iter = Literal::from(0xD_u8);
        assert_eq!(iter.collect::<String>(), r"\r");
        let iter = Literal::from(0xE_u8);
        assert_eq!(iter.collect::<String>(), r"\x0E");
        let iter = Literal::from(0xF_u8);
        assert_eq!(iter.collect::<String>(), r"\x0F");
        let iter = Literal::from(0x10_u8);
        assert_eq!(iter.collect::<String>(), r"\x10");
        let iter = Literal::from(0x11_u8);
        assert_eq!(iter.collect::<String>(), r"\x11");
        let iter = Literal::from(0x12_u8);
        assert_eq!(iter.collect::<String>(), r"\x12");
        let iter = Literal::from(0x13_u8);
        assert_eq!(iter.collect::<String>(), r"\x13");
        let iter = Literal::from(0x14_u8);
        assert_eq!(iter.collect::<String>(), r"\x14");
        let iter = Literal::from(0x15_u8);
        assert_eq!(iter.collect::<String>(), r"\x15");
        let iter = Literal::from(0x16_u8);
        assert_eq!(iter.collect::<String>(), r"\x16");
        let iter = Literal::from(0x17_u8);
        assert_eq!(iter.collect::<String>(), r"\x17");
        let iter = Literal::from(0x18_u8);
        assert_eq!(iter.collect::<String>(), r"\x18");
        let iter = Literal::from(0x19_u8);
        assert_eq!(iter.collect::<String>(), r"\x19");
        let iter = Literal::from(0x1A_u8);
        assert_eq!(iter.collect::<String>(), r"\x1A");
        let iter = Literal::from(0x1B_u8);
        assert_eq!(iter.collect::<String>(), r"\e");
        let iter = Literal::from(0x1C_u8);
        assert_eq!(iter.collect::<String>(), r"\x1C");
        let iter = Literal::from(0x1D_u8);
        assert_eq!(iter.collect::<String>(), r"\x1D");
        let iter = Literal::from(0x1E_u8);
        assert_eq!(iter.collect::<String>(), r"\x1E");
        let iter = Literal::from(0x1F_u8);
        assert_eq!(iter.collect::<String>(), r"\x1F");
        let iter = Literal::from(0x20_u8);
        assert_eq!(iter.collect::<String>(), r" ");
        let iter = Literal::from(0x21_u8);
        assert_eq!(iter.collect::<String>(), r"!");
        let iter = Literal::from(0x22_u8);
        assert_eq!(iter.collect::<String>(), r#"\""#);
        let iter = Literal::from(0x23_u8);
        assert_eq!(iter.collect::<String>(), r"#");
        let iter = Literal::from(0x24_u8);
        assert_eq!(iter.collect::<String>(), r"$");
        let iter = Literal::from(0x25_u8);
        assert_eq!(iter.collect::<String>(), r"%");
        let iter = Literal::from(0x26_u8);
        assert_eq!(iter.collect::<String>(), r"&");
        let iter = Literal::from(0x27_u8);
        assert_eq!(iter.collect::<String>(), r"'");
        let iter = Literal::from(0x28_u8);
        assert_eq!(iter.collect::<String>(), r"(");
        let iter = Literal::from(0x29_u8);
        assert_eq!(iter.collect::<String>(), r")");
        let iter = Literal::from(0x2A_u8);
        assert_eq!(iter.collect::<String>(), r"*");
        let iter = Literal::from(0x2B_u8);
        assert_eq!(iter.collect::<String>(), r"+");
        let iter = Literal::from(0x2C_u8);
        assert_eq!(iter.collect::<String>(), r",");
        let iter = Literal::from(0x2D_u8);
        assert_eq!(iter.collect::<String>(), r"-");
        let iter = Literal::from(0x2E_u8);
        assert_eq!(iter.collect::<String>(), r".");
        let iter = Literal::from(0x2F_u8);
        assert_eq!(iter.collect::<String>(), r"/");
        let iter = Literal::from(0x30_u8);
        assert_eq!(iter.collect::<String>(), r"0");
        let iter = Literal::from(0x31_u8);
        assert_eq!(iter.collect::<String>(), r"1");
        let iter = Literal::from(0x32_u8);
        assert_eq!(iter.collect::<String>(), r"2");
        let iter = Literal::from(0x33_u8);
        assert_eq!(iter.collect::<String>(), r"3");
        let iter = Literal::from(0x34_u8);
        assert_eq!(iter.collect::<String>(), r"4");
        let iter = Literal::from(0x35_u8);
        assert_eq!(iter.collect::<String>(), r"5");
        let iter = Literal::from(0x36_u8);
        assert_eq!(iter.collect::<String>(), r"6");
        let iter = Literal::from(0x37_u8);
        assert_eq!(iter.collect::<String>(), r"7");
        let iter = Literal::from(0x38_u8);
        assert_eq!(iter.collect::<String>(), r"8");
        let iter = Literal::from(0x39_u8);
        assert_eq!(iter.collect::<String>(), r"9");
        let iter = Literal::from(0x3A_u8);
        assert_eq!(iter.collect::<String>(), r":");
        let iter = Literal::from(0x3B_u8);
        assert_eq!(iter.collect::<String>(), r";");
        let iter = Literal::from(0x3C_u8);
        assert_eq!(iter.collect::<String>(), r"<");
        let iter = Literal::from(0x3D_u8);
        assert_eq!(iter.collect::<String>(), r"=");
        let iter = Literal::from(0x3E_u8);
        assert_eq!(iter.collect::<String>(), r">");
        let iter = Literal::from(0x3F_u8);
        assert_eq!(iter.collect::<String>(), r"?");
        let iter = Literal::from(0x40_u8);
        assert_eq!(iter.collect::<String>(), r"@");
        let iter = Literal::from(0x41_u8);
        assert_eq!(iter.collect::<String>(), r"A");
        let iter = Literal::from(0x42_u8);
        assert_eq!(iter.collect::<String>(), r"B");
        let iter = Literal::from(0x43_u8);
        assert_eq!(iter.collect::<String>(), r"C");
        let iter = Literal::from(0x44_u8);
        assert_eq!(iter.collect::<String>(), r"D");
        let iter = Literal::from(0x45_u8);
        assert_eq!(iter.collect::<String>(), r"E");
        let iter = Literal::from(0x46_u8);
        assert_eq!(iter.collect::<String>(), r"F");
        let iter = Literal::from(0x47_u8);
        assert_eq!(iter.collect::<String>(), r"G");
        let iter = Literal::from(0x48_u8);
        assert_eq!(iter.collect::<String>(), r"H");
        let iter = Literal::from(0x49_u8);
        assert_eq!(iter.collect::<String>(), r"I");
        let iter = Literal::from(0x4A_u8);
        assert_eq!(iter.collect::<String>(), r"J");
        let iter = Literal::from(0x4B_u8);
        assert_eq!(iter.collect::<String>(), r"K");
        let iter = Literal::from(0x4C_u8);
        assert_eq!(iter.collect::<String>(), r"L");
        let iter = Literal::from(0x4D_u8);
        assert_eq!(iter.collect::<String>(), r"M");
        let iter = Literal::from(0x4E_u8);
        assert_eq!(iter.collect::<String>(), r"N");
        let iter = Literal::from(0x4F_u8);
        assert_eq!(iter.collect::<String>(), r"O");
        let iter = Literal::from(0x50_u8);
        assert_eq!(iter.collect::<String>(), r"P");
        let iter = Literal::from(0x51_u8);
        assert_eq!(iter.collect::<String>(), r"Q");
        let iter = Literal::from(0x52_u8);
        assert_eq!(iter.collect::<String>(), r"R");
        let iter = Literal::from(0x53_u8);
        assert_eq!(iter.collect::<String>(), r"S");
        let iter = Literal::from(0x54_u8);
        assert_eq!(iter.collect::<String>(), r"T");
        let iter = Literal::from(0x55_u8);
        assert_eq!(iter.collect::<String>(), r"U");
        let iter = Literal::from(0x56_u8);
        assert_eq!(iter.collect::<String>(), r"V");
        let iter = Literal::from(0x57_u8);
        assert_eq!(iter.collect::<String>(), r"W");
        let iter = Literal::from(0x58_u8);
        assert_eq!(iter.collect::<String>(), r"X");
        let iter = Literal::from(0x59_u8);
        assert_eq!(iter.collect::<String>(), r"Y");
        let iter = Literal::from(0x5A_u8);
        assert_eq!(iter.collect::<String>(), r"Z");
        let iter = Literal::from(0x5B_u8);
        assert_eq!(iter.collect::<String>(), r"[");
        let iter = Literal::from(0x5C_u8);
        assert_eq!(iter.collect::<String>(), r"\\");
        let iter = Literal::from(0x5D_u8);
        assert_eq!(iter.collect::<String>(), r"]");
        let iter = Literal::from(0x5E_u8);
        assert_eq!(iter.collect::<String>(), r"^");
        let iter = Literal::from(0x5F_u8);
        assert_eq!(iter.collect::<String>(), r"_");
        let iter = Literal::from(0x60_u8);
        assert_eq!(iter.collect::<String>(), r"`");
        let iter = Literal::from(0x61_u8);
        assert_eq!(iter.collect::<String>(), r"a");
        let iter = Literal::from(0x62_u8);
        assert_eq!(iter.collect::<String>(), r"b");
        let iter = Literal::from(0x63_u8);
        assert_eq!(iter.collect::<String>(), r"c");
        let iter = Literal::from(0x64_u8);
        assert_eq!(iter.collect::<String>(), r"d");
        let iter = Literal::from(0x65_u8);
        assert_eq!(iter.collect::<String>(), r"e");
        let iter = Literal::from(0x66_u8);
        assert_eq!(iter.collect::<String>(), r"f");
        let iter = Literal::from(0x67_u8);
        assert_eq!(iter.collect::<String>(), r"g");
        let iter = Literal::from(0x68_u8);
        assert_eq!(iter.collect::<String>(), r"h");
        let iter = Literal::from(0x69_u8);
        assert_eq!(iter.collect::<String>(), r"i");
        let iter = Literal::from(0x6A_u8);
        assert_eq!(iter.collect::<String>(), r"j");
        let iter = Literal::from(0x6B_u8);
        assert_eq!(iter.collect::<String>(), r"k");
        let iter = Literal::from(0x6C_u8);
        assert_eq!(iter.collect::<String>(), r"l");
        let iter = Literal::from(0x6D_u8);
        assert_eq!(iter.collect::<String>(), r"m");
        let iter = Literal::from(0x6E_u8);
        assert_eq!(iter.collect::<String>(), r"n");
        let iter = Literal::from(0x6F_u8);
        assert_eq!(iter.collect::<String>(), r"o");
        let iter = Literal::from(0x70_u8);
        assert_eq!(iter.collect::<String>(), r"p");
        let iter = Literal::from(0x71_u8);
        assert_eq!(iter.collect::<String>(), r"q");
        let iter = Literal::from(0x72_u8);
        assert_eq!(iter.collect::<String>(), r"r");
        let iter = Literal::from(0x73_u8);
        assert_eq!(iter.collect::<String>(), r"s");
        let iter = Literal::from(0x74_u8);
        assert_eq!(iter.collect::<String>(), r"t");
        let iter = Literal::from(0x75_u8);
        assert_eq!(iter.collect::<String>(), r"u");
        let iter = Literal::from(0x76_u8);
        assert_eq!(iter.collect::<String>(), r"v");
        let iter = Literal::from(0x77_u8);
        assert_eq!(iter.collect::<String>(), r"w");
        let iter = Literal::from(0x78_u8);
        assert_eq!(iter.collect::<String>(), r"x");
        let iter = Literal::from(0x79_u8);
        assert_eq!(iter.collect::<String>(), r"y");
        let iter = Literal::from(0x7A_u8);
        assert_eq!(iter.collect::<String>(), r"z");
        let iter = Literal::from(0x7B_u8);
        assert_eq!(iter.collect::<String>(), r"{");
        let iter = Literal::from(0x7C_u8);
        assert_eq!(iter.collect::<String>(), r"|");
        let iter = Literal::from(0x7D_u8);
        assert_eq!(iter.collect::<String>(), r"}");
        let iter = Literal::from(0x7E_u8);
        assert_eq!(iter.collect::<String>(), r"~");
        let iter = Literal::from(0x7F_u8);
        assert_eq!(iter.collect::<String>(), r"\x7F");
        let iter = Literal::from(0x80_u8);
        assert_eq!(iter.collect::<String>(), r"\x80");
        let iter = Literal::from(0x81_u8);
        assert_eq!(iter.collect::<String>(), r"\x81");
        let iter = Literal::from(0x82_u8);
        assert_eq!(iter.collect::<String>(), r"\x82");
        let iter = Literal::from(0x83_u8);
        assert_eq!(iter.collect::<String>(), r"\x83");
        let iter = Literal::from(0x84_u8);
        assert_eq!(iter.collect::<String>(), r"\x84");
        let iter = Literal::from(0x85_u8);
        assert_eq!(iter.collect::<String>(), r"\x85");
        let iter = Literal::from(0x86_u8);
        assert_eq!(iter.collect::<String>(), r"\x86");
        let iter = Literal::from(0x87_u8);
        assert_eq!(iter.collect::<String>(), r"\x87");
        let iter = Literal::from(0x88_u8);
        assert_eq!(iter.collect::<String>(), r"\x88");
        let iter = Literal::from(0x89_u8);
        assert_eq!(iter.collect::<String>(), r"\x89");
        let iter = Literal::from(0x8A_u8);
        assert_eq!(iter.collect::<String>(), r"\x8A");
        let iter = Literal::from(0x8B_u8);
        assert_eq!(iter.collect::<String>(), r"\x8B");
        let iter = Literal::from(0x8C_u8);
        assert_eq!(iter.collect::<String>(), r"\x8C");
        let iter = Literal::from(0x8D_u8);
        assert_eq!(iter.collect::<String>(), r"\x8D");
        let iter = Literal::from(0x8E_u8);
        assert_eq!(iter.collect::<String>(), r"\x8E");
        let iter = Literal::from(0x8F_u8);
        assert_eq!(iter.collect::<String>(), r"\x8F");
        let iter = Literal::from(0x90_u8);
        assert_eq!(iter.collect::<String>(), r"\x90");
        let iter = Literal::from(0x91_u8);
        assert_eq!(iter.collect::<String>(), r"\x91");
        let iter = Literal::from(0x92_u8);
        assert_eq!(iter.collect::<String>(), r"\x92");
        let iter = Literal::from(0x93_u8);
        assert_eq!(iter.collect::<String>(), r"\x93");
        let iter = Literal::from(0x94_u8);
        assert_eq!(iter.collect::<String>(), r"\x94");
        let iter = Literal::from(0x95_u8);
        assert_eq!(iter.collect::<String>(), r"\x95");
        let iter = Literal::from(0x96_u8);
        assert_eq!(iter.collect::<String>(), r"\x96");
        let iter = Literal::from(0x97_u8);
        assert_eq!(iter.collect::<String>(), r"\x97");
        let iter = Literal::from(0x98_u8);
        assert_eq!(iter.collect::<String>(), r"\x98");
        let iter = Literal::from(0x99_u8);
        assert_eq!(iter.collect::<String>(), r"\x99");
        let iter = Literal::from(0x9A_u8);
        assert_eq!(iter.collect::<String>(), r"\x9A");
        let iter = Literal::from(0x9B_u8);
        assert_eq!(iter.collect::<String>(), r"\x9B");
        let iter = Literal::from(0x9C_u8);
        assert_eq!(iter.collect::<String>(), r"\x9C");
        let iter = Literal::from(0x9D_u8);
        assert_eq!(iter.collect::<String>(), r"\x9D");
        let iter = Literal::from(0x9E_u8);
        assert_eq!(iter.collect::<String>(), r"\x9E");
        let iter = Literal::from(0x9F_u8);
        assert_eq!(iter.collect::<String>(), r"\x9F");
        let iter = Literal::from(0xA0_u8);
        assert_eq!(iter.collect::<String>(), r"\xA0");
        let iter = Literal::from(0xA1_u8);
        assert_eq!(iter.collect::<String>(), r"\xA1");
        let iter = Literal::from(0xA2_u8);
        assert_eq!(iter.collect::<String>(), r"\xA2");
        let iter = Literal::from(0xA3_u8);
        assert_eq!(iter.collect::<String>(), r"\xA3");
        let iter = Literal::from(0xA4_u8);
        assert_eq!(iter.collect::<String>(), r"\xA4");
        let iter = Literal::from(0xA5_u8);
        assert_eq!(iter.collect::<String>(), r"\xA5");
        let iter = Literal::from(0xA6_u8);
        assert_eq!(iter.collect::<String>(), r"\xA6");
        let iter = Literal::from(0xA7_u8);
        assert_eq!(iter.collect::<String>(), r"\xA7");
        let iter = Literal::from(0xA8_u8);
        assert_eq!(iter.collect::<String>(), r"\xA8");
        let iter = Literal::from(0xA9_u8);
        assert_eq!(iter.collect::<String>(), r"\xA9");
        let iter = Literal::from(0xAA_u8);
        assert_eq!(iter.collect::<String>(), r"\xAA");
        let iter = Literal::from(0xAB_u8);
        assert_eq!(iter.collect::<String>(), r"\xAB");
        let iter = Literal::from(0xAC_u8);
        assert_eq!(iter.collect::<String>(), r"\xAC");
        let iter = Literal::from(0xAD_u8);
        assert_eq!(iter.collect::<String>(), r"\xAD");
        let iter = Literal::from(0xAE_u8);
        assert_eq!(iter.collect::<String>(), r"\xAE");
        let iter = Literal::from(0xAF_u8);
        assert_eq!(iter.collect::<String>(), r"\xAF");
        let iter = Literal::from(0xB0_u8);
        assert_eq!(iter.collect::<String>(), r"\xB0");
        let iter = Literal::from(0xB1_u8);
        assert_eq!(iter.collect::<String>(), r"\xB1");
        let iter = Literal::from(0xB2_u8);
        assert_eq!(iter.collect::<String>(), r"\xB2");
        let iter = Literal::from(0xB3_u8);
        assert_eq!(iter.collect::<String>(), r"\xB3");
        let iter = Literal::from(0xB4_u8);
        assert_eq!(iter.collect::<String>(), r"\xB4");
        let iter = Literal::from(0xB5_u8);
        assert_eq!(iter.collect::<String>(), r"\xB5");
        let iter = Literal::from(0xB6_u8);
        assert_eq!(iter.collect::<String>(), r"\xB6");
        let iter = Literal::from(0xB7_u8);
        assert_eq!(iter.collect::<String>(), r"\xB7");
        let iter = Literal::from(0xB8_u8);
        assert_eq!(iter.collect::<String>(), r"\xB8");
        let iter = Literal::from(0xB9_u8);
        assert_eq!(iter.collect::<String>(), r"\xB9");
        let iter = Literal::from(0xBA_u8);
        assert_eq!(iter.collect::<String>(), r"\xBA");
        let iter = Literal::from(0xBB_u8);
        assert_eq!(iter.collect::<String>(), r"\xBB");
        let iter = Literal::from(0xBC_u8);
        assert_eq!(iter.collect::<String>(), r"\xBC");
        let iter = Literal::from(0xBD_u8);
        assert_eq!(iter.collect::<String>(), r"\xBD");
        let iter = Literal::from(0xBE_u8);
        assert_eq!(iter.collect::<String>(), r"\xBE");
        let iter = Literal::from(0xBF_u8);
        assert_eq!(iter.collect::<String>(), r"\xBF");
        let iter = Literal::from(0xC0_u8);
        assert_eq!(iter.collect::<String>(), r"\xC0");
        let iter = Literal::from(0xC1_u8);
        assert_eq!(iter.collect::<String>(), r"\xC1");
        let iter = Literal::from(0xC2_u8);
        assert_eq!(iter.collect::<String>(), r"\xC2");
        let iter = Literal::from(0xC3_u8);
        assert_eq!(iter.collect::<String>(), r"\xC3");
        let iter = Literal::from(0xC4_u8);
        assert_eq!(iter.collect::<String>(), r"\xC4");
        let iter = Literal::from(0xC5_u8);
        assert_eq!(iter.collect::<String>(), r"\xC5");
        let iter = Literal::from(0xC6_u8);
        assert_eq!(iter.collect::<String>(), r"\xC6");
        let iter = Literal::from(0xC7_u8);
        assert_eq!(iter.collect::<String>(), r"\xC7");
        let iter = Literal::from(0xC8_u8);
        assert_eq!(iter.collect::<String>(), r"\xC8");
        let iter = Literal::from(0xC9_u8);
        assert_eq!(iter.collect::<String>(), r"\xC9");
        let iter = Literal::from(0xCA_u8);
        assert_eq!(iter.collect::<String>(), r"\xCA");
        let iter = Literal::from(0xCB_u8);
        assert_eq!(iter.collect::<String>(), r"\xCB");
        let iter = Literal::from(0xCC_u8);
        assert_eq!(iter.collect::<String>(), r"\xCC");
        let iter = Literal::from(0xCD_u8);
        assert_eq!(iter.collect::<String>(), r"\xCD");
        let iter = Literal::from(0xCE_u8);
        assert_eq!(iter.collect::<String>(), r"\xCE");
        let iter = Literal::from(0xCF_u8);
        assert_eq!(iter.collect::<String>(), r"\xCF");
        let iter = Literal::from(0xD0_u8);
        assert_eq!(iter.collect::<String>(), r"\xD0");
        let iter = Literal::from(0xD1_u8);
        assert_eq!(iter.collect::<String>(), r"\xD1");
        let iter = Literal::from(0xD2_u8);
        assert_eq!(iter.collect::<String>(), r"\xD2");
        let iter = Literal::from(0xD3_u8);
        assert_eq!(iter.collect::<String>(), r"\xD3");
        let iter = Literal::from(0xD4_u8);
        assert_eq!(iter.collect::<String>(), r"\xD4");
        let iter = Literal::from(0xD5_u8);
        assert_eq!(iter.collect::<String>(), r"\xD5");
        let iter = Literal::from(0xD6_u8);
        assert_eq!(iter.collect::<String>(), r"\xD6");
        let iter = Literal::from(0xD7_u8);
        assert_eq!(iter.collect::<String>(), r"\xD7");
        let iter = Literal::from(0xD8_u8);
        assert_eq!(iter.collect::<String>(), r"\xD8");
        let iter = Literal::from(0xD9_u8);
        assert_eq!(iter.collect::<String>(), r"\xD9");
        let iter = Literal::from(0xDA_u8);
        assert_eq!(iter.collect::<String>(), r"\xDA");
        let iter = Literal::from(0xDB_u8);
        assert_eq!(iter.collect::<String>(), r"\xDB");
        let iter = Literal::from(0xDC_u8);
        assert_eq!(iter.collect::<String>(), r"\xDC");
        let iter = Literal::from(0xDD_u8);
        assert_eq!(iter.collect::<String>(), r"\xDD");
        let iter = Literal::from(0xDE_u8);
        assert_eq!(iter.collect::<String>(), r"\xDE");
        let iter = Literal::from(0xDF_u8);
        assert_eq!(iter.collect::<String>(), r"\xDF");
        let iter = Literal::from(0xE0_u8);
        assert_eq!(iter.collect::<String>(), r"\xE0");
        let iter = Literal::from(0xE1_u8);
        assert_eq!(iter.collect::<String>(), r"\xE1");
        let iter = Literal::from(0xE2_u8);
        assert_eq!(iter.collect::<String>(), r"\xE2");
        let iter = Literal::from(0xE3_u8);
        assert_eq!(iter.collect::<String>(), r"\xE3");
        let iter = Literal::from(0xE4_u8);
        assert_eq!(iter.collect::<String>(), r"\xE4");
        let iter = Literal::from(0xE5_u8);
        assert_eq!(iter.collect::<String>(), r"\xE5");
        let iter = Literal::from(0xE6_u8);
        assert_eq!(iter.collect::<String>(), r"\xE6");
        let iter = Literal::from(0xE7_u8);
        assert_eq!(iter.collect::<String>(), r"\xE7");
        let iter = Literal::from(0xE8_u8);
        assert_eq!(iter.collect::<String>(), r"\xE8");
        let iter = Literal::from(0xE9_u8);
        assert_eq!(iter.collect::<String>(), r"\xE9");
        let iter = Literal::from(0xEA_u8);
        assert_eq!(iter.collect::<String>(), r"\xEA");
        let iter = Literal::from(0xEB_u8);
        assert_eq!(iter.collect::<String>(), r"\xEB");
        let iter = Literal::from(0xEC_u8);
        assert_eq!(iter.collect::<String>(), r"\xEC");
        let iter = Literal::from(0xED_u8);
        assert_eq!(iter.collect::<String>(), r"\xED");
        let iter = Literal::from(0xEE_u8);
        assert_eq!(iter.collect::<String>(), r"\xEE");
        let iter = Literal::from(0xEF_u8);
        assert_eq!(iter.collect::<String>(), r"\xEF");
        let iter = Literal::from(0xF0_u8);
        assert_eq!(iter.collect::<String>(), r"\xF0");
        let iter = Literal::from(0xF1_u8);
        assert_eq!(iter.collect::<String>(), r"\xF1");
        let iter = Literal::from(0xF2_u8);
        assert_eq!(iter.collect::<String>(), r"\xF2");
        let iter = Literal::from(0xF3_u8);
        assert_eq!(iter.collect::<String>(), r"\xF3");
        let iter = Literal::from(0xF4_u8);
        assert_eq!(iter.collect::<String>(), r"\xF4");
        let iter = Literal::from(0xF5_u8);
        assert_eq!(iter.collect::<String>(), r"\xF5");
        let iter = Literal::from(0xF6_u8);
        assert_eq!(iter.collect::<String>(), r"\xF6");
        let iter = Literal::from(0xF7_u8);
        assert_eq!(iter.collect::<String>(), r"\xF7");
        let iter = Literal::from(0xF8_u8);
        assert_eq!(iter.collect::<String>(), r"\xF8");
        let iter = Literal::from(0xF9_u8);
        assert_eq!(iter.collect::<String>(), r"\xF9");
        let iter = Literal::from(0xFA_u8);
        assert_eq!(iter.collect::<String>(), r"\xFA");
        let iter = Literal::from(0xFB_u8);
        assert_eq!(iter.collect::<String>(), r"\xFB");
        let iter = Literal::from(0xFC_u8);
        assert_eq!(iter.collect::<String>(), r"\xFC");
        let iter = Literal::from(0xFD_u8);
        assert_eq!(iter.collect::<String>(), r"\xFD");
        let iter = Literal::from(0xFE_u8);
        assert_eq!(iter.collect::<String>(), r"\xFE");
        let iter = Literal::from(0xFF_u8);
        assert_eq!(iter.collect::<String>(), r"\xFF");
    }
}
