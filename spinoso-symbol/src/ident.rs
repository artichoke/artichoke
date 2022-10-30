//! Parser for classifying byte strings as Ruby identifiers.
//!
//! This module exposes a parser for determining if a sequence of bytes is a
//! valid Ruby identifier. These routines also classify idents by type, for
//! example, a local variable (`is_spinoso`), constant name (`SPINOSO_SYMBOL`),
//! or class variable (`@@spinoso_symbol`).
//!
//! # Examples – local variable
//!
//! ```
//! # use spinoso_symbol::IdentifierType;
//! assert_eq!(
//!     "spinoso".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Local)
//! );
//! assert_eq!(
//!     "spinoso_symbol_features".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Local)
//! );
//! ```
//!
//! # Examples – constant
//!
//! ```
//! # use spinoso_symbol::IdentifierType;
//! assert_eq!(
//!     "Spinoso".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Constant)
//! );
//! assert_eq!(
//!     "SpinosoSymbol".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Constant)
//! );
//! assert_eq!(
//!     "SPINOSO_SYMBOL_FEATURES".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Constant)
//! );
//! ```
//!
//! # Examples – global
//!
//! ```
//! # use spinoso_symbol::IdentifierType;
//! assert_eq!(
//!     "$use_spinoso_symbol".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Global)
//! );
//! assert_eq!(
//!     "$USE_SPINOSO_SYMBOL".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Global)
//! );
//! ```
//!
//! # Examples – instance and class variables
//!
//! ```
//! # use spinoso_symbol::IdentifierType;
//! assert_eq!(
//!     "@artichoke".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Instance)
//! );
//! assert_eq!(
//!     "@@rumble".parse::<IdentifierType>(),
//!     Ok(IdentifierType::Class)
//! );
//! ```
//!
//! # Example – attribute setter
//!
//! Attribute setters are local idents that end in `=`.
//!
//! ```
//! # use spinoso_symbol::IdentifierType;
//! assert_eq!(
//!     "artichoke=".parse::<IdentifierType>(),
//!     Ok(IdentifierType::AttrSet)
//! );
//! assert_eq!(
//!     "spinoso_symbol=".parse::<IdentifierType>(),
//!     Ok(IdentifierType::AttrSet)
//! );
//! ```

use core::fmt;
use core::str::FromStr;

use bstr::ByteSlice;

/// Valid types for Ruby identifiers.
///
/// Spinoso symbol parses byte strings to determine if they are valid idents for
/// the [`Inspect`] iterator (which requires the **inspect** Cargo feature to be
/// enabled). Symbols that are valid idents do not get wrapped in `"` when
/// generating their debug output.
///
/// See variant documentation for the set of ident types.
///
/// `IdentifierType`'s primary interface is through the [`TryFrom`] and
/// [`FromStr`] conversion traits. Parsing `&str` and `&[u8]` is supported.
///
/// # Examples – local variable
///
/// ```
/// # use spinoso_symbol::IdentifierType;
/// assert_eq!(
///     "spinoso".parse::<IdentifierType>(),
///     Ok(IdentifierType::Local)
/// );
/// assert_eq!(
///     "spinoso_symbol_features".parse::<IdentifierType>(),
///     Ok(IdentifierType::Local)
/// );
/// ```
///
/// # Examples – constant
///
/// ```
/// # use spinoso_symbol::IdentifierType;
/// assert_eq!(
///     "Spinoso".parse::<IdentifierType>(),
///     Ok(IdentifierType::Constant)
/// );
/// assert_eq!(
///     "SpinosoSymbol".parse::<IdentifierType>(),
///     Ok(IdentifierType::Constant)
/// );
/// assert_eq!(
///     "SPINOSO_SYMBOL_FEATURES".parse::<IdentifierType>(),
///     Ok(IdentifierType::Constant)
/// );
/// ```
///
/// # Examples – global
///
/// ```
/// # use spinoso_symbol::IdentifierType;
/// assert_eq!(
///     "$use_spinoso_symbol".parse::<IdentifierType>(),
///     Ok(IdentifierType::Global)
/// );
/// assert_eq!(
///     "$USE_SPINOSO_SYMBOL".parse::<IdentifierType>(),
///     Ok(IdentifierType::Global)
/// );
/// ```
///
/// # Examples – instance and class variables
///
/// ```
/// # use spinoso_symbol::IdentifierType;
/// assert_eq!(
///     "@artichoke".parse::<IdentifierType>(),
///     Ok(IdentifierType::Instance)
/// );
/// assert_eq!(
///     "@@rumble".parse::<IdentifierType>(),
///     Ok(IdentifierType::Class)
/// );
/// ```
///
/// # Example – attribute setter
///
/// Attribute setters are local idents that end in `=`.
///
/// ```
/// # use spinoso_symbol::IdentifierType;
/// assert_eq!(
///     "artichoke=".parse::<IdentifierType>(),
///     Ok(IdentifierType::AttrSet)
/// );
/// assert_eq!(
///     "spinoso_symbol=".parse::<IdentifierType>(),
///     Ok(IdentifierType::AttrSet)
/// );
/// ```
///
/// [`Inspect`]: crate::Inspect
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdentifierType {
    /// Identifier that contains "junk".
    ///
    /// Junk mostly equates to non-sigil ASCII symbols. Identifiers like
    /// `empty?` and `flatten!` are junk idents. All special symbolic Ruby
    /// methods like `<=>` and `!~` are junk identifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::IdentifierType;
    /// assert_eq!("empty?".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// assert_eq!(
    ///     "flatten!".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Junk)
    /// );
    /// assert_eq!("<=>".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// assert_eq!("!~".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// assert_eq!("[]".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// assert_eq!("[]=".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// assert_eq!("=~".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// assert_eq!("==".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// assert_eq!("===".parse::<IdentifierType>(), Ok(IdentifierType::Junk));
    /// ```
    Junk,
    /// Identifier that is a global variable name.
    ///
    /// Global variables are prefixed with the sigil `$`. There are two types of
    /// global variables:
    ///
    /// - `$` followed by a `IdentifierType::Ident` sequence.
    /// - Special global variables, which include `Regexp` globals (`$1`..`$9`)
    ///   and `$-w` type globals.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
    /// assert_eq!(
    ///     "$".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!("$foo".parse::<IdentifierType>(), Ok(IdentifierType::Global));
    /// assert_eq!(
    ///     "$@foo".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!("$0".parse::<IdentifierType>(), Ok(IdentifierType::Global));
    /// assert_eq!("$1".parse::<IdentifierType>(), Ok(IdentifierType::Global));
    /// assert_eq!("$9".parse::<IdentifierType>(), Ok(IdentifierType::Global));
    /// assert_eq!("$-w".parse::<IdentifierType>(), Ok(IdentifierType::Global));
    /// assert_eq!(
    ///     "$-www".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// ```
    Global,
    /// Identifier that is an instance variable name.
    ///
    /// Instance variables are prefixed with a single `@` sigil. The remaining
    /// bytes must be a valid [`Constant`] or [`Local`] ident.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
    /// assert_eq!(
    ///     "@".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!(
    ///     "@foo".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Instance)
    /// );
    /// assert_eq!(
    ///     "@Foo".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Instance)
    /// );
    /// assert_eq!(
    ///     "@FOO".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Instance)
    /// );
    /// assert_eq!(
    ///     "@foo_bar".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Instance)
    /// );
    /// assert_eq!(
    ///     "@FooBar".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Instance)
    /// );
    /// assert_eq!(
    ///     "@FOO_BAR".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Instance)
    /// );
    /// assert_eq!(
    ///     "@$foo".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!(
    ///     "@0".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!(
    ///     "@1".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!(
    ///     "@9".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// ```
    ///
    /// [`Constant`]: Self::Constant
    /// [`Local`]: Self::Local
    Instance,
    /// Identifier that is a class variable name.
    ///
    /// Class variables are prefixed with a double `@@` sigil. The remaining
    /// bytes must be a valid [`Constant`] or [`Local`] ident.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
    /// assert_eq!(
    ///     "@@".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!("@@foo".parse::<IdentifierType>(), Ok(IdentifierType::Class));
    /// assert_eq!("@@Foo".parse::<IdentifierType>(), Ok(IdentifierType::Class));
    /// assert_eq!("@@FOO".parse::<IdentifierType>(), Ok(IdentifierType::Class));
    /// assert_eq!(
    ///     "@@foo_bar".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Class)
    /// );
    /// assert_eq!(
    ///     "@@FooBar".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Class)
    /// );
    /// assert_eq!(
    ///     "@@FOO_BAR".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Class)
    /// );
    /// assert_eq!(
    ///     "@@$foo".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!(
    ///     "@@0".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!(
    ///     "@@1".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!(
    ///     "@@9".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// ```
    ///
    /// [`Constant`]: Self::Constant
    /// [`Local`]: Self::Local
    Class,
    /// Identifier that is an "attribute setter" method name.
    ///
    /// `AttrSet` idents end in the `=` sigil and are otherwise valid [`Local`]
    /// or [`Constant`] idents. `AttrSet` idents cannot have any other "junk"
    /// symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
    /// assert_eq!(
    ///     "Foo=".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::AttrSet)
    /// );
    /// assert_eq!(
    ///     "foo=".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::AttrSet)
    /// );
    /// assert_eq!(
    ///     "foo_bar=".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::AttrSet)
    /// );
    /// assert_eq!(
    ///     "foo_bar?=".parse::<IdentifierType>(),
    ///     Err(ParseIdentifierError::new())
    /// );
    /// assert_eq!("ω=".parse::<IdentifierType>(), Ok(IdentifierType::AttrSet));
    /// ```
    ///
    /// [`Constant`]: Self::Constant
    /// [`Local`]: Self::Local
    AttrSet,
    /// Identifier that is a constant name.
    ///
    /// Constant names can be either ASCII or Unicode and must start with a
    /// uppercase character.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
    /// assert_eq!(
    ///     "Foo".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Constant)
    /// );
    /// assert_eq!(
    ///     "FOO".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Constant)
    /// );
    /// assert_eq!(
    ///     "FooBar".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Constant)
    /// );
    /// assert_eq!(
    ///     "FOO_BAR".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Constant)
    /// );
    /// assert_eq!("Ω".parse::<IdentifierType>(), Ok(IdentifierType::Constant));
    /// ```
    Constant,
    /// Identifier that is a local variable or method name.
    ///
    /// Local names can be either ASCII or Unicode and must start with a
    /// lowercase character.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
    /// assert_eq!("foo".parse::<IdentifierType>(), Ok(IdentifierType::Local));
    /// assert_eq!("fOO".parse::<IdentifierType>(), Ok(IdentifierType::Local));
    /// assert_eq!(
    ///     "fooBar".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Local)
    /// );
    /// assert_eq!(
    ///     "foo_bar".parse::<IdentifierType>(),
    ///     Ok(IdentifierType::Local)
    /// );
    /// assert_eq!("ω".parse::<IdentifierType>(), Ok(IdentifierType::Local));
    /// ```
    Local,
}

impl IdentifierType {
    /// Return a new, default `IdentifierType`.
    ///
    /// Prefer to use `new()` over `default()` since `new()` is const.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::IdentifierType;
    /// const ID_TYPE: IdentifierType = IdentifierType::new();
    /// assert_eq!(ID_TYPE, IdentifierType::Junk);
    /// assert_eq!(ID_TYPE, IdentifierType::default());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self::Junk
    }
}

impl Default for IdentifierType {
    /// Construct a "junk" identifier type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::IdentifierType;
    /// const ID_TYPE: IdentifierType = IdentifierType::new();
    /// assert_eq!(ID_TYPE, IdentifierType::Junk);
    /// assert_eq!(ID_TYPE, IdentifierType::default());
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for IdentifierType {
    type Err = ParseIdentifierError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s.as_bytes()).ok_or_else(ParseIdentifierError::new)
    }
}

impl TryFrom<&str> for IdentifierType {
    type Error = ParseIdentifierError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse(value.as_bytes()).ok_or_else(ParseIdentifierError::new)
    }
}

impl TryFrom<&[u8]> for IdentifierType {
    type Error = ParseIdentifierError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse(value).ok_or_else(ParseIdentifierError::new)
    }
}

/// Error type returned from the [`FromStr`] implementation on [`IdentifierType`].
///
/// # Examples
///
/// ```
/// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
/// const ERR: ParseIdentifierError = ParseIdentifierError::new();
/// assert_eq!("not a valid ident".parse::<IdentifierType>(), Err(ERR));
/// ```
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseIdentifierError {
    _private: (),
}

impl ParseIdentifierError {
    /// Construct a new `ParseIdentifierError`.
    ///
    /// Prefer to use `new()` over `default()` since `new()` is const.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::{IdentifierType, ParseIdentifierError};
    /// const ERR: ParseIdentifierError = ParseIdentifierError::new();
    /// assert_eq!("not a valid ident".parse::<IdentifierType>(), Err(ERR));
    /// assert_eq!(ERR, ParseIdentifierError::default());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl fmt::Display for ParseIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to parse given string as a known identifier type")
    }
}

#[inline]
fn parse(name: &[u8]) -> Option<IdentifierType> {
    match name {
        [] | [b'\0'] => None,
        // special global variable
        [b'$', name @ ..] if is_special_global_name(name) => Some(IdentifierType::Global),
        // global variable
        [b'$', name @ ..] => parse_ident(name, IdentifierType::Global),
        // class variable
        [b'@', b'@', name @ ..] => parse_ident(name, IdentifierType::Class),
        // instance variable
        [b'@', name @ ..] => parse_ident(name, IdentifierType::Instance),
        // Symbolic method names
        name if is_symbolic_method_name(name) => Some(IdentifierType::Junk),
        [b'=' | b'!' | b'[', ..] => None,
        [first, ..] if *first != b'_' && first.is_ascii() && !first.is_ascii_alphabetic() => None,
        // Constant name
        name if is_const_name(name) => parse_ident(name, IdentifierType::Constant),
        // Local variable
        name => parse_ident(name, IdentifierType::Local),
    }
}

#[inline]
fn parse_ident(name: &[u8], id_type: IdentifierType) -> Option<IdentifierType> {
    match name {
        [] => None,
        [first, name @ .., b'='] if *first != b'_' && first.is_ascii() && !first.is_ascii_alphabetic() => {
            if let None | Some(IdentifierType::AttrSet) = parse_ident(name, id_type) {
                None
            } else {
                Some(id_type)
            }
        }
        [first, ..] if *first != b'_' && first.is_ascii() && !first.is_ascii_alphabetic() => None,
        name if is_ident_until(name).is_none() => Some(id_type),
        [name @ .., b'!' | b'?'] if is_ident_until(name).is_none() => {
            if matches!(
                id_type,
                IdentifierType::Global | IdentifierType::Class | IdentifierType::Instance
            ) {
                return None;
            }
            Some(IdentifierType::Junk)
        }
        [name @ .., b'='] if is_ident_until(name).is_none() => {
            if matches!(id_type, IdentifierType::Local | IdentifierType::Constant) {
                return Some(IdentifierType::AttrSet);
            }
            None
        }
        _ => None,
    }
}

#[inline]
#[allow(clippy::match_same_arms)] // for clarity
fn is_special_global_name(name: &[u8]) -> bool {
    match name {
        [] => false,
        [first, rest @ ..] if is_special_global_punct(*first) => rest.is_empty(),
        [b'-'] => false,
        [b'-', rest @ ..] if is_next_ident_exhausting(rest) => true,
        [b'-', ..] => false,
        name => name.chars().all(char::is_numeric),
    }
}

/// Return whether the input is a "junk" symbolic method name.
///
/// There are fixed number of valid Ruby method names that only contain ASCII
/// symbols.
#[inline]
fn is_symbolic_method_name(name: &[u8]) -> bool {
    matches!(
        name,
        b"<" | b"<<"
            | b"<="
            | b"<=>"
            | b">"
            | b">>"
            | b">="
            | b"=~"
            | b"=="
            | b"==="
            | b"*"
            | b"**"
            | b"+"
            | b"-"
            | b"+@"
            | b"-@"
            | b"|"
            | b"^"
            | b"&"
            | b"/"
            | b"%"
            | b"~"
            | b"`"
            | b"[]"
            | b"[]="
            | b"!"
            | b"!="
            | b"!~"
    )
}

/// Return whether the input is a valid constant name.
///
/// Constant names require the first character to be either ASCII or Unicode
/// uppercase.
#[inline]
fn is_const_name(name: &[u8]) -> bool {
    match name {
        [] => false,
        name if name.is_ascii() => name.iter().next().map(u8::is_ascii_uppercase).unwrap_or_default(),
        name if name.is_utf8() => name
            .chars()
            .next()
            .map(char::is_uppercase) // uses Unicode `Uppercase` property
            .unwrap_or_default(),
        _ => false,
    }
}

/// Determine if a [`char`] can be used in a valid identifier.
///
/// # Header declaration
///
/// Ported from the following C macro in `string.c`:
///
/// ```c
/// #define is_identchar(p,e,enc) (ISALNUM((unsigned char)*(p)) || (*(p)) == '_' || !ISASCII(*(p)))
/// ```
#[inline]
fn is_ident_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || !ch.is_ascii()
}

/// Consume the input until a non-ident character is found.
///
/// Scan the [`char`]s in the input until either invalid UTF-8 or an invalid
/// ident is found. See [`is_ident_char`].
///
/// This method returns `Some(index)` of the start of the first invalid ident
/// or `None` if the whole input is a valid ident.
///
/// Empty slices are not valid idents.
#[inline]
fn is_ident_until(mut name: &[u8]) -> Option<usize> {
    // Empty strings are not idents.
    if name.is_empty() {
        return Some(0);
    }
    let mut start = 0;
    while !name.is_empty() {
        let (ch, size) = bstr::decode_utf8(name);
        match ch {
            Some(ch) if !is_ident_char(ch) => return Some(start),
            None => return Some(start),
            Some(_) => {
                name = &name[size..];
                start += size;
            }
        }
    }
    None
}

/// Determine if the next char is a valid ident char and consumes all bytes in
/// the input.
///
/// This function is used to determine whether certain kinds of single character
/// globals are valid idents.
///
/// See also [`is_ident_char`].
#[inline]
fn is_next_ident_exhausting(name: &[u8]) -> bool {
    let (ch, size) = bstr::decode_utf8(name);
    match ch {
        Some(ch) if is_ident_char(ch) => name.len() == size,
        Some(_) | None => false,
    }
}

// This function is defined by a macro in `parse.y` in MRI.
//
// ```c
// #define BIT(c, idx) (((c) / 32 - 1 == idx) ? (1U << ((c) % 32)) : 0)
// #define SPECIAL_PUNCT(idx) ( \
// 	BIT('~', idx) | BIT('*', idx) | BIT('$', idx) | BIT('?', idx) | \
// 	BIT('!', idx) | BIT('@', idx) | BIT('/', idx) | BIT('\\', idx) | \
// 	BIT(';', idx) | BIT(',', idx) | BIT('.', idx) | BIT('=', idx) | \
// 	BIT(':', idx) | BIT('<', idx) | BIT('>', idx) | BIT('\"', idx) | \
// 	BIT('&', idx) | BIT('`', idx) | BIT('\'', idx) | BIT('+', idx) | \
// 	BIT('0', idx))
// const unsigned int ruby_global_name_punct_bits[] = {
//     SPECIAL_PUNCT(0),
//     SPECIAL_PUNCT(1),
//     SPECIAL_PUNCT(2),
// };
// ```
//
// The contents of `ruby_global_name_punct_bits` are:
//
// ```console
// [2.6.6] > def bit(c, idx); c / 32 - 1 == idx ? 1 << (c % 32) : 0; end
// [2.6.6] > chars = ["~", "*", "$", "?", "!", "@", "/", "\\", ";", ",", ".", "=", ":", "<", ">", "\"", "&", "`", "'", "+", "0"]
//
// [2.6.6] > chars.map(&:ord).map { |ch| bit(ch, 0) }.reduce(0, :|)
// => 4227980502
// [2.6.6] > chars.map(&:ord).map { |ch| bit(ch, 1) }.reduce(0, :|)
// => 268435457
// [2.6.6] > chars.map(&:ord).map { |ch| bit(ch, 2) }.reduce(0, :|)
// => 1073741825
// ```
//
// Which corresponds to a fixed set of 21 ASCII symbols:
//
// ```ruby
// def is_special_global_punct(ch)
//   idx = (ch - 0x20) / 32;
//   case idx
//   when 0 then (4_227_980_502 >> (ch % 32)) & 1 > 0
//   when 1 then (268_435_457 >> (ch % 32)) & 1 > 0
//   when 2 then (1_073_741_825 >> (ch % 32)) & 1 > 0
//   else
//     false
//   end
// end
//
// h = {}
// (0..255).each do |ch|
//   h[ch.chr] = ch if is_special_global_punct(ch)
// end
// h.keys.map {|k| "b'#{k.inspect[1..-2]}'"}.join(" | ")
// ```
//
// TODO: Switch to generating this table inside the const function once const
// functions are expressive enough. This requires const `match`, `if`, and loop
// which will be stable in Rust 1.46.0.
#[inline]
fn is_special_global_punct(ch: u8) -> bool {
    matches!(
        ch,
        b'!' | b'"'
            | b'$'
            | b'&'
            | b'\''
            | b'*'
            | b'+'
            | b','
            | b'.'
            | b'/'
            | b'0'
            | b':'
            | b';'
            | b'<'
            | b'='
            | b'>'
            | b'?'
            | b'@'
            | b'\\'
            | b'`'
            | b'~'
    )
}

#[cfg(test)]
mod tests {
    use super::{
        is_ident_until, is_next_ident_exhausting, is_special_global_name, IdentifierType, ParseIdentifierError,
    };

    #[test]
    fn special_global_name() {
        let name = &b"a"[..];
        assert!(!is_special_global_name(name));
        let name = "💎";
        assert!(!is_special_global_name(name.as_bytes()));
        let name = &b"ab"[..];
        assert!(!is_special_global_name(name));
        let name = "-💎";
        assert!(is_special_global_name(name.as_bytes()));
        let name = &b"$"[..];
        assert!(is_special_global_name(name));
        let name = &b"~"[..];
        assert!(is_special_global_name(name));
        let name = "�";
        assert!(!is_special_global_name(name.as_bytes()));
        let name = "-�";
        assert!(is_special_global_name(name.as_bytes()));
    }

    #[test]
    fn is_ident_until_empty() {
        let name = &[];
        assert_eq!(is_ident_until(name), Some(0));
    }

    #[test]
    fn is_ident_until_lowercase_ascii() {
        let name = &b"abc"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"abc_123"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"_"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"_e"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"_1"[..];
        assert_eq!(is_ident_until(name), None);
    }

    #[test]
    fn is_ident_until_ascii_constant() {
        let name = &b"Abc"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"ABC"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"ABC_XYZ"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"ABC_123"[..];
        assert_eq!(is_ident_until(name), None);
        let name = &b"HTTP2"[..];
        assert_eq!(is_ident_until(name), None);
    }

    #[test]
    fn is_ident_until_unicode() {
        let name = "ábc";
        assert_eq!(is_ident_until(name.as_bytes()), None);
        let name = "abç";
        assert_eq!(is_ident_until(name.as_bytes()), None);
        let name = "abc_�";
        assert_eq!(is_ident_until(name.as_bytes()), None);
        let name = "abc_💎";
        assert_eq!(is_ident_until(name.as_bytes()), None);

        let name = "Ábc";
        assert_eq!(is_ident_until(name.as_bytes()), None);
        let name = "Abç";
        assert_eq!(is_ident_until(name.as_bytes()), None);
        let name = "Abc_�";
        assert_eq!(is_ident_until(name.as_bytes()), None);
        let name = "Abc_💎";
        assert_eq!(is_ident_until(name.as_bytes()), None);

        let name = "💎";
        assert_eq!(is_ident_until(name.as_bytes()), None);
        let name = "💎abc";
        assert_eq!(is_ident_until(name.as_bytes()), None);
    }

    #[test]
    fn is_ident_until_invalid_utf8() {
        let name = &b"\xFF"[..];
        assert_eq!(is_ident_until(name), Some(0));
        let name = &b"abc\xFF"[..];
        assert_eq!(is_ident_until(name), Some(3));
        let name = &b"abc\xFFxyz"[..];
        assert_eq!(is_ident_until(name), Some(3));

        let name = &b"\xFF\xFE"[..];
        assert_eq!(is_ident_until(name), Some(0));
        let name = &b"abc\xFF\xFE"[..];
        assert_eq!(is_ident_until(name), Some(3));
        let name = &b"abc\xFF\xFExyz"[..];
        assert_eq!(is_ident_until(name), Some(3));

        let name = &b"\xEF\xBF\xBD\xFF"[..];
        assert_eq!(is_ident_until(name), Some(3));
        let name = &b"\xF0\x9F\x92\x8E\xFF"[..];
        assert_eq!(is_ident_until(name), Some(4));
    }

    #[test]
    fn is_next_ident_exhausting_empty() {
        let name = &[];
        assert!(!is_next_ident_exhausting(name));
    }

    #[test]
    fn is_next_ident_exhausting_lowercase_ascii() {
        let name = &b"a"[..];
        assert!(is_next_ident_exhausting(name));
        let name = &b"abc"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"1"[..];
        assert!(is_next_ident_exhausting(name));
        let name = &b"abc_123"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"_"[..];
        assert!(is_next_ident_exhausting(name));
        let name = &b"_e"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"_1"[..];
        assert!(!is_next_ident_exhausting(name));
    }

    #[test]
    fn is_next_ident_exhausting_ascii_constant() {
        let name = &b"A"[..];
        assert!(is_next_ident_exhausting(name));
        let name = &b"Abc"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"ABC"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"ABC_XYZ"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"ABC_123"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"HTTP2"[..];
        assert!(!is_next_ident_exhausting(name));
    }

    #[test]
    fn is_next_ident_exhausting_unicode() {
        let name = "ábc";
        assert!(!is_next_ident_exhausting(name.as_bytes()));
        let name = "abç";
        assert!(!is_next_ident_exhausting(name.as_bytes()));
        let name = "abc_�";
        assert!(!is_next_ident_exhausting(name.as_bytes()));
        let name = "abc_💎";
        assert!(!is_next_ident_exhausting(name.as_bytes()));

        let name = "Ábc";
        assert!(!is_next_ident_exhausting(name.as_bytes()));
        let name = "Abç";
        assert!(!is_next_ident_exhausting(name.as_bytes()));
        let name = "Abc_�";
        assert!(!is_next_ident_exhausting(name.as_bytes()));
        let name = "Abc_💎";
        assert!(!is_next_ident_exhausting(name.as_bytes()));
        let name = "💎abc";
        assert!(!is_next_ident_exhausting(name.as_bytes()));

        let name = "á";
        assert!(is_next_ident_exhausting(name.as_bytes()));
        let name = "ç";
        assert!(is_next_ident_exhausting(name.as_bytes()));
        let name = "�";
        assert!(is_next_ident_exhausting(name.as_bytes()));
        let name = "💎";
        assert!(is_next_ident_exhausting(name.as_bytes()));
    }

    #[test]
    fn is_next_ident_exhausting_invalid_utf8() {
        let name = &b"\xFF"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"abc\xFF"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"abc\xFFxyz"[..];
        assert!(!is_next_ident_exhausting(name));

        let name = &b"\xFF\xFE"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"abc\xFF\xFE"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"abc\xFF\xFExyz"[..];
        assert!(!is_next_ident_exhausting(name));

        let name = &b"\xEF\xBF\xBD\xFF"[..];
        assert!(!is_next_ident_exhausting(name));
        let name = &b"\xF0\x9F\x92\x8E\xFF"[..];
        assert!(!is_next_ident_exhausting(name));
    }

    #[test]
    fn ascii_ident() {
        assert_eq!("foobar".parse::<IdentifierType>(), Ok(IdentifierType::Local));
        assert_eq!("ruby_is_simple".parse::<IdentifierType>(), Ok(IdentifierType::Local));
    }

    #[test]
    fn ascii_constant() {
        assert_eq!("Foobar".parse::<IdentifierType>(), Ok(IdentifierType::Constant));
        assert_eq!("FooBar".parse::<IdentifierType>(), Ok(IdentifierType::Constant));
        assert_eq!("FOOBAR".parse::<IdentifierType>(), Ok(IdentifierType::Constant));
        assert_eq!("FOO_BAR".parse::<IdentifierType>(), Ok(IdentifierType::Constant));
        assert_eq!("RUBY_IS_SIMPLE".parse::<IdentifierType>(), Ok(IdentifierType::Constant));
    }

    #[test]
    fn empty() {
        assert_eq!("".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
    }

    #[test]
    fn single_nul() {
        assert_eq!("\0".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
    }

    #[test]
    fn non_ascii_numerics() {
        assert_eq!("١".parse::<IdentifierType>(), Ok(IdentifierType::Local));
        assert_eq!(
            "١١١١١١١١١١١١١١١١١١".parse::<IdentifierType>(),
            Ok(IdentifierType::Local)
        );
        assert_eq!("①".parse::<IdentifierType>(), Ok(IdentifierType::Local));
    }

    #[test]
    fn recursive_ident() {
        assert_eq!("@@@foo".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@@@@foo".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@$foo".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@$-w".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@@$foo".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@@$-w".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("$@foo".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("$@@foo".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("$$-w".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
    }

    #[test]
    fn attr_bang() {
        assert_eq!("@foo!".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@@foo!".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("$foo!".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
    }

    #[test]
    fn attr_question() {
        assert_eq!("@foo?".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@@foo?".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("$foo?".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
    }

    #[test]
    fn attr_setter() {
        assert_eq!("@foo=".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("@@foo=".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
        assert_eq!("$foo=".parse::<IdentifierType>(), Err(ParseIdentifierError::new()));
    }

    #[test]
    fn invalid_utf8() {
        assert_eq!(
            IdentifierType::try_from(&b"invalid-\xFF-utf8"[..]),
            Err(ParseIdentifierError::new())
        );
    }

    #[test]
    fn emoji() {
        assert_eq!(IdentifierType::try_from("💎"), Ok(IdentifierType::Local));
        assert_eq!(IdentifierType::try_from("$💎"), Ok(IdentifierType::Global));
        assert_eq!(IdentifierType::try_from("@💎"), Ok(IdentifierType::Instance));
        assert_eq!(IdentifierType::try_from("@@💎"), Ok(IdentifierType::Class));
    }

    #[test]
    fn unicode_replacement_char() {
        assert_eq!(IdentifierType::try_from("�"), Ok(IdentifierType::Local));
        assert_eq!(IdentifierType::try_from("$�"), Ok(IdentifierType::Global));
        assert_eq!(IdentifierType::try_from("@�"), Ok(IdentifierType::Instance));
        assert_eq!(IdentifierType::try_from("@@�"), Ok(IdentifierType::Class));

        assert_eq!(IdentifierType::try_from("abc�"), Ok(IdentifierType::Local));
        assert_eq!(IdentifierType::try_from("$abc�"), Ok(IdentifierType::Global));
        assert_eq!(IdentifierType::try_from("@abc�"), Ok(IdentifierType::Instance));
        assert_eq!(IdentifierType::try_from("@@abc�"), Ok(IdentifierType::Class));
    }

    #[test]
    fn invalid_utf8_special_global() {
        assert_eq!(
            IdentifierType::try_from(&b"$-\xFF"[..]),
            Err(ParseIdentifierError::new())
        );
    }

    #[test]
    fn replacement_char_special_global() {
        assert_eq!(IdentifierType::try_from("$-�"), Ok(IdentifierType::Global));
        assert_eq!(IdentifierType::try_from("$-�a"), Err(ParseIdentifierError::new()));
        assert_eq!(IdentifierType::try_from("$-��"), Err(ParseIdentifierError::new()));
    }
}

#[cfg(test)]
mod specs {
    use super::IdentifierType;

    // From `spec/core/symbol/inspect_spec.rb`:
    //
    // ```ruby
    // symbols = {
    //   fred:         ":fred",
    //   :fred?     => ":fred?",
    //   :fred!     => ":fred!",
    //   :$ruby     => ":$ruby",
    //   :@ruby     => ":@ruby",
    //   :@@ruby    => ":@@ruby",
    //   :"$ruby!"  => ":\"$ruby!\"",
    //   :"$ruby?"  => ":\"$ruby?\"",
    //   :"@ruby!"  => ":\"@ruby!\"",
    //   :"@ruby?"  => ":\"@ruby?\"",
    //   :"@@ruby!" => ":\"@@ruby!\"",
    //   :"@@ruby?" => ":\"@@ruby?\"",
    //
    //   :$-w       => ":$-w",
    //   :"$-ww"    => ":\"$-ww\"",
    //   :"$+"      => ":$+",
    //   :"$~"      => ":$~",
    //   :"$:"      => ":$:",
    //   :"$?"      => ":$?",
    //   :"$<"      => ":$<",
    //   :"$_"      => ":$_",
    //   :"$/"      => ":$/",
    //   :"$'"      => ":$'",
    //   :"$\""     => ":$\"",
    //   :"$$"      => ":$$",
    //   :"$."      => ":$.",
    //   :"$,"      => ":$,",
    //   :"$`"      => ":$`",
    //   :"$!"      => ":$!",
    //   :"$;"      => ":$;",
    //   :"$\\"     => ":$\\",
    //   :"$="      => ":$=",
    //   :"$*"      => ":$*",
    //   :"$>"      => ":$>",
    //   :"$&"      => ":$&",
    //   :"$@"      => ":$@",
    //   :"$1234"   => ":$1234",
    //
    //   :-@        => ":-@",
    //   :+@        => ":+@",
    //   :%         => ":%",
    //   :&         => ":&",
    //   :*         => ":*",
    //   :**        => ":**",
    //   :"/"       => ":/",     # lhs quoted for emacs happiness
    //   :<         => ":<",
    //   :<=        => ":<=",
    //   :<=>       => ":<=>",
    //   :==        => ":==",
    //   :===       => ":===",
    //   :=~        => ":=~",
    //   :>         => ":>",
    //   :>=        => ":>=",
    //   :>>        => ":>>",
    //   :[]        => ":[]",
    //   :[]=       => ":[]=",
    //   :"\<\<"    => ":\<\<",
    //   :^         => ":^",
    //   :"`"       => ":`",     # for emacs, and justice!
    //   :~         => ":~",
    //   :|         => ":|",
    //
    //   :"!"       => [":\"!\"",  ":!" ],
    //   :"!="      => [":\"!=\"", ":!="],
    //   :"!~"      => [":\"!~\"", ":!~"],
    //   :"\$"      => ":\"$\"", # for justice!
    //   :"&&"      => ":\"&&\"",
    //   :"'"       => ":\"\'\"",
    //   :","       => ":\",\"",
    //   :"."       => ":\".\"",
    //   :".."      => ":\"..\"",
    //   :"..."     => ":\"...\"",
    //   :":"       => ":\":\"",
    //   :"::"      => ":\"::\"",
    //   :";"       => ":\";\"",
    //   :"="       => ":\"=\"",
    //   :"=>"      => ":\"=>\"",
    //   :"\?"      => ":\"?\"", # rawr!
    //   :"@"       => ":\"@\"",
    //   :"||"      => ":\"||\"",
    //   :"|||"     => ":\"|||\"",
    //   :"++"      => ":\"++\"",
    //
    //   :"\""      => ":\"\\\"\"",
    //   :"\"\""    => ":\"\\\"\\\"\"",
    //
    //   :"9"       => ":\"9\"",
    //   :"foo bar" => ":\"foo bar\"",
    //   :"*foo"    => ":\"*foo\"",
    //   :"foo "    => ":\"foo \"",
    //   :" foo"    => ":\" foo\"",
    //   :" "       => ":\" \"",
    // }
    // ```

    #[test]
    fn specs() {
        // idents
        assert!("fred".parse::<IdentifierType>().is_ok());
        assert!("fred?".parse::<IdentifierType>().is_ok());
        assert!("fred!".parse::<IdentifierType>().is_ok());
        assert!("$ruby".parse::<IdentifierType>().is_ok());
        assert!("@ruby".parse::<IdentifierType>().is_ok());
        assert!("@@ruby".parse::<IdentifierType>().is_ok());

        // idents can't end in bang or question
        assert!("$ruby!".parse::<IdentifierType>().is_err());
        assert!("$ruby?".parse::<IdentifierType>().is_err());
        assert!("@ruby!".parse::<IdentifierType>().is_err());
        assert!("@ruby?".parse::<IdentifierType>().is_err());
        assert!("@@ruby!".parse::<IdentifierType>().is_err());
        assert!("@@ruby?".parse::<IdentifierType>().is_err());

        // globals
        assert!("$-w".parse::<IdentifierType>().is_ok());
        assert!("$-ww".parse::<IdentifierType>().is_err());
        assert!("$+".parse::<IdentifierType>().is_ok());
        assert!("$~".parse::<IdentifierType>().is_ok());
        assert!("$:".parse::<IdentifierType>().is_ok());
        assert!("$?".parse::<IdentifierType>().is_ok());
        assert!("$<".parse::<IdentifierType>().is_ok());
        assert!("$_".parse::<IdentifierType>().is_ok());
        assert!("$/".parse::<IdentifierType>().is_ok());
        assert!("$\"".parse::<IdentifierType>().is_ok());
        assert!("$$".parse::<IdentifierType>().is_ok());
        assert!("$.".parse::<IdentifierType>().is_ok());
        assert!("$,".parse::<IdentifierType>().is_ok());
        assert!("$`".parse::<IdentifierType>().is_ok());
        assert!("$!".parse::<IdentifierType>().is_ok());
        assert!("$;".parse::<IdentifierType>().is_ok());
        assert!("$\\".parse::<IdentifierType>().is_ok());
        assert!("$=".parse::<IdentifierType>().is_ok());
        assert!("$*".parse::<IdentifierType>().is_ok());
        assert!("$>".parse::<IdentifierType>().is_ok());
        assert!("$&".parse::<IdentifierType>().is_ok());
        assert!("$@".parse::<IdentifierType>().is_ok());
        assert!("$1234".parse::<IdentifierType>().is_ok());

        // symbolic methods
        assert!("-@".parse::<IdentifierType>().is_ok());
        assert!("+@".parse::<IdentifierType>().is_ok());
        assert!("%".parse::<IdentifierType>().is_ok());
        assert!("&".parse::<IdentifierType>().is_ok());
        assert!("*".parse::<IdentifierType>().is_ok());
        assert!("**".parse::<IdentifierType>().is_ok());
        assert!("/".parse::<IdentifierType>().is_ok());
        assert!("<".parse::<IdentifierType>().is_ok());
        assert!("<=".parse::<IdentifierType>().is_ok());
        assert!("<=>".parse::<IdentifierType>().is_ok());
        assert!("==".parse::<IdentifierType>().is_ok());
        assert!("===".parse::<IdentifierType>().is_ok());
        assert!("=~".parse::<IdentifierType>().is_ok());
        assert!(">".parse::<IdentifierType>().is_ok());
        assert!(">=".parse::<IdentifierType>().is_ok());
        assert!(">>".parse::<IdentifierType>().is_ok());
        assert!("[]".parse::<IdentifierType>().is_ok());
        assert!("[]=".parse::<IdentifierType>().is_ok());
        assert!("<<".parse::<IdentifierType>().is_ok());
        assert!("^".parse::<IdentifierType>().is_ok());
        assert!("`".parse::<IdentifierType>().is_ok());
        assert!("~".parse::<IdentifierType>().is_ok());
        assert!("|".parse::<IdentifierType>().is_ok());

        // non-symbol symbolics
        assert!("!".parse::<IdentifierType>().is_ok());
        assert!("!=".parse::<IdentifierType>().is_ok());
        assert!("!~".parse::<IdentifierType>().is_ok());
        assert!("$".parse::<IdentifierType>().is_err());
        assert!("&&".parse::<IdentifierType>().is_err());
        assert!("'".parse::<IdentifierType>().is_err());
        assert!(",".parse::<IdentifierType>().is_err());
        assert!(".".parse::<IdentifierType>().is_err());
        assert!("..".parse::<IdentifierType>().is_err());
        assert!("...".parse::<IdentifierType>().is_err());
        assert!(":".parse::<IdentifierType>().is_err());
        assert!("::".parse::<IdentifierType>().is_err());
        assert!(";".parse::<IdentifierType>().is_err());
        assert!("=".parse::<IdentifierType>().is_err());
        assert!("=>".parse::<IdentifierType>().is_err());
        assert!("?".parse::<IdentifierType>().is_err());
        assert!("@".parse::<IdentifierType>().is_err());
        assert!("||".parse::<IdentifierType>().is_err());
        assert!("|||".parse::<IdentifierType>().is_err());
        assert!("++".parse::<IdentifierType>().is_err());

        // quotes
        assert!(r#"""#.parse::<IdentifierType>().is_err());
        assert!(r#""""#.parse::<IdentifierType>().is_err());

        assert!("9".parse::<IdentifierType>().is_err());
        assert!("foo bar".parse::<IdentifierType>().is_err());
        assert!("*foo".parse::<IdentifierType>().is_err());
        assert!("foo ".parse::<IdentifierType>().is_err());
        assert!(" foo".parse::<IdentifierType>().is_err());
        assert!(" ".parse::<IdentifierType>().is_err());
    }
}

/// Tests generated from symbols loaded at MRI interpreter boot.
///
/// # Generation
///
/// ```shell
/// cat <<EOF | ruby --disable-gems --disable-did_you_mean
/// def boot_identifier_symbols
///   syms = Symbol.all_symbols.map(&:inspect)
///   # remove symbols that must be debug wrapped in quotes
///   syms = syms.reject { |s| s[0..1] == ':"' }
///
///   fixture = syms.map { |s| "r##\"#{s[1..]}\"##" }
///   puts fixture.join(",\n")
/// end
///
/// boot_identifier_symbols
/// EOF
/// ```
#[cfg(test)]
mod functionals {
    use super::IdentifierType;
    use crate::fixtures::IDENTS;

    #[test]
    fn mri_symbol_idents() {
        for &sym in IDENTS {
            assert!(
                sym.parse::<IdentifierType>().is_ok(),
                "'{}' should parse as a valid identifier, but did not.",
                sym
            );
        }
    }
}
