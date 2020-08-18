//! Parse options parameter to `Regexp#initialize` and `Regexp::compile`.

use bstr::ByteSlice;
use std::fmt;

use crate::extn::core::regexp;
use crate::extn::prelude::*;

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
    pub fn is_enabled(self) -> bool {
        self == Self::Enabled
    }
}

impl Default for RegexpOption {
    fn default() -> Self {
        Self::new()
    }
}

impl From<bool> for RegexpOption {
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

impl From<RegexpOption> for bool {
    fn from(value: RegexpOption) -> Self {
        value == RegexpOption::Enabled
    }
}

/// Configuration options for Ruby Regexps.
///
/// Options can be supplied either as an `Integer` object to `Regexp::new` or
/// inline in Regexp literals like `/artichoke/i`.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {
    /// Multiline mode.
    pub multiline: RegexpOption,
    /// Case-insensitive mode.
    pub ignore_case: RegexpOption,
    /// Extended mode with insignificant whitespace.
    pub extended: RegexpOption,
    /// Whether the Regexp was parsed as a literal, e.g. `'/artichoke/i`.
    ///
    /// This enables Ruby parsers to inject whether a Regexp is a literal to
    /// the core library. Literal Regexps have some special behavior regrding
    /// capturing groups and report parse failures differently.
    pub literal: bool,
}

impl From<Options> for Int {
    /// Convert an `Options` to its bitflag representation.
    fn from(opts: Options) -> Self {
        let mut bits = 0;
        if opts.multiline.is_enabled() {
            bits |= regexp::MULTILINE;
        }
        if opts.ignore_case.is_enabled() {
            bits |= regexp::IGNORECASE;
        }
        if opts.extended.is_enabled() {
            bits |= regexp::EXTENDED;
        }
        bits
    }
}

impl From<Int> for Options {
    fn from(options: Int) -> Self {
        Self {
            multiline: (options & regexp::MULTILINE > 0).into(),
            ignore_case: (options & regexp::IGNORECASE > 0).into(),
            extended: (options & regexp::EXTENDED > 0).into(),
            literal: options & regexp::LITERAL > 0,
        }
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

impl From<&[u8]> for Options {
    fn from(options: &[u8]) -> Self {
        let multiline = options
            .find_byte(b'm')
            .map_or(RegexpOption::Disabled, |_| RegexpOption::Enabled);
        let ignore_case = options
            .find_byte(b'i')
            .map_or(RegexpOption::Disabled, |_| RegexpOption::Enabled);
        let extended = options
            .find_byte(b'x')
            .map_or(RegexpOption::Disabled, |_| RegexpOption::Enabled);
        Self {
            multiline,
            ignore_case,
            extended,
            literal: false,
        }
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
        Self {
            multiline: RegexpOption::Disabled,
            ignore_case: RegexpOption::Disabled,
            extended: RegexpOption::Disabled,
            literal: false,
        }
    }

    /// An options instance that has only case insensitive mode enabled.
    #[must_use]
    pub fn with_ignore_case() -> Self {
        let mut opts = Self::new();
        opts.ignore_case = RegexpOption::Enabled;
        opts
    }

    /// Convert an `Options` to its bitflag representation.
    ///
    /// Alias for the corresponding `Into<Int>` implementation.
    #[must_use]
    pub fn bitflags(self) -> Int {
        self.into()
    }

    /// Serialize the option flags to a string suitable for a `Regexp` display
    /// or debug implementation.
    ///
    /// See also [`Regexp#inspect`][regexp-inspect].
    ///
    /// [regexp-inspect]: https://ruby-doc.org/core-2.7.1/Regexp.html#method-i-inspect
    #[must_use]
    pub fn as_display_modifier(self) -> &'static str {
        use RegexpOption::{Disabled, Enabled};

        match (self.multiline, self.ignore_case, self.extended) {
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
    /// pattern for configuring an underlying Regexp backend.
    #[must_use]
    pub fn as_inline_modifier(self) -> &'static str {
        use RegexpOption::{Disabled, Enabled};

        match (self.multiline, self.ignore_case, self.extended) {
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
}

impl ConvertMut<Value, Options> for Artichoke {
    fn convert_mut(&mut self, value: Value) -> Options {
        // If options is an Integer, it should be one or more of the constants
        // Regexp::EXTENDED, Regexp::IGNORECASE, and Regexp::MULTILINE, logically
        // or-ed together. Otherwise, if options is not nil or false, the regexp
        // will be case insensitive.
        if let Ok(options) = value.implicitly_convert_to_int(self) {
            Options::from(options)
        } else if let Ok(options) = value.try_into::<Option<bool>>(self) {
            Options::from(options)
        } else if let Ok(options) = value.try_into_mut::<&[u8]>(self) {
            Options::from(options)
        } else {
            Options::with_ignore_case()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Options, RegexpOption};
    use crate::extn::core::regexp::{EXTENDED, IGNORECASE, MULTILINE};
    use crate::test::prelude::*;

    // If options is an `Integer`, it should be one or more of the constants
    // `Regexp::EXTENDED`, `Regexp::IGNORECASE`, and `Regexp::MULTILINE`, or-ed
    // together. Otherwise, if options is not `nil` or `false`, the regexp will
    // be case insensitive.
    #[test]
    fn parse_options() {
        assert_eq!(Options::from(None), Options::new());
        assert_eq!(Options::from(Some(false)), Options::new());
        assert_eq!(Options::from(Some(true)), Options::with_ignore_case());

        let mut opts = Options::new();
        opts.ignore_case = RegexpOption::Enabled;
        assert_eq!(Options::with_ignore_case(), opts);

        let mut opts = Options::new();
        opts.extended = RegexpOption::Enabled;
        assert_eq!(Options::from(EXTENDED), opts);
        assert_ne!(Options::from(EXTENDED | Int::MAX), opts);
        assert_eq!(Options::from(EXTENDED | 4096), opts);
        assert_ne!(Options::from(EXTENDED | IGNORECASE), opts);
        assert_ne!(Options::from(EXTENDED | MULTILINE), opts);
        assert_ne!(Options::from(EXTENDED | IGNORECASE | MULTILINE), opts);

        let mut opts = Options::new();
        opts.ignore_case = RegexpOption::Enabled;
        assert_eq!(Options::from(IGNORECASE), opts);
        assert_ne!(Options::from(IGNORECASE | Int::MAX), opts);
        assert_eq!(Options::from(IGNORECASE | 4096), opts);
        assert_ne!(Options::from(IGNORECASE | EXTENDED), opts);
        assert_ne!(Options::from(IGNORECASE | MULTILINE), opts);
        assert_ne!(Options::from(EXTENDED | IGNORECASE | MULTILINE), opts);

        let mut opts = Options::new();
        opts.multiline = RegexpOption::Enabled;
        assert_eq!(Options::from(MULTILINE), opts);
        assert_ne!(Options::from(MULTILINE | Int::MAX), opts);
        assert_eq!(Options::from(MULTILINE | 4096), opts);
        assert_ne!(Options::from(MULTILINE | IGNORECASE), opts);
        assert_ne!(Options::from(MULTILINE | EXTENDED), opts);
        assert_ne!(Options::from(EXTENDED | IGNORECASE | MULTILINE), opts);

        let mut opts = Options::new();
        opts.extended = RegexpOption::Enabled;
        opts.ignore_case = RegexpOption::Enabled;
        assert_ne!(Options::from(EXTENDED), opts);
        assert_ne!(Options::from(IGNORECASE), opts);
        assert_ne!(Options::from(EXTENDED | Int::MAX), opts);
        assert_ne!(Options::from(IGNORECASE | Int::MAX), opts);
        assert_ne!(Options::from(EXTENDED | IGNORECASE | Int::MAX), opts);
        assert_ne!(Options::from(EXTENDED | 4096), opts);
        assert_ne!(Options::from(MULTILINE | 4096), opts);
        assert_ne!(Options::from(EXTENDED | MULTILINE | 4096), opts);
        assert_eq!(Options::from(EXTENDED | IGNORECASE), opts);
        assert_ne!(Options::from(EXTENDED | IGNORECASE | MULTILINE), opts);

        let mut opts = Options::new();
        opts.extended = RegexpOption::Enabled;
        opts.ignore_case = RegexpOption::Enabled;
        opts.multiline = RegexpOption::Enabled;
        assert_ne!(Options::from(EXTENDED), opts);
        assert_ne!(Options::from(IGNORECASE), opts);
        assert_ne!(Options::from(MULTILINE), opts);
        assert_ne!(Options::from(EXTENDED | Int::MAX), opts);
        assert_ne!(Options::from(IGNORECASE | Int::MAX), opts);
        assert_ne!(Options::from(MULTILINE | Int::MAX), opts);
        assert_ne!(Options::from(EXTENDED | 4096), opts);
        assert_ne!(Options::from(IGNORECASE | 4096), opts);
        assert_ne!(Options::from(MULTILINE | 4096), opts);
        assert_ne!(Options::from(EXTENDED | MULTILINE | 4096), opts);
        assert_ne!(Options::from(EXTENDED | IGNORECASE), opts);
        assert_ne!(Options::from(MULTILINE | IGNORECASE), opts);
        assert_eq!(Options::from(EXTENDED | IGNORECASE | MULTILINE), opts);
    }
}
