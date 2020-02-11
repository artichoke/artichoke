use std::cell::RefCell;
use std::error;
use std::ffi::c_void;
use std::fmt;
use std::ptr::NonNull;
use std::rc::Rc;

use crate::exception::{Exception, RubyException};
use crate::extn;
use crate::extn::core::exception::Fatal;
use crate::gc::MrbGarbageCollection;
use crate::state::State;
use crate::sys::{self, DescribeState};
use crate::{Artichoke, ConvertMut, Eval};

/// Create and initialize an [`Artichoke`] interpreter.
///
/// This function creates a new [`State`], embeds it in the [`sys::mrb_state`],
/// initializes an [in memory virtual filesystem](crate::fs::Virtual), and loads
/// the [`extn`] extensions to Ruby Core and Stdlib.
pub fn interpreter() -> Result<Artichoke, Exception> {
    let mut mrb = if let Some(mrb) = NonNull::new(unsafe { sys::mrb_open() }) {
        mrb
    } else {
        error!("Failed to allocate Artichoke interprter");
        return Err(Exception::from(InterpreterAllocError));
    };

    let state = State::new(unsafe { mrb.as_mut() }).ok_or(InterpreterAllocError)?;
    let api = Rc::new(RefCell::new(state));

    // Transmute the smart pointer that wraps the API and store it in the user
    // data of the mrb interpreter. After this operation, `Rc::strong_count`
    // will still be 1.
    let userdata = Rc::into_raw(api);
    unsafe {
        mrb.as_mut().ud = userdata as *mut c_void;
    }

    // Transmute the void * pointer to the Rc back into the Artichoke type. After this
    // operation `Rc::strong_count` will still be 1. This dance is required to
    // avoid leaking Artichoke objects, which will let the `Drop` impl close the mrb
    // context and interpreter.
    let mut interp = Artichoke(unsafe { Rc::from_raw(userdata) });

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

    debug!("Allocated {}", mrb.debug());

    // mruby lazily initializes some core objects like top_self and generates a
    // lot of garbage on startup. Eagerly initialize the interpreter to provide
    // predictable initialization behavior.
    let arena = interp.create_arena_savepoint();
    let _ = interp.eval(&[])?;
    arena.restore();

    interp.enable_gc();
    interp.full_gc();

    Ok(interp)
}

#[derive(Debug, Clone)]
pub struct InterpreterAllocError;

impl fmt::Display for InterpreterAllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to allocate Artichoke interpreter")
    }
}

impl error::Error for InterpreterAllocError {}

impl RubyException for InterpreterAllocError {
    fn box_clone(&self) -> Box<dyn RubyException> {
        Box::new(self.clone())
    }

    fn message(&self) -> &[u8] {
        &b"Failed to allocate Artichoke Ruby interpreter"[..]
    }

    fn name(&self) -> String {
        String::from("fatal")
    }

    fn vm_backtrace(&self, interp: &Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let borrow = interp.0.borrow();
        let spec = borrow.class_spec::<Fatal>()?;
        let value = spec.new_instance(interp, &[message])?;
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
