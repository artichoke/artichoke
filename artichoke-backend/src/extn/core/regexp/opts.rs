//! Parse options parameter to `Regexp#initialize` and `Regexp::compile`.

use bstr::ByteSlice;
use std::fmt;

use crate::extn::core::regexp;
use crate::extn::prelude::*;

/// Configuration options for Ruby Regexps.
///
/// Options can be supplied either as an `Integer` object to `Regexp::new` or
/// inline in Regexp literals like `/artichoke/i`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Options {
    /// Multiline mode.
    pub multiline: bool,
    /// Case-insensitive mode.
    pub ignore_case: bool,
    /// Extended mode with insignificant whitespace.
    pub extended: bool,
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
        if opts.multiline {
            bits |= regexp::MULTILINE;
        }
        if opts.ignore_case {
            bits |= regexp::IGNORECASE;
        }
        if opts.extended {
            bits |= regexp::EXTENDED;
        }
        bits
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_display_modifier())
    }
}

impl Options {
    /// An options instance that has only case insensitive mode enabled.
    #[must_use]
    pub fn ignore_case() -> Self {
        let mut opts = Self::default();
        opts.ignore_case = true;
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
        match (self.multiline, self.ignore_case, self.extended) {
            (true, true, true) => "mix",
            (true, true, false) => "mi",
            (true, false, true) => "mx",
            (true, false, false) => "m",
            (false, true, true) => "ix",
            (false, true, false) => "i",
            (false, false, true) => "x",
            (false, false, false) => "",
        }
    }

    /// Serialize the option flags to a string suitable for including in a raw
    /// pattern for configuring an underlying Regexp backend.
    #[must_use]
    pub fn as_inline_modifier(self) -> &'static str {
        match (self.multiline, self.ignore_case, self.extended) {
            (true, true, true) => "mix",
            (true, true, false) => "mi-x",
            (true, false, true) => "mx-i",
            (true, false, false) => "m-ix",
            (false, true, true) => "ix-m",
            (false, true, false) => "i-mx",
            (false, false, true) => "x-mi",
            (false, false, false) => "-mix",
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
            Options {
                multiline: options & regexp::MULTILINE > 0,
                ignore_case: options & regexp::IGNORECASE > 0,
                extended: options & regexp::EXTENDED > 0,
                literal: options & regexp::LITERAL > 0,
            }
        } else if let Ok(options) = value.try_into::<Option<bool>>(self) {
            match options {
                Some(false) | None => Options::default(),
                _ => Options::ignore_case(),
            }
        } else if let Ok(options) = value.try_into_mut::<&[u8]>(self) {
            Options {
                multiline: options.find_byte(b'm').is_some(),
                ignore_case: options.find_byte(b'i').is_some(),
                extended: options.find_byte(b'x').is_some(),
                literal: false,
            }
        } else {
            Options::ignore_case()
        }
    }
}
