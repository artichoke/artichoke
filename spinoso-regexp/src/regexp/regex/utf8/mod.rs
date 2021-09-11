use core::fmt;
use core::str;

use bstr::{ByteSlice, ByteVec};
use regex::{Match, Regex, RegexBuilder};
use scolapasta_string_escape::format_debug_escape_into;

use crate::{ArgumentError, Config, Debug, Encoding, Error, RegexpError, SyntaxError};

mod iter;

pub use iter::{CaptureIndices, Captures};

#[derive(Debug, Clone)]
pub struct Utf8 {
    literal: Config,
    derived: Config,
    encoding: Encoding,
    regex: Regex,
}

impl fmt::Display for Utf8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pattern = self.derived.pattern.as_slice();
        format_debug_escape_into(f, pattern)?;
        Ok(())
    }
}

impl Utf8 {
    pub fn with_literal_derived_encoding(literal: Config, derived: Config, encoding: Encoding) -> Result<Self, Error> {
        let pattern = str::from_utf8(derived.pattern.as_slice()).map_err(|_| {
            ArgumentError::with_message("regex crate utf8 backend for Regexp only supports UTF-8 patterns")
        })?;
        let mut builder = RegexBuilder::new(pattern);
        builder.case_insensitive(derived.options.ignore_case().is_enabled());
        builder.multi_line(derived.options.multiline().is_enabled());
        builder.ignore_whitespace(derived.options.extended().is_enabled());

        let regex = match builder.build() {
            Ok(regex) => regex,
            Err(err) if literal.options.is_literal() => {
                return Err(SyntaxError::from(err.to_string()).into());
            }
            Err(err) => return Err(RegexpError::from(err.to_string()).into()),
        };
        let regexp = Self {
            literal,
            derived,
            encoding,
            regex,
        };
        Ok(regexp)
    }

    pub fn captures<'a>(&self, haystack: &'a [u8]) -> Result<Option<Captures<'a>>, Error> {
        let haystack =
            str::from_utf8(haystack).map_err(|_| ArgumentError::with_message("invalid byte sequence in UTF-8"))?;
        Ok(self.regex.captures(haystack).map(Captures::from))
    }

    pub fn capture_indexes_for_name<'a, 'b>(&'a self, name: &'b [u8]) -> CaptureIndices<'a, 'b> {
        CaptureIndices::with_name_and_iter(name, self.regex.capture_names())
    }

    /// Returns the number of captures.
    pub fn captures_len(&self) -> usize {
        self.regex.captures_len()
    }

    /// The number of captures for a match of `haystack` against this regexp.
    ///
    /// Captures represents a group of captured strings for a single match.
    ///
    /// If there is a match, the returned value is always greater than 0; the
    /// 0th capture always corresponds to the entire match.
    pub fn capture_count_for_haystack(&self, haystack: &[u8]) -> Result<usize, ArgumentError> {
        let haystack =
            str::from_utf8(haystack).map_err(|_| ArgumentError::with_message("invalid byte sequence in UTF-8"))?;
        if let Some(captures) = self.regex.captures(haystack) {
            Ok(captures.len())
        } else {
            Ok(0)
        }
    }

    /// Return the 0th capture group if `haystack` is matched by this regexp.
    ///
    /// The 0th capture always corresponds to the entire match.
    pub fn entire_match<'a>(&self, haystack: &'a [u8]) -> Result<Option<&'a [u8]>, Error> {
        let haystack =
            str::from_utf8(haystack).map_err(|_| ArgumentError::with_message("invalid byte sequence in UTF-8"))?;
        if let Some(captures) = self.regex.captures(haystack) {
            let entire_match = captures.get(0);
            Ok(entire_match.as_ref().map(Match::as_str).map(str::as_bytes))
        } else {
            Ok(None)
        }
    }

    pub fn debug(&self) -> Debug<'_> {
        Debug::new(
            self.literal.pattern(),
            self.literal.options.as_display_modifier(),
            self.encoding.as_modifier_string(),
        )
    }

    pub fn literal_config(&self) -> &Config {
        &self.literal
    }

    pub fn derived_config(&self) -> &Config {
        &self.derived
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub fn inspect(&self) -> Vec<u8> {
        // pattern length + 2x '/' + mix + encoding
        let mut inspect = Vec::with_capacity(self.literal.pattern.len() + 2 + 4);
        inspect.push_byte(b'/');
        if self.literal.pattern.contains_str("/") {
            let mut escaped = self.literal.pattern.replace("/", r"\/");
            inspect.append(&mut escaped);
        } else {
            inspect.extend_from_slice(&self.literal.pattern);
        }
        inspect.push_byte(b'/');
        inspect.push_str(self.literal.options.as_display_modifier());
        inspect.push_str(self.encoding.as_modifier_string());
        inspect
    }

    pub fn string(&self) -> &[u8] {
        &self.derived.pattern
    }
}
