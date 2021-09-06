#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::option_if_let_else)]
#![cfg_attr(test, allow(clippy::non_ascii_literal))]
#![allow(unknown_lints)]
// TODO: warn on missing docs once crate is API-complete.
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

// Ensure code blocks in README.md compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

#[macro_use]
extern crate bitflags;

use bstr::ByteSlice;
use core::fmt;
use core::num::NonZeroUsize;
use std::borrow::Cow;

mod debug;
mod encoding;
mod error;
mod options;
mod regexp;

pub use debug::Debug;
pub use encoding::{Encoding, InvalidEncodingError};
pub use error::{ArgumentError, Error, RegexpError, SyntaxError};
pub use options::{Options, RegexpOption};

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const IGNORECASE      = 0b0000_0001;
        const EXTENDED        = 0b0000_0010;
        const MULTILINE       = 0b0000_0100;
        const ALL_REGEXP_OPTS = Self::IGNORECASE.bits | Self::EXTENDED.bits | Self::MULTILINE.bits;

        const FIXEDENCODING   = 0b0001_0000;
        const NOENCODING      = 0b0010_0000;

        const LITERAL         = 0b1000_0000;
    }
}

/// The string matched by the last successful match.
pub const LAST_MATCHED_STRING: &[u8] = b"$&";

/// The string to the left of the last successful match.
pub const STRING_LEFT_OF_MATCH: &[u8] = b"$`";

/// The string to the right of the last successful match.
pub const STRING_RIGHT_OF_MATCH: &[u8] = b"$'";

/// The highest group matched by the last successful match.
// TODO: implement this.
pub const HIGHEST_MATCH_GROUP: &[u8] = b"$+";

/// The information about the last match in the current scope.
pub const LAST_MATCH: &[u8] = b"$~";

#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Source {
    pattern: Vec<u8>,
    options: Options,
}

impl fmt::Debug for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Source")
            .field("pattern", &self.pattern.as_bstr())
            .field("options", &self.options)
            .finish()
    }
}

impl From<Config> for Source {
    fn from(config: Config) -> Self {
        Self::with_pattern_and_options(config.pattern.clone(), config.options)
    }
}

impl From<&Config> for Source {
    fn from(config: &Config) -> Self {
        Self::with_pattern_and_options(config.pattern.clone(), config.options)
    }
}

impl Source {
    /// Construct a new, empty `Source`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pattern: Vec::new(),
            options: Options::new(),
        }
    }

    /// Construct a new `Source` with the given pattern and [`Options`].
    #[must_use]
    pub const fn with_pattern_and_options(pattern: Vec<u8>, options: Options) -> Self {
        Self { pattern, options }
    }

    /// Whether this source was parsed with ignore case enabled.
    #[must_use]
    pub const fn is_casefold(&self) -> bool {
        self.options.ignore_case().is_enabled()
    }

    /// Whether the Regexp was parsed as a literal, e.g. `'/artichoke/i`.
    ///
    /// This enables Ruby parsers to inject whether a Regexp is a literal to the
    /// core library. Literal Regexps have some special behavior regrding
    /// capturing groups and report parse failures differently.
    #[must_use]
    pub const fn is_literal(&self) -> bool {
        self.options.is_literal()
    }

    /// Extracts a slice containing the entire pattern.
    #[must_use]
    pub fn pattern(&self) -> &[u8] {
        self.pattern.as_slice()
    }

    /// Return a copy of the underlying [`Options`].
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config {
    pattern: Vec<u8>,
    options: Options,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Source")
            .field("pattern", &self.pattern.as_bstr())
            .field("options", &self.options)
            .finish()
    }
}

impl From<Source> for Config {
    fn from(source: Source) -> Self {
        Self::with_pattern_and_options(source.pattern.clone(), source.options)
    }
}

impl From<&Source> for Config {
    fn from(source: &Source) -> Self {
        Self::with_pattern_and_options(source.pattern.clone(), source.options)
    }
}

impl Config {
    /// Construct a new, empty `Config`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pattern: Vec::new(),
            options: Options::new(),
        }
    }

    /// Construct a new `Config` with the given pattern and [`Options`].
    #[must_use]
    pub const fn with_pattern_and_options(pattern: Vec<u8>, options: Options) -> Self {
        Self { pattern, options }
    }

    /// Extracts a slice containing the entire pattern.
    #[must_use]
    pub fn pattern(&self) -> &[u8] {
        self.pattern.as_slice()
    }

    /// Return a copy of the underlying [`Options`].
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

/// Global variable name for the nth capture group from a `Regexp` match.
#[inline]
#[must_use]
pub fn nth_match_group(group: NonZeroUsize) -> Cow<'static, [u8]> {
    match group.get() {
        1 => Cow::Borrowed(b"$1"),
        2 => Cow::Borrowed(b"$2"),
        3 => Cow::Borrowed(b"$3"),
        4 => Cow::Borrowed(b"$4"),
        5 => Cow::Borrowed(b"$5"),
        6 => Cow::Borrowed(b"$6"),
        7 => Cow::Borrowed(b"$7"),
        8 => Cow::Borrowed(b"$8"),
        9 => Cow::Borrowed(b"$9"),
        10 => Cow::Borrowed(b"$10"),
        11 => Cow::Borrowed(b"$11"),
        12 => Cow::Borrowed(b"$12"),
        13 => Cow::Borrowed(b"$13"),
        14 => Cow::Borrowed(b"$14"),
        15 => Cow::Borrowed(b"$15"),
        16 => Cow::Borrowed(b"$16"),
        17 => Cow::Borrowed(b"$17"),
        18 => Cow::Borrowed(b"$18"),
        19 => Cow::Borrowed(b"$19"),
        20 => Cow::Borrowed(b"$20"),
        num => {
            let mut buf = String::from("$");
            // Suppress fmt errors because this function is infallible.
            //
            // In practice `itoa::fmt` will never error because the `fmt::Write`
            // impl for `String` never panics.
            let _ = itoa::fmt(&mut buf, num);
            Cow::Owned(buf.into_bytes())
        }
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;
    use std::borrow::Cow;

    use super::nth_match_group;

    #[test]
    fn match_group_symbol() {
        for num in 1..=1024 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym = nth_match_group(num);
            let num = format!("{}", num);
            assert!(sym.len() > 1);
            assert_eq!(sym[0..1], *b"$");
            assert_eq!(sym[1..], *num.as_bytes());
        }
    }

    #[test]
    fn some_globals_are_static_slices() {
        for num in 1..=20 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym = nth_match_group(num);
            assert!(matches!(sym, Cow::Borrowed(_)));
        }
        for num in 21..=1024 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym = nth_match_group(num);
            assert!(matches!(sym, Cow::Owned(_)));
        }
    }
}
