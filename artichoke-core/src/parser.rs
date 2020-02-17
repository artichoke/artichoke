//! Parse code on an Artichoke interpreter.

use std::error;
use std::fmt;

/// Manage parser state, active filename context, and line number metadata.
///
/// Parsers maintain a stack of `Context`s which are modified as successive
/// sources are parsed, for example as a set of nested `require`s.
pub trait Parser {
    /// Concrete type for parser context.
    type Context;

    /// Reset parser state to initial values.
    fn reset_parser(&mut self);

    /// Fetch the current line number from the parser state.
    fn fetch_lineno(&self) -> usize;

    /// Increment line number and return the new value.
    ///
    /// # Errors
    ///
    /// This function returns [`IncrementLinenoError`] if the increment results
    /// in an overflow of the internal parser line number counter.
    fn add_fetch_lineno(&mut self, val: usize) -> Result<usize, IncrementLinenoError>;

    /// Set the currently active context by modifying the parser stack.
    fn push_context(&mut self, context: Self::Context);

    /// Remove the current active context and return it.
    fn pop_context(&mut self) -> Option<Self::Context>;

    /// Return a reference to the currently active context.
    fn peek_context(&self) -> Option<&Self::Context>;
}

/// Errors encountered when incrementing line numbers on parser state.
///
/// Errors include overflows of the interpreters line counter.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncrementLinenoError {
    /// An overflow occurred when incrementing the line number.
    ///
    /// This error is reported based on the internal parser storage width
    /// and contains the max value the parser can store.
    Overflow(usize),
}

impl fmt::Display for IncrementLinenoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Overflow(max) => write!(f, "Parser exceeded maximum line count: {}", max),
        }
    }
}

impl error::Error for IncrementLinenoError {}
