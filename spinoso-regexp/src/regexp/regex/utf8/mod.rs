use core::fmt;
use core::str;
use std::collections::HashSet;

use regex::{Match, Regex, RegexBuilder};
use scolapasta_string_escape::format_debug_escape_into;

use crate::debug::Debug;
use crate::encoding::Encoding;
use crate::error::{ArgumentError, Error, RegexpError, SyntaxError};
use crate::named_captures::{NamedCapture, NamedCaptures, NamedCapturesForHaystack};
use crate::{Config, Source};

mod iter;

pub use iter::{CaptureIndices, Captures};

#[derive(Debug, Clone)]
pub struct Utf8 {
    source: Source,
    config: Config,
    encoding: Encoding,
    regex: Regex,
}

impl fmt::Display for Utf8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pattern = self.config.pattern();
        format_debug_escape_into(f, pattern)?;
        Ok(())
    }
}

impl Utf8 {
    /// Construct a Regexp with a UTF-8 [`regex`] backend.
    ///
    /// The constructed regexp is Unicode aware. All character classes used in
    /// patterns other than POSIX character classes support all of Unicode.
    ///
    /// `Utf8` regexps require their patterns and haystacks to be valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_regexp::{Config, Encoding, Error, Options, Source, Utf8};
    /// # fn example() -> Result<(), Error> {
    /// let pattern = br"[[:alpha:]]\d+ \d+";
    /// let source = Source::with_pattern_and_options(pattern.to_vec(), Options::default());
    /// let config = Config::from(&source);
    /// let regexp = Utf8::with_literal_derived_encoding(source, config, Encoding::None)?;
    /// assert!(regexp.is_match("a123 १०೩೬".as_bytes(), None)?);
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// # Errors
    ///
    /// If the pattern in the given source is not valid UTF-8, an
    /// [`ArgumentError`] is returned. If the given source pattern fails to
    /// parse, either a [`SyntaxError`] or [`RegexpError`] is returned depending
    /// on the source [`Options`].
    ///
    /// [`regex`]: regex::Regex
    /// [`Options`]: crate::Options
    pub fn with_literal_derived_encoding(source: Source, config: Config, encoding: Encoding) -> Result<Self, Error> {
        let pattern = str::from_utf8(config.pattern()).map_err(|_| ArgumentError::unsupported_pattern_encoding())?;
        let mut builder = RegexBuilder::new(pattern);
        builder.case_insensitive(config.options.ignore_case().is_enabled());
        builder.multi_line(config.options.multiline().is_enabled());
        builder.ignore_whitespace(config.options.extended().is_enabled());

        let regex = match builder.build() {
            Ok(regex) => regex,
            Err(err) if source.options.is_literal() => {
                return Err(SyntaxError::from(err.to_string()).into());
            }
            Err(err) => return Err(RegexpError::from(err.to_string()).into()),
        };
        let regexp = Self {
            source,
            config,
            encoding,
            regex,
        };
        Ok(regexp)
    }

    /// # Errors
    ///
    /// If the given haystack is not valid UTF-8, an error is returned.
    pub fn captures<'a>(&self, haystack: &'a [u8]) -> Result<Option<Captures<'a>>, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        Ok(self.regex.captures(haystack).map(Captures::from))
    }

    pub fn capture_indices_for_name<'a, 'b>(&'a self, name: &'b [u8]) -> CaptureIndices<'a, 'b> {
        CaptureIndices::with_name_and_iter(name, self.regex.capture_names())
    }

    /// Returns the number of captures.
    #[must_use]
    pub fn captures_len(&self) -> usize {
        self.regex.captures_len()
    }

    /// The number of captures for a match of `haystack` against this regexp.
    ///
    /// Captures represents a group of captured strings for a single match.
    ///
    /// If there is a match, the returned value is always greater than 0; the
    /// 0th capture always corresponds to the entire match.
    ///
    /// # Errors
    ///
    /// If the given haystack is not valid UTF-8, an error is returned.
    pub fn capture_count_for_haystack(&self, haystack: &[u8]) -> Result<usize, ArgumentError> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        if let Some(captures) = self.regex.captures(haystack) {
            Ok(captures.len())
        } else {
            Ok(0)
        }
    }

    /// Return the 0th capture group if `haystack` is matched by this regexp.
    ///
    /// The 0th capture always corresponds to the entire match.
    ///
    /// # Errors
    ///
    /// If the given haystack is not valid UTF-8, an error is returned.
    pub fn entire_match<'a>(&self, haystack: &'a [u8]) -> Result<Option<&'a [u8]>, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        if let Some(captures) = self.regex.captures(haystack) {
            let entire_match = captures.get(0);
            Ok(entire_match.as_ref().map(Match::as_str).map(str::as_bytes))
        } else {
            Ok(None)
        }
    }

    /// Returns a hash representing information about the named captures of this
    /// `Regexp`.
    ///
    /// A key of the hash is a name of the named captures. A value of the hash
    /// is an array which is list of indexes of corresponding named captures.
    pub fn named_captures(&self) -> NamedCaptures {
        // Use a Vec of key-value pairs because insertion order matters for spec
        // compliance.
        let mut map = vec![];
        for group in self.regex.capture_names().flatten() {
            let indices = self.capture_indices_for_name(group.as_bytes()).collect::<Vec<_>>();
            if !indices.is_empty() {
                map.push(NamedCapture::new(group.into(), indices));
            }
        }
        map.into()
    }

    /// # Errors
    ///
    /// If the given haystack is not valid UTF-8, an error is returned.
    pub fn named_captures_for_haystack(&self, haystack: &[u8]) -> Result<Option<NamedCapturesForHaystack>, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        let captures = if let Some(captures) = self.regex.captures(haystack) {
            captures
        } else {
            return Ok(None);
        };
        let mut map = NamedCapturesForHaystack::with_capacity(captures.len());
        for named_capture in self.named_captures() {
            let (group, indices) = named_capture.into_group_and_indices();
            let capture = indices.iter().rev().copied().find_map(|index| captures.get(index));
            if let Some(capture) = capture {
                map.insert(group, Some(capture.as_str().into()));
            } else {
                map.insert(group, None);
            }
        }
        Ok(Some(map))
    }

    #[must_use]
    pub fn names(&self) -> Vec<Vec<u8>> {
        let mut names = vec![];
        let mut capture_names = self.named_captures().collect::<Vec<_>>();
        capture_names.sort_by(|left, right| {
            let left = left.indices().iter().min().copied().unwrap_or(usize::MAX);
            let right = right.indices().iter().min().copied().unwrap_or(usize::MAX);
            left.cmp(&right)
        });
        let mut set = HashSet::with_capacity(capture_names.len());
        for cn in capture_names {
            let name = cn.into_group();
            if set.contains(&name) {
                continue;
            }
            names.push(name.clone());
            set.insert(name);
        }
        names
    }

    /// # Errors
    ///
    /// If the given haystack is not valid UTF-8, an error is returned.
    pub fn pos(&self, haystack: &[u8], at: usize) -> Result<Option<(usize, usize)>, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        let pos = self
            .regex
            .captures(haystack)
            .and_then(|captures| captures.get(at))
            .map(|match_pos| (match_pos.start(), match_pos.end()));
        Ok(pos)
    }

    /// Check whether this regexp matches the given haystack starting at an offset.
    ///
    /// If the given offset is negative, it counts backward from the end of the
    /// haystack.
    ///
    /// # Errors
    ///
    /// If the given haystack is not valid UTF-8, an error is returned.
    pub fn is_match(&self, haystack: &[u8], pos: Option<i64>) -> Result<bool, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        let haystack_char_len = haystack.chars().count();
        let pos = pos.unwrap_or_default();
        let pos = if let Some(pos) = scolapasta_aref::offset_to_index(pos, haystack_char_len) {
            pos
        } else {
            return Ok(false);
        };
        let offset = haystack.chars().take(pos).map(char::len_utf8).sum();
        let haystack = &haystack[offset..];
        Ok(self.regex.find(haystack).is_some())
    }

    pub fn debug(&self) -> Debug<'_> {
        Debug::new(
            self.source.pattern(),
            self.source.options.as_display_modifier(),
            self.encoding.as_modifier_str(),
        )
    }

    #[must_use]
    pub fn is_literal(&self) -> bool {
        self.source.options().is_literal()
    }

    #[must_use]
    pub fn source(&self) -> &Source {
        &self.source
    }

    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    #[must_use]
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    #[must_use]
    pub fn string(&self) -> &[u8] {
        self.config.pattern()
    }
}

#[cfg(test)]
mod tests {
    use bstr::{ByteSlice, B};

    use super::Utf8;
    use crate::{Config, Encoding, Error, Flags, Options, Source};

    fn make(pattern: impl AsRef<[u8]>, options: Option<Options>, encoding: Encoding) -> Utf8 {
        let source = Source::with_pattern_and_options(pattern.as_ref().to_vec(), options.unwrap_or_default());
        let config = Config::from(&source);
        Utf8::with_literal_derived_encoding(source, config, encoding).unwrap()
    }

    #[test]
    fn can_compile_posix_character_classes() {
        let regexp = make("[[:digit:]][[:space:]][[:alpha:]][[:punct:]]", None, Encoding::None);
        assert!(regexp.is_match(b"1 a&", None).unwrap());
    }

    #[test]
    fn can_compile_perl_unicode_patterns() {
        let regexp = make(r"\d+ \d+", None, Encoding::None);
        // This haystack contains non-ASCII numerals in the Unicode Nd character
        // class. The sequence contains Devanagari 1, Devanagari 0, Kannada 3,
        // and Kannada 6.
        //
        // See:
        //
        // - https://en.wikipedia.org/wiki/Devanagari_numerals#Table
        // - https://en.wikipedia.org/wiki/Kannada_script#Numerals
        let haystack = "123 १०೩೬";
        assert!(regexp.is_match(haystack.as_bytes(), None).unwrap());
    }

    #[test]
    fn requires_utf8_encoding_for_pattern() {
        let source = Source::with_pattern_and_options(b"abc \xFF\xFE 123".to_vec(), Options::default());
        let config = Config::from(&source);
        let err = Utf8::with_literal_derived_encoding(source, config, Encoding::None).unwrap_err();
        assert!(matches!(err, Error::Argument(err) if err.message() == "Unsupported pattern encoding"));
    }

    #[test]
    fn invalid_pattern_is_syntax_error_for_literal() {
        let options = Options::from(Flags::LITERAL);
        let source = Source::with_pattern_and_options(b"[".to_vec(), options);
        let config = Config::from(&source);
        let err = Utf8::with_literal_derived_encoding(source, config, Encoding::None).unwrap_err();
        assert!(matches!(err, Error::Syntax(..)));
    }

    #[test]
    fn invalid_pattern_is_syntax_error_for_compiled() {
        let options = Options::from(Flags::ALL_REGEXP_OPTS);
        let source = Source::with_pattern_and_options(b"[".to_vec(), options);
        let config = Config::from(&source);
        let err = Utf8::with_literal_derived_encoding(source, config, Encoding::None).unwrap_err();
        assert!(matches!(err, Error::Regexp(..)));
    }

    #[test]
    fn literal_pattern_backrefs_are_not_supported() {
        let options = Options::from(Flags::LITERAL);
        let source = Source::with_pattern_and_options(br"\0".to_vec(), options);
        let config = Config::from(&source);
        let err = Utf8::with_literal_derived_encoding(source, config, Encoding::None).unwrap_err();
        assert!(matches!(err, Error::Syntax(err) if err.message().contains("backreferences are not supported")));
    }

    #[test]
    fn compiled_pattern_backrefs_are_not_supported() {
        let options = Options::from(Flags::ALL_REGEXP_OPTS);
        let source = Source::with_pattern_and_options(br"\0".to_vec(), options);
        let config = Config::from(&source);
        let err = Utf8::with_literal_derived_encoding(source, config, Encoding::None).unwrap_err();
        assert!(matches!(err, Error::Regexp(err) if err.message().contains("backreferences are not supported")));
    }

    #[test]
    fn is_literal() {
        let options = Options::from(Flags::LITERAL);
        let regexp = make("abc", Some(options), Encoding::None);
        assert!(regexp.is_literal());

        let options = Options::from(Flags::empty());
        let regexp = make("abc", Some(options), Encoding::None);
        assert!(!regexp.is_literal());

        let options = Options::from(Flags::ALL_REGEXP_OPTS);
        let regexp = make("abc", Some(options), Encoding::None);
        assert!(!regexp.is_literal());

        let regexp = make("abc", None, Encoding::None);
        assert!(!regexp.is_literal());
    }

    #[test]
    fn string() {
        let test_cases = [
            ("abc", B("abc")),
            ("xyz", B("xyz")),
            ("🦀", B("🦀")),
            ("铁锈", B("铁锈")),
        ];
        for (pattern, string) in test_cases {
            let regexp = make(pattern, None, Encoding::None);
            assert_eq!(
                regexp.string().as_bstr(),
                string.as_bstr(),
                "Mismatched string for pattern"
            );
        }
    }

    #[test]
    fn fmt_display() {
        let test_cases = [
            (B("abc"), "abc"),
            (B("xyz"), "xyz"),
            (B("🦀"), "🦀"),
            (B("铁锈"), "铁锈"),
            // Invalid UTF-8 patterns are not supported 👇
            // (B(b"\xFF\xFE"), r"\xFF\xFE"),
            // (B(b"abc \xFF\xFE xyz"), r"abc \xFF\xFE xyz"),
        ];
        for (pattern, display) in test_cases {
            let regexp = make(pattern, None, Encoding::None);
            assert_eq!(regexp.to_string(), display, "Mismatched display impl for pattern");
        }
    }

    #[test]
    fn debug() {
        let test_cases = [
            (B("\0"), r"/\x00/", Options::default()),
            (B("\0"), r"/\x00/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (B("\0"), r"/\x00/ix", Options::from(Flags::IGNORECASE | Flags::EXTENDED)),
            (B("\0"), r"/\x00/m", Options::from(Flags::MULTILINE)),
            (B(b"\x0a"), "/\n/", Options::default()),
            (B("\x0B"), "/\x0B/", Options::default()),
            // NOTE: the control characters, not a raw string, are in the debug output.
            (B("\n\r\t"), "/\n\r\t/", Options::default()),
            (B("\n\r\t"), "/\n\r\t/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (
                B("\n\r\t"),
                "/\n\r\t/ix",
                Options::from(Flags::IGNORECASE | Flags::EXTENDED),
            ),
            (B("\n\r\t"), "/\n\r\t/m", Options::from(Flags::MULTILINE)),
            (B("\x7F"), r"/\x7F/", Options::default()),
            (B("\x7F"), r"/\x7F/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (
                B("\x7F"),
                r"/\x7F/ix",
                Options::from(Flags::IGNORECASE | Flags::EXTENDED),
            ),
            (B("\x7F"), r"/\x7F/m", Options::from(Flags::MULTILINE)),
            (B(r"\a"), r"/\a/", Options::default()),
            (B(r"\a"), r"/\a/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (B(r"\a"), r"/\a/ix", Options::from(Flags::IGNORECASE | Flags::EXTENDED)),
            (B(r"\a"), r"/\a/m", Options::from(Flags::MULTILINE)),
            (B("abc"), "/abc/", Options::default()),
            (B("abc"), "/abc/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (B("abc"), "/abc/ix", Options::from(Flags::IGNORECASE | Flags::EXTENDED)),
            (B("abc"), "/abc/m", Options::from(Flags::MULTILINE)),
            (B("a+b*c"), "/a+b*c/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (B("xyz"), "/xyz/", Options::default()),
            (B("xyz"), "/xyz/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (B("xyz"), "/xyz/ix", Options::from(Flags::IGNORECASE | Flags::EXTENDED)),
            (B("xyz"), "/xyz/m", Options::from(Flags::MULTILINE)),
            (B("x+y*z"), "/x+y*z/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (B("🦀💎"), "/🦀💎/", Options::default()),
            (B("🦀💎"), "/🦀💎/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (
                B("🦀💎"),
                "/🦀💎/ix",
                Options::from(Flags::IGNORECASE | Flags::EXTENDED),
            ),
            (B("🦀💎"), "/🦀💎/m", Options::from(Flags::MULTILINE)),
            (B("🦀+💎*"), "/🦀+💎*/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (B("铁锈"), "/铁锈/", Options::default()),
            (B("铁锈"), "/铁锈/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            (
                B("铁锈"),
                "/铁锈/ix",
                Options::from(Flags::IGNORECASE | Flags::EXTENDED),
            ),
            (B("铁锈"), "/铁锈/m", Options::from(Flags::MULTILINE)),
            (B("铁+锈*"), "/铁+锈*/mix", Options::from(Flags::ALL_REGEXP_OPTS)),
            // Invalid UTF-8 patterns are not supported 👇
            // (B(b"\xFF\xFE"), r"\xFF\xFE", Options::default()),
            // (B(b"abc \xFF\xFE xyz"), r"abc \xFF\xFE xyz", Options::default()),
        ];
        for (pattern, debug, options) in test_cases {
            let regexp = make(pattern, Some(options), Encoding::None);
            assert_eq!(
                regexp.debug().collect::<String>(),
                debug,
                "Mismatched debug iterator for pattern"
            );
        }
    }
}
