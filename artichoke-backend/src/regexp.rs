use crate::core::Regexp;
use crate::ffi::InterpreterExtractError;
use crate::Artichoke;

impl Regexp for Artichoke {
    type Error = InterpreterExtractError;

    fn active_regexp_globals(&self) -> Result<usize, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let count = state.regexp.active_regexp_globals();
        Ok(count)
    }

    fn set_active_regexp_globals(&mut self, count: usize) -> Result<(), Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.regexp.set_active_regexp_globals(count);
        Ok(())
    }

    fn clear_regexp(&mut self) -> Result<(), Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.regexp.clear();
        Ok(())
    }
}
