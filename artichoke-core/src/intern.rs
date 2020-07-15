//! Intern [`Symbol`][symbol]s on an interpreter.
//!
//! `Symbol`s are immutable bytestrings that have the same lifetime as the
//! interpreter.
//!
//! [symbol]: https://ruby-doc.org/core-2.6.3/Symbol.html

use alloc::borrow::Cow;

/// Store and retrieve bytestrings that have the same lifetime as the
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

    /// Concrete type for errors returned while interning symbols.
    type Error;

    /// Store an immutable bytestring for the life of the interpreter.
    ///
    /// Returns an identifier that enables retrieving the original bytes.
    ///
    /// # Errors
    ///
    /// If the symbol store cannot be accessed, an error is returned.
    ///
    /// If the symbol table overflows, an error is returned.
    fn intern_bytes<T>(&mut self, symbol: T) -> Result<Self::Symbol, Self::Error>
    where
        T: Into<Cow<'static, [u8]>>;

    /// Check if a bytestring is already interned and return its symbol
    /// identifier.  Return `None` if the string has not been interned before.
    ///
    /// Returns an identifier that enables retrieving the original bytes.
    ///
    /// # Errors
    ///
    /// If the symbol store cannot be accessed, an error is returned.
    fn check_interned_bytes(&self, symbol: &[u8]) -> Result<Option<Self::Symbol>, Self::Error>;

    /// Store an immutable string for the life of the interpreter.
    ///
    /// Returns an identifier that enables retrieving the original bytes.
    ///
    /// By default, this method is implemented by delegating to
    /// [`Intern::intern_bytes`].
    ///
    /// # Errors
    ///
    /// If the symbol store cannot be accessed, an error is returned.
    ///
    /// If the symbol table overflows, an error is returned.
    fn intern_string<T>(&mut self, symbol: T) -> Result<Self::Symbol, Self::Error>
    where
        T: Into<Cow<'static, str>>,
    {
        match symbol.into() {
            Cow::Borrowed(string) => self.intern_bytes(string.as_bytes()),
            Cow::Owned(string) => self.intern_bytes(string.into_bytes()),
        }
    }

    /// Check if a string is already interned and return its symbol identifier.
    /// Return `None` if the string has not been interned before.
    ///
    /// Returns an identifier that enables retrieving the original bytes.
    ///
    /// By default, this method is implemented by delegating to
    /// [`Intern::check_interned_bytes`].
    ///
    /// # Errors
    ///
    /// If the symbol store cannot be accessed, an error is returned.
    fn check_interned_string(&self, symbol: &str) -> Result<Option<Self::Symbol>, Self::Error> {
        self.check_interned_bytes(symbol.as_bytes())
    }

    /// Retrieve the original byte content of an interned bytestring.
    ///
    /// Returns `None` if the symbol identifier is invalid.
    ///
    /// # Errors
    ///
    /// If the symbol store cannot be accessed, an error is returned.
    fn lookup_symbol(&self, symbol: Self::Symbol) -> Result<Option<&[u8]>, Self::Error>;
}
