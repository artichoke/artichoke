use log::{debug, error};
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use crate::eval::MrbEval;
use crate::extn;
use crate::gc::MrbGarbageCollection;
use crate::state::State;
use crate::sys::{self, DescribeState};
use crate::{Mrb, MrbError};

pub const RUBY_LOAD_PATH: &str = "/src/lib";

pub struct Interpreter;

impl Interpreter {
    pub fn create() -> Result<Mrb, MrbError> {
        let mrb = unsafe { sys::mrb_open() };
        if mrb.is_null() {
            error!("Failed to allocate mrb interprter");
            return Err(MrbError::New);
        }

        let context = unsafe { sys::mrbc_context_new(mrb) };
        let api = Rc::new(RefCell::new(State::new(mrb, context, RUBY_LOAD_PATH)));

        // Transmute the smart pointer that wraps the API and store it in the
        // user data of the mrb interpreter. After this operation,
        // `Rc::strong_count` will still be 1.
        let ptr = Rc::into_raw(api);
        unsafe {
            (*mrb).ud = ptr as *mut c_void;
        }

        // Transmute the void * pointer to the Rc back into the Mrb type. After
        // this operation `Rc::strong_count` will still be 1. This dance is
        // required to avoid leaking Mrb objects, which will let the `Drop` impl
        // close the mrb context and interpreter.
        let interp = unsafe { Rc::from_raw(ptr) };

        // Patch mruby builtins with Rust extensions
        extn::patch(&interp)?;

        debug!("Allocated {}", mrb.debug());

        // mruby lazily initializes some core objects like top_self and
        // generates a lot of garbage on startup. Eagerly initialize the
        // interpreter to provide predictable initialization behavior.
        let arena = interp.create_arena_savepoint();
        interp.eval("").map_err(|_| MrbError::New)?;
        arena.restore();
        interp.full_gc();
        Ok(interp)
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::sys;

    #[test]
    fn open_close() {
        let interp = Interpreter::create().expect("mrb init");
        drop(interp);
    }

    #[test]
    fn load_code() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let result = interp.eval("255").expect("eval");
            assert_eq!(sys::mrb_sys_fixnum_to_cint(result.inner()), 255);
        }
    }
}
