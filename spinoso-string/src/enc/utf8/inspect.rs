use core::iter::FusedIterator;

use scolapasta_string_escape::{ascii_char_with_escape, InvalidUtf8ByteSequence};

use super::Utf8String;
use crate::inspect::Flags;

#[derive(Debug, Clone)]
#[must_use = "this `Inspect` is an `Iterator`, which should be consumed if constructed"]
pub struct Inspect<'a> {
    flags: Flags,
    escaped_bytes: &'static [u8],
    byte_literal: InvalidUtf8ByteSequence,
    bytes: &'a [u8],
}

impl<'a> From<&'a Utf8String> for Inspect<'a> {
    #[inline]
    fn from(value: &'a Utf8String) -> Self {
        Self::new(value.as_slice())
    }
}

impl<'a> Inspect<'a> {
    /// Construct a UTF-8 `Inspect` for the given byte slice.
    ///
    /// This constructor produces inspect contents like `"fred"`.
    #[inline]
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            flags: Flags::DEFAULT,
            escaped_bytes: &[],
            byte_literal: InvalidUtf8ByteSequence::new(),
            bytes,
        }
    }
}

impl<'a> Default for Inspect<'a> {
    /// Construct an `Inspect` that will render debug output for the empty
    /// slice.
    ///
    /// This constructor produces inspect contents like `""`.
    #[inline]
    fn default() -> Self {
        Self::new(b"")
    }
}

impl<'a> Iterator for Inspect<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.flags.emit_leading_quote() {
            return Some(ch);
        }
        if let Some((&head, tail)) = self.escaped_bytes.split_first() {
            self.escaped_bytes = tail;
            return Some(head.into());
        }
        if let Some(ch) = self.byte_literal.next() {
            return Some(ch);
        }
        let (ch, size) = bstr::decode_utf8(self.bytes);
        match ch.map(|ch| {
            ascii_char_with_escape(ch)
                .and_then(|esc| esc.as_bytes().split_first())
                .ok_or(ch)
        }) {
            Some(Ok((&head, tail))) => {
                self.escaped_bytes = tail;
                self.bytes = &self.bytes[size..];
                return Some(head.into());
            }
            Some(Err(ch)) => {
                self.bytes = &self.bytes[size..];
                return Some(ch);
            }
            None if size == 0 => {}
            None => {
                let (invalid_utf8_bytes, remainder) = self.bytes.split_at(size);
                // This conversion is safe to unwrap due to the documented
                // behavior of `bstr::decode_utf8` and `InvalidUtf8ByteSequence`
                // which indicate that `size` is always in the range of 0..=3.
                self.byte_literal = InvalidUtf8ByteSequence::try_from(invalid_utf8_bytes)
                    .expect("Invalid UTF-8 byte sequence should be at most 3 bytes long");
                self.bytes = remainder;
                return self.byte_literal.next();
            }
        };
        self.flags.emit_trailing_quote()
    }
}

impl<'a> FusedIterator for Inspect<'a> {}

#[cfg(test)]
mod tests {
    use alloc::string::{String, ToString};

    use super::{Inspect, Utf8String};

    #[test]
    fn empty() {
        let s = "";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""""#);
    }

    #[test]
    fn fred() {
        let s = "fred";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""fred""#);
    }

    #[test]
    fn invalid_utf8_byte() {
        let s = b"\xFF";
        let s = Utf8String::new(s.to_vec());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\xFF""#);
    }

    #[test]
    fn invalid_utf8() {
        let s = b"invalid-\xFF-utf8";
        let s = Utf8String::new(s.to_vec());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""invalid-\xFF-utf8""#);
    }

    #[test]
    fn quote_collect() {
        let s = r#"a"b"#;
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);
        assert_eq!(inspect.collect::<String>(), r#""a\"b""#);
    }

    #[test]
    fn quote_iter() {
        let s = r#"a"b"#;
        let s = Utf8String::new(s.to_string().into_bytes());
        let mut inspect = Inspect::from(&s);

        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('a'));
        assert_eq!(inspect.next(), Some('\\'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('b'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), None);
    }

    #[test]
    fn emoji() {
        let s = "💎";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"💎\"");
    }

    #[test]
    fn emoji_global() {
        let s = "$💎";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"$💎\"");
    }

    #[test]
    fn emoji_ivar() {
        let s = "@💎";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"@💎\"");
    }

    #[test]
    fn emoji_cvar() {
        let s = "@@💎";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"@@💎\"");
    }

    #[test]
    fn unicode_replacement_char() {
        let s = "�";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"�\"");
    }

    #[test]
    fn unicode_replacement_char_global() {
        let s = "$�";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"$�\"");
    }

    #[test]
    fn unicode_replacement_char_ivar() {
        let s = "@�";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"@�\"");
    }

    #[test]
    fn unicode_replacement_char_cvar() {
        let s = "@@�";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"@@�\"");
    }

    #[test]
    fn escape_slash() {
        let s = r"\";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\\""#);
    }

    #[test]
    fn escape_inner_slash() {
        let s = r"foo\bar";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""foo\\bar""#);
    }

    #[test]
    fn nul() {
        let s = "\0";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\x00""#);
    }

    #[test]
    fn del() {
        let s = "\x7F";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\x7F""#);
    }

    #[test]
    fn ascii_control() {
        let test_cases = [
            ["\x00", r#""\x00""#],
            ["\x01", r#""\x01""#],
            ["\x02", r#""\x02""#],
            ["\x03", r#""\x03""#],
            ["\x04", r#""\x04""#],
            ["\x05", r#""\x05""#],
            ["\x06", r#""\x06""#],
            ["\x07", r#""\a""#],
            ["\x08", r#""\b""#],
            ["\x09", r#""\t""#],
            ["\x0A", r#""\n""#],
            ["\x0B", r#""\v""#],
            ["\x0C", r#""\f""#],
            ["\x0D", r#""\r""#],
            ["\x0E", r#""\x0E""#],
            ["\x0F", r#""\x0F""#],
            ["\x10", r#""\x10""#],
            ["\x11", r#""\x11""#],
            ["\x12", r#""\x12""#],
            ["\x13", r#""\x13""#],
            ["\x14", r#""\x14""#],
            ["\x15", r#""\x15""#],
            ["\x16", r#""\x16""#],
            ["\x17", r#""\x17""#],
            ["\x18", r#""\x18""#],
            ["\x19", r#""\x19""#],
            ["\x1A", r#""\x1A""#],
            ["\x1B", r#""\e""#],
            ["\x1C", r#""\x1C""#],
            ["\x1D", r#""\x1D""#],
            ["\x1E", r#""\x1E""#],
            ["\x1F", r#""\x1F""#],
            ["\x20", r#"" ""#],
        ];
        for [s, r] in test_cases {
            let s = Utf8String::new(s.to_string().into_bytes());
            let inspect = Inspect::from(&s);
            assert_eq!(inspect.collect::<String>(), r, "For {s:?}, expected {r}");
        }
    }

    #[test]
    fn special_double_quote() {
        let s = "\x22";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\"""#);

        let s = "\"";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\"""#);
    }

    #[test]
    fn special_backslash() {
        let s = "\x5C";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\\""#);

        let s = "\\";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\\""#);
    }

    #[test]
    fn invalid_utf8_special_global() {
        let s = b"$-\xFF";
        let s = Utf8String::new(s.to_vec());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""$-\xFF""#);
    }

    #[test]
    fn replacement_char_special_global() {
        let s = "$-�";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), "\"$-�\"");

        let s = "$-�a";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""$-�a""#);

        let s = "$-��";
        let s = Utf8String::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""$-��""#);
    }
}
