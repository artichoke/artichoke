//! [ruby/spec](https://github.com/ruby/spec) compliant implementation of
//! [`Regexp`](https://ruby-doc.org/core-2.6.3/Regexp.html).
//!
//! Each function on `Regexp` is implemented as its own module which contains
//! the `Args` struct for invoking the function.

use bstr::BString;
use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::str;

use crate::extn::core::array::Array;
use crate::extn::prelude::*;

pub mod backend;
mod boxing;
pub mod enc;
pub mod mruby;
pub mod opts;
pub mod pattern;
pub mod syntax;
pub mod trampoline;

pub use backend::{NilableString, RegexpType, Scan};
pub use enc::Encoding;
pub use opts::Options;

use backend::lazy::Lazy;
#[cfg(feature = "core-regexp-oniguruma")]
use backend::onig::Onig;
use backend::regex::utf8::Utf8;

pub type NameToCaptureLocations = Vec<(Vec<u8>, Vec<Int>)>;

pub const IGNORECASE: Int = 1;
pub const EXTENDED: Int = 2;
pub const MULTILINE: Int = 4;
const ALL_REGEXP_OPTS: Int = IGNORECASE | EXTENDED | MULTILINE;

pub const FIXEDENCODING: Int = 16;
pub const NOENCODING: Int = 32;

pub const LITERAL: Int = 128;

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

/// Global variable name for the nth capture group from a `Regexp` match.
#[inline]
#[must_use]
pub fn nth_match_group(group: NonZeroUsize) -> Cow<'static, [u8]> {
    match group.get() {
        1 => b"$1".as_ref().into(),
        2 => b"$2".as_ref().into(),
        3 => b"$3".as_ref().into(),
        4 => b"$4".as_ref().into(),
        5 => b"$5".as_ref().into(),
        6 => b"$6".as_ref().into(),
        7 => b"$7".as_ref().into(),
        8 => b"$8".as_ref().into(),
        9 => b"$9".as_ref().into(),
        10 => b"$10".as_ref().into(),
        11 => b"$11".as_ref().into(),
        12 => b"$12".as_ref().into(),
        13 => b"$13".as_ref().into(),
        14 => b"$14".as_ref().into(),
        15 => b"$15".as_ref().into(),
        16 => b"$16".as_ref().into(),
        17 => b"$17".as_ref().into(),
        18 => b"$18".as_ref().into(),
        19 => b"$19".as_ref().into(),
        20 => b"$20".as_ref().into(),
        num => {
            let mut buf = String::from("$");
            // Suppress io errors because this function is infallible.
            //
            // In practice string::format_int_into will never error because the
            // fmt::Write impl for String never panics.
            let _ = string::format_int_into(&mut buf, num);
            buf.into_bytes().into()
        }
    }
}

pub fn clear_capture_globals(interp: &mut Artichoke) -> Result<(), Exception> {
    let mut idx = interp.active_regexp_globals()?;
    while let Some(group) = NonZeroUsize::new(idx) {
        interp.unset_global_variable(nth_match_group(group))?;
        idx -= 1
    }
    interp.clear_regexp()?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Regexp(Box<dyn RegexpType>);

impl Hash for Regexp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for Regexp {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl Eq for Regexp {}

impl Regexp {
    pub fn new(
        literal_config: Config,
        derived_config: Config,
        encoding: Encoding,
    ) -> Result<Self, Exception> {
        #[cfg(feature = "core-regexp-oniguruma")]
        {
            // Patterns must be parsable by Oniguruma.
            let onig = Onig::new(literal_config.clone(), derived_config.clone(), encoding)?;
            if let Ok(regex) = Utf8::new(literal_config, derived_config, encoding) {
                Ok(Self(Box::new(regex)))
            } else {
                Ok(Self(Box::new(onig)))
            }
        }
        #[cfg(not(feature = "core-regexp-oniguruma"))]
        {
            let regex = Utf8::new(literal_config, derived_config, encoding)?;
            Ok(Self(Box::new(regex)))
        }
    }

    #[must_use]
    pub fn lazy(pattern: Vec<u8>) -> Self {
        let literal_config = Config {
            pattern: pattern.into(),
            options: Options::default(),
        };
        let backend = Box::new(Lazy::from(literal_config));
        Self(backend)
    }

    pub fn initialize(
        interp: &mut Artichoke,
        mut pattern: Value,
        options: Option<opts::Options>,
        encoding: Option<enc::Encoding>,
    ) -> Result<Self, Exception> {
        let literal_config =
            if let Ok(regexp) = unsafe { Self::unbox_from_value(&mut pattern, interp) } {
                if options.is_some() || encoding.is_some() {
                    interp.warn(&b"flags ignored when initializing from Regexp"[..])?;
                }
                let options = regexp.inner().literal_config().options;
                Config {
                    pattern: regexp.inner().literal_config().pattern.clone(),
                    options,
                }
            } else {
                let bytes = pattern.implicitly_convert_to_string(interp)?;
                Config {
                    pattern: bytes.into(),
                    options: options.unwrap_or_default(),
                }
            };
        let pattern = pattern::parse(&literal_config.pattern, literal_config.options);
        let options = pattern.options();
        let derived_config = Config {
            pattern: pattern.into_pattern().into(),
            options,
        };
        Self::new(literal_config, derived_config, encoding.unwrap_or_default())
    }

    pub fn escape(pattern: &[u8]) -> Result<String, Exception> {
        if let Ok(pattern) = str::from_utf8(pattern) {
            Ok(syntax::escape(pattern))
        } else {
            Err(ArgumentError::from("invalid encoding (non UTF-8)").into())
        }
    }

    pub fn union<T>(interp: &mut Artichoke, patterns: T) -> Result<Self, Exception>
    where
        T: IntoIterator<Item = Value>,
    {
        fn extract_pattern(
            interp: &mut Artichoke,
            value: &mut Value,
        ) -> Result<Vec<u8>, Exception> {
            if let Ok(regexp) = unsafe { Regexp::unbox_from_value(value, interp) } {
                Ok(regexp.inner().derived_config().pattern.clone().into())
            } else {
                let bytes = value.implicitly_convert_to_string(interp)?;
                let pattern = if let Ok(pattern) = str::from_utf8(bytes) {
                    pattern
                } else {
                    // drop(bytes);
                    return Err(ArgumentError::from("invalid encoding (non UTF-8)").into());
                };
                Ok(syntax::escape(pattern).into_bytes())
            }
        }
        let mut iter = patterns.into_iter();
        let pattern = if let Some(mut first) = iter.next() {
            if let Some(mut second) = iter.next() {
                let mut patterns = vec![];
                patterns.push(extract_pattern(interp, &mut first)?);
                patterns.push(extract_pattern(interp, &mut second)?);
                for mut value in iter {
                    patterns.push(extract_pattern(interp, &mut value)?);
                }
                bstr::join(b"|", patterns)
            } else if let Ok(ary) = unsafe { Array::unbox_from_value(&mut first, interp) } {
                let mut patterns = Vec::with_capacity(ary.len());
                for mut value in &*ary {
                    patterns.push(extract_pattern(interp, &mut value)?);
                }
                bstr::join(b"|", patterns)
            } else {
                extract_pattern(interp, &mut first)?
            }
        } else {
            b"(?!)".to_vec()
        };

        let derived_config = {
            let pattern = pattern::parse(&pattern, Options::default());
            let options = pattern.options();
            Config {
                pattern: pattern.into_pattern().into(),
                options,
            }
        };
        let literal_config = Config {
            pattern: pattern.into(),
            options: Options::default(),
        };
        Self::new(literal_config, derived_config, Encoding::default())
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &dyn RegexpType {
        self.0.as_ref()
    }

    pub fn case_compare(&self, interp: &mut Artichoke, other: Value) -> Result<bool, Exception> {
        let pattern = if let Ok(pattern) = other.implicitly_convert_to_string(interp) {
            pattern
        } else {
            interp.unset_global_variable(LAST_MATCH)?;
            return Ok(false);
        };
        self.0.case_match(interp, pattern)
    }

    #[must_use]
    pub fn eql(&self, interp: &mut Artichoke, mut other: Value) -> bool {
        if let Ok(other) = unsafe { Self::unbox_from_value(&mut other, interp) } {
            self.inner() == other.inner()
        } else {
            false
        }
    }

    #[inline]
    #[must_use]
    pub fn hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.0.hash(&mut s);
        s.finish()
    }

    #[inline]
    #[must_use]
    pub fn inspect(&self) -> Vec<u8> {
        self.0.inspect()
    }

    #[inline]
    #[must_use]
    pub fn is_casefold(&self) -> bool {
        self.0.literal_config().options.ignore_case
    }

    #[must_use]
    pub fn is_fixed_encoding(&self) -> bool {
        match self.0.encoding() {
            Encoding::No | Encoding::None => false,
            Encoding::Fixed => true,
        }
    }

    pub fn is_match(&self, pattern: Option<&[u8]>, pos: Option<Int>) -> Result<bool, Exception> {
        if let Some(pattern) = pattern {
            self.0.is_match(pattern, pos)
        } else {
            Ok(false)
        }
    }

    pub fn match_(
        &self,
        interp: &mut Artichoke,
        pattern: Option<&[u8]>,
        pos: Option<Int>,
        block: Option<Block>,
    ) -> Result<Value, Exception> {
        if let Some(pattern) = pattern {
            self.0.match_(interp, pattern, pos, block)
        } else {
            interp.unset_global_variable(LAST_MATCH)?;
            Ok(Value::nil())
        }
    }

    #[inline]
    pub fn match_operator(
        &self,
        interp: &mut Artichoke,
        pattern: Option<&[u8]>,
    ) -> Result<Option<usize>, Exception> {
        if let Some(pattern) = pattern {
            self.0.match_operator(interp, pattern)
        } else {
            Ok(None)
        }
    }

    pub fn named_captures(&self) -> Result<NameToCaptureLocations, Exception> {
        let captures = self.0.named_captures()?;
        let mut converted = Vec::with_capacity(captures.len());
        for (name, indexes) in captures {
            let mut fixnums = Vec::with_capacity(indexes.len());
            for idx in indexes {
                if let Ok(idx) = Int::try_from(idx) {
                    fixnums.push(idx);
                } else {
                    return Err(ArgumentError::from("string too long").into());
                }
            }
            converted.push((name, fixnums));
        }
        Ok(converted)
    }

    #[inline]
    #[must_use]
    pub fn names(&self) -> Vec<Vec<u8>> {
        self.0.names()
    }

    #[inline]
    #[must_use]
    pub fn options(&self) -> Int {
        let opts = self.0.literal_config().options;
        Int::from(opts) | Int::from(self.0.encoding())
    }

    #[inline]
    #[must_use]
    pub fn source(&self) -> &[u8] {
        self.0.literal_config().pattern.as_slice()
    }

    #[inline]
    #[must_use]
    pub fn string(&self) -> &[u8] {
        self.0.string()
    }
}

impl From<Box<dyn RegexpType>> for Regexp {
    fn from(regexp: Box<dyn RegexpType>) -> Self {
        Self(regexp)
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config {
    pattern: BString,
    options: opts::Options,
}

impl TryConvertMut<(Option<Value>, Option<Value>), (Option<opts::Options>, Option<enc::Encoding>)>
    for Artichoke
{
    type Error = Exception;

    fn try_convert_mut(
        &mut self,
        value: (Option<Value>, Option<Value>),
    ) -> Result<(Option<opts::Options>, Option<enc::Encoding>), Self::Error> {
        let (options, encoding) = value;
        if let Some(encoding) = encoding {
            let encoding = if let Ok(encoding) = self.try_convert_mut(encoding) {
                Some(encoding)
            } else {
                let mut warning = Vec::from(&b"encoding option is ignored -- "[..]);
                warning.extend(encoding.to_s(self));
                self.warn(warning.as_slice())?;
                None
            };
            let options = options.map(|options| self.convert_mut(options));
            Ok((options, encoding))
        } else if let Some(options) = options {
            let encoding = if let Ok(encoding) = self.try_convert_mut(options) {
                Some(encoding)
            } else {
                let mut warning = Vec::from(&b"encoding option is ignored -- "[..]);
                warning.extend(options.to_s(self));
                self.warn(warning.as_slice())?;
                None
            };
            let options = self.convert_mut(options);
            Ok((Some(options), encoding))
        } else {
            Ok((None, None))
        }
    }
}
