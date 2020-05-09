//! Track `Regexp` global state.

use std::error;

/// Track the state of `Regexp` globals and global interpreter state.
pub trait Regexp {
    /// Concrete error type for errors encountered when manipulating `Regexp`
    /// state.
    type Error: error::Error;

    /// Retrieve the current number of set `Regexp` global variables.
    ///
    /// `Regexp` global variables like `$1` and `$7` are defined after certain
    /// `Regexp` matching methods for each capturing group in the regular
    /// expression.
    ///
    /// # Errors
    ///
    /// If the `Regexp` state is inaccessible, an error is returned.
    fn active_regexp_globals(&self) -> Result<usize, Self::Error>;

    /// Set the current number of set `Regexp` global variables.
    ///
    /// `Regexp` global variables like `$1` and `$7` are defined after certain
    /// `Regexp` matching methods for each capturing group in the regular
    /// expression.
    ///
    /// # Errors
    ///
    /// If the `Regexp` state is inaccessible, an error is returned.
    fn set_active_regexp_globals(&mut self, count: usize) -> Result<(), Self::Error>;

    /// Clear all `Regexp` state.
    ///
    /// # Errors
    ///
    /// If the `Regexp` state is inaccessible, an error is returned.
    fn clear_regexp(&mut self) -> Result<(), Self::Error>;
}
