#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
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

//! Symbol type, etc.

// `spinoso-symbol` is a `no_std` crate unless the `std` feature is enabled.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
extern crate alloc;

#[cfg(feature = "artichoke")]
use artichoke_core::intern::Intern;
use core::fmt;
use core::mem::size_of;
use core::num::TryFromIntError;

#[doc(inline)]
#[cfg(feature = "artichoke")]
#[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
pub use focaccia::{CaseFold, NoSuchCaseFoldingScheme};

// Spinoso symbol assumes symbols are `u32` and requires `usize` to be at least
// as big as `u32`.
//
// This const-evaluated expression will fail to compile if this invariant does
// not hold.
const _: () = [()][!(size_of::<usize>() >= size_of::<u32>()) as usize];

#[cfg(feature = "artichoke")]
mod all_symbols;
#[cfg(feature = "artichoke")]
mod casecmp;
mod convert;
mod eq;
#[cfg(feature = "artichoke")]
mod inspect;

#[cfg(feature = "artichoke")]
pub use all_symbols::{AllSymbols, InternerAllSymbols};
#[cfg(feature = "artichoke")]
pub use casecmp::{ascii_casecmp, unicode_case_eq};
#[cfg(feature = "artichoke")]
pub use inspect::Inspect;

/// Error returned when a symbol identifier overflows.
///
/// Spinoso symbol uses `u32` identifiers for symbols to save space. If more
/// than `u32::MAX` symbols are stored in the underlying table, no more
/// identifiers can be generated.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolOverflowError {
    _private: (),
}

impl SymbolOverflowError {
    /// The maximum identifier of a `Symbol`.
    pub const MAX_CAPACITY: usize = u32::MAX as usize;

    /// Construct a new, default `SymbolOverflowError`.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl From<TryFromIntError> for SymbolOverflowError {
    #[inline]
    fn from(err: TryFromIntError) -> Self {
        let _ = err;
        Self::new()
    }
}

impl fmt::Display for SymbolOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Symbol overflow")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SymbolOverflowError {}

/// Identifier bound to an interned bytestring.
///
/// A `Symbol` allows retrieving a reference to the original interned
/// bytestring.  Equivalent `Symbol`s will resolve to an identical bytestring.
///
/// `Symbol`s are based on a `u32` index. They are cheap to compare and cheap to
/// copy.
///
/// `Symbol`s are not constrained to the interner which created them.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(u32);

impl Symbol {
    /// Construct a new `Symbol` from the given `u32`.
    ///
    /// `Symbol`s constructed manually may fail to resolve to an underlying
    /// bytesstring.
    ///
    /// `Symbol`s are not constrained to the interner which created them.
    /// No runtime checks ensure that the underlying interner is called with a
    /// `Symbol` that the interner itself issued.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::Symbol;
    /// let sym = Symbol::new(263);
    /// assert_eq!(sym.id(), 263);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Return the `u32` identifier from this `Symbol`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_symbol::Symbol;
    /// let sym = Symbol::new(263);
    /// assert_eq!(sym.id(), 263);
    /// assert_eq!(u32::from(sym), 263);
    /// ```
    #[inline]
    #[must_use]
    pub const fn id(self) -> u32 {
        self.0
    }

    /// Returns whether the symbol is the empty byteslice `b""` in the
    /// underlying interner.
    ///
    /// If there symbol does not exist in the underlying interner or there is an
    /// error looking up the symbol in the underlying interner, `true` is
    /// returned.
    #[inline]
    #[must_use]
    #[cfg(feature = "artichoke")]
    #[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
    pub fn is_empty<T, U>(self, interner: &T) -> bool
    where
        T: Intern<Symbol = U>,
        U: Copy + From<Symbol>,
    {
        if let Ok(Some(bytes)) = interner.lookup_symbol(self.into()) {
            bytes.is_empty()
        } else {
            true
        }
    }

    /// Returns the length of the byteslice associated with the symbol in the
    /// underlying interner.
    ///
    /// If there symbol does not exist in the underlying interner or there is an
    /// error looking up the symbol in the underlying interner, `0` is returned.
    #[inline]
    #[must_use]
    #[cfg(feature = "artichoke")]
    #[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
    pub fn len<T, U>(self, interner: &T) -> usize
    where
        T: Intern<Symbol = U>,
        U: Copy + From<Symbol>,
    {
        if let Ok(Some(bytes)) = interner.lookup_symbol(self.into()) {
            bytes.len()
        } else {
            0_usize
        }
    }

    /// Returns the interned byteslice associated with the symbol in the
    /// underlying interner.
    ///
    /// If there symbol does not exist in the underlying interner or there is an
    /// error looking up the symbol in the underlying interner, `&[]` is
    /// returned.
    #[inline]
    #[must_use]
    #[cfg(feature = "artichoke")]
    #[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
    pub fn bytes<'a, T, U>(&self, interner: &'a T) -> &'a [u8]
    where
        T: Intern<Symbol = U>,
        U: Copy + From<Symbol>,
    {
        let sym = *self;
        if let Ok(Some(bytes)) = interner.lookup_symbol(sym.into()) {
            bytes
        } else {
            &[]
        }
    }

    /// Returns an iterator that yields a debug representation of the interned
    /// byteslice associated with the symbol in the underlying interner.
    ///
    /// This iterator produces strings like `:spinoso` and `:invalid-\xFF-utf8`.
    ///
    /// If there symbol does not exist in the underlying interner or there is an
    /// error looking up the symbol in the underlying interner, a default
    /// iterator is returned.
    #[inline]
    #[must_use]
    #[cfg(feature = "artichoke")]
    #[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
    pub fn inspect<'a, T, U>(&self, interner: &'a T) -> Inspect<'a>
    where
        T: Intern<Symbol = U>,
        U: Copy + From<Symbol>,
    {
        let sym = *self;
        if let Ok(Some(bytes)) = interner.lookup_symbol(sym.into()) {
            Inspect::from(bytes)
        } else {
            Inspect::default()
        }
    }
}
