use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::error;
use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::rc::Rc;

use crate::convert::*;
use crate::file::MrbFile;
use crate::sys;
use crate::value::*;

#[macro_export]
macro_rules! interpreter_or_raise {
    ($mrb:expr) => {
        match $crate::Interpreter::from_user_data($mrb) {
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
                return $crate::MrbApi::nil(&$interp).inner();
            }
            std::result::Result::Ok(value) => value.inner(),
        }
    };
}

pub type Mrb = Rc<RefCell<MrbState>>;
pub type MrbFreeFunc = unsafe extern "C" fn(mrb: *mut sys::mrb_state, arg1: *mut c_void);
pub type RequireFunc = fn(interp: Mrb);

extern "C" fn require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    unsafe {
        let interp = interpreter_or_raise!(mrb);
        // Extract required filename from arguments
        let name = std::mem::uninitialized::<*const std::os::raw::c_char>();
        let argspec = CString::new(sys::specifiers::CSTRING).expect("argspec");
        sys::mrb_get_args(mrb, argspec.as_ptr(), &name);
        let name = match CStr::from_ptr(name).to_str() {
            Ok(name) => name,
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
            borrow.required_files.contains(name)
        };
        if already_required {
            return interp.bool(false).inner();
        }

        let req = {
            let borrow = interp.borrow();
            borrow
                .file_registry
                .get(name)
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
            interp.bool(true).inner()
        } else {
            let eclass = CString::new("RuntimeError").expect("RuntimeError class");
            let message = format!("cannot load such file -- {}", name);
            let msg = CString::new(message).expect("error message");
            sys::mrb_sys_raise(interp.borrow().mrb, eclass.as_ptr(), msg.as_ptr());
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
                return Err(MrbError::New);
            }

            let context = sys::mrbc_context_new(mrb);
            let api = Rc::new(RefCell::new(MrbState {
                mrb,
                ctx: context,
                data_types: HashMap::new(),
                file_registry: HashMap::new(),
                required_files: HashSet::new(),
            }));

            // Transmute the smart pointer that wraps the API and store it in
            // the user data of the mrb interpreter. After this operation,
            // `Rc::strong_count` will still be 1.
            let ptr = std::mem::transmute::<Mrb, *mut c_void>(api);
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

            // Transmute the void * pointer to the Rc back into the Mrb type.
            // After this operation `Rc::strong_count` will still be 1. This
            // dance is required to avoid leaking Mrb objects, which will let
            // the `Drop` impl close the mrb context and interpreter.
            Ok(std::mem::transmute::<*mut c_void, Mrb>(ptr))
        }
    }

    // TODO: Add a benchmark to make sure this function does not leak memory.
    pub unsafe fn from_user_data(mrb: *mut sys::mrb_state) -> Result<Mrb, MrbError> {
        if mrb.is_null() {
            return Err(MrbError::Uninitialized);
        }
        let ptr = (*mrb).ud;
        if ptr.is_null() {
            return Err(MrbError::Uninitialized);
        }
        // Extract the smart pointer that wraps the API from the user data on
        // the mrb interpreter. The `mrb_state` should retain ownership of its
        // copy of the smart pointer.
        let ud = std::mem::transmute::<*mut c_void, Mrb>(ptr);
        // Clone the API smart pointer and increase its ref count to return a
        // reference to the caller.
        let api = Rc::clone(&ud);
        // Forget the transmuted API extracted from the user data to make sure
        // the `mrb_state` maintains ownership and the smart pointer does not
        // get deallocated before `mrb_close` is called.
        std::mem::forget(ud);
        // At this point, `Rc::strong_count` will be increased by 1.
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

pub struct MrbState {
    // TODO: Make this private
    pub mrb: *mut sys::mrb_state,
    // TODO: Make this private
    pub ctx: *mut sys::mrbc_context,
    data_types: HashMap<String, (CString, sys::mrb_data_type)>,
    file_registry: HashMap<String, Box<RequireFunc>>,
    required_files: HashSet<String>,
}

impl MrbState {
    pub fn close(self) {
        drop(self)
    }

    // NOTE: This function must return a reference. mruby expects mrb_data_type
    // structs to live the life of the mrb interpreter. If a data type is
    // deallocated because it is dropped, mrb_close will segfault.
    pub fn get_or_create_data_type<T: AsRef<str>>(
        &mut self,
        class: T,
        free: Option<MrbFreeFunc>,
    ) -> &sys::mrb_data_type {
        let class = class.as_ref().to_owned();
        &self
            .data_types
            .entry(class.clone())
            .or_insert_with(|| {
                let class = CString::new(class).expect("class for data type");
                let data_type = sys::mrb_data_type {
                    struct_name: class.as_ptr(),
                    dfree: free,
                };
                (class, data_type)
            })
            .1
    }
}

impl Drop for MrbState {
    fn drop(&mut self) {
        unsafe {
            // At this point, the only ref to the smart poitner wrapping the
            // API is stored in the `mrb_state->ud` pointer. Rematerialize the
            // `Rc`, set the userdata pointer to null, and drop the `Rc` to
            // ensure no memory leaks. After this operation, `Rc::strong_count`
            // will be 0 and the `Rc`, `RefCell`, and `MrbApi` will be
            // deallocated.
            let ptr = (*self.mrb).ud;
            if !ptr.is_null() {
                let ud = std::mem::transmute::<*mut c_void, Mrb>(ptr);
                // cleanup pointers
                (*self.mrb).ud = std::ptr::null_mut();
                std::mem::drop(ud);
            }

            // Free mrb data structures
            sys::mrbc_context_free(self.mrb, self.ctx);
            sys::mrb_close(self.mrb);
            // Cleanup dangling pointers in `MrbApi`
            self.ctx = std::ptr::null_mut();
            self.mrb = std::ptr::null_mut();
        };
    }
}

/// `MrbApi` is the mutable API around the [`MrbState`]. `MrbApi` should provide
/// safe wrappers around unsafe functions from [`mruby_sys`] and the
/// [`TryFromMrb`] converters.
pub trait MrbApi {
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>;

    fn def_file<T>(&mut self, filename: T, require_func: RequireFunc)
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

// We need to implement the `MrbApi` on the `Rc` instead of the `MrbState`
// because we need to not have a borrow on the `MrbState` when evaling code
// because eval may recurse through the interpreter, e.g. a nested require.
impl MrbApi for Mrb {
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>,
    {
        let (mrb, ctx) = {
            let borrow = self.borrow();
            (borrow.mrb, borrow.ctx)
        };
        let code = code.as_ref();
        let result =
            unsafe { sys::mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), ctx) };
        let exception = Value::new(unsafe { sys::mrb_sys_get_current_exception(mrb) });
        let exception = unsafe { <Option<String>>::try_from_mrb(&self, exception) };
        if let Some(backtrace) = exception.map_err(MrbError::ConvertToRust)? {
            return Err(MrbError::Exec(backtrace));
        }
        Ok(Value::new(result))
    }

    fn def_file<T>(&mut self, filename: T, require_func: RequireFunc)
    where
        T: AsRef<str>,
    {
        self.borrow_mut()
            .file_registry
            .insert(filename.as_ref().to_owned(), Box::new(require_func));
    }

    fn def_file_for_type<T, F>(&mut self, filename: T)
    where
        T: AsRef<str>,
        F: MrbFile,
    {
        let require_func = |interp: Self| F::require(interp);
        self.def_file(filename.as_ref(), require_func);
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
    // - require side effects (e.g. ivar set or class def) affect the
    //   interpreter
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
}
