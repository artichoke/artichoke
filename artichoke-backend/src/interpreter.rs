use std::error;
use std::ffi::c_void;
use std::fmt;
use std::ptr::NonNull;

use crate::class_registry::ClassRegistry;
use crate::core::{ConvertMut, Eval};
use crate::exception::{Exception, RubyException};
use crate::extn;
use crate::extn::core::exception::Fatal;
use crate::gc::MrbGarbageCollection;
use crate::state::State;
use crate::sys;
use crate::Artichoke;

/// Create and initialize an [`Artichoke`] interpreter.
///
/// This function creates a new [`State`], embeds it in the [`sys::mrb_state`],
/// initializes an [in memory virtual filesystem](crate::fs::Virtual), and loads
/// the [`extn`] extensions to Ruby Core and Stdlib.
pub fn interpreter() -> Result<Artichoke, Exception> {
    let raw = unsafe { sys::mrb_open() };
    let mut mrb = if let Some(mrb) = NonNull::new(raw) {
        mrb
    } else {
        error!("Failed to allocate Artichoke interprter");
        return Err(Exception::from(InterpreterAllocError));
    };

    let state = State::new(unsafe { mrb.as_mut() }).ok_or(InterpreterAllocError)?;
    let state = Box::new(state);
    let mut interp = Artichoke { mrb, state };

    // mruby garbage collection relies on a fully initialized Array, which we
    // won't have until after `extn::core` is initialized. Disable GC before
    // init and clean up afterward.
    interp.disable_gc();

    // Initialize Artichoke Core and Standard Library runtime
    extn::init(&mut interp, "mruby")?;

    // Load mrbgems
    let arena = interp.create_arena_savepoint();
    unsafe {
        sys::mrb_init_mrbgems(mrb.as_mut());
    }
    arena.restore();

    debug!(
        "Allocated {}",
        sys::mrb_sys_state_debug(unsafe { mrb.as_mut() })
    );

    // mruby lazily initializes some core objects like top_self and generates a
    // lot of garbage on startup. Eagerly initialize the interpreter to provide
    // predictable initialization behavior.
    interp.create_arena_savepoint().interp().eval(&[])?;

    interp.enable_gc();
    interp.full_gc();

    Ok(interp)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct InterpreterAllocError;

impl fmt::Display for InterpreterAllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to allocate Artichoke interpreter")
    }
}

impl error::Error for InterpreterAllocError {}

impl RubyException for InterpreterAllocError {
    fn message(&self) -> &[u8] {
        &b"Failed to allocate Artichoke Ruby interpreter"[..]
    }

    fn name(&self) -> String {
        String::from("fatal")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let value = interp.new_instance::<Fatal>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<InterpreterAllocError> for Exception {
    fn from(exception: InterpreterAllocError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<InterpreterAllocError>> for Exception {
    fn from(exception: Box<InterpreterAllocError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<InterpreterAllocError> for Box<dyn RubyException> {
    fn from(exception: InterpreterAllocError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<InterpreterAllocError>> for Box<dyn RubyException> {
    fn from(exception: Box<InterpreterAllocError>) -> Box<dyn RubyException> {
        exception
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn open_close() {
        let interp = super::interpreter().unwrap();
        drop(interp);
    }
}
