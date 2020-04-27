use crate::class_registry::ClassRegistry;
use crate::core::{IncrementLinenoError, Parser};
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception::ScriptError;
use crate::ffi::InterpreterExtractError;
use crate::state::parser::Context;
use crate::{Artichoke, ConvertMut};

impl Parser for Artichoke {
    type Context = Context;
    type Error = Exception;

    fn reset_parser(&mut self) -> Result<(), Self::Error> {
        let mrb = unsafe { self.mrb.as_mut() };
        self.state.ok_or(InterpreterExtractError)?.parser.reset(mrb);
        Ok(())
    }

    fn fetch_lineno(&self) -> Result<usize, Self::Error> {
        let lineno = self
            .state
            .ok_or(InterpreterExtractError)?
            .parser
            .fetch_lineno();
        Ok(lineno)
    }

    fn add_fetch_lineno(&mut self, val: usize) -> Result<usize, Self::Error> {
        let lineno = self
            .state
            .ok_or(InterpreterExtractError)?
            .parser
            .add_fetch_lineno(val)?;
        Ok(lineno)
    }

    fn push_context(&mut self, context: Self::Context) -> Result<(), Self::Error> {
        let mrb = unsafe { self.mrb.as_mut() };
        self.state
            .ok_or(InterpreterExtractError)?
            .parser
            .push_context(mrb, context);
        Ok(())
    }

    fn pop_context(&mut self) -> Result<Option<Self::Context>, Self::Error> {
        let mrb = unsafe { self.mrb.as_mut() };
        let context = self
            .state
            .ok_or(InterpreterExtractError)?
            .parser
            .pop_context(mrb);
        Ok(context)
    }

    fn peek_context(&self) -> Result<Option<&Self::Context>, Self::Error> {
        let context = self
            .state
            .ok_or(InterpreterExtractError)?
            .parser
            .peek_context();
        Ok(context)
    }
}

impl RubyException for IncrementLinenoError {
    fn message(&self) -> &[u8] {
        b"parser exceeded maximum line count"
    }

    fn name(&self) -> String {
        String::from("ScriptError")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let value = interp
            .new_instance::<ScriptError>(&[message])
            .ok()
            .flatten()?;
        Some(value.inner())
    }
}

impl From<IncrementLinenoError> for Exception {
    fn from(exception: IOError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<IncrementLinenoError>> for Exception {
    fn from(exception: Box<IOError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<IncrementLinenoError> for Box<dyn RubyException> {
    fn from(exception: IOError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<IncrementLinenoError>> for Box<dyn RubyException> {
    fn from(exception: Box<IOError>) -> Box<dyn RubyException> {
        exception
    }
}
