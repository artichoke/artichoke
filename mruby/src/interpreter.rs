use log::{debug, error, trace};
use std::cell::RefCell;
use std::convert::AsRef;
use std::ffi::c_void;
use std::mem;
use std::rc::Rc;

use crate::convert::{Float, FromMrb, Int};
use crate::eval::MrbEval;
use crate::extn;
use crate::gc::GarbageCollection;
use crate::state::State;
use crate::sys::{self, DescribeState};
use crate::value::types::Ruby;
use crate::value::{Value, ValueLike};
use crate::MrbError;

pub const RUBY_LOAD_PATH: &str = "/src/lib";

pub type Mrb = Rc<RefCell<State>>;

pub struct Interpreter;

impl Interpreter {
    pub fn create() -> Result<Mrb, MrbError> {
        unsafe {
            let mrb = sys::mrb_open();
            if mrb.is_null() {
                error!("Failed to allocate mrb interprter");
                return Err(MrbError::New);
            }

            let context = sys::mrbc_context_new(mrb);
            let api = Rc::new(RefCell::new(State::new(mrb, context, RUBY_LOAD_PATH)));

            // Transmute the smart pointer that wraps the API and store it in
            // the user data of the mrb interpreter. After this operation,
            // `Rc::strong_count` will still be 1.
            let ptr = mem::transmute::<Mrb, *mut c_void>(api);
            (*mrb).ud = ptr;

            // Transmute the void * pointer to the Rc back into the Mrb type.
            // After this operation `Rc::strong_count` will still be 1. This
            // dance is required to avoid leaking Mrb objects, which will let
            // the `Drop` impl close the mrb context and interpreter.
            let interp = mem::transmute::<*mut c_void, Mrb>(ptr);

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

    // TODO: Add a benchmark to make sure this function does not leak memory.
    pub unsafe fn from_user_data(mrb: *mut sys::mrb_state) -> Result<Mrb, MrbError> {
        if mrb.is_null() {
            error!("Attempted to extract Mrb from null mrb_state");
            return Err(MrbError::Uninitialized);
        }
        let ptr = (*mrb).ud;
        if ptr.is_null() {
            error!("Attempted to extract Mrb from null mrb_state->ud pointer");
            return Err(MrbError::Uninitialized);
        }
        // Extract the smart pointer that wraps the API from the user data on
        // the mrb interpreter. The `mrb_state` should retain ownership of its
        // copy of the smart pointer.
        let ud = mem::transmute::<*mut c_void, Mrb>(ptr);
        // Clone the API smart pointer and increase its ref count to return a
        // reference to the caller.
        let api = Rc::clone(&ud);
        // Forget the transmuted API extracted from the user data to make sure
        // the `mrb_state` maintains ownership and the smart pointer does not
        // get deallocated before `mrb_close` is called.
        mem::forget(ud);
        // At this point, `Rc::strong_count` will be increased by 1.
        trace!("Extracted Mrb from user data pointer on {}", mrb.debug());
        Ok(api)
    }
}

pub trait MrbApi {
    fn current_exception(&self) -> Option<String>;

    fn nil(&self) -> Value;

    fn bool(&self, b: bool) -> Value;

    fn bytes<T: AsRef<[u8]>>(&self, b: T) -> Value;

    fn fixnum(&self, i: Int) -> Value;

    fn float(&self, i: Float) -> Value;

    fn string<T: AsRef<str>>(&self, s: T) -> Value;
}

impl MrbApi for Mrb {
    /// Extract a `String` representation of the current exception on the mruby
    /// interpreter if there is one. The string will contain the exception
    /// class, message, and backtrace.
    fn current_exception(&self) -> Option<String> {
        let _arena = self.create_arena_savepoint();
        let mrb = { self.borrow().mrb };
        let exc = unsafe {
            let exc = (*mrb).exc;
            // Clear the current exception from the mruby interpreter so
            // subsequent calls to the mruby VM are not tainted by an error they
            // did not generate.
            //
            // We must do this at the beginning of `current_exception` so we can
            // use the mruby VM to inspect the exception once we turn it into an
            // `mrb_value`. `ValueLike::funcall` handles errors by calling this
            // function, so not clearing the exception results in a stack
            // overflow.
            (*mrb).exc = std::ptr::null_mut();
            exc
        };
        if exc.is_null() {
            trace!("Last eval had no runtime errors: mrb_state has no current exception");
            return None;
        }
        // Generate an exception backtrace in a `String` by executing the
        // following Ruby code with the C API:
        //
        // ```ruby
        // exception = exc.inspect
        // backtrace = exc.backtrace
        // backtrace.unshift(exception)
        // backtrace.join("\n")
        // ```
        let value = Value::new(self, unsafe { sys::mrb_sys_obj_value(exc as *mut c_void) });
        let backtrace = value.funcall::<Value, _, _>("backtrace", &[]).ok()?;
        if backtrace.ruby_type() == Ruby::Array {
            let exception = value.funcall::<Value, _, _>("inspect", &[]).ok()?;
            backtrace
                .funcall::<(), _, _>("unshift", &[exception])
                .ok()?;
            backtrace
                .funcall::<String, _, _>("join", &[Value::from_mrb(self, "\n")])
                .ok()
        } else {
            value.funcall::<String, _, _>("inspect", &[]).ok()
        }
    }

    fn nil(&self) -> Value {
        Value::from_mrb(self, None::<Value>)
    }

    fn bool(&self, b: bool) -> Value {
        Value::from_mrb(self, b)
    }

    fn bytes<T: AsRef<[u8]>>(&self, b: T) -> Value {
        Value::from_mrb(self, b.as_ref())
    }

    fn fixnum(&self, i: Int) -> Value {
        Value::from_mrb(self, i)
    }

    fn float(&self, f: Float) -> Value {
        Value::from_mrb(self, f)
    }

    fn string<T: AsRef<str>>(&self, s: T) -> Value {
        Value::from_mrb(self, s.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::sys;
    use crate::MrbError;

    #[test]
    fn from_user_data_null_pointer() {
        unsafe {
            let err = Interpreter::from_user_data(std::ptr::null_mut());
            assert_eq!(err.err(), Some(MrbError::Uninitialized));
        }
    }

    #[test]
    fn from_user_data_null_user_data() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow();
            let mrb = api.mrb;
            // fake null user data
            (*mrb).ud = std::ptr::null_mut();
            let err = Interpreter::from_user_data(mrb);
            assert_eq!(err.err(), Some(MrbError::Uninitialized));
        }
    }

    #[test]
    fn from_user_data() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let mrb = interp.borrow().mrb;
            let res = Interpreter::from_user_data(mrb);
            assert!(res.is_ok());
        }
    }

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

    #[test]
    fn return_raised_exception() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp
            .eval("raise ArgumentError.new('waffles')")
            .map(|_| ());
        let expected = r#"
(eval):1: waffles (ArgumentError)
(eval):1
       "#;
        assert_eq!(result, Err(MrbError::Exec(expected.trim().to_owned())));
    }
}
