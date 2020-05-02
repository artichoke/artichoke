//! Track `Regexp` global state.

/// Track the state of `Regexp` globals and global interpreter state.
pub trait Regexp {
    /// Retrieve the current number of set `Regexp` global variables.
    ///
    /// `Regexp` global variables like `$1` and `$7` are defined after certain
    /// `Regexp` matching methods for each capturing group in the regular
    /// expression.
    fn active_regexp_globals(&self) -> usize;

    /// Set the current number of set `Regexp` global variables.
    ///
    /// `Regexp` global variables like `$1` and `$7` are defined after certain
    /// `Regexp` matching methods for each capturing group in the regular
    /// expression.
    fn set_active_regexp_globals(&mut self, count: usize);
}
