//! Track `Regexp` global state.

/// Track the state of [`Regexp`] special [global variables] and global
/// interpreter state.
///
/// [`Regexp`]: https://ruby-doc.org/core-2.6.3/Regexp.html
/// [global variables]: https://ruby-doc.org/core-2.6.3/Regexp.html#class-Regexp-label-Special+global+variables
pub trait Regexp {
    /// Concrete error type for errors encountered when manipulating `Regexp`
    /// state.
    type Error;

    /// Retrieve the current number of set `Regexp` global variables.
    ///
    /// `Regexp` global variables like `$1` and `$7` are defined after certain
    /// `Regexp` matching methods for each capturing group in the regular
    /// expression.
    ///
    /// Per the Ruby documentation:
    ///
    /// > `$1`, `$2` and so on contain text matching first, second, etc capture
    /// > group.
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
    /// Per the Ruby documentation:
    ///
    /// > `$1`, `$2` and so on contain text matching first, second, etc capture
    /// > group.
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
