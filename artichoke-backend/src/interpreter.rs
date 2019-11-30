use artichoke_core::eval::Eval;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use crate::extn;
use crate::fs::Filesystem;
use crate::gc::MrbGarbageCollection;
use crate::state::State;
use crate::sys::{self, DescribeState};
use crate::{Artichoke, ArtichokeError};

/// Create and initialize an [`Artichoke`] interpreter.
///
/// This function creates a new [`State`], embeds it in the [`sys::mrb_state`],
/// initializes an [in memory virtual filesystem](Filesystem), and loads the
/// [`extn`] extensions to Ruby Core and Stdlib.
pub fn interpreter() -> Result<Artichoke, ArtichokeError> {
    let vfs = Filesystem::new()?;
    let mrb = unsafe { sys::mrb_open() };
    if mrb.is_null() {
        error!("Failed to allocate mrb interprter");
        return Err(ArtichokeError::New);
    }

    let context = unsafe { sys::mrbc_context_new(mrb) };
    let api = Rc::new(RefCell::new(State::new(mrb, context, vfs)));

    // Transmute the smart pointer that wraps the API and store it in the user
    // data of the mrb interpreter. After this operation, `Rc::strong_count`
    // will still be 1.
    let ptr = Rc::into_raw(api);
    unsafe {
        (*mrb).ud = ptr as *mut c_void;
    }

    // Transmute the void * pointer to the Rc back into the Artichoke type. After this
    // operation `Rc::strong_count` will still be 1. This dance is required to
    // avoid leaking Artichoke objects, which will let the `Drop` impl close the mrb
    // context and interpreter.
    let interp = Artichoke(unsafe { Rc::from_raw(ptr) });

    // mruby garbage collection relies on a fully initialized Array, which we
    // won't have until after `extn::core` is initialized. Disable GC before
    // init and clean up afterward.
    interp.disable_gc();

    // Initialize Artichoke Core and Standard Library runtime
    extn::init(&interp, "mruby")?;

    // Load mrbgems
    unsafe {
        let arena = interp.create_arena_savepoint();
        sys::mrb_init_mrbgems(mrb);
        arena.restore();
    }

    debug!("Allocated {}", mrb.debug());

    // mruby lazily initializes some core objects like top_self and generates a
    // lot of garbage on startup. Eagerly initialize the interpreter to provide
    // predictable initialization behavior.
    let arena = interp.create_arena_savepoint();
    interp.eval(&[]).unwrap();
    arena.restore();

    interp.enable_gc();
    interp.full_gc();

    Ok(interp)
}

#[cfg(test)]
mod tests {
    #[test]
    fn open_close() {
        let interp = super::interpreter().unwrap();
        drop(interp);
    }
}
