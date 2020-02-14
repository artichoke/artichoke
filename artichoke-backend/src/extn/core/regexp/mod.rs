//! [ruby/spec](https://github.com/ruby/spec) compliant implementation of
//! [`Regexp`](https://ruby-doc.org/core-2.6.3/Regexp.html).
//!
//! Each function on `Regexp` is implemented as its own module which contains
//! the `Args` struct for invoking the function.

use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str;

use crate::extn::core::array::Array;
use crate::extn::prelude::*;

pub mod backend;
pub mod enc;
pub mod mruby;
pub mod opts;
pub mod syntax;
pub mod trampoline;

pub use backend::RegexpType;
pub use enc::Encoding;
pub use opts::Options;

use backend::lazy::Lazy;
use backend::onig::Onig;
use backend::regex::utf8::Utf8;

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

/// The Nth group of the last successful match. May be > 1.
#[inline]
#[must_use]
pub fn nth_match_group(group: usize) -> Cow<'static, [u8]> {
    match group {
        0 => panic!("$0 is the name of the current script, not a capture group"),
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
            let mut buf = Vec::from(b"$".as_ref());
            buf.extend(num.to_string().as_bytes());
            buf.into()
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Regexp(Box<dyn RegexpType>);

impl Regexp {
    pub fn new(
        interp: &Artichoke,
        literal_config: Config,
        derived_config: Config,
        encoding: Encoding,
    ) -> Result<Self, Exception> {
        // Patterns must be parsable by Oniguruma.
        let onig = Onig::new(
            interp,
            literal_config.clone(),
            derived_config.clone(),
            encoding,
        )?;
        if let Ok(regex) = Utf8::new(interp, literal_config, derived_config, encoding) {
            Ok(Self(Box::new(regex)))
        } else {
            Ok(Self(Box::new(onig)))
        }
    }

    #[must_use]
    pub fn lazy(pattern: &[u8]) -> Self {
        let literal_config = Config {
            pattern: pattern.to_vec(),
            options: Options::default(),
        };
        let backend = Box::new(Lazy::new(literal_config));
        Self(backend)
    }

    pub fn initialize(
        interp: &mut Artichoke,
        pattern: Value,
        options: Option<Value>,
        encoding: Option<Value>,
        into: Option<Value>,
    ) -> Result<Value, Exception> {
        let (options, encoding) = if let Some(encoding) = encoding {
            let encoding = match enc::parse(&encoding) {
                Ok(encoding) => Some(encoding),
                Err(enc::Error::InvalidEncoding) => {
                    let mut warning = Vec::from(&b"encoding option is ignored -- "[..]);
                    warning.extend(encoding.to_s());
                    interp.warn(warning.as_slice())?;
                    None
                }
            };
            let options = options.as_ref().map(opts::parse);
            (options, encoding)
        } else if let Some(options) = options {
            let encoding = match enc::parse(&options) {
                Ok(encoding) => Some(encoding),
                Err(enc::Error::InvalidEncoding) => {
                    let mut warning = Vec::from(&b"encoding option is ignored -- "[..]);
                    warning.extend(options.to_s());
                    interp.warn(warning.as_slice())?;
                    None
                }
            };
            let options = opts::parse(&options);
            (Some(options), encoding)
        } else {
            (None, None)
        };
        let literal_config = if let Ok(regexp) = unsafe { Self::try_from_ruby(interp, &pattern) } {
            if options.is_some() || encoding.is_some() {
                interp.warn(&b"flags ignored when initializing from Regexp"[..])?;
            }
            let borrow = regexp.borrow();
            let options = borrow.0.literal_config().options;
            Config {
                pattern: borrow.0.literal_config().pattern.clone(),
                options,
            }
        } else {
            let bytes = pattern.implicitly_convert_to_string()?;
            Config {
                pattern: bytes.to_vec(),
                options: options.unwrap_or_default(),
            }
        };
        let (pattern, options) =
            opts::parse_pattern(literal_config.pattern.as_slice(), literal_config.options);
        let derived_config = Config { pattern, options };
        let regexp = Self::new(
            interp,
            literal_config,
            derived_config,
            encoding.unwrap_or_default(),
        )?;
        let regexp = regexp.try_into_ruby(interp, into.as_ref().map(Value::inner))?;
        Ok(regexp)
    }

    pub fn escape(interp: &mut Artichoke, pattern: Value) -> Result<Value, Exception> {
        let pattern = pattern.implicitly_convert_to_string()?;
        let pattern = str::from_utf8(pattern).map_err(|_| {
            ArgumentError::new(interp, "Regexp::escape only supports UTF-8 patterns")
        })?;

        Ok(interp.convert_mut(syntax::escape(pattern)))
    }

    pub fn union(interp: &Artichoke, patterns: &[Value]) -> Result<Value, Exception> {
        let mut iter = patterns.iter().peekable();
        let pattern = if let Some(first) = iter.next() {
            if iter.peek().is_none() {
                if let Ok(ary) = unsafe { Array::try_from_ruby(interp, &first) } {
                    let ary = ary.borrow().as_vec(interp);
                    let mut patterns = Vec::with_capacity(ary.len());
                    for pattern in ary {
                        if let Ok(regexp) = unsafe { Self::try_from_ruby(&interp, &pattern) } {
                            patterns.push(regexp.borrow().0.derived_config().pattern.clone());
                        } else {
                            let pattern = pattern.implicitly_convert_to_string()?;
                            let pattern = str::from_utf8(pattern).map_err(|_| {
                                ArgumentError::new(
                                    interp,
                                    "Regexp::union only supports UTF-8 patterns",
                                )
                            })?;
                            patterns.push(syntax::escape(pattern).into_bytes());
                        }
                    }
                    bstr::join(b"|", patterns)
                } else {
                    let pattern = first;
                    if let Ok(regexp) = unsafe { Self::try_from_ruby(&interp, &pattern) } {
                        regexp.borrow().0.derived_config().pattern.clone()
                    } else {
                        let pattern = pattern.implicitly_convert_to_string()?;
                        let pattern = str::from_utf8(pattern).map_err(|_| {
                            ArgumentError::new(interp, "Regexp::union only supports UTF-8 patterns")
                        })?;
                        syntax::escape(pattern).into_bytes()
                    }
                }
            } else {
                let mut patterns = Vec::with_capacity(patterns.len());
                if let Ok(regexp) = unsafe { Self::try_from_ruby(&interp, &first) } {
                    patterns.push(regexp.borrow().0.derived_config().pattern.clone());
                } else {
                    let bytes = first.implicitly_convert_to_string()?;
                    let pattern = str::from_utf8(bytes).map_err(|_| {
                        ArgumentError::new(interp, "Self::union only supports UTF-8 patterns")
                    })?;
                    patterns.push(syntax::escape(pattern).into_bytes());
                }
                for pattern in iter {
                    if let Ok(regexp) = unsafe { Self::try_from_ruby(&interp, &pattern) } {
                        patterns.push(regexp.borrow().0.derived_config().pattern.clone());
                    } else {
                        let bytes = pattern.implicitly_convert_to_string()?;
                        let pattern = str::from_utf8(bytes).map_err(|_| {
                            ArgumentError::new(interp, "Self::union only supports UTF-8 patterns")
                        })?;
                        patterns.push(syntax::escape(pattern).into_bytes());
                    }
                }
                bstr::join(b"|", patterns)
            }
        } else {
            Vec::from(&b"(?!)"[..])
        };
        let derived_config = {
            let (pattern, options) = opts::parse_pattern(pattern.as_slice(), Options::default());
            Config { pattern, options }
        };
        let literal_config = Config {
            pattern,
            options: Options::default(),
        };
        let regexp = Self::new(interp, literal_config, derived_config, Encoding::default())?;
        let regexp = regexp.try_into_ruby(interp, None)?;
        Ok(regexp)
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &dyn RegexpType {
        self.0.as_ref()
    }

    pub fn case_compare(&self, interp: &mut Artichoke, other: Value) -> Result<Value, Exception> {
        let pattern = if let Ok(pattern) = other.implicitly_convert_to_string() {
            pattern
        } else {
            let sym = interp.intern_symbol(LAST_MATCH);
            let mrb = interp.0.borrow().mrb;
            unsafe {
                sys::mrb_gv_set(mrb, sym, interp.convert(None::<Value>).inner());
            }
            return Ok(interp.convert(false));
        };
        let result = self.0.case_match(interp, pattern)?;
        Ok(interp.convert(result))
    }

    pub fn eql(&self, interp: &Artichoke, other: Value) -> Result<Value, Exception> {
        if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
            Ok(interp.convert(self.inner() == other.borrow().inner()))
        } else {
            Ok(interp.convert(false))
        }
    }

    pub fn hash(&self, interp: &Artichoke) -> Result<Value, Exception> {
        let mut s = DefaultHasher::new();
        self.0.hash(&mut s);
        let hash = s.finish();
        #[allow(clippy::cast_possible_wrap)]
        Ok(interp.convert(hash as Int))
    }

    pub fn inspect(&self, interp: &mut Artichoke) -> Result<Value, Exception> {
        let debug = self.0.inspect(interp);
        Ok(interp.convert_mut(debug))
    }

    pub fn is_casefold(&self, interp: &Artichoke) -> Result<Value, Exception> {
        Ok(interp.convert(self.0.literal_config().options.ignore_case))
    }

    pub fn is_fixed_encoding(&self, interp: &Artichoke) -> Result<Value, Exception> {
        match self.0.encoding() {
            Encoding::No => {
                let opts = Int::try_from(self.0.literal_config().options.flags().bits())
                    .map_err(|_| Fatal::new(interp, "Regexp options do not fit in Integer"))?;
                Ok(interp.convert(opts & NOENCODING != 0))
            }
            Encoding::Fixed => Ok(interp.convert(true)),
            Encoding::None => Ok(interp.convert(false)),
        }
    }

    pub fn is_match(
        &self,
        interp: &Artichoke,
        pattern: Value,
        pos: Option<Value>,
    ) -> Result<Value, Exception> {
        let pattern = pattern.implicitly_convert_to_nilable_string()?;
        let pattern = if let Some(pattern) = pattern {
            pattern
        } else {
            return Ok(interp.convert(false));
        };
        let pos = if let Some(pos) = pos {
            Some(pos.implicitly_convert_to_int()?)
        } else {
            None
        };
        Ok(interp.convert(self.0.is_match(interp, pattern, pos)?))
    }

    pub fn match_(
        &self,
        interp: &mut Artichoke,
        pattern: Value,
        pos: Option<Value>,
        block: Option<Block>,
    ) -> Result<Value, Exception> {
        let pattern = pattern.implicitly_convert_to_nilable_string()?;
        let pattern = if let Some(pattern) = pattern {
            pattern
        } else {
            let mrb = interp.0.borrow().mrb;
            let sym = interp.intern_symbol(LAST_MATCH);
            let matchdata = interp.convert(None::<Value>);
            unsafe {
                sys::mrb_gv_set(mrb, sym, matchdata.inner());
            }
            return Ok(matchdata);
        };
        let pos = if let Some(pos) = pos {
            Some(pos.implicitly_convert_to_int()?)
        } else {
            None
        };
        let result = self.0.match_(interp, pattern, pos, block)?;
        Ok(interp.convert(result))
    }

    pub fn match_operator(
        &self,
        interp: &mut Artichoke,
        pattern: Value,
    ) -> Result<Value, Exception> {
        let pattern = pattern.implicitly_convert_to_nilable_string()?;
        let pattern = if let Some(pattern) = pattern {
            pattern
        } else {
            return Ok(interp.convert(None::<Value>));
        };
        let result = self.0.match_operator(interp, pattern)?;
        Ok(interp.convert(result))
    }

    pub fn named_captures(&self, interp: &mut Artichoke) -> Result<Value, Exception> {
        let captures = self.0.named_captures(interp)?;
        Ok(interp.convert_mut(captures))
    }

    pub fn names(&self, interp: &mut Artichoke) -> Result<Value, Exception> {
        let names = self.0.names(interp);
        Ok(interp.convert_mut(names))
    }

    pub fn options(&self, interp: &Artichoke) -> Result<Value, Exception> {
        let opts = Int::try_from(self.0.literal_config().options.flags().bits())
            .map_err(|_| Fatal::new(interp, "Regexp options do not fit in Integer"))?;
        let opts = opts | self.0.encoding().flags();
        Ok(interp.convert(opts))
    }

    pub fn source(&self, interp: &mut Artichoke) -> Result<Value, Exception> {
        Ok(interp.convert_mut(self.0.literal_config().pattern.as_slice()))
    }

    pub fn string(&self, interp: &mut Artichoke) -> Result<Value, Exception> {
        let string = self.0.string(interp);
        Ok(interp.convert_mut(string))
    }
}

impl RustBackedValue for Regexp {
    fn ruby_type_name() -> &'static str {
        "Regexp"
    }
}

impl From<Box<dyn RegexpType>> for Regexp {
    fn from(regexp: Box<dyn RegexpType>) -> Self {
        Self(regexp)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Config {
    pattern: Vec<u8>,
    options: opts::Options,
}

impl Clone for Box<dyn RegexpType> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl fmt::Debug for Box<dyn RegexpType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug())
    }
}

impl Hash for Box<dyn RegexpType> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.literal_config().hash(state);
    }
}

impl PartialEq for Box<dyn RegexpType> {
    fn eq(&self, other: &Self) -> bool {
        self.derived_config().pattern == other.derived_config().pattern
            && self.encoding() == other.encoding()
    }
}

impl Eq for Box<dyn RegexpType> {}

impl fmt::Debug for &dyn RegexpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug())
    }
}

impl Hash for &dyn RegexpType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.literal_config().hash(state);
    }
}

impl PartialEq for &dyn RegexpType {
    fn eq(&self, other: &Self) -> bool {
        self.derived_config().pattern == other.derived_config().pattern
            && self.encoding() == other.encoding()
    }
}

impl Eq for &dyn RegexpType {}
