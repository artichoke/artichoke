use crate::core::Regexp;
use crate::ffi::InterpreterExtractError;
use crate::Artichoke;

#[cfg_attr(docsrs, doc(cfg(feature = "core-regexp")))]
impl Regexp for Artichoke {
    type Error = InterpreterExtractError;

    fn capture_group_globals(&self) -> Result<usize, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let count = state.regexp.capture_group_globals();
        Ok(count)
    }

    fn set_capture_group_globals(&mut self, count: usize) -> Result<(), Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.regexp.set_capture_group_globals(count);
        Ok(())
    }

    fn clear_regexp(&mut self) -> Result<(), Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.regexp.clear();
        Ok(())
    }
}
