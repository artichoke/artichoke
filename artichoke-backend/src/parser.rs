use std::ptr::NonNull;

use crate::core::parser::{IncrementLinenoError, Parser};
use crate::state::parser::Context;
use crate::Artichoke;

impl Parser for Artichoke {
    type Context = Context;

    fn reset_parser(&mut self) {
        let mrb = self.0.borrow().mrb;
        if let Some(mut mrb) = NonNull::new(mrb) {
            self.0.borrow_mut().parser.reset(unsafe { mrb.as_mut() });
        }
    }

    fn fetch_lineno(&self) -> usize {
        self.0.borrow().parser.fetch_lineno()
    }

    fn add_fetch_lineno(&mut self, val: usize) -> Result<usize, IncrementLinenoError> {
        self.0.borrow_mut().parser.add_fetch_lineno(val)
    }

    fn push_context(&mut self, context: Self::Context) {
        let mrb = self.0.borrow().mrb;
        if let Some(mut mrb) = NonNull::new(mrb) {
            self.0
                .borrow_mut()
                .parser
                .push_context(unsafe { mrb.as_mut() }, context);
        }
    }

    fn pop_context(&mut self) -> Option<Self::Context> {
        let mrb = self.0.borrow().mrb;
        if let Some(mut mrb) = NonNull::new(mrb) {
            self.0
                .borrow_mut()
                .parser
                .pop_context(unsafe { mrb.as_mut() })
        } else {
            None
        }
    }

    fn peek_context(&self) -> Option<&Self::Context> {
        // TODO(GH-468): Implement `Parser::peek_context`.
        unimplemented!("GH-468: cannot implement Parser::peek_context due to internal RefCell");
    }
}
