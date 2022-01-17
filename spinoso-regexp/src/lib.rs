#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
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

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

use core::fmt::{self, Write as _};
use core::num::NonZeroUsize;
use std::borrow::Cow;

use bstr::ByteSlice;

mod debug;
mod encoding;
mod error;
mod options;
mod regexp;

pub use debug::Debug;
pub use encoding::{Encoding, InvalidEncodingError};
pub use error::{ArgumentError, Error, RegexpError, SyntaxError};
pub use options::{Options, RegexpOption};

bitflags::bitflags! {
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

/// A `Source` represents the literal contents used to construct a given
/// `Regexp`.
///
/// When [`Regexp`]s are constructed with a `/.../` literal, [`Regexp#source`]
/// refers to the literal characters contained within the `/` delimiters.
/// For example, `/\t/.source.bytes` has byte sequence `[92, 116]`.
///
/// When `Regexp`s are constructed with [`Regexp::compile`], [`Regexp#source`]
/// refers to the argument passed to `compile`. For example,
/// `Regexp.compile("\t").source.bytes` has byte sequence `[9]`.
///
/// [`Regexp#inspect`] prints `"/#{source}/"`.
///
/// [`Regexp`]: https://ruby-doc.org/core-2.6.3/Regexp.html
/// [`Regexp#source`]: https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-source
/// [`Regexp::compile`]: https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-compile
/// [`Regexp#inspect`]: https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-inspect
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
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::Source;
    ///
    /// const SOURCE: Source = Source::new();
    /// assert!(SOURCE.pattern().is_empty());
    /// assert!(SOURCE.options().as_display_modifier().is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pattern: Vec::new(),
            options: Options::new(),
        }
    }

    /// Construct a new `Source` with the given pattern and [`Options`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::{Options, Source};
    ///
    /// let source = Source::with_pattern_and_options(
    ///     b"Artichoke( Ruby)?".to_vec(),
    ///     Options::with_ignore_case(),
    /// );
    /// assert_eq!(source.pattern(), b"Artichoke( Ruby)?");
    /// assert_eq!(source.options().as_display_modifier(), "i");
    /// ```
    #[must_use]
    pub const fn with_pattern_and_options(pattern: Vec<u8>, options: Options) -> Self {
        Self { pattern, options }
    }

    /// Whether this source was parsed with ignore case enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::{Options, Source};
    ///
    /// let source = Source::new();
    /// assert!(!source.is_casefold());
    ///
    /// let source = Source::with_pattern_and_options(
    ///     b"Artichoke( Ruby)?".to_vec(),
    ///     Options::with_ignore_case(),
    /// );
    /// assert!(source.is_casefold());
    /// ```
    #[must_use]
    pub const fn is_casefold(&self) -> bool {
        self.options.ignore_case().is_enabled()
    }

    /// Whether the Regexp was parsed as a literal, e.g. `'/artichoke/i`.
    ///
    /// This enables Ruby parsers to inject whether a Regexp is a literal to the
    /// core library. Literal Regexps have some special behavior regarding
    /// capturing groups and report parse failures differently.
    ///
    /// A source's literal flag can only be set using [`Options::try_from_int`].
    #[must_use]
    pub const fn is_literal(&self) -> bool {
        self.options.is_literal()
    }

    /// Extracts a slice containing the entire pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::{Options, Source};
    ///
    /// let source = Source::with_pattern_and_options(
    ///     b"Artichoke( Ruby)?".to_vec(),
    ///     Options::with_ignore_case(),
    /// );
    /// assert_eq!(source.pattern(), b"Artichoke( Ruby)?");
    /// ```
    #[must_use]
    pub fn pattern(&self) -> &[u8] {
        self.pattern.as_slice()
    }

    /// Return a copy of the underlying [`Options`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::{Options, Source};
    ///
    /// let source = Source::with_pattern_and_options(
    ///     b"Artichoke( Ruby)?".to_vec(),
    ///     Options::with_ignore_case(),
    /// );
    /// assert_eq!(source.options().as_display_modifier(), "i");
    /// ```
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

/// A `Config` represents the parsed, expanded, and normalized pattern and
/// options used to initialize a `Regexp`.
///
/// A `Config` is derived from a [`Source`].
///
/// When a `Regexp` is cloned, it is cloned from its compiled `Config`.
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
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::Config;
    ///
    /// const CONFIG: Config = Config::new();
    /// assert!(CONFIG.pattern().is_empty());
    /// assert!(CONFIG.options().as_display_modifier().is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pattern: Vec::new(),
            options: Options::new(),
        }
    }

    /// Construct a new `Config` with the given pattern and [`Options`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::{Config, Options};
    ///
    /// let config = Config::with_pattern_and_options(
    ///     b"Artichoke( Ruby)?".to_vec(),
    ///     Options::with_ignore_case(),
    /// );
    /// assert_eq!(config.pattern(), b"Artichoke( Ruby)?");
    /// assert_eq!(config.options().as_display_modifier(), "i");
    /// ```
    #[must_use]
    pub const fn with_pattern_and_options(pattern: Vec<u8>, options: Options) -> Self {
        Self { pattern, options }
    }

    /// Extracts a slice containing the entire pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::{Config, Options};
    ///
    /// let config = Config::with_pattern_and_options(
    ///     b"Artichoke( Ruby)?".to_vec(),
    ///     Options::with_ignore_case(),
    /// );
    /// assert_eq!(config.pattern(), b"Artichoke( Ruby)?");
    /// ```
    #[must_use]
    pub fn pattern(&self) -> &[u8] {
        self.pattern.as_slice()
    }

    /// Return a copy of the underlying [`Options`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::{Config, Options};
    ///
    /// let config = Config::with_pattern_and_options(
    ///     b"Artichoke( Ruby)?".to_vec(),
    ///     Options::with_ignore_case(),
    /// );
    /// assert_eq!(config.options().as_display_modifier(), "i");
    /// ```
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

/// Global variable name for the nth capture group from a `Regexp` match.
///
/// Ruby tags captures from the last `Regexp` match with global variables of the
/// form `$1`, `$2`, `$3`, etc. This function accepts [`NonZeroUsize`] because
/// `$0` is not a valid `Regexp` capture group name in Ruby (`$0` refers to the
/// program name).
///
/// This function may return either a `&'static str` or an owned [`String`] for
/// a given capture group name. This function differs from
/// [`nth_match_group_bytes`] by returning `Cow<'static, str>`.
///
///
/// # Examples
///
/// ```
/// use core::num::NonZeroUsize;
/// use spinoso_regexp::nth_match_group;
///
/// # fn example() -> Option<()> {
/// let group = NonZeroUsize::new(1)?;
/// let global_name = nth_match_group(group);
/// assert_eq!(&*global_name, "$1");
///
/// let group = NonZeroUsize::new(27)?;
/// let global_name = nth_match_group(group);
/// assert_eq!(&*global_name, "$27");
/// # None
/// # }
/// ```
#[must_use]
pub fn nth_match_group(group: NonZeroUsize) -> Cow<'static, str> {
    match group.get() {
        1 => Cow::Borrowed("$1"),
        2 => Cow::Borrowed("$2"),
        3 => Cow::Borrowed("$3"),
        4 => Cow::Borrowed("$4"),
        5 => Cow::Borrowed("$5"),
        6 => Cow::Borrowed("$6"),
        7 => Cow::Borrowed("$7"),
        8 => Cow::Borrowed("$8"),
        9 => Cow::Borrowed("$9"),
        10 => Cow::Borrowed("$10"),
        11 => Cow::Borrowed("$11"),
        12 => Cow::Borrowed("$12"),
        13 => Cow::Borrowed("$13"),
        14 => Cow::Borrowed("$14"),
        15 => Cow::Borrowed("$15"),
        16 => Cow::Borrowed("$16"),
        17 => Cow::Borrowed("$17"),
        18 => Cow::Borrowed("$18"),
        19 => Cow::Borrowed("$19"),
        20 => Cow::Borrowed("$20"),
        num => {
            let mut buf = String::new();
            // Suppress formatting errors because this function is infallible.
            //
            // In practice `write!` will never error because the `fmt::Write`
            // impl for `String` never panics.
            let _ = write!(&mut buf, "${}", num);
            Cow::Owned(buf)
        }
    }
}

/// Global variable name for the nth capture group from a `Regexp` match.
///
/// Ruby tags captures from the last `Regexp` match with global variables of the
/// form `$1`, `$2`, `$3`, etc. This function accepts [`NonZeroUsize`] because
/// `$0` is not a valid `Regexp` capture group name in Ruby (`$0` refers to the
/// program name).
///
/// This function may return either a `&'static [u8]` or an owned [`Vec<u8>`]
/// for a given capture group name.  This function differs from
/// [`nth_match_group`] by returning `Cow<'static, [u8]>`.
///
/// # Examples
///
/// ```
/// use core::num::NonZeroUsize;
/// use spinoso_regexp::nth_match_group_bytes;
///
/// # fn example() -> Option<()> {
/// let group = NonZeroUsize::new(1)?;
/// let global_name = nth_match_group_bytes(group);
/// assert_eq!(&*global_name, b"$1");
///
/// let group = NonZeroUsize::new(27)?;
/// let global_name = nth_match_group_bytes(group);
/// assert_eq!(&*global_name, b"$27");
/// # None
/// # }
/// ```
///
/// [`Vec<u8>`]: std::vec::Vec
#[must_use]
pub fn nth_match_group_bytes(group: NonZeroUsize) -> Cow<'static, [u8]> {
    match nth_match_group(group) {
        Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
        Cow::Owned(s) => Cow::Owned(s.into_bytes()),
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;
    use std::borrow::Cow;

    use super::{nth_match_group, nth_match_group_bytes};

    #[test]
    fn match_group_symbol() {
        for num in 1..=1024 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym = nth_match_group(num);
            let num = format!("{num}");
            assert!(sym.len() > 1);
            assert_eq!(&sym[0..1], "$");
            assert_eq!(&sym[1..], num);
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

    #[test]
    fn nth_group_matches_nth_group_bytes() {
        for num in 1..=1024 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym_str = nth_match_group(num);
            let sym_bytes = nth_match_group_bytes(num);
            assert_eq!(&*sym_str.as_bytes(), &*sym_bytes);
        }
    }
}
