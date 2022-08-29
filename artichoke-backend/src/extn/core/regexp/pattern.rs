//! Regexp pattern parsers.

use core::iter;

use bstr::ByteSlice;

use super::{Flags, Options, RegexpOption};

/// A Regexp pattern including its derived `Options`.
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pattern {
    pattern: Vec<u8>,
    options: Options,
}

impl Pattern {
    /// Consume self and return the inner pattern byte vector.
    #[must_use]
    pub fn into_pattern(self) -> Vec<u8> {
        self.pattern
    }

    /// Return the `Options` parsed when constructing this `Pattern`.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

#[inline]
#[must_use]
fn build_pattern<T>(pattern: T, options: Options) -> Pattern
where
    T: IntoIterator<Item = u8>,
{
    let iter = pattern.into_iter();
    let hint = iter.size_hint();
    let modifiers = options.as_inline_modifier();
    let mut parsed = Vec::with_capacity(2 + modifiers.len() + 2 + hint.1.unwrap_or(hint.0));
    parsed.extend_from_slice(b"(?");
    parsed.extend_from_slice(modifiers.as_bytes());
    parsed.push(b':');
    parsed.extend(iter);
    parsed.push(b')');
    Pattern {
        pattern: parsed,
        options,
    }
}

#[must_use]
pub fn parse<T: AsRef<[u8]>>(pattern: T, options: Options) -> Pattern {
    let pattern = pattern.as_ref();
    let mut chars = pattern.bytes().enumerate();

    match chars.next() {
        Some((_, b'(')) => {}
        Some(_) => return build_pattern(pattern.bytes(), options),
        None => return build_pattern(iter::empty(), options),
    }
    match chars.next() {
        Some((_, b'?')) => {}
        Some(_) => return build_pattern(pattern.bytes(), options),
        None => return build_pattern(iter::once(b'('), options),
    }

    let orignal_options = options;
    let mut options = options;
    let mut enable_literal_option = RegexpOption::Enabled;

    for (_, token) in &mut chars {
        match token {
            b'-' => enable_literal_option = RegexpOption::Disabled,
            b'i' => {
                options.set(Flags::IGNORECASE, enable_literal_option.into());
            }
            b'm' => {
                options.set(Flags::MULTILINE, enable_literal_option.into());
            }
            b'x' => {
                options.set(Flags::EXTENDED, enable_literal_option.into());
            }
            b':' => break,
            _ => return build_pattern(pattern.bytes(), options),
        }
    }

    let mut chars = chars.peekable();
    let cursor = if let Some((idx, _)) = chars.peek() {
        *idx
    } else {
        pattern.len()
    };

    let mut nest = 1;
    while let Some((_, token)) = chars.next() {
        if token == b'(' {
            nest += 1;
        } else if token == b')' {
            nest -= 1;
            if nest == 0 && chars.next().is_some() {
                return build_pattern(pattern.bytes(), orignal_options);
            }
            break;
        }
    }

    let slice = pattern.get(cursor..).unwrap_or_default();
    let modifiers = options.as_inline_modifier();
    let mut parsed = Vec::with_capacity(2 + modifiers.len() + 1 + slice.len());
    parsed.extend(b"(?");
    parsed.extend(modifiers.as_bytes());
    parsed.push(b':');
    parsed.extend_from_slice(slice);
    Pattern {
        pattern: parsed,
        options,
    }
}

#[cfg(test)]
mod tests {
    use bstr::BString;

    use crate::extn::core::regexp::{Flags, Options};

    #[test]
    fn parse_literal_string_pattern() {
        let opts = Options::new();
        let parsed = super::parse("foo", opts);
        assert_eq!(BString::from("(?-mix:foo)"), BString::from(parsed.into_pattern()));
    }

    // The below tests are extracted from `Regexp#to_s` ruby/specs.

    #[test]
    fn parse_options_if_included_and_expand() {
        let opts = Options::from(Flags::ALL_REGEXP_OPTS);
        let parsed = super::parse("abc", opts);
        assert_eq!(BString::from("(?mix:abc)"), BString::from(parsed.into_pattern()),);
    }

    #[test]
    fn parse_non_included_options_and_embed_expanded_modifiers_prefixed_by_a_minus_sign() {
        let opts = Options::from(Flags::IGNORECASE);
        let parsed = super::parse("abc", opts);
        assert_eq!(BString::from("(?i-mx:abc)"), BString::from(parsed.into_pattern()),);
    }

    #[test]
    fn parse_patterns_with_no_enabled_options_and_expand_with_all_modifiers_excluded() {
        let opts = Options::new();
        let parsed = super::parse("abc", opts);
        assert_eq!(BString::from("(?-mix:abc)"), BString::from(parsed.into_pattern()),);
    }

    #[test]
    fn embeds_the_pattern_after_the_options_after_parsing() {
        let opts = Options::from(Flags::ALL_REGEXP_OPTS);
        let parsed = super::parse("ab+c", opts);
        assert_eq!(BString::from("(?mix:ab+c)"), BString::from(parsed.into_pattern()),);
        let opts = Options::new();
        let parsed = super::parse("xyz", opts);
        assert_eq!(BString::from("(?-mix:xyz)"), BString::from(parsed.into_pattern()),);
    }

    #[test]
    fn parse_groups_with_options() {
        let opts = Options::new();
        let parsed = super::parse("(?ix:foo)(?m:bar)", opts);
        assert_eq!(
            BString::from("(?-mix:(?ix:foo)(?m:bar))"),
            BString::from(parsed.into_pattern()),
        );
        let opts = Options::from(Flags::MULTILINE);
        let parsed = super::parse("(?ix:foo)bar", opts);
        assert_eq!(
            BString::from("(?m-ix:(?ix:foo)bar)"),
            BString::from(parsed.into_pattern()),
        );
    }

    #[test]
    fn parse_a_single_group_with_options_as_the_main_regexp() {
        let opts = Options::new();
        let parsed = super::parse("(?i:nothing outside this group)", opts);
        assert_eq!(
            BString::from("(?i-mx:nothing outside this group)"),
            BString::from(parsed.into_pattern())
        );
    }

    #[test]
    fn parse_uncaptured_groups() {
        let opts = Options::from(Flags::IGNORECASE | Flags::EXTENDED);
        let parsed = super::parse("whatever(?:0d)", opts);
        assert_eq!(
            BString::from("(?ix-m:whatever(?:0d))"),
            BString::from(parsed.into_pattern()),
        );
    }

    #[test]
    fn parse_lookahead_groups() {
        let opts = Options::new();
        let parsed = super::parse("(?=5)", opts);
        assert_eq!(BString::from("(?-mix:(?=5))"), BString::from(parsed.into_pattern()));
        let opts = Options::new();
        let parsed = super::parse("(?!5)", opts);
        assert_eq!(BString::from("(?-mix:(?!5))"), BString::from(parsed.into_pattern()));
    }

    #[test]
    fn parse_to_fully_expanded_options_inline() {
        let opts = Options::from(Flags::IGNORECASE | Flags::EXTENDED);
        let parsed = super::parse("ab+c", opts);
        assert_eq!(BString::from("(?ix-m:ab+c)"), BString::from(parsed.into_pattern()),);
        let opts = Options::new();
        let parsed = super::parse("(?i:.)", opts);
        assert_eq!(BString::from("(?i-mx:.)"), BString::from(parsed.into_pattern()),);
        let opts = Options::new();
        let parsed = super::parse("(?:.)", opts);
        assert_eq!(BString::from("(?-mix:.)"), BString::from(parsed.into_pattern()),);
    }

    #[test]
    fn parse_abusive_options_literals() {
        let opts = Options::new();
        let parsed = super::parse("(?mmmmix-miiiix:)", opts);
        assert_eq!(BString::from("(?-mix:)"), BString::from(parsed.into_pattern()),);
    }
}
