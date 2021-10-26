use core::fmt;
use core::iter::FusedIterator;
use scolapasta_string_escape::{is_ascii_char_with_escape, InvalidUtf8ByteSequence};

/// An iterator that yields a debug representation of a `String` and its byte
/// contents as a sequence of `char`s.
///
/// This struct is created by the [`inspect`] method on [`String`]. See its
/// documentation for more.
///
/// To format a `String` directly into a writer, see [`format_into`] or
/// [`write_into`].
///
/// # Examples
///
/// To inspect an empty bytestring:
///
/// ```
/// # extern crate alloc;
/// use alloc::string::String;
/// # use spinoso_string::Inspect;
/// let inspect = Inspect::default();
/// let debug = inspect.collect::<String>();
/// assert_eq!(debug, r#""""#);
/// ```
///
/// To inspect a well-formed UTF-8 bytestring:
///
/// ```
/// # extern crate alloc;
/// use alloc::string::String;
/// # use spinoso_string::Inspect;
/// let inspect = Inspect::from("spinoso");
/// let debug = inspect.collect::<String>();
/// assert_eq!(debug, "\"spinoso\"");
/// ```
///
/// To inspect a bytestring with invalid UTF-8 bytes:
///
/// ```
/// # extern crate alloc;
/// use alloc::string::String;
/// # use spinoso_string::Inspect;
/// let inspect = Inspect::from(&b"invalid-\xFF-utf8"[..]);
/// let debug = inspect.collect::<String>();
/// assert_eq!(debug, r#""invalid-\xFF-utf8""#);
/// ```
///
/// [`inspect`]: crate::String::inspect
/// [`String`]: crate::String
/// [`format_into`]: Self::format_into
/// [`write_into`]: Self::write_into
#[derive(Default, Debug, Clone)]
#[must_use = "this `Inspect` is an `Iterator`, which should be consumed if constructed"]
#[cfg_attr(docsrs, doc(cfg(feature = "inspect")))]
pub struct Inspect<'a>(State<'a>);

impl<'a> From<&'a str> for Inspect<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for Inspect<'a> {
    #[inline]
    fn from(value: &'a [u8]) -> Self {
        Self(State::new(value))
    }
}

impl<'a> Iterator for Inspect<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> DoubleEndedIterator for Inspect<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a> FusedIterator for Inspect<'a> {}

impl<'a> Inspect<'a> {
    /// Write an `Inspect` iterator into the given destination using the debug
    /// representation of the byte buffer associated with a source `String`.
    ///
    /// This formatter writes content like `"spinoso"` and `"invalid-\xFF-utf8"`.
    /// To see example output of the underlying iterator, see the `Inspect`
    /// documentation.
    ///
    /// To write binary output, use [`write_into`], which requires the **std**
    /// feature to be activated.
    ///
    /// # Errors
    ///
    /// If the given writer returns an error as it is being written to, that
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::fmt::Write;
    /// # use spinoso_string::Inspect;
    /// let mut buf = String::new();
    /// let iter = Inspect::from("spinoso");
    /// iter.format_into(&mut buf);
    /// assert_eq!(buf, "\"spinoso\"");
    ///
    /// let mut buf = String::new();
    /// let iter = Inspect::from(&b"\xFF"[..]);
    /// iter.format_into(&mut buf);
    /// assert_eq!(buf, r#""\xFF""#);
    /// ```
    ///
    /// [`write_into`]: Self::write_into
    #[inline]
    pub fn format_into<W>(self, mut dest: W) -> fmt::Result
    where
        W: fmt::Write,
    {
        for ch in self {
            dest.write_char(ch)?;
        }
        Ok(())
    }

    /// Write an `Inspect` iterator into the given destination using the debug
    /// representation of the byte buffer associated with a source `String`.
    ///
    /// This formatter writes content like `"spinoso"` and `"invalid-\xFF-utf8"`.
    /// To see example output of the underlying iterator, see the `Inspect`
    /// documentation.
    ///
    /// To write to a [formatter], use [`format_into`].
    ///
    /// # Errors
    ///
    /// If the given writer returns an error as it is being written to, that
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::Write;
    /// # use spinoso_string::Inspect;
    /// let mut buf = Vec::new();
    /// let iter = Inspect::from("spinoso");
    /// iter.write_into(&mut buf);
    /// assert_eq!(buf, &b"\"spinoso\""[..]);
    ///
    /// let mut buf = Vec::new();
    /// let iter = Inspect::from(&b"\xFF"[..]);
    /// iter.write_into(&mut buf);
    /// assert_eq!(buf, &[b'"', b'\\', b'x', b'F', b'F', b'"']);
    /// ```
    ///
    /// [formatter]: fmt::Write
    /// [`format_into`]: Self::format_into
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn write_into<W>(self, mut dest: W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let mut buf = [0; 4];
        for ch in self {
            let utf8 = ch.encode_utf8(&mut buf);
            dest.write_all(utf8.as_bytes())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Flags {
    bits: u8,
}

impl Flags {
    // Bit flags
    const EMIT_LEADING_QUOTE: Self = Self { bits: 0b0000_0001 };
    const EMIT_TRAILING_QUOTE: Self = Self { bits: 0b0000_0010 };

    // Initial states
    const DEFAULT: Self = Self {
        bits: Self::EMIT_LEADING_QUOTE.bits | Self::EMIT_TRAILING_QUOTE.bits,
    };

    #[inline]
    fn emit_leading_quote(&mut self) -> Option<char> {
        if (self.bits & Self::EMIT_LEADING_QUOTE.bits) == Self::EMIT_LEADING_QUOTE.bits {
            self.bits &= !Self::EMIT_LEADING_QUOTE.bits;
            Some('"')
        } else {
            None
        }
    }

    #[inline]
    fn emit_trailing_quote(&mut self) -> Option<char> {
        if (self.bits & Self::EMIT_TRAILING_QUOTE.bits) == Self::EMIT_TRAILING_QUOTE.bits {
            self.bits &= !Self::EMIT_TRAILING_QUOTE.bits;
            Some('"')
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
#[must_use = "this `State` is an `Iterator`, which should be consumed if constructed"]
struct State<'a> {
    flags: Flags,
    forward_byte_literal: InvalidUtf8ByteSequence,
    bytes: &'a [u8],
    reverse_byte_literal: InvalidUtf8ByteSequence,
}

impl<'a> State<'a> {
    /// Construct a `State` for the given byte slice.
    ///
    /// This constructor produces inspect contents like `"fred"`.
    #[inline]
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            flags: Flags::DEFAULT,
            forward_byte_literal: InvalidUtf8ByteSequence::new(),
            bytes,
            reverse_byte_literal: InvalidUtf8ByteSequence::new(),
        }
    }
}

impl<'a> Default for State<'a> {
    /// Construct a `State` that will render debug output for the empty slice.
    ///
    /// This constructor produces inspect contents like `""`.
    #[inline]
    fn default() -> Self {
        Self::new(b"")
    }
}

impl<'a> Iterator for State<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.flags.emit_leading_quote() {
            return Some(ch);
        }
        if let Some(ch) = self.forward_byte_literal.next() {
            return Some(ch);
        }
        let (ch, size) = bstr::decode_utf8(self.bytes);
        match ch {
            Some(ch) if is_ascii_char_with_escape(ch) => {
                let (ascii_byte, remainder) = self.bytes.split_at(size);
                // This conversion is safe to unwrap due to the documented
                // behavior of `bstr::decode_utf8` and `InvalidUtf8ByteSequence`
                // which indicate that `size` is always in the range of 0..=3.
                //
                // While not an invalid byte, we rely on the documented
                // behavior of `InvalidUtf8ByteSequence` to always escape
                // any bytes given to it.
                self.forward_byte_literal = InvalidUtf8ByteSequence::try_from(ascii_byte).unwrap();
                self.bytes = remainder;
                return self.forward_byte_literal.next();
            }
            Some(ch) => {
                self.bytes = &self.bytes[size..];
                return Some(ch);
            }
            None if size == 0 => {}
            None => {
                let (invalid_utf8_bytes, remainder) = self.bytes.split_at(size);
                // This conversion is safe to unwrap due to the documented
                // behavior of `bstr::decode_utf8` and `InvalidUtf8ByteSequence`
                // which indicate that `size` is always in the range of 0..=3.
                self.forward_byte_literal = InvalidUtf8ByteSequence::try_from(invalid_utf8_bytes).unwrap();
                self.bytes = remainder;
                return self.forward_byte_literal.next();
            }
        };
        if let Some(ch) = self.reverse_byte_literal.next() {
            return Some(ch);
        }
        if let Some(ch) = self.flags.emit_trailing_quote() {
            return Some(ch);
        }
        None
    }
}

impl<'a> DoubleEndedIterator for State<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.flags.emit_trailing_quote() {
            return Some(ch);
        }
        if let Some(ch) = self.reverse_byte_literal.next_back() {
            return Some(ch);
        }
        let (ch, size) = bstr::decode_last_utf8(self.bytes);
        match ch {
            Some(ch) if is_ascii_char_with_escape(ch) => {
                let (remainder, ascii_byte) = self.bytes.split_at(self.bytes.len() - size);
                // This conversion is safe to unwrap due to the documented
                // behavior of `bstr::decode_utf8` and `InvalidUtf8ByteSequence`
                // which indicate that `size` is always in the range of 0..=3.
                //
                // While not an invalid byte, we rely on the documented
                // behavior of `InvalidUtf8ByteSequence` to always escape
                // any bytes given to it.
                self.reverse_byte_literal = InvalidUtf8ByteSequence::try_from(ascii_byte).unwrap();
                self.bytes = remainder;
                return self.reverse_byte_literal.next_back();
            }
            Some(ch) => {
                self.bytes = &self.bytes[..self.bytes.len() - size];
                return Some(ch);
            }
            None if size == 0 => {}
            None => {
                let (remainder, invalid_utf8_bytes) = self.bytes.split_at(self.bytes.len() - size);
                // This conversion is safe to unwrap due to the documented
                // behavior of `bstr::decode_utf8` and `InvalidUtf8ByteSequence`
                // which indicate that `size` is always in the range of 0..=3.
                self.reverse_byte_literal = InvalidUtf8ByteSequence::try_from(invalid_utf8_bytes).unwrap();
                self.bytes = remainder;
                return self.reverse_byte_literal.next_back();
            }
        };
        if let Some(ch) = self.forward_byte_literal.next_back() {
            return Some(ch);
        }
        if let Some(ch) = self.flags.emit_leading_quote() {
            return Some(ch);
        }
        None
    }
}

impl<'a> FusedIterator for State<'a> {}

#[cfg(test)]
mod tests {
    use std::string::String;

    use super::Inspect;

    #[test]
    fn empty() {
        let inspect = Inspect::from("");
        let debug = inspect.collect::<String>();
        assert_eq!(debug, r#""""#);
    }

    #[test]
    fn empty_backwards() {
        let mut inspect = Inspect::from("");
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);

        let mut inspect = Inspect::from("");
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);

        let mut inspect = Inspect::from("");
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);
    }

    #[test]
    fn fred() {
        let inspect = Inspect::from("fred");
        let debug = inspect.collect::<String>();
        assert_eq!(debug, "\"fred\"");
    }

    #[test]
    fn fred_backwards() {
        let mut inspect = Inspect::from("fred");
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('d'));
        assert_eq!(inspect.next_back(), Some('e'));
        assert_eq!(inspect.next_back(), Some('r'));
        assert_eq!(inspect.next_back(), Some('f'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);
    }

    #[test]
    fn invalid_utf8_byte() {
        assert_eq!(Inspect::from(&b"\xFF"[..]).collect::<String>(), r#""\xFF""#);
    }

    #[test]
    fn invalid_utf8() {
        let inspect = Inspect::from(&b"invalid-\xFF-utf8"[..]);
        let debug = inspect.collect::<String>();
        assert_eq!(debug, r#""invalid-\xFF-utf8""#);
    }

    #[test]
    fn invalid_utf8_backwards() {
        let mut inspect = Inspect::from(&b"invalid-\xFF-utf8"[..]);
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('8'));
        assert_eq!(inspect.next_back(), Some('f'));
        assert_eq!(inspect.next_back(), Some('t'));
        assert_eq!(inspect.next_back(), Some('u'));
        assert_eq!(inspect.next_back(), Some('-'));
        assert_eq!(inspect.next_back(), Some('F'));
        assert_eq!(inspect.next_back(), Some('F'));
        assert_eq!(inspect.next_back(), Some('x'));
        assert_eq!(inspect.next_back(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('-'));
        assert_eq!(inspect.next_back(), Some('d'));
        assert_eq!(inspect.next_back(), Some('i'));
        assert_eq!(inspect.next_back(), Some('l'));
        assert_eq!(inspect.next_back(), Some('a'));
        assert_eq!(inspect.next_back(), Some('v'));
        assert_eq!(inspect.next_back(), Some('n'));
        assert_eq!(inspect.next_back(), Some('i'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);
    }

    #[test]
    fn quoted() {
        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('a'));
        assert_eq!(inspect.next(), Some('\\'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('b'));
        assert_eq!(inspect.next(), Some('"'));

        assert_eq!(Inspect::from(r#"a"b"#).collect::<String>(), r#""a\"b""#);
    }

    #[test]
    fn quote_backwards() {
        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('b'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('a'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
    }

    #[test]
    fn quote_double_ended() {
        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('a'));
        assert_eq!(inspect.next(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('b'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next(), None);

        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('a'));
        assert_eq!(inspect.next(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('b'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);

        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('b'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('a'));
        assert_eq!(inspect.next(), Some('\\'));
        assert_eq!(inspect.next(), None);

        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('b'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('a'));
        assert_eq!(inspect.next(), Some('\\'));
        assert_eq!(inspect.next_back(), None);

        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('b'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next_back(), Some('\\'));

        let mut inspect = Inspect::from(r#"a"b"#);
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('a'));
        assert_eq!(inspect.next(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next(), Some('"'));
    }

    #[test]
    fn emoji() {
        assert_eq!(Inspect::from("💎").collect::<String>(), "\"💎\"");
        assert_eq!(Inspect::from("$💎").collect::<String>(), "\"$💎\"");
        assert_eq!(Inspect::from("@💎").collect::<String>(), "\"@💎\"");
        assert_eq!(Inspect::from("@@💎").collect::<String>(), "\"@@💎\"");
    }

    #[test]
    fn unicode_replacement_char() {
        assert_eq!(Inspect::from("�").collect::<String>(), "\"�\"");
        assert_eq!(Inspect::from("$�").collect::<String>(), "\"$�\"");
        assert_eq!(Inspect::from("@�").collect::<String>(), "\"@�\"");
        assert_eq!(Inspect::from("@@�").collect::<String>(), "\"@@�\"");

        assert_eq!(Inspect::from("abc�").collect::<String>(), "\"abc�\"");
        assert_eq!(Inspect::from("$abc�").collect::<String>(), "\"$abc�\"");
        assert_eq!(Inspect::from("@abc�").collect::<String>(), "\"@abc�\"");
        assert_eq!(Inspect::from("@@abc�").collect::<String>(), "\"@@abc�\"");
    }

    #[test]
    fn escape_slash() {
        assert_eq!(Inspect::from("\\").collect::<String>(), r#""\\""#);
        assert_eq!(Inspect::from("foo\\bar").collect::<String>(), r#""foo\\bar""#);
    }

    #[test]
    fn escape_slash_backwards() {
        let mut inspect = Inspect::from("a\\b");
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('b'));
        assert_eq!(inspect.next_back(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('a'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);
    }

    #[test]
    fn nul() {
        assert_eq!(Inspect::from("\0").collect::<String>(), r#""\x00""#);
    }

    #[test]
    fn del() {
        assert_eq!(Inspect::from("\x7F").collect::<String>(), r#""\x7F""#);
    }

    #[test]
    fn ascii_control() {
        assert_eq!(Inspect::from("\0").collect::<String>(), r#""\x00""#);
        assert_eq!(Inspect::from("\x01").collect::<String>(), r#""\x01""#);
        assert_eq!(Inspect::from("\x02").collect::<String>(), r#""\x02""#);
        assert_eq!(Inspect::from("\x03").collect::<String>(), r#""\x03""#);
        assert_eq!(Inspect::from("\x04").collect::<String>(), r#""\x04""#);
        assert_eq!(Inspect::from("\x05").collect::<String>(), r#""\x05""#);
        assert_eq!(Inspect::from("\x06").collect::<String>(), r#""\x06""#);
        assert_eq!(Inspect::from("\x07").collect::<String>(), r#""\a""#);
        assert_eq!(Inspect::from("\x08").collect::<String>(), r#""\b""#);
        assert_eq!(Inspect::from("\x09").collect::<String>(), r#""\t""#);
        assert_eq!(Inspect::from("\x0A").collect::<String>(), r#""\n""#);
        assert_eq!(Inspect::from("\x0B").collect::<String>(), r#""\v""#);
        assert_eq!(Inspect::from("\x0C").collect::<String>(), r#""\f""#);
        assert_eq!(Inspect::from("\x0D").collect::<String>(), r#""\r""#);
        assert_eq!(Inspect::from("\x0E").collect::<String>(), r#""\x0E""#);
        assert_eq!(Inspect::from("\x0F").collect::<String>(), r#""\x0F""#);
        assert_eq!(Inspect::from("\x10").collect::<String>(), r#""\x10""#);
        assert_eq!(Inspect::from("\x11").collect::<String>(), r#""\x11""#);
        assert_eq!(Inspect::from("\x12").collect::<String>(), r#""\x12""#);
        assert_eq!(Inspect::from("\x13").collect::<String>(), r#""\x13""#);
        assert_eq!(Inspect::from("\x14").collect::<String>(), r#""\x14""#);
        assert_eq!(Inspect::from("\x15").collect::<String>(), r#""\x15""#);
        assert_eq!(Inspect::from("\x16").collect::<String>(), r#""\x16""#);
        assert_eq!(Inspect::from("\x17").collect::<String>(), r#""\x17""#);
        assert_eq!(Inspect::from("\x18").collect::<String>(), r#""\x18""#);
        assert_eq!(Inspect::from("\x19").collect::<String>(), r#""\x19""#);
        assert_eq!(Inspect::from("\x1A").collect::<String>(), r#""\x1A""#);
        assert_eq!(Inspect::from("\x1B").collect::<String>(), r#""\e""#);
        assert_eq!(Inspect::from("\x1C").collect::<String>(), r#""\x1C""#);
        assert_eq!(Inspect::from("\x1D").collect::<String>(), r#""\x1D""#);
        assert_eq!(Inspect::from("\x1E").collect::<String>(), r#""\x1E""#);
        assert_eq!(Inspect::from("\x1F").collect::<String>(), r#""\x1F""#);
        assert_eq!(Inspect::from("\x20").collect::<String>(), r#"" ""#);
    }

    #[test]
    fn special_escapes() {
        // double quote
        assert_eq!(Inspect::from("\x22").collect::<String>(), r#""\"""#);
        assert_eq!(Inspect::from("\"").collect::<String>(), r#""\"""#);
        // backslash
        assert_eq!(Inspect::from("\x5C").collect::<String>(), r#""\\""#);
        assert_eq!(Inspect::from("\\").collect::<String>(), r#""\\""#);
    }

    #[test]
    fn invalid_utf8_special_global() {
        assert_eq!(Inspect::from(&b"$-\xFF"[..]).collect::<String>(), r#""$-\xFF""#);
    }

    #[test]
    fn replacement_char_special_global() {
        assert_eq!(Inspect::from("$-�").collect::<String>(), "\"$-�\"");
        assert_eq!(Inspect::from("$-�a").collect::<String>(), r#""$-�a""#);
        assert_eq!(Inspect::from("$-��").collect::<String>(), r#""$-��""#);
    }
}

#[cfg(test)]
mod specs {
    use super::Flags;

    #[test]
    fn flags_default() {
        let mut flags = Flags::DEFAULT;

        assert_eq!(flags.emit_leading_quote(), Some('"'));
        assert_eq!(flags.emit_leading_quote(), None);

        assert_eq!(flags.emit_trailing_quote(), Some('"'));
        assert_eq!(flags.emit_trailing_quote(), None);
    }
}
