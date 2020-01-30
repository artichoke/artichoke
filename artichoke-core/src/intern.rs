//! Intern symbols on an Artichoke interpreter.
//!
//! Symbols are immutable byte vectors that have the same lifetime as the
//! interpreter.

use std::borrow::Cow;

/// Interpreters that implement [`Intern`] expose methods for storing and
/// retrieving byte content that lives for the life of the interpreter.
///
/// See the [Ruby `Symbol` type][symbol].
///
/// [symbol]: https://ruby-doc.org/core-2.6.3/Symbol.html
pub trait Intern {
    /// Concrete type for symbol identifiers.
    ///
    /// The symbol identifier enables lookups in the underlying storage.
    type Symbol;

    /// Store an immutable byte vector for the life of the interpreter.
    ///
    /// Returns an identifier that enables retrieving the original bytes.
    fn intern_symbol<T>(&mut self, symbol: T) -> Self::Symbol
    where
        T: Into<Cow<'static, [u8]>>;

    /// Retrieve the original byte content of an interned vector.
    ///
    /// Returns `None` if the symbol identifier is invalid.
    fn lookup_symbol(&self, symbol: Self::Symbol) -> Option<&[u8]>;
}
