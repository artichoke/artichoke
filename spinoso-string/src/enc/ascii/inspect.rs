use core::iter::FusedIterator;
use core::slice;
use core::str::Chars;

use scolapasta_string_escape::Literal;

use super::AsciiString;
use crate::inspect::Flags;

#[derive(Default, Debug, Clone)]
#[must_use = "this `Inspect` is an `Iterator`, which should be consumed if constructed"]
pub struct Inspect<'a>(State<'a>);

impl<'a> From<&'a AsciiString> for Inspect<'a> {
    #[inline]
    fn from(value: &'a AsciiString) -> Self {
        Self(State::new(value.as_slice()))
    }
}

impl<'a> Iterator for Inspect<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> FusedIterator for Inspect<'a> {}

#[derive(Debug, Clone)]
#[must_use = "this `State` is an `Iterator`, which should be consumed if constructed"]
struct State<'a> {
    flags: Flags,
    literal: Chars<'static>,
    bytes: slice::Iter<'a, u8>,
}

impl<'a> State<'a> {
    /// Construct a `State` for the given byte slice.
    ///
    /// This constructor produces inspect contents like `"fred"`.
    #[inline]
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            flags: Flags::DEFAULT,
            literal: "".chars(),
            bytes: bytes.iter(),
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
        if let Some(ch) = self.literal.next() {
            return Some(ch);
        }
        if let Some(&ch) = self.bytes.next() {
            self.literal = Literal::debug_escape(ch).chars();
        }
        if let Some(ch) = self.literal.next() {
            return Some(ch);
        }
        self.flags.emit_trailing_quote()
    }
}

impl<'a> FusedIterator for State<'a> {}

#[cfg(test)]
mod tests {
    use alloc::string::{String, ToString};

    use super::{AsciiString, Inspect};

    #[test]
    fn empty() {
        let s = "";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""""#);
    }

    #[test]
    fn fred() {
        let s = "fred";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""fred""#);
    }

    #[test]
    fn invalid_utf8_byte() {
        let s = b"\xFF";
        let s = AsciiString::new(s.to_vec());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\xFF""#);
    }

    #[test]
    fn invalid_utf8() {
        let s = b"invalid-\xFF-utf8";
        let s = AsciiString::new(s.to_vec());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""invalid-\xFF-utf8""#);
    }

    #[test]
    fn quote_collect() {
        let s = r#"a"b"#;
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);
        assert_eq!(inspect.collect::<String>(), r#""a\"b""#);
    }

    #[test]
    fn quote_iter() {
        let s = r#"a"b"#;
        let s = AsciiString::new(s.to_string().into_bytes());
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
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\xF0\x9F\x92\x8E""#);
    }

    #[test]
    fn unicode_replacement_char() {
        let s = "�";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\xEF\xBF\xBD""#);
    }

    #[test]
    fn escape_slash() {
        let s = r"\";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\\""#);
    }

    #[test]
    fn escape_inner_slash() {
        let s = r"foo\bar";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""foo\\bar""#);
    }

    #[test]
    fn nul() {
        let s = "\0";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\x00""#);
    }

    #[test]
    fn del() {
        let s = "\x7F";
        let s = AsciiString::new(s.to_string().into_bytes());
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
            let s = AsciiString::new(s.to_string().into_bytes());
            let inspect = Inspect::from(&s);
            assert_eq!(inspect.collect::<String>(), r, "For {s:?}, expected {r}");
        }
    }

    #[test]
    fn special_double_quote() {
        let s = "\x22";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\"""#);

        let s = "\"";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\"""#);
    }

    #[test]
    fn special_backslash() {
        let s = "\x5C";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\\""#);

        let s = "\\";
        let s = AsciiString::new(s.to_string().into_bytes());
        let inspect = Inspect::from(&s);

        assert_eq!(inspect.collect::<String>(), r#""\\""#);
    }
}
