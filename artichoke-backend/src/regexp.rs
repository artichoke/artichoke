use crate::core::Regexp;
use crate::Artichoke;

impl Regexp for Artichoke {
    fn active_regexp_globals(&self) -> usize {
        self.0.borrow().regexp.active_regexp_globals()
    }

    fn set_active_regexp_globals(&mut self, count: usize) {
        self.0.borrow_mut().regexp.set_active_regexp_globals(count)
    }
}
