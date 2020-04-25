use std::ptr::NonNull;

use crate::core::{IncrementLinenoError, Parser};
use crate::state::parser::Context;
use crate::Artichoke;

impl Parser for Artichoke {
    type Context = Context;

    fn reset_parser(&mut self) {
        let mrb = unsafe { self.mrb.as_mut() };
        self.state.parser.reset(mrb);
    }

    fn fetch_lineno(&self) -> usize {
        self.state.parser.fetch_lineno()
    }

    fn add_fetch_lineno(&mut self, val: usize) -> Result<usize, IncrementLinenoError> {
        self.state.parser.add_fetch_lineno(val)
    }

    fn push_context(&mut self, context: Self::Context) {
        let mrb = unsafe { self.mrb.as_mut() };
        self.state.parser.push_context(mrb, context);
    }

    fn pop_context(&mut self) -> Option<Self::Context> {
        let mrb = unsafe { self.mrb.as_mut() };
        self.state.parser.pop_context(mrb)
    }

    fn peek_context(&self) -> Option<&Self::Context> {
        self.state.parser.peek_context()
    }
}
