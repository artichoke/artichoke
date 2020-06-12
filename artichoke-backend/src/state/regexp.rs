#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    active_regexp_globals: usize,
}

impl State {
    /// Constructs a new, default Regexp `State`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    #[inline]
    #[must_use]
    pub fn active_regexp_globals(self) -> usize {
        self.active_regexp_globals
    }

    #[inline]
    pub fn set_active_regexp_globals(&mut self, count: usize) {
        self.active_regexp_globals = count;
    }
}
