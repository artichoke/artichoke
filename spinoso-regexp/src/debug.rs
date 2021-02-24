use core::convert::TryFrom;
use core::fmt;
use core::iter::FusedIterator;
use scolapasta_string_escape::InvalidUtf8ByteSequence;

#[derive(Debug, Clone, Copy)]
pub struct Debug<'a> {
    // When `Regexp`s are constructed with a `/.../` literal, `Regexp#source`
    // refers to the literal characters contained within the `/` delimeters.
    // For example, `/\t/.source.bytes` has byte sequence `[92, 116]`.
    //
    // When `Regexp`s are constructed with `Regexp::compile`, `Regexp#source`
    // refers to the argument passed to `compile`. For example,
    // `Regexp.compile("\t").source.bytes` has byte sequence `[9]`.
    //
    // `Regexp#inspect` prints `"/#{source}/"`.
    source: &'a [u8],
    options: &'static str,
    encoding: &'static str,
}

impl<'a> Debug<'a> {
    pub fn iter(&self) -> Iter<'a> {
        Iter {
            prefix: Some('/'),
            source: self.source,
            literal: InvalidUtf8ByteSequence::new(),
            suffix: Some('/'),
            options: self.options,
            encoding: self.encoding,
        }
    }

    pub fn fmt_into<W: fmt::Write>(&self, mut dest: W) -> fmt::Result {
        let mut buf = [0; 4];
        for ch in self {
            let enc = ch.encode_utf8(&mut buf);
            dest.write_str(enc)?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for Debug<'a> {
    type Item = char;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &Debug<'a> {
    type Item = char;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a> {
    prefix: Option<char>,
    source: &'a [u8],
    literal: InvalidUtf8ByteSequence,
    suffix: Option<char>,
    options: &'static str,
    encoding: &'static str,
}

impl<'a> Iterator for Iter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prefix) = self.prefix.take() {
            return Some(prefix);
        }
        if let Some(literal) = self.literal.next() {
            return Some(literal);
        }
        if !self.source.is_empty() {
            let (ch, size) = bstr::decode_utf8(self.source);
            let next = match ch {
                // '/' is the `Regexp` literal delimeter, so escape it.
                Some('/') => {
                    // While not an invalid byte, we rely on the documented
                    // behavior of `InvalidUtf8ByteSequence` to always escape
                    // any bytes given to it.
                    self.literal = InvalidUtf8ByteSequence::with_byte(b'/');
                    Some('\\')
                }
                Some(ch) => Some(ch),
                // Otherwise, we've gotten invalid UTF-8, which means this is not an
                // printable char.
                None => {
                    // This conversion is safe to unwrap due to the documented
                    // behavior of `bstr::decode_utf8` and `InvalidUtf8ByteSequence`
                    // which indicate that `size` is always in the range of 0..=3.
                    self.literal = InvalidUtf8ByteSequence::try_from(&self.source[..size]).unwrap();
                    // `size` is non-zero because `pattern` is non-empty.
                    // `Literal`s created from > one byte are always non-empty.
                    self.literal.next()
                }
            };
            self.source = &self.source[size..];
            return next;
        }
        if let Some(suffix) = self.suffix.take() {
            return Some(suffix);
        }
        if let (Some(ch), size) = bstr::decode_utf8(self.options) {
            self.options = &self.options[size..];
            return Some(ch);
        }
        if let (Some(ch), size) = bstr::decode_utf8(self.encoding) {
            self.encoding = &self.encoding[size..];
            return Some(ch);
        }
        None
    }
}

impl<'a> FusedIterator for Iter<'a> {}

#[cfg(test)]
mod tests {
    use super::Debug;

    // fmt::Write

    #[test]
    fn fmt_utf8_pattern_no_opt_no_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/
        // => /Artichoke Ruby/
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/Artichoke Ruby/");
    }

    #[test]
    fn fmt_utf8_pattern_with_opts_no_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/i
        // => /Artichoke Ruby/i
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "i",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/Artichoke Ruby/i");

        // ```ruby
        // [2.6.6] > /Artichoke Ruby/mix
        // => /Artichoke Ruby/mix
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "mix",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/Artichoke Ruby/mix");
    }

    #[test]
    fn fmt_utf8_pattern_no_opts_with_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/n
        // => /Artichoke Ruby/n
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "",
            encoding: "n",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/Artichoke Ruby/n");
    }

    #[test]
    fn fmt_utf8_pattern_with_opts_with_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/nix
        // => /Artichoke Ruby/ixn
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "ix",
            encoding: "n",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/Artichoke Ruby/ixn");
    }

    #[test]
    fn fmt_utf8_emoji_pattern_no_opt_no_enc() {
        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/
        // => /crab 🦀 for Rust/
        // ```
        let debug = Debug {
            source: "crab 🦀 for Rust".as_bytes(),
            options: "",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/crab 🦀 for Rust/");
    }

    #[test]
    fn fmt_utf8_emoji_pattern_with_opts_no_enc() {
        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/i
        // => /crab 🦀 for Rust/i
        // ```
        let debug = Debug {
            source: "crab 🦀 for Rust".as_bytes(),
            options: "i",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/crab 🦀 for Rust/i");

        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/mix
        // => /crab 🦀 for Rust/mix
        // ```
        let debug = Debug {
            source: "crab 🦀 for Rust".as_bytes(),
            options: "mix",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/crab 🦀 for Rust/mix");
    }

    #[test]
    fn fmt_ascii_escaped_byte_pattern_literal_exhaustive() {
        // ```ruby
        // [2.6.6] > /"\a\b\c\e\f\r\n\\\"$$"/
        // => /"\a\b\c\e\f\r\n\\\"$$"/
        // [2.6.6] > /"\a\b\c\e\f\r\n\\\"$$"/.source.bytes
        // => [34, 92, 97, 92, 98, 92, 99, 92, 101, 92, 102, 92, 114, 92, 110, 92, 92, 92, 34, 36, 36, 34]
        // ```
        let pattern = [
            34, 92, 97, 92, 98, 92, 99, 92, 101, 92, 102, 92, 114, 92, 110, 92, 92, 92, 34, 36, 36, 34,
        ];
        let debug = Debug {
            source: &pattern,
            options: "",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, r#"/"\a\b\c\e\f\r\n\\\"$$"/"#);
    }

    #[test]
    fn fmt_ascii_escaped_byte_pattern_literal() {
        // ```ruby
        // [2.6.6] > /\t\v\f\n/
        // => /\t\v\f\n/
        // [2.6.6] > /\t\v\f\n/.source.bytes
        // => [92, 116, 92, 118, 92, 102, 92, 110]
        // ```
        let pattern = [92, 116, 92, 118, 92, 102, 92, 110];
        let debug = Debug {
            source: &pattern,
            options: "",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, r"/\t\v\f\n/");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/i
        // => /\t\v\f\n/i
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "i",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, r"/\t\v\f\n/i");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/mix
        // => /\t\v\f\n/mix
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "mix",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, r"/\t\v\f\n/mix");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/n
        // => /\t\v\f\n/n
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "",
            encoding: "n",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, r"/\t\v\f\n/n");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/nix
        // => /\t\v\f\n/ixn
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "ix",
            encoding: "n",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, r"/\t\v\f\n/ixn");
    }

    #[test]
    fn fmt_ascii_escaped_byte_pattern_compiled() {
        // ```ruby
        // [2.6.6] > Regexp.compile('      "')
        // => /	"/
        // [2.6.6] > Regexp.compile('      "').source.bytes
        // => [9, 34]
        // ```
        let pattern = [9, 34];
        let debug = Debug {
            source: &pattern,
            options: "",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, "/\t\"/");
    }

    #[test]
    fn fmt_invalid_utf8_pattern() {
        // ```ruby
        // [2.6.6] > Regexp.compile("\xFF\xFE".force_encoding(Encoding::BINARY))
        // => /\xFF\xFE/
        // ```
        let debug = Debug {
            source: b"\xFF\xFE",
            options: "",
            encoding: "",
        };
        let mut s = String::new();
        debug.fmt_into(&mut s).unwrap();
        assert_eq!(s, r"/\xFF\xFE/");
    }

    // Iterator + Collect

    #[test]
    fn iter_utf8_pattern_no_opt_no_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/
        // => /Artichoke Ruby/
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/");
    }

    #[test]
    fn iter_utf8_pattern_with_opts_no_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/i
        // => /Artichoke Ruby/i
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "i",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/i");

        // ```ruby
        // [2.6.6] > /Artichoke Ruby/mix
        // => /Artichoke Ruby/mix
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "mix",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/mix");
    }

    #[test]
    fn iter_utf8_pattern_no_opts_with_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/n
        // => /Artichoke Ruby/n
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "",
            encoding: "n",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/n");
    }

    #[test]
    fn iter_utf8_pattern_with_opts_with_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/nix
        // => /Artichoke Ruby/ixn
        // ```
        let debug = Debug {
            source: b"Artichoke Ruby",
            options: "ix",
            encoding: "n",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/ixn");
    }

    #[test]
    fn iter_utf8_emoji_pattern_no_opt_no_enc() {
        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/
        // => /crab 🦀 for Rust/
        // ```
        let debug = Debug {
            source: "crab 🦀 for Rust".as_bytes(),
            options: "",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/crab 🦀 for Rust/");
    }

    #[test]
    fn iter_utf8_emoji_pattern_with_opts_no_enc() {
        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/i
        // => /crab 🦀 for Rust/i
        // ```
        let debug = Debug {
            source: "crab 🦀 for Rust".as_bytes(),
            options: "i",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/crab 🦀 for Rust/i");

        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/mix
        // => /crab 🦀 for Rust/mix
        // ```
        let debug = Debug {
            source: "crab 🦀 for Rust".as_bytes(),
            options: "mix",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/crab 🦀 for Rust/mix");
    }

    #[test]
    fn iter_ascii_escaped_byte_pattern_literal_exhaustive() {
        // ```ruby
        // [2.6.6] > /"\a\b\c\e\f\r\n\\\"$$"/
        // => /"\a\b\c\e\f\r\n\\\"$$"/
        // [2.6.6] > /"\a\b\c\e\f\r\n\\\"$$"/.source.bytes
        // => [34, 92, 97, 92, 98, 92, 99, 92, 101, 92, 102, 92, 114, 92, 110, 92, 92, 92, 34, 36, 36, 34]
        // ```
        let pattern = [
            34, 92, 97, 92, 98, 92, 99, 92, 101, 92, 102, 92, 114, 92, 110, 92, 92, 92, 34, 36, 36, 34,
        ];
        let debug = Debug {
            source: &pattern,
            options: "",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, r#"/"\a\b\c\e\f\r\n\\\"$$"/"#);
    }

    #[test]
    fn iter_ascii_escaped_byte_pattern_literal() {
        // ```ruby
        // [2.6.6] > /\t\v\f\n/
        // => /\t\v\f\n/
        // [2.6.6] > /\t\v\f\n/.source.bytes
        // => [92, 116, 92, 118, 92, 102, 92, 110]
        // ```
        let pattern = [92, 116, 92, 118, 92, 102, 92, 110];
        let debug = Debug {
            source: &pattern,
            options: "",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/i
        // => /\t\v\f\n/i
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "i",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/i");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/mix
        // => /\t\v\f\n/mix
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "mix",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/mix");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/n
        // => /\t\v\f\n/n
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "",
            encoding: "n",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/n");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/nix
        // => /\t\v\f\n/ixn
        // ```
        let debug = Debug {
            source: br"\t\v\f\n",
            options: "ix",
            encoding: "n",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/ixn");
    }

    #[test]
    fn iter_ascii_escaped_byte_pattern_compiled() {
        // ```ruby
        // [2.6.6] > Regexp.compile('      "')
        // => /	"/
        // [2.6.6] > Regexp.compile('      "').source.bytes
        // => [9, 34]
        // ```
        let pattern = [9, 34];
        let debug = Debug {
            source: &pattern,
            options: "",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, "/\t\"/");
    }

    #[test]
    fn iter_invalid_utf8_pattern() {
        // ```ruby
        // [2.6.6] > Regexp.compile("\xFF\xFE".force_encoding(Encoding::BINARY))
        // => /\xFF\xFE/
        // ```
        let debug = Debug {
            source: b"\xFF\xFE",
            options: "",
            encoding: "",
        };
        let s = debug.iter().collect::<String>();
        assert_eq!(s, r"/\xFF\xFE/");
    }
}
