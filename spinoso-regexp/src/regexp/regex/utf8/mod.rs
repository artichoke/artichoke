use core::fmt;
use core::str;
use std::collections::HashSet;

use bstr::{ByteSlice, ByteVec};
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

    pub fn captures<'a>(&self, haystack: &'a [u8]) -> Result<Option<Captures<'a>>, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        Ok(self.regex.captures(haystack).map(Captures::from))
    }

    pub fn capture_indices_for_name<'a, 'b>(&'a self, name: &'b [u8]) -> CaptureIndices<'a, 'b> {
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
    pub fn named_captures(&self) -> Result<NamedCaptures, Error> {
        // Use a Vec of key-value pairs because insertion order matters for spec
        // compliance.
        let mut map = vec![];
        for group in self.regex.capture_names().flatten() {
            let indices = self.capture_indices_for_name(group.as_bytes()).collect::<Vec<_>>();
            if !indices.is_empty() {
                map.push(NamedCapture::new(group.into(), indices));
            }
        }
        Ok(map.into())
    }

    pub fn named_captures_for_haystack(&self, haystack: &[u8]) -> Result<Option<NamedCapturesForHaystack>, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        let captures = if let Some(captures) = self.regex.captures(haystack) {
            captures
        } else {
            return Ok(None);
        };
        let mut map = NamedCapturesForHaystack::with_capacity(captures.len());
        for named_capture in self.named_captures()? {
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

    pub fn names(&self) -> Vec<Vec<u8>> {
        let mut names = vec![];
        let mut capture_names = self.named_captures().unwrap_or_default().collect::<Vec<_>>();
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

    pub fn pos(&self, haystack: &[u8], at: usize) -> Result<Option<(usize, usize)>, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        let pos = self
            .regex
            .captures(haystack)
            .and_then(|captures| captures.get(at))
            .map(|match_pos| (match_pos.start(), match_pos.end()));
        Ok(pos)
    }

    // Check whether this regexp matches the given haystack starting at an offset.
    //
    // If the given offset is negative, it counts backward from the end of the
    // haystack.
    pub fn is_match(&self, haystack: &[u8], pos: Option<i64>) -> Result<bool, Error> {
        let haystack = str::from_utf8(haystack).map_err(|_| ArgumentError::unsupported_haystack_encoding())?;
        let haystack_char_len = haystack.chars().count();
        let pos = pos.unwrap_or_default();
        let pos = if let Ok(pos) = usize::try_from(pos) {
            pos
        } else {
            let pos = pos
                .checked_neg()
                .and_then(|pos| usize::try_from(pos).ok())
                .and_then(|pos| haystack_char_len.checked_sub(pos));
            if let Some(pos) = pos {
                pos
            } else {
                return Ok(false);
            }
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

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub fn inspect(&self) -> Vec<u8> {
        // pattern length + 2x '/' + mix + encoding
        let mut inspect = Vec::with_capacity(self.source.pattern.len() + 2 + 4);
        inspect.push_byte(b'/');
        if self.source.pattern.contains_str("/") {
            let mut escaped = self.source.pattern.replace("/", r"\/");
            inspect.append(&mut escaped);
        } else {
            inspect.extend_from_slice(self.source.pattern());
        }
        inspect.push_byte(b'/');
        inspect.push_str(self.source.options.as_display_modifier());
        inspect.push_str(self.encoding.as_modifier_str());
        inspect
    }

    pub fn string(&self) -> &[u8] {
        self.config.pattern()
    }
}
