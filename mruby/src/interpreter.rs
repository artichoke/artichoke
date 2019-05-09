use log::{debug, error, trace, warn};
use std::cell::RefCell;
use std::convert::AsRef;
use std::error;
use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::mem;
use std::rc::Rc;

use crate::convert::{Error, Float, Int, TryFromMrb};
use crate::file::MrbFile;
use crate::state::State;
use crate::sys::{self, DescribeState};
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

#[macro_export]
macro_rules! interpreter_or_raise {
    ($mrb:expr) => {
        match $crate::interpreter::Interpreter::from_user_data($mrb) {
            std::result::Result::Err(err) => {
                // Unable to retrieve interpreter from user data pointer in
                // `mrb_state`.
                let eclass = std::ffi::CString::new("RuntimeError");
                let message = std::ffi::CString::new(format!("{}", err));
                if let (std::result::Result::Ok(eclass), std::result::Result::Ok(message)) =
                    (eclass, message)
                {
                    $crate::sys::mrb_sys_raise($mrb, eclass.as_ptr(), message.as_ptr());
                }
                return $crate::sys::mrb_sys_nil_value();
            }
            std::result::Result::Ok(interpreter) => interpreter,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_raise {
    ($interp:expr, $result:expr) => {
        match $result {
            std::result::Result::Err(err) => {
                // There was a TypeError converting to the desired Rust type.
                let eclass = std::ffi::CString::new("RuntimeError");
                let message = std::ffi::CString::new(format!("{}", err));
                if let (std::result::Result::Ok(eclass), std::result::Result::Ok(message)) =
                    (eclass, message)
                {
                    $crate::sys::mrb_sys_raise(
                        $interp.borrow().mrb,
                        eclass.as_ptr(),
                        message.as_ptr(),
                    );
                }
                return $crate::interpreter::MrbApi::nil(&$interp).inner();
            }
            std::result::Result::Ok(value) => value.inner(),
        }
    };
}

pub type Mrb = Rc<RefCell<State>>;

#[derive(Debug, Clone, Copy)]
pub struct ArenaIndex(i32);

extern "C" fn require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    unsafe {
        let interp = interpreter_or_raise!(mrb);
        // Extract required filename from arguments
        let name = mem::uninitialized::<*const std::os::raw::c_char>();
        let argspec = CString::new(sys::specifiers::CSTRING).expect("argspec");
        sys::mrb_get_args(mrb, argspec.as_ptr(), &name);
        let name = match CStr::from_ptr(name).to_str() {
            Ok(name) => name.to_owned(),
            Err(err) => {
                let eclass = CString::new("ArgumentError");
                let message = CString::new(format!("{}", err));
                if let (Ok(eclass), Ok(message)) = (eclass, message) {
                    sys::mrb_sys_raise(interp.borrow().mrb, eclass.as_ptr(), message.as_ptr());
                }
                return interp.nil().inner();
            }
        };

        let already_required = {
            let borrow = interp.borrow();
            borrow.required_files.contains(&name)
        };
        if already_required {
            return interp.bool(false).inner();
        }

        let req = {
            let borrow = interp.borrow();
            borrow
                .file_registry
                .get(&name)
                .or_else(|| borrow.file_registry.get(&format!("{}.rb", name)))
                .or_else(|| borrow.file_registry.get(&format!("{}.mrb", name)))
                .map(Clone::clone)
        };

        if let Some(req) = req {
            req(Rc::clone(&interp));
            {
                let mut borrow = interp.borrow_mut();
                borrow.required_files.insert(name.to_owned());
            }
            trace!("Successful require of '{}' on {:?}", name, interp.borrow());
            interp.bool(true).inner()
        } else {
            let eclass = CString::new("RuntimeError").expect("RuntimeError class");
            let message = format!("cannot load such file -- {}", name);
            let msg = CString::new(message).expect("error message");
            sys::mrb_sys_raise(interp.borrow().mrb, eclass.as_ptr(), msg.as_ptr());
            debug!("Failed require '{}' on {:?}", name, interp.borrow());
            interp.bool(false).inner()
        }
    }
}

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
            let api = Rc::new(RefCell::new(State::new(mrb, context)));

            // Transmute the smart pointer that wraps the API and store it in
            // the user data of the mrb interpreter. After this operation,
            // `Rc::strong_count` will still be 1.
            let ptr = mem::transmute::<Mrb, *mut c_void>(api);
            (*mrb).ud = ptr;

            // Add global extension functions
            // Support for requiring files via `Kernel#require`
            let kernel = CString::new("Kernel").expect("Kernel module");
            let kernel_module = sys::mrb_module_get(mrb, kernel.as_ptr());
            let require_method = CString::new("require").expect("require method");
            let aspec = sys::mrb_args_rest();
            sys::mrb_define_module_function(
                mrb,
                kernel_module,
                require_method.as_ptr(),
                Some(require),
                aspec,
            );
            trace!("Installed Kernel#require on {}", mrb.debug());
            debug!("Allocated {}", mrb.debug());

            // Transmute the void * pointer to the Rc back into the Mrb type.
            // After this operation `Rc::strong_count` will still be 1. This
            // dance is required to avoid leaking Mrb objects, which will let
            // the `Drop` impl close the mrb context and interpreter.
            let interp = mem::transmute::<*mut c_void, Mrb>(ptr);

            // mruby lazily initializes some core objects like top_self and
            // generates a lot of garbage on startup. Eagerly initialize the
            // interpreter to provide predictable initialization behavior.
            let arena = interp.create_arena_savepoint();
            interp.eval("").map_err(|_| MrbError::New)?;
            interp.restore_arena(arena);
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MrbError {
    ConvertToRuby(Error<Rust, Ruby>),
    ConvertToRust(Error<Ruby, Rust>),
    Exec(String),
    New,
    Uninitialized,
}

impl fmt::Display for MrbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MrbError::Exec(backtrace) => write!(f, "mruby exception: {}", backtrace),
            MrbError::New => write!(f, "failed to create mrb interpreter"),
            MrbError::ConvertToRuby(inner) => write!(f, "conversion error: {}", inner),
            MrbError::ConvertToRust(inner) => write!(f, "conversion error: {}", inner),
            MrbError::Uninitialized => write!(f, "mrb interpreter not initialized"),
        }
    }
}

impl error::Error for MrbError {
    fn description(&self) -> &str {
        "mruby interpreter error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            MrbError::ConvertToRuby(inner) => Some(inner),
            MrbError::ConvertToRust(inner) => Some(inner),
            _ => None,
        }
    }
}

/// `MrbApi` is the mutable API around the [`MrbState`]. `MrbApi` should provide
/// safe wrappers around unsafe functions from [`mruby_sys`] and the
/// [`TryFromMrb`] converters.
pub trait MrbApi {
    fn create_arena_savepoint(&self) -> ArenaIndex;

    fn restore_arena(&self, savepoint: ArenaIndex);

    fn live_object_count(&self) -> i32;

    fn incremental_gc(&self);

    fn full_gc(&self);

    fn enable_gc(&self);

    fn disable_gc(&self);

    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>;

    fn current_exception(&self) -> Option<String>;

    fn def_file<T>(&mut self, filename: T, require: fn(Self))
    where
        T: AsRef<str>;

    fn def_file_for_type<T, F>(&mut self, filename: T)
    where
        T: AsRef<str>,
        F: MrbFile;

    fn nil(&self) -> Value;

    fn bool(&self, b: bool) -> Value;

    fn bytes<T: AsRef<[u8]>>(&self, b: T) -> Value;

    fn fixnum(&self, i: Int) -> Value;

    fn float(&self, i: Float) -> Value;

    fn string<T: AsRef<str>>(&self, s: T) -> Value;
}

/// We need to implement the [`MrbApi`] on the [`Rc`] smart pointer [`Mrb`]
/// type instead of the [`MrbState`] because we store the [`Rc`] in the userdata
/// pointer of the [`sys::mrb_state`]. If the `MrbApi` were implemented on the
/// `MrbState`, there would be duplicate borrows on the `Mrb` smart pointer
/// during nested access to the interpreter.
///
/// Implementing `MrbApi` on `Mrb` means callers do not need to manipulate
/// borrows when evaling code. This is convenient because eval may recursively
/// call [`MrbApi::eval`], e.g. during a nested require.
impl MrbApi for Mrb {
    fn create_arena_savepoint(&self) -> ArenaIndex {
        // Create a savepoint in the GC arena which will allow mruby to
        // deallocate all of the objects we create via the C API. Normally
        // objects created via the C API are marked as permannently alive
        // ("white" GC color) with a call to `mrb_gc_protect`.
        ArenaIndex(unsafe { sys::mrb_sys_gc_arena_save(self.borrow().mrb) })
    }

    fn restore_arena(&self, savepoint: ArenaIndex) {
        // Restore the GC arena to its stack position before calling `eval`
        // to allow objects created via the evaled code to get garbage
        // collected.
        unsafe { sys::mrb_sys_gc_arena_restore(self.borrow().mrb, savepoint.0) };
    }

    fn live_object_count(&self) -> i32 {
        unsafe { sys::mrb_sys_gc_live_objects(self.borrow().mrb) }
    }

    fn incremental_gc(&self) {
        unsafe { sys::mrb_incremental_gc(self.borrow().mrb) };
    }

    fn full_gc(&self) {
        unsafe { sys::mrb_full_gc(self.borrow().mrb) };
    }

    fn enable_gc(&self) {
        unsafe { sys::mrb_sys_gc_enable(self.borrow().mrb) };
    }

    fn disable_gc(&self) {
        unsafe { sys::mrb_sys_gc_disable(self.borrow().mrb) };
    }

    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>,
    {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Mrb` to
        // get access to the underlying `MrbState`.
        let (mrb, ctx) = {
            let borrow = self.borrow();
            (borrow.mrb, borrow.ctx)
        };
        let code = code.as_ref();
        debug!("Evaling code on {}", mrb.debug());
        let result = unsafe {
            // Execute arbitrary ruby code, which may generate objects with C
            // APIs if backed by Rust functions.
            //
            // `mrb_load_nstring_ctx` sets the "stack keep" field on the context
            // which means the most recent value returned by eval will always be
            // considered live by the GC.
            sys::mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), ctx)
        };
        if let Some(backtrace) = self.current_exception() {
            warn!("runtime error with exception backtrace: {}", backtrace);
            return Err(MrbError::Exec(backtrace));
        }
        Ok(Value::new(Rc::clone(self), result))
    }

    /// Extract a `String` representation of the current exception on the mruby
    /// interpreter if there is one. The string will contain the exception
    /// class, message, and backtrace.
    fn current_exception(&self) -> Option<String> {
        let mrb = { self.borrow().mrb };
        let exc = unsafe { (*mrb).exc };
        if exc.is_null() {
            trace!("Last eval had no runtime errors: mrb_state has no current exception");
            return None;
        }
        let error = unsafe {
            // Do operations that can early return before accesing the GC arena
            let inspect = CString::new("inspect").ok()?;
            let unshift = CString::new("unshift").ok()?;

            // We are about to create some temporary objects with the C API.
            // Create a savepoint so we can clean them up when we are done.
            let arena = self.create_arena_savepoint();
            // Generate an exception backtrace in a `String` by executing the
            // following Ruby code with the C API:
            //
            // ```ruby
            // exception = exc.inspect
            // backtrace = exc.backtrace
            // backtrace.unshift(exception)
            // backtrace.join("\n")
            // ```
            let exc = exc as *mut c_void;
            let exception = sys::mrb_funcall(mrb, sys::mrb_sys_obj_value(exc), inspect.as_ptr(), 0);
            let backtrace = sys::mrb_exc_backtrace(mrb, sys::mrb_sys_obj_value(exc));
            sys::mrb_funcall(mrb, backtrace, unshift.as_ptr(), 1, exception);

            let error = <Vec<String>>::try_from_mrb(self, Value::new(Rc::clone(self), backtrace));
            // Mark all C created objects as garbage now that we've extracted a
            // Rust value.
            self.restore_arena(arena);

            // Clear the current exception from the mruby interpreter so
            // subsequent calls to eval are not tainted by an error they did not
            // generate.
            (*mrb).exc = std::ptr::null_mut();
            error
        };
        error.ok().map(|exception| exception.join("\n"))
    }

    fn def_file<T>(&mut self, filename: T, require: fn(Self))
    where
        T: AsRef<str>,
    {
        self.borrow_mut()
            .file_registry
            .insert(filename.as_ref().to_owned(), Box::new(require));
    }

    fn def_file_for_type<T, F>(&mut self, filename: T)
    where
        T: AsRef<str>,
        F: MrbFile,
    {
        self.def_file(filename.as_ref(), F::require);
    }

    fn nil(&self) -> Value {
        let nil = None::<Value>;
        unsafe { Value::try_from_mrb(self, nil) }.expect("None -> nil conversion is infallible")
    }

    fn bool(&self, b: bool) -> Value {
        unsafe { Value::try_from_mrb(self, b) }.expect("bool conversion is infallible")
    }

    fn bytes<T: AsRef<[u8]>>(&self, b: T) -> Value {
        unsafe { Value::try_from_mrb(self, b.as_ref()) }.expect("bytes conversion is infallible")
    }

    fn fixnum(&self, i: Int) -> Value {
        unsafe { Value::try_from_mrb(self, i) }.expect("fixnum conversion is infallible")
    }

    fn float(&self, i: Float) -> Value {
        unsafe { Value::try_from_mrb(self, i) }.expect("float conversion is infallible")
    }

    fn string<T: AsRef<str>>(&self, s: T) -> Value {
        self.bytes(s.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(
            result,
            Err(MrbError::Exec("ArgumentError: waffles".to_owned()))
        );
    }

    #[test]
    // Test that require behaves as expected:
    // - require side effects (e.g. ivar set or class def) affect the interpreter
    // - Successful first require returns `true`.
    // - Second require returns `false`.
    // - Second require does not cause require side effects.
    // - Require non-existing file raises and returns `nil`.
    fn require() {
        struct InterpreterRequireTest;
        impl MrbFile for InterpreterRequireTest {
            fn require(interp: Mrb) {
                interp.eval("@i = 255").expect("eval");
            }
        }

        unsafe {
            let mut interp = Interpreter::create().expect("mrb init");
            interp.def_file_for_type::<_, InterpreterRequireTest>("interpreter-require-test");
            let result = interp
                .eval("require 'interpreter-require-test'")
                .expect("eval");
            let require_result = bool::try_from_mrb(&interp, result);
            assert_eq!(require_result, Ok(true));
            let result = interp.eval("@i").expect("eval");
            let i_result = i64::try_from_mrb(&interp, result);
            assert_eq!(i_result, Ok(255));
            let result = interp
                .eval("@i = 1000; require 'interpreter-require-test'")
                .expect("eval");
            let second_require_result = bool::try_from_mrb(&interp, result);
            assert_eq!(second_require_result, Ok(false));
            let result = interp.eval("@i").expect("eval");
            let second_i_result = i64::try_from_mrb(&interp, result);
            assert_eq!(second_i_result, Ok(1000));
            let result = interp.eval("require 'non-existent-source'").map(|_| ());
            assert_eq!(
                result,
                Err(MrbError::Exec(
                    "RuntimeError: cannot load such file -- non-existent-source".to_owned()
                ))
            );
        }
    }

    #[test]
    fn enable_disable_gc() {
        let interp = Interpreter::create().expect("mrb init");
        interp.disable_gc();
        let arena = interp.create_arena_savepoint();
        interp
            .eval(
                r#"
                # this value will be garbage collected because it is eventually
                # shadowed and becomes unreachable
                a = []
                # this value will not be garbage collected because it is a local
                # variable in top self
                a = []
                # this value will not be garbage collected because it is a local
                # variable in top self
                b = []
                # this value will not be garbage collected because the last value
                # returned by eval is retained with "stack keep"
                []
                "#,
            )
            .expect("eval");
        let live = interp.live_object_count();
        interp.full_gc();
        assert_eq!(
            interp.live_object_count(),
            live,
            "GC is disabled. No objects should be collected"
        );
        interp.restore_arena(arena);
        interp.enable_gc();
        interp.full_gc();
        assert_eq!(
            interp.live_object_count(),
            live - 2,
            "Arrays should be collected after enabling GC and running a full GC"
        );
    }

    mod functional {
        use super::*;

        #[test]
        fn empty_eval() {
            let interp = Interpreter::create().expect("mrb init");
            let arena = interp.create_arena_savepoint();
            let live = interp.live_object_count();
            drop(interp.eval("").expect("eval"));
            interp.restore_arena(arena);
            interp.full_gc();
            assert_eq!(interp.live_object_count(), live);
        }

        #[test]
        fn gc() {
            let slack = 1; // The most recent evaled object is always live
            let interp = Interpreter::create().expect("mrb init");
            let initial_objects = interp.live_object_count();
            let initial_arena = interp.create_arena_savepoint();
            for _ in 0..2000 {
                let arena = interp.create_arena_savepoint();
                let result = interp.eval("'gc test'");
                let value = result.unwrap();
                assert!(!value.is_dead());
                interp.restore_arena(arena);
                interp.incremental_gc();
            }
            interp.restore_arena(initial_arena);
            interp.full_gc();
            let ending_arena = interp.create_arena_savepoint();
            assert!(
                interp.live_object_count() <= initial_objects + slack,
                "Started with {} live ojectes, ended with {}. Potential memory leak!",
                initial_objects,
                interp.live_object_count()
            );
            assert_eq!(
                ending_arena.0,
                initial_arena.0,
                "After 2000 iterations, the GC arena has grown to {} objects. Potential memory leak!",
                ending_arena.0 - initial_arena.0
            );
        }
    }
}
