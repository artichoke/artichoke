//! Parse code on an Artichoke interpreter.

use core::fmt;

/// Manage parser state, active filename context, and line number metadata.
///
/// Parsers maintain a stack of `Context`s which are modified as successive
/// sources are parsed, for example as a set of nested `require`s.
pub trait Parser {
    /// Concrete type for parser context.
    type Context;
    /// Error type for Parser APIs.
    type Error;

    /// Reset parser state to initial values.
    ///
    /// # Errors
    ///
    /// If the parser state is inaccessible, an error is returned.
    fn reset_parser(&mut self) -> Result<(), Self::Error>;

    /// Fetch the current line number from the parser state.
    ///
    /// # Errors
    ///
    /// If the parser state is inaccessible, an error is returned.
    fn fetch_lineno(&self) -> Result<usize, Self::Error>;

    /// Increment line number and return the new value.
    ///
    /// # Errors
    ///
    /// If the parser state is inaccessible, an error is returned.
    ///
    /// This function returns [`IncrementLinenoError`] if the increment results
    /// in an overflow of the internal parser line number counter.
    fn add_fetch_lineno(&mut self, val: usize) -> Result<usize, Self::Error>;

    /// Set the currently active context by modifying the parser stack.
    ///
    /// # Errors
    ///
    /// If the parser state is inaccessible, an error is returned.
    fn push_context(&mut self, context: Self::Context) -> Result<(), Self::Error>;

    /// Remove the current active context and return it.
    ///
    /// # Errors
    ///
    /// If the parser state is inaccessible, an error is returned.
    fn pop_context(&mut self) -> Result<Option<Self::Context>, Self::Error>;

    /// Return a reference to the currently active context.
    ///
    /// # Errors
    ///
    /// If the parser state is inaccessible, an error is returned.
    fn peek_context(&self) -> Result<Option<&Self::Context>, Self::Error>;
}

/// Errors encountered when incrementing line numbers on parser state.
///
/// Errors include overflows of the interpreters line counter.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncrementLinenoError {
    /// An overflow occurred when incrementing the line number.
    ///
    /// This error is reported based on the internal parser storage width
    /// and contains the max value the parser can store.
    Overflow(usize),
}

impl fmt::Display for IncrementLinenoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow(max) => write!(f, "Parser exceeded maximum line count: {max}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IncrementLinenoError {}
