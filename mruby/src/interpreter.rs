use log::{debug, error, trace, warn};
use mruby_vfs::FileSystem;
use std::cell::RefCell;
use std::convert::AsRef;
use std::error;
use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use crate::class;
use crate::convert::{Error, Float, Int, TryFromMrb};
use crate::def::{ClassLike, Define};
use crate::gc::GarbageCollection;
use crate::module;
use crate::state::{State, VfsMetadata};
use crate::sys::{self, DescribeState};
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

pub const RUBY_LOAD_PATH: &str = "/src/lib";

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

fn raise_load_error(interp: &Mrb, file: &str) -> sys::mrb_value {
    let eclass = CString::new("LoadError").expect("RuntimeError class");
    let message = format!("cannot load such file -- {}", file);
    let msg = CString::new(message).expect("error message");
    unsafe { sys::mrb_sys_raise(interp.borrow().mrb, eclass.as_ptr(), msg.as_ptr()) };
    debug!("Failed require '{}' on {:?}", file, interp.borrow());
    interp.bool(false).inner()
}

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

        let mut path = PathBuf::from(&name);
        if path.is_relative() {
            path = PathBuf::from(RUBY_LOAD_PATH);
        }
        let (path, metadata) = {
            let api = interp.borrow();
            let candidates = vec![
                path.join(&name),
                path.join(format!("{}.rb", name)),
                path.join(format!("{}.mrb", name)),
            ];
            let path = candidates.into_iter().find(|path| api.vfs.is_file(path));
            let metadata = path.as_ref().and_then(|path| api.vfs.metadata(path));
            (path.clone(), metadata)
        };
        if let Some(ref path) = path {
            if let Some(metadata) = metadata {
                if metadata.is_already_required() {
                    return interp.bool(false).inner();
                }
                if let Some(require) = metadata.require {
                    // dynamic, Rust-backed require
                    require(Rc::clone(&interp));
                } else {
                    // source-backed require
                    let contents = {
                        let api = interp.borrow();
                        api.vfs.read_file(path)
                    };
                    // this should be infallible because the mrb interpreter is
                    // single threaded.
                    if let Ok(contents) = contents {
                        unwrap_or_raise!(interp, interp.eval(contents));
                    } else {
                        return raise_load_error(&interp, &name);
                    }
                }
                {
                    let api = interp.borrow();
                    unwrap_or_raise!(
                        interp,
                        api.vfs
                            .set_metadata(path, metadata.mark_required())
                            .map(|_| interp.nil())
                    );
                }
            } else {
                // maybe a source-backed require
                {
                    let api = interp.borrow();
                    // this should be infallible because the mrb interpreter
                    // is single threaded.
                    if let Ok(contents) = api.vfs.read_file(path) {
                        unwrap_or_raise!(interp, interp.eval(contents));
                    }
                    // Create the missing metadata struct to prevent double
                    // requires.
                    let metadata = VfsMetadata::new(None).mark_required();
                    unwrap_or_raise!(
                        interp,
                        api.vfs.set_metadata(path, metadata).map(|_| interp.nil())
                    );
                }
            }
            trace!(
                r#"Successful require of "{}" at {:?} on {:?}"#,
                name,
                path,
                interp.borrow()
            );
            interp.bool(true).inner()
        } else {
            raise_load_error(&interp, &name)
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

            // Add global extension functions
            // Support for requiring files via `Kernel#require`
            let mut kernel = module::Spec::new("Kernel", None);
            kernel.add_self_method("require", require, sys::mrb_args_rest());
            kernel.define(&interp).map_err(|_| MrbError::New)?;
            trace!("Installed Kernel#require on {}", mrb.debug());
            let exception = class::Spec::new("Exception", None, None);
            let mut script_error = class::Spec::new("ScriptError", None, None);
            script_error.with_super_class(Rc::new(exception));
            script_error.define(&interp).map_err(|_| MrbError::New)?;
            let mut load_error = class::Spec::new("LoadError", None, None);
            load_error.with_super_class(Rc::new(script_error));
            load_error.define(&interp).map_err(|_| MrbError::New)?;
            trace!("Installed LoadError on {}", mrb.debug());

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

#[derive(Debug)]
pub enum MrbError {
    ConvertToRuby(Error<Rust, Ruby>),
    ConvertToRust(Error<Ruby, Rust>),
    Exec(String),
    New,
    Uninitialized,
    Vfs(io::Error),
}

impl Eq for MrbError {}

impl PartialEq for MrbError {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

impl fmt::Display for MrbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MrbError::Exec(backtrace) => write!(f, "mruby exception: {}", backtrace),
            MrbError::New => write!(f, "failed to create mrb interpreter"),
            MrbError::ConvertToRuby(inner) => write!(f, "conversion error: {}", inner),
            MrbError::ConvertToRust(inner) => write!(f, "conversion error: {}", inner),
            MrbError::Uninitialized => write!(f, "mrb interpreter not initialized"),
            MrbError::Vfs(err) => write!(f, "mrb vfs io error: {}", err),
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
            MrbError::Vfs(inner) => Some(inner),
            _ => None,
        }
    }
}

/// `MrbApi` is the mutable API around the [`MrbState`]. `MrbApi` should provide
/// safe wrappers around unsafe functions from [`mruby_sys`] and the
/// [`TryFromMrb`] converters.
pub trait MrbApi {
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>;

    fn current_exception(&self) -> Option<String>;

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
            arena.restore();

            // Clear the current exception from the mruby interpreter so
            // subsequent calls to eval are not tainted by an error they did not
            // generate.
            (*mrb).exc = std::ptr::null_mut();
            error
        };
        error.ok().map(|exception| exception.join("\n"))
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
    use crate::convert::TryFromMrb;
    use crate::file::MrbFile;
    use crate::interpreter::{Interpreter, Mrb, MrbApi, MrbError};
    use crate::load::MrbLoadSources;
    use crate::sys;

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
            interp
                .def_file_for_type::<_, InterpreterRequireTest>("interpreter-require-test")
                .expect("def file");
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
                    "LoadError: cannot load such file -- non-existent-source".to_owned()
                ))
            );
        }
    }

    #[test]
    fn require_absolute_path() {
        let mut interp = Interpreter::create().expect("mrb init");
        interp
            .def_rb_source_file("/foo/bar/source.rb", "# a source file")
            .expect("def file");
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(unsafe { bool::try_from_mrb(&interp, result).expect("convert") });
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(!unsafe { bool::try_from_mrb(&interp, result).expect("convert") });
    }

    #[test]
    fn require_directory() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp.eval("require '/src'").map(|_| ());
        let expected = Err(MrbError::Exec(
            "LoadError: cannot load such file -- /src".to_owned(),
        ));
        assert_eq!(result, expected);
    }
}
