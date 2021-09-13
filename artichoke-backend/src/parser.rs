// This `allow` pragma suppresses false positives.
//
// See:
//
// - https://github.com/rust-lang/rust-clippy/issues/6141
// - https://github.com/rust-lang/rust-clippy/issues/6563
#![allow(clippy::shadow_unrelated)]

use std::borrow::Cow;

use crate::core::{ClassRegistry, IncrementLinenoError, Parser, TryConvertMut};
use crate::error::{Error, RubyException};
use crate::extn::core::exception::ScriptError;
use crate::ffi::InterpreterExtractError;
use crate::state::parser::Context;
use crate::sys;
use crate::Artichoke;

impl Parser for Artichoke {
    type Context = Context;
    type Error = Error;

    fn reset_parser(&mut self) -> Result<(), Self::Error> {
        let mrb = unsafe { self.mrb.as_mut() };
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let parser = state.parser.as_mut().ok_or_else(InterpreterExtractError::new)?;
        parser.reset(mrb);
        Ok(())
    }

    fn fetch_lineno(&self) -> Result<usize, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let parser = state.parser.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let lineno = parser.fetch_lineno();
        Ok(lineno)
    }

    fn add_fetch_lineno(&mut self, val: usize) -> Result<usize, Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let parser = state.parser.as_mut().ok_or_else(InterpreterExtractError::new)?;
        let lineno = parser.add_fetch_lineno(val)?;
        Ok(lineno)
    }

    fn push_context(&mut self, context: Self::Context) -> Result<(), Self::Error> {
        let mrb = unsafe { self.mrb.as_mut() };
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let parser = state.parser.as_mut().ok_or_else(InterpreterExtractError::new)?;
        parser.push_context(mrb, context);
        Ok(())
    }

    fn pop_context(&mut self) -> Result<Option<Self::Context>, Self::Error> {
        let mrb = unsafe { self.mrb.as_mut() };
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let parser = state.parser.as_mut().ok_or_else(InterpreterExtractError::new)?;
        let context = parser.pop_context(mrb);
        Ok(context)
    }

    fn peek_context(&self) -> Result<Option<&Self::Context>, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let parser = state.parser.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let context = parser.peek_context();
        Ok(context)
    }
}

impl RubyException for IncrementLinenoError {
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"parser exceeded maximum line count")
    }

    fn name(&self) -> Cow<'_, str> {
        "ScriptError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.message()).ok()?;
        let value = interp.new_instance::<ScriptError>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<IncrementLinenoError> for Error {
    fn from(exception: IncrementLinenoError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<IncrementLinenoError>> for Error {
    fn from(exception: Box<IncrementLinenoError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<IncrementLinenoError> for Box<dyn RubyException> {
    fn from(exception: IncrementLinenoError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<IncrementLinenoError>> for Box<dyn RubyException> {
    fn from(exception: Box<IncrementLinenoError>) -> Box<dyn RubyException> {
        exception
    }
}
