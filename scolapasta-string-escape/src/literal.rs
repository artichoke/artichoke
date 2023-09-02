use core::fmt;
use core::iter::FusedIterator;
use core::slice;
use core::str;

/// Returns whether the given [`char`] has an ASCII literal escape code.
///
/// Control characters in the range `0x00..=0x1F`, `"`, `\` and `DEL` have
/// non-trivial escapes.
///
/// # Examples
///
/// ```
/// # use core::char::REPLACEMENT_CHARACTER;
/// # use scolapasta_string_escape::ascii_char_with_escape;
/// assert_eq!(ascii_char_with_escape('\0'), Some(r"\x00"));
/// assert_eq!(ascii_char_with_escape('\n'), Some(r"\n"));
/// assert_eq!(ascii_char_with_escape('"'), Some(r#"\""#));
/// assert_eq!(ascii_char_with_escape('\\'), Some(r"\\"));
///
/// assert_eq!(ascii_char_with_escape('a'), None);
/// assert_eq!(ascii_char_with_escape('Z'), None);
/// assert_eq!(ascii_char_with_escape(';'), None);
/// assert_eq!(ascii_char_with_escape('💎'), None);
/// assert_eq!(ascii_char_with_escape(REPLACEMENT_CHARACTER), None);
/// ```
#[inline]
#[must_use]
pub const fn ascii_char_with_escape(ch: char) -> Option<&'static str> {
    if !ch.is_ascii() {
        return None;
    }
    let [ascii_byte, ..] = (ch as u32).to_le_bytes();
    let escape = Literal::debug_escape(ascii_byte);
    if escape.len() > 1 {
        Some(escape)
    } else {
        None
    }
}

/// Iterator of Ruby debug escape sequences for a byte.
///
/// This iterator's item type is [`char`].
///
/// Non-printable bytes like `0xFF` or `0x0C` are escaped to `\xFF` or `\f`.
///
/// ASCII printable characters are passed through as is unless they are `"` or
/// `\` since these fields are used to delimit strings and escape sequences.
///
/// # Usage notes
///
/// This iterator operates on individual bytes, which makes it unsuitable for
/// debug printing a conventionally UTF-8 byte string on its own. See
/// [`format_debug_escape_into`] to debug format an entire byte string.
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
        // [2.6.3] > '"'.ord
        // => 34
        // [2.6.3] > '"'.ord.to_s(16)
        // => "22"
        // [2.6.3] > :"\x22"
        // => :"\""
        // [2.6.3] > '\\'.ord
        // => 92
        // [2.6.3] > '\\'.ord.to_s(16)
        // => "5c"
        // [2.6.3] > :"\x5C"
        // => :"\\"
        // ```
        #[rustfmt::skip]
        const TABLE: [&str; 256] = [
            r"\x00", r"\x01", r"\x02", r"\x03", r"\x04", r"\x05", r"\x06",   r"\a",
              r"\b",   r"\t",   r"\n",   r"\v",   r"\f",   r"\r", r"\x0E", r"\x0F",
            r"\x10", r"\x11", r"\x12", r"\x13", r"\x14", r"\x15", r"\x16", r"\x17",
            r"\x18", r"\x19", r"\x1A",   r"\e", r"\x1C", r"\x1D", r"\x1E", r"\x1F",
                " ",     "!", r#"\""#,    "#",     "$",     "%",     "&",      "'",
                "(",     ")",     "*",    "+",     ",",     "-",     ".",      "/",
                "0",     "1",     "2",    "3",     "4",     "5",     "6",      "7",
                "8",     "9",     ":",    ";",     "<",     "=",     ">",      "?",
                "@",     "A",     "B",    "C",     "D",     "E",     "F",      "G",
                "H",     "I",     "J",    "K",     "L",     "M",     "N",      "O",
                "P",     "Q",     "R",    "S",     "T",     "U",     "V",      "W",
                "X",     "Y",     "Z",    "[",   r"\\",     "]",     "^",      "_",
                "`",     "a",     "b",    "c",     "d",     "e",     "f",      "g",
                "h",     "i",     "j",    "k",     "l",     "m",     "n",      "o",
                "p",     "q",     "r",    "s",     "t",     "u",     "v",      "w",
                "x",     "y",     "z",    "{",     "|",     "}",     "~",  r"\x7F",
            r"\x80", r"\x81", r"\x82", r"\x83", r"\x84", r"\x85", r"\x86", r"\x87",
            r"\x88", r"\x89", r"\x8A", r"\x8B", r"\x8C", r"\x8D", r"\x8E", r"\x8F",
            r"\x90", r"\x91", r"\x92", r"\x93", r"\x94", r"\x95", r"\x96", r"\x97",
            r"\x98", r"\x99", r"\x9A", r"\x9B", r"\x9C", r"\x9D", r"\x9E", r"\x9F",
            r"\xA0", r"\xA1", r"\xA2", r"\xA3", r"\xA4", r"\xA5", r"\xA6", r"\xA7",
            r"\xA8", r"\xA9", r"\xAA", r"\xAB", r"\xAC", r"\xAD", r"\xAE", r"\xAF",
            r"\xB0", r"\xB1", r"\xB2", r"\xB3", r"\xB4", r"\xB5", r"\xB6", r"\xB7",
            r"\xB8", r"\xB9", r"\xBA", r"\xBB", r"\xBC", r"\xBD", r"\xBE", r"\xBF",
            r"\xC0", r"\xC1", r"\xC2", r"\xC3", r"\xC4", r"\xC5", r"\xC6", r"\xC7",
            r"\xC8", r"\xC9", r"\xCA", r"\xCB", r"\xCC", r"\xCD", r"\xCE", r"\xCF",
            r"\xD0", r"\xD1", r"\xD2", r"\xD3", r"\xD4", r"\xD5", r"\xD6", r"\xD7",
            r"\xD8", r"\xD9", r"\xDA", r"\xDB", r"\xDC", r"\xDD", r"\xDE", r"\xDF",
            r"\xE0", r"\xE1", r"\xE2", r"\xE3", r"\xE4", r"\xE5", r"\xE6", r"\xE7",
            r"\xE8", r"\xE9", r"\xEA", r"\xEB", r"\xEC", r"\xED", r"\xEE", r"\xEF",
            r"\xF0", r"\xF1", r"\xF2", r"\xF3", r"\xF4", r"\xF5", r"\xF6", r"\xF7",
            r"\xF8", r"\xF9", r"\xFA", r"\xFB", r"\xFC", r"\xFD", r"\xFE", r"\xFF",
        ];

        TABLE[byte as usize]
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
    /// assert_eq!(
    ///     err.message(),
    ///     "Invalid UTF-8 byte literal sequences can be at most three bytes long"
    /// );
    /// ```
    #[inline]
    #[must_use]
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
/// Non-printable bytes like `0xFF` or `0x0C` are escaped to `\xFF` or `\f`.
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
/// pairs = (0x00..0xFF).to_a.map {|ch| ["0x#{ch.to_s(16).upcase}_u8", "r#{[ch].pack('c*').inspect}"]}
/// puts "let test_cases = [#{pairs.map {|a, b| "#{b}"}.join ", "}];"
/// ```
#[cfg(test)]
mod tests {
    use alloc::string::String;

    use super::Literal;

    #[test]
    fn exhaustive() {
        #[rustfmt::skip]
        let test_cases = (u8::MIN..=u8::MAX).zip(
            [
                r"\x00", r"\x01", r"\x02", r"\x03", r"\x04", r"\x05", r"\x06",   r"\a",
                  r"\b",   r"\t",   r"\n",   r"\v",   r"\f",   r"\r", r"\x0E", r"\x0F",
                r"\x10", r"\x11", r"\x12", r"\x13", r"\x14", r"\x15", r"\x16", r"\x17",
                r"\x18", r"\x19", r"\x1A",   r"\e", r"\x1C", r"\x1D", r"\x1E", r"\x1F",
                    " ",     "!", r#"\""#,    "#",     "$",     "%",     "&",      "'",
                    "(",     ")",     "*",    "+",     ",",     "-",     ".",      "/",
                    "0",     "1",     "2",    "3",     "4",     "5",     "6",      "7",
                    "8",     "9",     ":",    ";",     "<",     "=",     ">",      "?",
                    "@",     "A",     "B",    "C",     "D",     "E",     "F",      "G",
                    "H",     "I",     "J",    "K",     "L",     "M",     "N",      "O",
                    "P",     "Q",     "R",    "S",     "T",     "U",     "V",      "W",
                    "X",     "Y",     "Z",    "[",   r"\\",     "]",     "^",      "_",
                    "`",     "a",     "b",    "c",     "d",     "e",     "f",      "g",
                    "h",     "i",     "j",    "k",     "l",     "m",     "n",      "o",
                    "p",     "q",     "r",    "s",     "t",     "u",     "v",      "w",
                    "x",     "y",     "z",    "{",     "|",     "}",     "~",  r"\x7F",
                r"\x80", r"\x81", r"\x82", r"\x83", r"\x84", r"\x85", r"\x86", r"\x87",
                r"\x88", r"\x89", r"\x8A", r"\x8B", r"\x8C", r"\x8D", r"\x8E", r"\x8F",
                r"\x90", r"\x91", r"\x92", r"\x93", r"\x94", r"\x95", r"\x96", r"\x97",
                r"\x98", r"\x99", r"\x9A", r"\x9B", r"\x9C", r"\x9D", r"\x9E", r"\x9F",
                r"\xA0", r"\xA1", r"\xA2", r"\xA3", r"\xA4", r"\xA5", r"\xA6", r"\xA7",
                r"\xA8", r"\xA9", r"\xAA", r"\xAB", r"\xAC", r"\xAD", r"\xAE", r"\xAF",
                r"\xB0", r"\xB1", r"\xB2", r"\xB3", r"\xB4", r"\xB5", r"\xB6", r"\xB7",
                r"\xB8", r"\xB9", r"\xBA", r"\xBB", r"\xBC", r"\xBD", r"\xBE", r"\xBF",
                r"\xC0", r"\xC1", r"\xC2", r"\xC3", r"\xC4", r"\xC5", r"\xC6", r"\xC7",
                r"\xC8", r"\xC9", r"\xCA", r"\xCB", r"\xCC", r"\xCD", r"\xCE", r"\xCF",
                r"\xD0", r"\xD1", r"\xD2", r"\xD3", r"\xD4", r"\xD5", r"\xD6", r"\xD7",
                r"\xD8", r"\xD9", r"\xDA", r"\xDB", r"\xDC", r"\xDD", r"\xDE", r"\xDF",
                r"\xE0", r"\xE1", r"\xE2", r"\xE3", r"\xE4", r"\xE5", r"\xE6", r"\xE7",
                r"\xE8", r"\xE9", r"\xEA", r"\xEB", r"\xEC", r"\xED", r"\xEE", r"\xEF",
                r"\xF0", r"\xF1", r"\xF2", r"\xF3", r"\xF4", r"\xF5", r"\xF6", r"\xF7",
                r"\xF8", r"\xF9", r"\xFA", r"\xFB", r"\xFC", r"\xFD", r"\xFE", r"\xFF",
            ]
        );
        for (byte, literal) in test_cases {
            let iter = Literal::from(byte);
            assert_eq!(
                iter.collect::<String>(),
                literal,
                "tested byte {byte}, expected {literal}"
            );
        }
    }
}
