//! Create or retrieve an interpreter for a request.

use mruby::eval::MrbEval;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::MrbError;
use mruby_gems::rubygems::rack;
use std::sync::{Arc, Mutex};

use crate::rubygems::nemesis;
use crate::Error;

pub type InitFunc = Box<dyn Fn(&Mrb) -> Result<(), MrbError> + Send>;

/// Execution mode of an interpreter for a given mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecMode {
    /// A single interpreter will be used for a worker executing the mount.
    PerMountPerWorker {
        /// After `max_requests`, close the interpreter and lazily initialize a
        /// new one.
        ///
        /// If `max_requests` is `0`, the interpreter is never recycled.
        max_requests: usize,
    },
    /// A new interpreter will be initialized for each request and closed at the
    /// end of the request.
    SingleUse,
}

impl ExecMode {
    pub fn interpreter(&self, init: &Option<Arc<Mutex<InitFunc>>>) -> Result<Mrb, Error> {
        if let ExecMode::SingleUse = self {
            let interp = Interpreter::create()?;
            rack::init(&interp)?;
            nemesis::init(&interp)?;
            // Preload required gem sources
            interp.eval("require 'rack'")?;
            interp.eval("require 'nemesis'")?;
            interp.eval("require 'nemesis/response'")?;
            if let Some(init) = init {
                let init = init.lock().map_err(|_| MrbError::New)?;
                init(&interp)?;
            }
            Ok(interp)
        } else {
            // TODO: implement support for all exec modes.
            panic!("Exec mode not implemented {:?}", self);
        }
    }

    /// Maybe execute a garbage collection on the interpreter.
    ///
    /// Returns true if a GC was performed, false otherwise.
    pub fn gc(&self, interp: &Mrb) -> bool {
        if let ExecMode::SingleUse = self {
            false
        } else {
            let _ = interp;
            // TODO: implement support for all exec modes.
            panic!("Exec mode not implemented {:?}", self);
        }
    }
}

impl Default for ExecMode {
    fn default() -> Self {
        ExecMode::SingleUse
    }
}
