use crate::core::Regexp;
use crate::ffi::InterpreterExtractError;
use crate::Artichoke;

impl Regexp for Artichoke {
    type Error = InterpreterExtractError;

    fn active_regexp_globals(&self) -> Result<usize, Self::Error> {
        let count = self
            .state
            .as_ref()
            .ok_or(InterpreterExtractError)?
            .regexp
            .active_regexp_globals();
        Ok(count)
    }

    fn set_active_regexp_globals(&mut self, count: usize) -> Result<(), Self::Error> {
        self.state
            .as_mut()
            .ok_or(InterpreterExtractError)?
            .regexp
            .set_active_regexp_globals(count);
        Ok(())
    }

    fn clear_regexp(&mut self) -> Result<(), Self::Error> {
        self.state
            .as_mut()
            .ok_or(InterpreterExtractError)?
            .regexp
            .clear();
        Ok(())
    }
}
