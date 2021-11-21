//! [ruby/spec](https://github.com/ruby/spec) compliant implementation of
//! [`Regexp`](https://ruby-doc.org/core-2.6.3/Regexp.html).
//!
//! Each function on `Regexp` is implemented as its own module which contains
//! the `Args` struct for invoking the function.

#![allow(clippy::module_name_repetitions)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::str;

#[doc(inline)]
pub use spinoso_regexp::{
    nth_match_group_bytes as nth_match_group, Config, Encoding, Flags, InvalidEncodingError, Options, RegexpError,
    RegexpOption, Source, HIGHEST_MATCH_GROUP, LAST_MATCH, LAST_MATCHED_STRING, STRING_LEFT_OF_MATCH,
    STRING_RIGHT_OF_MATCH,
};

use crate::convert::implicitly_convert_to_string;
use crate::extn::core::array::Array;
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

pub mod backend;
mod boxing;
pub mod enc;
pub mod mruby;
pub mod opts;
pub mod pattern;
pub mod syntax;
pub mod trampoline;

use backend::lazy::Lazy;
#[cfg(feature = "core-regexp-oniguruma")]
use backend::onig::Onig;
use backend::regex::utf8::Utf8;
pub use backend::{NilableString, RegexpType, Scan};

pub type NameToCaptureLocations = Vec<(Vec<u8>, Vec<i64>)>;

pub fn clear_capture_globals(interp: &mut Artichoke) -> Result<(), Error> {
    let mut idx = interp.active_regexp_globals()?;
    while let Some(group) = NonZeroUsize::new(idx) {
        interp.unset_global_variable(nth_match_group(group))?;
        idx -= 1;
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
    pub fn new(source: Source, config: Config, encoding: Encoding) -> Result<Self, Error> {
        #[cfg(feature = "core-regexp-oniguruma")]
        {
            // Patterns must be parsable by Oniguruma.
            let onig = Onig::new(source.clone(), config.clone(), encoding)?;
            if let Ok(regex) = Utf8::new(source, config, encoding) {
                Ok(Self(Box::new(regex)))
            } else {
                Ok(Self(Box::new(onig)))
            }
        }
        #[cfg(not(feature = "core-regexp-oniguruma"))]
        {
            let regex = Utf8::new(source, config, encoding)?;
            Ok(Self(Box::new(regex)))
        }
    }

    #[must_use]
    pub fn lazy(pattern: Vec<u8>) -> Self {
        let config = Config::with_pattern_and_options(pattern, Options::new());
        let backend = Box::new(Lazy::from(config));
        Self(backend)
    }

    pub fn initialize(
        interp: &mut Artichoke,
        mut pattern: Value,
        options: Option<Options>,
        encoding: Option<Encoding>,
    ) -> Result<Self, Error> {
        let source = if let Ok(regexp) = unsafe { Self::unbox_from_value(&mut pattern, interp) } {
            if options.is_some() || encoding.is_some() {
                interp.warn(&b"flags ignored when initializing from Regexp"[..])?;
            }
            regexp.inner().source().clone()
        } else {
            // Safety:
            //
            // `bytes` is converted to an owned byte vec before any additional
            // operations are run on the interpreter which might trigger a
            // garbage collection of `pattern` and its backing `RString`.
            let bytes = unsafe { implicitly_convert_to_string(interp, &mut pattern)? };
            Source::with_pattern_and_options(bytes.to_vec(), options.unwrap_or_default())
        };
        let pattern = pattern::parse(source.pattern(), source.options());
        let options = pattern.options();
        let config = Config::with_pattern_and_options(pattern.into_pattern(), options);
        Self::new(source, config, encoding.unwrap_or_default())
    }

    pub fn escape(pattern: &[u8]) -> Result<String, Error> {
        if let Ok(pattern) = str::from_utf8(pattern) {
            Ok(syntax::escape(pattern))
        } else {
            Err(ArgumentError::with_message("invalid encoding (non UTF-8)").into())
        }
    }

    pub fn union<T>(interp: &mut Artichoke, patterns: T) -> Result<Self, Error>
    where
        T: IntoIterator<Item = Value>,
    {
        fn extract_pattern(interp: &mut Artichoke, value: &mut Value) -> Result<Vec<u8>, Error> {
            if let Ok(regexp) = unsafe { Regexp::unbox_from_value(value, interp) } {
                let source = regexp.inner().config();
                Ok(source.pattern().to_vec())
            } else {
                // Safety:
                //
                // `bytes` is converted to an owned `String` before any
                // additional operations are run on the interpreter which might
                // trigger a garbage collection of `pattern` and its backing
                // `RString`.
                let bytes = unsafe { implicitly_convert_to_string(interp, value)? };
                if let Ok(pattern) = str::from_utf8(bytes) {
                    Ok(syntax::escape(pattern).into_bytes())
                } else {
                    Err(ArgumentError::with_message("invalid encoding (non UTF-8)").into())
                }
            }
        }
        let mut iter = patterns.into_iter();
        let pattern = if let Some(mut first) = iter.next() {
            if let Some(mut second) = iter.next() {
                let mut patterns = vec![
                    extract_pattern(interp, &mut first)?,
                    extract_pattern(interp, &mut second)?,
                ];
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

        let config = {
            let pattern = pattern::parse(&pattern, Options::new());
            let options = pattern.options();
            Config::with_pattern_and_options(pattern.into_pattern(), options)
        };
        let source = Source::with_pattern_and_options(pattern, Options::new());
        Self::new(source, config, Encoding::new())
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &dyn RegexpType {
        self.0.as_ref()
    }

    pub fn case_compare(&self, interp: &mut Artichoke, mut other: Value) -> Result<bool, Error> {
        let pattern_vec;
        let pattern = if matches!(other.ruby_type(), Ruby::Symbol) {
            let symbol = unsafe { Symbol::unbox_from_value(&mut other, interp)? };
            pattern_vec = symbol.bytes(interp).to_vec();
            pattern_vec.as_slice()
        } else if let Ok(pattern) = unsafe { implicitly_convert_to_string(interp, &mut other) } {
            // Safety:
            //
            // `pattern` is converted to an owned byte vec before any
            // intervening operations on the VM which may trigger a garbage
            // collection of the `RString` that backs `other`.
            pattern_vec = pattern.to_vec();
            pattern_vec.as_slice()
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
        self.0.source().is_casefold()
    }

    #[must_use]
    pub fn is_fixed_encoding(&self) -> bool {
        match self.0.encoding() {
            Encoding::No | Encoding::None => false,
            Encoding::Fixed => true,
        }
    }

    pub fn is_match(&self, pattern: Option<&[u8]>, pos: Option<i64>) -> Result<bool, Error> {
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
        pos: Option<i64>,
        block: Option<Block>,
    ) -> Result<Value, Error> {
        if let Some(pattern) = pattern {
            self.0.match_(interp, pattern, pos, block)
        } else {
            interp.unset_global_variable(LAST_MATCH)?;
            Ok(Value::nil())
        }
    }

    #[inline]
    pub fn match_operator(&self, interp: &mut Artichoke, pattern: Option<&[u8]>) -> Result<Option<usize>, Error> {
        if let Some(pattern) = pattern {
            self.0.match_operator(interp, pattern)
        } else {
            Ok(None)
        }
    }

    pub fn named_captures(&self) -> Result<NameToCaptureLocations, Error> {
        let captures = self.0.named_captures()?;
        let mut converted = Vec::with_capacity(captures.len());
        for (name, indexes) in captures {
            let mut fixnums = Vec::with_capacity(indexes.len());
            for idx in indexes {
                if let Ok(idx) = i64::try_from(idx) {
                    fixnums.push(idx);
                } else {
                    return Err(ArgumentError::with_message("string too long").into());
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
    pub fn options(&self) -> i64 {
        let options = self.0.source().options().flags();
        let encoding = self.0.encoding().flags();
        i64::from((options | encoding).bits())
    }

    #[inline]
    #[must_use]
    pub fn source(&self) -> &[u8] {
        self.0.source().pattern()
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

impl TryConvertMut<(Option<Value>, Option<Value>), (Option<Options>, Option<Encoding>)> for Artichoke {
    type Error = Error;

    fn try_convert_mut(
        &mut self,
        value: (Option<Value>, Option<Value>),
    ) -> Result<(Option<Options>, Option<Encoding>), Self::Error> {
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
