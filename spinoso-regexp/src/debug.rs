use core::iter::FusedIterator;

use scolapasta_string_escape::InvalidUtf8ByteSequence;

#[derive(Debug, Clone)]
struct Delimiters {
    bits: u8,
}

impl Default for Delimiters {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Delimiters {
    const EMIT_LEFT_DELIMITER: Self = Self { bits: 0b0000_0001 };
    const EMIT_RIGHT_DELIMITER: Self = Self { bits: 0b0000_0010 };

    const DEFAULT: Self = Self {
        bits: Self::EMIT_LEFT_DELIMITER.bits | Self::EMIT_RIGHT_DELIMITER.bits,
    };

    #[inline]
    fn emit_left_delimiter(&mut self) -> Option<char> {
        if (self.bits & Self::EMIT_LEFT_DELIMITER.bits) == Self::EMIT_LEFT_DELIMITER.bits {
            self.bits &= !Self::EMIT_LEFT_DELIMITER.bits;
            Some('/')
        } else {
            None
        }
    }

    #[inline]
    fn emit_right_delimiter(&mut self) -> Option<char> {
        if (self.bits & Self::EMIT_RIGHT_DELIMITER.bits) == Self::EMIT_RIGHT_DELIMITER.bits {
            self.bits &= !Self::EMIT_RIGHT_DELIMITER.bits;
            Some('/')
        } else {
            None
        }
    }
}

/// An iterator that yields a debug representation of a `Regexp` as a sequence
/// of `char`s.
///
/// This struct is created by the `debug` method on the regexp implementations
/// in this crate. See these functions' documentation for more.
///
/// This iterator can be used to implement Ruby's [`Regexp#inspect`].
///
/// # Examples
///
/// UTF-8 regexp patterns and options are formatted in a debug
/// representation:
///
/// ```
/// use spinoso_regexp::Debug;
///
/// let debug = Debug::new("crab 🦀 for Rust".as_bytes(), "mix", "");
/// let s = debug.collect::<String>();
/// assert_eq!(s, "/crab 🦀 for Rust/mix");
/// ```
///
/// Binary content is hex escaped:
///
/// ```
/// use spinoso_regexp::Debug;
///
/// let debug = Debug::new(b"\xFF\xFE", "", "");
/// let s = debug.collect::<String>();
/// assert_eq!(s, r"/\xFF\xFE/");
/// ```
///
/// [`Regexp#inspect`]: https://ruby-doc.org/core-2.4.1/Regexp.html#method-i-inspect
#[derive(Default, Debug, Clone)]
#[must_use = "this `Debug` is an `Iterator`, which should be consumed if constructed"]
pub struct Debug<'a> {
    delimiters: Delimiters,
    // When `Regexp`s are constructed with a `/.../` literal, `Regexp#source`
    // refers to the literal characters contained within the `/` delimiters.
    // For example, `/\t/.source.bytes` has byte sequence `[92, 116]`.
    //
    // When `Regexp`s are constructed with `Regexp::compile`, `Regexp#source`
    // refers to the argument passed to `compile`. For example,
    // `Regexp.compile("\t").source.bytes` has byte sequence `[9]`.
    //
    // `Regexp#inspect` prints `"/#{source}/"`.
    source: &'a [u8],
    non_standard_control_escapes: &'static [u8],
    literal: InvalidUtf8ByteSequence,
    options: &'static str,
    encoding: &'static str,
}

impl<'a> Debug<'a> {
    /// Construct a new `Debug` iterator with a regexp source, [options
    /// modifiers], and [encoding modifiers].
    ///
    /// # Examples
    ///
    /// UTF-8 regexp patterns and options are formatted in a debug
    /// representation:
    ///
    /// ```
    /// use spinoso_regexp::Debug;
    ///
    /// let debug = Debug::new("crab 🦀 for Rust".as_bytes(), "mix", "");
    /// let s = debug.collect::<String>();
    /// assert_eq!(s, "/crab 🦀 for Rust/mix");
    /// ```
    ///
    /// Binary content is hex escaped:
    ///
    /// ```
    /// use spinoso_regexp::Debug;
    ///
    /// let debug = Debug::new(b"\xFF\xFE", "", "");
    /// let s = debug.collect::<String>();
    /// assert_eq!(s, r"/\xFF\xFE/");
    /// ```
    ///
    /// [options modifiers]: crate::Options::as_display_modifier
    /// [encoding modifiers]: crate::Encoding::as_modifier_str
    pub fn new(source: &'a [u8], options: &'static str, encoding: &'static str) -> Self {
        Self {
            delimiters: Delimiters::DEFAULT,
            source,
            non_standard_control_escapes: &[],
            literal: InvalidUtf8ByteSequence::new(),
            options,
            encoding,
        }
    }
}

impl<'a> Iterator for Debug<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prefix) = self.delimiters.emit_left_delimiter() {
            return Some(prefix);
        }
        if let Some((&next, tail)) = self.non_standard_control_escapes.split_first() {
            self.non_standard_control_escapes = tail;
            return Some(next.into());
        }
        if let Some(literal) = self.literal.next() {
            return Some(literal);
        }
        if !dbg!(self.source).is_empty() {
            let (ch, size) = bstr::decode_utf8(self.source);
            return match ch {
                // '/' is the `Regexp` literal delimiter, so escape it.
                Some('/') => {
                    self.source = &self.source[1..];
                    // While not an invalid byte, we rely on the documented
                    // behavior of `InvalidUtf8ByteSequence` to always escape
                    // any bytes given to it.
                    self.literal = InvalidUtf8ByteSequence::with_byte(b'/');
                    Some('\\')
                }
                Some('\x07') => {
                    self.source = &self.source[1..];
                    let (&next, tail) = br"\x07".split_first().unwrap();
                    self.non_standard_control_escapes = tail;
                    Some(next.into())
                }
                Some('\x08') => {
                    self.source = &self.source[1..];
                    let (&next, tail) = br"\x08".split_first().unwrap();
                    self.non_standard_control_escapes = tail;
                    Some(next.into())
                }
                Some('\x1B') => {
                    self.source = &self.source[1..];
                    let (&next, tail) = br"\x1B".split_first().unwrap();
                    self.non_standard_control_escapes = tail;
                    Some(next.into())
                }
                Some(ch @ '"' | ch @ '\'' | ch @ '\\') => {
                    self.source = &self.source[1..];
                    Some(ch)
                }
                Some(ch) if ch.is_ascii() && posix_space::is_space(ch as u8) => {
                    self.source = &self.source[1..];
                    Some(ch)
                }
                Some(ch) if ch.is_ascii() => {
                    self.source = &self.source[1..];
                    // While not an invalid byte, we rely on the documented
                    // behavior of `InvalidUtf8ByteSequence` to always escape
                    // any bytes given to it.
                    self.literal = dbg!(InvalidUtf8ByteSequence::with_byte(ch as u8));
                    self.literal.next()
                }
                Some(ch) => {
                    self.source = &self.source[size..];
                    Some(ch)
                }
                // Otherwise, we've gotten invalid UTF-8, which means this is not an
                // printable char.
                None => {
                    let (chunk, remainder) = self.source.split_at(size);
                    self.source = remainder;
                    // This conversion is safe to unwrap due to the documented
                    // behavior of `bstr::decode_utf8` and `InvalidUtf8ByteSequence`
                    // which indicate that `size` is always in the range of 0..=3.
                    self.literal = InvalidUtf8ByteSequence::try_from(chunk).unwrap();
                    // `size` is non-zero because `pattern` is non-empty.
                    // `Literal`s created from > one byte are always non-empty.
                    self.literal.next()
                }
            };
        }
        if let Some(suffix) = self.delimiters.emit_right_delimiter() {
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

impl<'a> FusedIterator for Debug<'a> {}

#[cfg(test)]
mod tests {
    use bstr::{ByteSlice, B};

    use super::Debug;

    // Iterator + Collect

    #[test]
    fn iter_utf8_pattern_no_opt_no_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/
        // => /Artichoke Ruby/
        // ```
        let debug = Debug::new(b"Artichoke Ruby", "", "");
        let s = debug.collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/");
    }

    #[test]
    fn iter_utf8_pattern_with_opts_no_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/i
        // => /Artichoke Ruby/i
        // ```
        let debug = Debug::new(b"Artichoke Ruby", "i", "");
        let s = debug.collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/i");

        // ```ruby
        // [2.6.6] > /Artichoke Ruby/mix
        // => /Artichoke Ruby/mix
        // ```
        let debug = Debug::new(b"Artichoke Ruby", "mix", "");
        let s = debug.collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/mix");
    }

    #[test]
    fn iter_utf8_pattern_no_opts_with_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/n
        // => /Artichoke Ruby/n
        // ```
        let debug = Debug::new(b"Artichoke Ruby", "", "n");
        let s = debug.collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/n");
    }

    #[test]
    fn iter_utf8_pattern_with_opts_with_enc() {
        // ```ruby
        // [2.6.6] > /Artichoke Ruby/nix
        // => /Artichoke Ruby/ixn
        // ```
        let debug = Debug::new(b"Artichoke Ruby", "ix", "n");
        let s = debug.collect::<String>();
        assert_eq!(s, "/Artichoke Ruby/ixn");
    }

    #[test]
    fn iter_utf8_emoji_pattern_no_opt_no_enc() {
        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/
        // => /crab 🦀 for Rust/
        // ```
        let debug = Debug::new("crab 🦀 for Rust".as_bytes(), "", "");
        let s = debug.collect::<String>();
        assert_eq!(s, "/crab 🦀 for Rust/");
    }

    #[test]
    fn iter_utf8_emoji_pattern_with_opts_no_enc() {
        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/i
        // => /crab 🦀 for Rust/i
        // ```
        let debug = Debug::new("crab 🦀 for Rust".as_bytes(), "i", "");
        let s = debug.collect::<String>();
        assert_eq!(s, "/crab 🦀 for Rust/i");

        // ```ruby
        // [2.6.6] > /crab 🦀 for Rust/mix
        // => /crab 🦀 for Rust/mix
        // ```
        let debug = Debug::new("crab 🦀 for Rust".as_bytes(), "mix", "");
        let s = debug.collect::<String>();
        assert_eq!(s, "/crab 🦀 for Rust/mix");
    }

    #[test]
    fn iter_ascii_escaped_byte_pattern_literal_ascii_control() {
        // ```ruby
        // [3.1.2] > Regexp.compile((0..0x1F).to_a.map(&:chr).join).inspect.bytes
        // ```
        let pattern = (0x00..=0x1F).collect::<Vec<u8>>();
        let debug = Debug::new(&pattern, "", "");
        let s = debug.collect::<String>();
        assert_eq!(
            s.as_bytes().as_bstr(),
            B(&[
                47, 92, 120, 48, 48, 92, 120, 48, 49, 92, 120, 48, 50, 92, 120, 48, 51, 92, 120, 48, 52, 92, 120, 48,
                53, 92, 120, 48, 54, 92, 120, 48, 55, 92, 120, 48, 56, 9, 10, 11, 12, 13, 92, 120, 48, 69, 92, 120,
                48, 70, 92, 120, 49, 48, 92, 120, 49, 49, 92, 120, 49, 50, 92, 120, 49, 51, 92, 120, 49, 52, 92, 120,
                49, 53, 92, 120, 49, 54, 92, 120, 49, 55, 92, 120, 49, 56, 92, 120, 49, 57, 92, 120, 49, 65, 92, 120,
                49, 66, 92, 120, 49, 67, 92, 120, 49, 68, 92, 120, 49, 69, 92, 120, 49, 70, 47_u8
            ])
            .as_bstr(),
        );
    }
    #[test]
    fn iter_ascii_pattern_exhaustive() {
        // ```ruby
        // Regexp.compile((0..0x7F).to_a.reject {|b| "[](){}".include?(b.chr) }.map(&:chr).join).inspect.bytes
        // ```
        let pattern = (0x00..=0x7F).filter(|b| !b"[](){}".contains(b)).collect::<Vec<u8>>();
        let debug = Debug::new(&pattern, "", "");
        let s = debug.collect::<String>();
        assert_eq!(
            s.as_bytes().as_bstr(),
            B(&[
                47, 92, 120, 48, 48, 92, 120, 48, 49, 92, 120, 48, 50, 92, 120, 48, 51, 92, 120, 48, 52, 92, 120, 48,
                53, 92, 120, 48, 54, 92, 120, 48, 55, 92, 120, 48, 56, 9, 10, 11, 12, 13, 92, 120, 48, 69, 92, 120,
                48, 70, 92, 120, 49, 48, 92, 120, 49, 49, 92, 120, 49, 50, 92, 120, 49, 51, 92, 120, 49, 52, 92, 120,
                49, 53, 92, 120, 49, 54, 92, 120, 49, 55, 92, 120, 49, 56, 92, 120, 49, 57, 92, 120, 49, 65, 92, 120,
                49, 66, 92, 120, 49, 67, 92, 120, 49, 68, 92, 120, 49, 69, 92, 120, 49, 70, 32, 33, 34, 35, 36, 37,
                38, 39, 42, 43, 44, 45, 46, 92, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
                64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
                89, 90, 92, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
                113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 124, 126, 92, 120, 55, 70, 47_u8
            ])
            .as_bstr(),
        );
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
        let debug = Debug::new(&pattern, "", "");
        let s = debug.collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/i
        // => /\t\v\f\n/i
        // ```
        let debug = Debug::new(br"\t\v\f\n", "i", "");
        let s = debug.collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/i");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/mix
        // => /\t\v\f\n/mix
        // ```
        let debug = Debug::new(br"\t\v\f\n", "mix", "");
        let s = debug.collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/mix");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/n
        // => /\t\v\f\n/n
        // ```
        let debug = Debug::new(br"\t\v\f\n", "", "n");
        let s = debug.collect::<String>();
        assert_eq!(s, r"/\t\v\f\n/n");

        // ```ruby
        // [2.6.6] > /\t\v\f\n/nix
        // => /\t\v\f\n/ixn
        // ```
        let debug = Debug::new(br"\t\v\f\n", "ix", "n");
        let s = debug.collect::<String>();
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
        let debug = Debug::new(&pattern, "", "");
        let s = debug.collect::<String>();
        assert_eq!(s, "/\t\"/");
    }

    #[test]
    fn iter_invalid_utf8_pattern() {
        // ```ruby
        // [2.6.6] > Regexp.compile("\xFF\xFE".force_encoding(Encoding::BINARY))
        // => /\xFF\xFE/
        // ```
        let debug = Debug::new(b"\xFF\xFE", "", "");
        let s = debug.collect::<String>();
        assert_eq!(s, r"/\xFF\xFE/");
    }
}
