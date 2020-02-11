//! Intern [`Symbol`][symbol]s on an interpreter.
//!
//! `Symbol`s are immutable byte vectors that have the same lifetime as the
//! interpreter.
//!
//! [symbol]: https://ruby-doc.org/core-2.6.3/Symbol.html

use std::borrow::Cow;

/// Store and retrieve byte vectors that have the same lifetime as the
/// interpreter.
///
/// See the [Ruby `Symbol` type][symbol].
///
/// [symbol]: https://ruby-doc.org/core-2.6.3/Symbol.html
pub trait Intern {
    /// Concrete type for symbol identifiers.
    ///
    /// The symbol identifier enables lookups in the underlying storage.
    type Symbol: Copy;

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
