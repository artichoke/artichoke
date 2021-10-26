//! Parse options parameter to `Regexp#initialize` and `Regexp::compile`.

use core::fmt;

use bstr::ByteSlice;

use crate::Flags;

/// The state of a Regexp engine flag in [`Options`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegexpOption {
    /// Engine feature is disabled.
    ///
    /// Features are disabled by default.
    Disabled,
    /// Engine feature is disabled.
    Enabled,
}

impl RegexpOption {
    /// Construct a new, disabled `RegexpOption`.
    #[must_use]
    pub const fn new() -> Self {
        Self::Disabled
    }

    /// Return whether this option is enabled.
    ///
    /// An option is enabled if it is equal to [`RegexpOption::Enabled`].
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

impl Default for RegexpOption {
    /// Create a disabled `RegexpOption`.
    fn default() -> Self {
        Self::Disabled
    }
}

impl From<bool> for RegexpOption {
    /// Convert from `bool` to its `RegexpOption` representation.
    ///
    /// `true` creates a [`RegexpOption::Enabled`]. `false` creates a
    /// [`RegexpOption::Disabled`].
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

impl From<RegexpOption> for bool {
    /// Convert from `RegexpOption` to its boolean representation.
    ///
    /// See also [`is_enabled`].
    ///
    /// [`is_enabled`]: RegexpOption::is_enabled
    fn from(value: RegexpOption) -> Self {
        matches!(value, RegexpOption::Enabled)
    }
}

/// Configuration options for Ruby Regexps.
///
/// Options can be supplied either as an `Integer` object to `Regexp::new` or
/// inline in Regexp literals like `/artichoke/i`.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {
    flags: Flags,
}

impl From<Options> for Flags {
    /// Convert an `Options` to its bit flag representation.
    fn from(opts: Options) -> Self {
        opts.flags
    }
}

impl From<Options> for u8 {
    /// Convert an `Options` to its bit representation.
    fn from(opts: Options) -> Self {
        opts.flags.bits()
    }
}

impl From<Options> for i64 {
    /// Convert an `Options` to its widened bit representation.
    fn from(opts: Options) -> Self {
        opts.flags.bits().into()
    }
}

impl From<Flags> for Options {
    fn from(mut flags: Flags) -> Self {
        flags.remove(Flags::FIXEDENCODING | Flags::NOENCODING | Flags::LITERAL);
        Self { flags }
    }
}

impl From<u8> for Options {
    fn from(flags: u8) -> Self {
        let flags = Flags::from_bits_truncate(flags);
        Self::from(flags)
    }
}

impl From<i64> for Options {
    /// Truncate the given `i64` to one byte and generate flags.
    ///
    /// See `From<u8>`. For a conversion that fails if the given `i64` is
    /// larger than [`u8::MAX`], see [`try_from_int`].
    ///
    /// [`try_from_int`]: Self::try_from_int
    fn from(flags: i64) -> Self {
        let [byte, _, _, _, _, _, _, _] = flags.to_le_bytes();
        Self::from(byte)
    }
}

impl From<Option<bool>> for Options {
    fn from(options: Option<bool>) -> Self {
        match options {
            Some(false) | None => Self::new(),
            Some(true) => Self::with_ignore_case(),
        }
    }
}

impl From<&str> for Options {
    fn from(options: &str) -> Self {
        let mut flags = Flags::empty();
        flags.set(Flags::MULTILINE, options.contains('m'));
        flags.set(Flags::IGNORECASE, options.contains('i'));
        flags.set(Flags::EXTENDED, options.contains('x'));
        Self { flags }
    }
}

impl From<&[u8]> for Options {
    fn from(options: &[u8]) -> Self {
        let mut flags = Flags::empty();
        flags.set(Flags::MULTILINE, options.find_byte(b'm').is_some());
        flags.set(Flags::IGNORECASE, options.find_byte(b'i').is_some());
        flags.set(Flags::EXTENDED, options.find_byte(b'x').is_some());
        Self { flags }
    }
}

impl From<String> for Options {
    fn from(options: String) -> Self {
        Self::from(options.as_str())
    }
}

impl From<Vec<u8>> for Options {
    fn from(options: Vec<u8>) -> Self {
        Self::from(options.as_slice())
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_display_modifier())
    }
}

impl Options {
    /// Constructs a new, default `Options`.
    #[must_use]
    pub const fn new() -> Self {
        Self { flags: Flags::empty() }
    }

    /// An options instance that has only case insensitive mode enabled.
    #[must_use]
    pub const fn with_ignore_case() -> Self {
        Self {
            flags: Flags::IGNORECASE,
        }
    }

    /// Try to parse an `Options` from a full-width `i64`.
    ///
    /// If `options` cannot be converted losslessly to a `u8`, this function
    /// returns [`None`]. See `From<u8>`.
    ///
    /// For a conversion from `i64` that truncates the given `options` to `u8`,
    /// see `From<i64>`.
    #[must_use]
    pub fn try_from_int(options: i64) -> Option<Self> {
        let options = u8::try_from(options).ok()?;
        Some(Self::from(options))
    }

    /// Convert an `Options` to its bit flag representation.
    ///
    /// Alias for the corresponding `Into<Flags>` implementation.
    #[must_use]
    pub const fn flags(self) -> Flags {
        self.flags
    }

    /// Convert an `Options` to its bit representation.
    ///
    /// Alias for the corresponding `Into<u8>` implementation.
    #[must_use]
    pub const fn into_bits(self) -> u8 {
        self.flags.bits()
    }

    /// Whether these `Options` are configured for multiline mode.
    #[must_use]
    pub const fn multiline(self) -> RegexpOption {
        if self.flags.intersects(Flags::MULTILINE) {
            RegexpOption::Enabled
        } else {
            RegexpOption::Disabled
        }
    }

    /// Whether these `Options` are configured for case-insensitive mode.
    #[must_use]
    pub const fn ignore_case(self) -> RegexpOption {
        if self.flags.intersects(Flags::IGNORECASE) {
            RegexpOption::Enabled
        } else {
            RegexpOption::Disabled
        }
    }

    /// Whether these `Options` are configured for extended mode with
    /// insignificant whitespace.
    #[must_use]
    pub const fn extended(self) -> RegexpOption {
        if self.flags.intersects(Flags::EXTENDED) {
            RegexpOption::Enabled
        } else {
            RegexpOption::Disabled
        }
    }

    /// Whether the Regexp was parsed as a literal, e.g. `'/artichoke/i`.
    ///
    /// This enables Ruby parsers to inject whether a Regexp is a literal to the
    /// core library. Literal Regexps have some special behavior regrding
    /// capturing groups and report parse failures differently.
    #[must_use]
    pub const fn is_literal(self) -> bool {
        self.flags.intersects(Flags::LITERAL)
    }

    /// Serialize the option flags to a string suitable for a `Regexp` display
    /// or debug implementation.
    ///
    /// See also [`Regexp#inspect`][regexp-inspect].
    ///
    /// [regexp-inspect]: https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-inspect
    #[must_use]
    pub const fn as_display_modifier(self) -> &'static str {
        use RegexpOption::{Disabled, Enabled};

        match (self.multiline(), self.ignore_case(), self.extended()) {
            (Enabled, Enabled, Enabled) => "mix",
            (Enabled, Enabled, Disabled) => "mi",
            (Enabled, Disabled, Enabled) => "mx",
            (Enabled, Disabled, Disabled) => "m",
            (Disabled, Enabled, Enabled) => "ix",
            (Disabled, Enabled, Disabled) => "i",
            (Disabled, Disabled, Enabled) => "x",
            (Disabled, Disabled, Disabled) => "",
        }
    }

    /// Serialize the option flags to a string suitable for including in a raw
    /// pattern for configuring an underlying `Regexp`.
    #[must_use]
    pub const fn as_inline_modifier(self) -> &'static str {
        use RegexpOption::{Disabled, Enabled};

        match (self.multiline(), self.ignore_case(), self.extended()) {
            (Enabled, Enabled, Enabled) => "mix",
            (Enabled, Enabled, Disabled) => "mi-x",
            (Enabled, Disabled, Enabled) => "mx-i",
            (Enabled, Disabled, Disabled) => "m-ix",
            (Disabled, Enabled, Enabled) => "ix-m",
            (Disabled, Enabled, Disabled) => "i-mx",
            (Disabled, Disabled, Enabled) => "x-mi",
            (Disabled, Disabled, Disabled) => "-mix",
        }
    }

    /// Inserts or removes the specified flags depending on the passed value.
    pub fn set(&mut self, other: Flags, value: bool) {
        self.flags.set(other, value);
    }
}

#[cfg(test)]
mod tests {
    use super::{Options, RegexpOption};
    use crate::Flags;

    #[test]
    fn new_is_empty_flags() {
        assert_eq!(Options::new(), Options::from(Flags::empty()));
    }

    #[test]
    fn from_all_flags_ignores_encoding_and_literal() {
        assert_eq!(Options::from(Flags::all()), Options::from(Flags::ALL_REGEXP_OPTS));
    }

    // If options is an `Integer`, it should be one or more of the constants
    // `Regexp::EXTENDED`, `Regexp::IGNORECASE`, and `Regexp::MULTILINE`, or-ed
    // together. Otherwise, if options is not `nil` or `false`, the regexp will
    // be case insensitive.
    #[test]
    #[allow(clippy::too_many_lines)]
    fn parse_options() {
        assert_eq!(Options::from(None), Options::new());
        assert_eq!(Options::from(Some(false)), Options::new());
        assert_eq!(Options::from(Some(true)), Options::with_ignore_case());

        let mut opts = Options::new();
        opts.flags |= Flags::IGNORECASE;
        assert_eq!(Options::with_ignore_case(), opts);

        let mut opts = Options::new();
        opts.flags |= Flags::EXTENDED;
        assert_eq!(Options::from(Flags::EXTENDED), opts);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::MAX),
            None
        );
        assert_eq!(Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | 4096), None);
        assert_eq!(Options::from(Flags::EXTENDED.bits() | 64), opts);
        assert_ne!(Options::from(Flags::EXTENDED | Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::EXTENDED | Flags::MULTILINE), opts);
        assert_ne!(
            Options::from(Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE),
            opts
        );
        assert_eq!(opts.ignore_case(), RegexpOption::Disabled);
        assert_eq!(opts.extended(), RegexpOption::Enabled);
        assert_eq!(opts.multiline(), RegexpOption::Disabled);

        let mut opts = Options::new();
        opts.flags |= Flags::IGNORECASE;
        assert_eq!(Options::from(Flags::IGNORECASE), opts);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | i64::MAX),
            None
        );
        assert_eq!(Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | 4096), None);
        assert_eq!(Options::from(Flags::IGNORECASE.bits() | 64), opts);
        assert_ne!(Options::from(Flags::IGNORECASE | Flags::EXTENDED), opts);
        assert_ne!(Options::from(Flags::IGNORECASE | Flags::MULTILINE), opts);
        assert_ne!(
            Options::from(Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE),
            opts
        );
        assert_eq!(opts.ignore_case(), RegexpOption::Enabled);
        assert_eq!(opts.extended(), RegexpOption::Disabled);
        assert_eq!(opts.multiline(), RegexpOption::Disabled);

        let mut opts = Options::new();
        opts.flags |= Flags::MULTILINE;
        assert_eq!(Options::from(Flags::MULTILINE), opts);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | i64::MAX),
            None
        );
        assert_eq!(Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | 4096), None);
        assert_eq!(Options::from(Flags::MULTILINE.bits() | 64), opts);
        assert_ne!(Options::from(Flags::MULTILINE | Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::MULTILINE | Flags::EXTENDED), opts);
        assert_ne!(
            Options::from(Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE),
            opts
        );
        assert_eq!(opts.ignore_case(), RegexpOption::Disabled);
        assert_eq!(opts.extended(), RegexpOption::Disabled);
        assert_eq!(opts.multiline(), RegexpOption::Enabled);

        let mut opts = Options::new();
        opts.flags |= Flags::EXTENDED | Flags::IGNORECASE;
        assert_ne!(Options::from(Flags::EXTENDED), opts);
        assert_ne!(Options::from(Flags::IGNORECASE), opts);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::from(Flags::IGNORECASE.bits()) | i64::MAX),
            None
        );
        assert_eq!(Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | 4096), None);
        assert_eq!(Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | 4096), None);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::from(Flags::MULTILINE.bits()) | 4096),
            None
        );
        assert_eq!(
            Options::from(Flags::EXTENDED.bits() | Flags::IGNORECASE.bits() | 64),
            opts
        );
        assert_eq!(Options::from(Flags::EXTENDED | Flags::IGNORECASE), opts);
        assert_ne!(
            Options::from(Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE),
            opts
        );
        assert_eq!(opts.ignore_case(), RegexpOption::Enabled);
        assert_eq!(opts.extended(), RegexpOption::Enabled);
        assert_eq!(opts.multiline(), RegexpOption::Disabled);

        let mut opts = Options::new();
        opts.flags |= Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE;
        assert_ne!(Options::from(Flags::EXTENDED), opts);
        assert_ne!(Options::from(Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::MULTILINE), opts);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | i64::MAX),
            None
        );
        assert_eq!(Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | 4096), None);
        assert_eq!(Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | 4096), None);
        assert_eq!(Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | 4096), None);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::from(Flags::MULTILINE.bits()) | 4096),
            None
        );
        assert_ne!(Options::from(Flags::EXTENDED.bits() | 64), opts);
        assert_ne!(Options::from(Flags::IGNORECASE.bits() | 64), opts);
        assert_ne!(Options::from(Flags::MULTILINE.bits() | 64), opts);
        assert_ne!(
            Options::from(Flags::EXTENDED.bits() | Flags::MULTILINE.bits() | 64),
            opts
        );
        assert_ne!(Options::from(Flags::EXTENDED | Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::MULTILINE | Flags::IGNORECASE), opts);
        assert_eq!(
            Options::from(Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE),
            opts
        );
        assert_eq!(Options::from(Flags::ALL_REGEXP_OPTS), opts);
        assert_eq!(opts.ignore_case(), RegexpOption::Enabled);
        assert_eq!(opts.extended(), RegexpOption::Enabled);
        assert_eq!(opts.multiline(), RegexpOption::Enabled);

        // `ALL_REGEXP_OPTS` is equivalent to `EXTENDED | IGNORECASE | MULTILINE` flags.
        let mut opts = Options::new();
        opts.flags |= Flags::ALL_REGEXP_OPTS;
        assert_ne!(Options::from(Flags::EXTENDED), opts);
        assert_ne!(Options::from(Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::MULTILINE), opts);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | i64::MAX),
            None
        );
        assert_eq!(Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | 4096), None);
        assert_eq!(Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | 4096), None);
        assert_eq!(Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | 4096), None);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::from(Flags::MULTILINE.bits()) | 4096),
            None
        );
        assert_ne!(Options::from(Flags::EXTENDED.bits() | 64), opts);
        assert_ne!(Options::from(Flags::IGNORECASE.bits() | 64), opts);
        assert_ne!(Options::from(Flags::MULTILINE.bits() | 64), opts);
        assert_ne!(
            Options::from(Flags::EXTENDED.bits() | Flags::MULTILINE.bits() | 64),
            opts
        );
        assert_ne!(Options::from(Flags::EXTENDED | Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::MULTILINE | Flags::IGNORECASE), opts);
        assert_eq!(
            Options::from(Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE),
            opts
        );
        assert_eq!(Options::from(Flags::ALL_REGEXP_OPTS), opts);
        assert_eq!(opts.ignore_case(), RegexpOption::Enabled);
        assert_eq!(opts.extended(), RegexpOption::Enabled);
        assert_eq!(opts.multiline(), RegexpOption::Enabled);

        // Ignore encoding and literal flags.
        let opts = Options::from(Flags::all());
        assert_ne!(Options::from(Flags::EXTENDED), opts);
        assert_ne!(Options::from(Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::MULTILINE), opts);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | i64::MAX),
            None
        );
        assert_eq!(
            Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | i64::MAX),
            None
        );
        assert_eq!(Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | 4096), None);
        assert_eq!(Options::try_from_int(i64::from(Flags::IGNORECASE.bits()) | 4096), None);
        assert_eq!(Options::try_from_int(i64::from(Flags::MULTILINE.bits()) | 4096), None);
        assert_eq!(
            Options::try_from_int(i64::from(Flags::EXTENDED.bits()) | i64::from(Flags::MULTILINE.bits()) | 4096),
            None
        );
        assert_ne!(Options::from(Flags::EXTENDED.bits() | 64), opts);
        assert_ne!(Options::from(Flags::IGNORECASE.bits() | 64), opts);
        assert_ne!(Options::from(Flags::MULTILINE.bits() | 64), opts);
        assert_ne!(
            Options::from(Flags::EXTENDED.bits() | Flags::MULTILINE.bits() | 64),
            opts
        );
        assert_ne!(Options::from(Flags::EXTENDED | Flags::IGNORECASE), opts);
        assert_ne!(Options::from(Flags::MULTILINE | Flags::IGNORECASE), opts);
        assert_eq!(
            Options::from(Flags::EXTENDED | Flags::IGNORECASE | Flags::MULTILINE),
            opts
        );
        assert_eq!(Options::from(Flags::ALL_REGEXP_OPTS), opts);
        assert_eq!(opts.ignore_case(), RegexpOption::Enabled);
        assert_eq!(opts.extended(), RegexpOption::Enabled);
        assert_eq!(opts.multiline(), RegexpOption::Enabled);
    }
}
