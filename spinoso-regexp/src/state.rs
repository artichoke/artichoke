/// Container for Ruby VM-level Regexp engine state.
///
/// Using [`Regexp`]s in Ruby code can set [regexp global variables]:
///
/// - `$~` is equivalent to `Regexp.last_match`.
/// - `$&` contains the complete matched text.
/// - `` $` `` contains string before match.
/// - `$'` contains string after match.
/// - `$1`, `$2` and so on contain text matching first, second, etc capture group.
/// - `$+` contains last capture group.
///
/// This struct is used by the implementation of `Regexp` in Artichoke's Ruby
/// Core to track this global state.
///
/// [`Regexp`]: https://ruby-doc.org/3.2.2/Regexp.html
/// [regexp global variables]: https://ruby-doc.org/3.2.2/Regexp.html#class-Regexp-label-Regexp+Global+Variables
#[allow(missing_copy_implementations)] // this is a mutable container
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    capture_group_globals: usize,
}

impl State {
    /// Constructs a new, empty `Regexp` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::State;
    /// let state = State::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            capture_group_globals: 0,
        }
    }

    /// Reset the state to empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::State;
    /// let mut state = State::new();
    /// state.clear();
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Retrieve the count of currently active `Regexp` capture group globals.
    ///
    /// This count is used to track how many `$1`, `$2`, etc. variables are set
    /// to non-nil values.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::State;
    ///
    /// let mut state = State::new();
    /// assert_eq!(state.capture_group_globals(), 0);
    ///
    /// state.set_capture_group_globals(7);
    /// assert_eq!(state.capture_group_globals(), 7);
    /// ```
    #[inline]
    #[must_use]
    pub const fn capture_group_globals(&self) -> usize {
        self.capture_group_globals
    }

    /// Set the count of currently active `Regexp` capture group globals.
    ///
    /// This count is used to track how many `$1`, `$2`, etc. variables are set
    /// to non-nil values.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_regexp::State;
    ///
    /// let mut state = State::new();
    /// assert_eq!(state.capture_group_globals(), 0);
    ///
    /// state.set_capture_group_globals(7);
    /// assert_eq!(state.capture_group_globals(), 7);
    /// ```
    #[inline]
    pub fn set_capture_group_globals(&mut self, count: usize) {
        self.capture_group_globals = count;
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::State;

    #[test]
    fn new_is_const() {
        const _: State = State::new();
    }

    #[test]
    fn new_is_empty() {
        let state = State::new();
        assert_eq!(state.capture_group_globals(), 0);
    }

    #[test]
    fn default_is_empty() {
        let state = State::default();
        assert_eq!(state.capture_group_globals(), 0);
    }

    #[test]
    fn clear_sets_capture_group_globals_to_zero() {
        let mut state = State::new();
        state.clear();
        assert_eq!(state.capture_group_globals(), 0);

        state.set_capture_group_globals(9);
        state.clear();
        assert_eq!(state.capture_group_globals(), 0);
    }

    #[test]
    fn set_get_capture_group_globals() {
        let test_cases = [
            0,
            1,
            2,
            3,
            4,
            6,
            6,
            7,
            8,
            9,
            10,
            99,
            1024,
            usize::try_from(i32::MAX).unwrap(),
            usize::MAX,
        ];
        for count in test_cases {
            let mut state = State::new();
            state.set_capture_group_globals(count);
            assert_eq!(state.capture_group_globals(), count);
        }
    }

    #[test]
    fn debug_is_not_empty() {
        let state = State::new();
        let mut s = String::new();
        write!(&mut s, "{state:?}").unwrap();
        assert!(!s.is_empty());
    }

    #[test]
    fn clone_preserves_state() {
        let mut state = State::new();
        assert_eq!(state.capture_group_globals(), state.clone().capture_group_globals());

        state.set_capture_group_globals(29);
        assert_eq!(state.capture_group_globals(), state.clone().capture_group_globals());
    }

    #[test]
    fn partial_eq_is_reflexive() {
        let mut state = State::new();
        assert_eq!(&state, &state);

        state.set_capture_group_globals(17);
        assert_eq!(&state, &state);
    }

    #[test]
    fn partial_eq_not_eq() {
        let mut left = State::new();
        left.set_capture_group_globals(56);

        let mut right = State::new();
        right.set_capture_group_globals(35);

        assert_ne!(left, right);
        assert_ne!(right, left);
    }
}
