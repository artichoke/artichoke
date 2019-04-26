use mruby_sys::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::ffi::{CStr, CString};
use std::rc::Rc;

use crate::convert::*;
use crate::file::MrbFile;
use crate::value::*;

pub type Mrb = Rc<RefCell<MrbApi>>;
pub type MrbFreeFunc = unsafe extern "C" fn(mrb: *mut mrb_state, arg1: *mut std::ffi::c_void);
pub type RequireFunc = fn(interp: Mrb);

pub struct Interpreter;

extern "C" fn require(mrb: *mut mrb_state, _slf: mrb_value) -> mrb_value {
    unsafe {
        let interp = Interpreter::from_user_data(mrb).expect("interpreter");
        // Extract required filename from arguments
        let name = std::mem::uninitialized::<*const std::os::raw::c_char>();
        let argspec = CString::new(specifiers::CSTRING).expect("argspec");
        mrb_get_args(
            mrb,
            argspec.as_ptr(),
            &name,
        );
        let name = CStr::from_ptr(name).to_str().expect("required filename");

        if { interp.borrow().required_files.contains(name) } {
            return interp.borrow().bool(false).expect("bool").inner();
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

        match req {
            Some(req) => {
                req(Rc::clone(&interp));
                {
                    let mut borrow = interp.borrow_mut();
                    borrow.required_files.insert(name.to_owned());
                }
                interp.borrow().bool(true).expect("bool").inner()
            }
            None => {
                let borrow = interp.borrow();
                let eclass = CString::new("RuntimeError").expect("RuntimeError class");
                let message = format!("cannot load such file -- {}", name);
                let msg = CString::new(message).expect("error message");
                mrb_sys_raise(borrow.mrb(), eclass.as_ptr(), msg.as_ptr());
                borrow.bool(false).expect("bool").inner()
            }
        }
    }
}

impl Interpreter {
    pub fn create() -> Result<Mrb, MrbError> {
        unsafe {
            let mrb = mrb_open();
            if mrb.is_null() {
                return Err(MrbError::New);
            }

            let context = mrbc_context_new(mrb);
            let api = Rc::new(RefCell::new(MrbApi {
                mrb,
                ctx: context,
                data_types: HashMap::new(),
                file_registry: HashMap::new(),
                required_files: HashSet::new(),
            }));

            // Clone the smart pointer that wraps the API and store it in the
            // user data of the mrb interpreter. After this operation,
            // `Rc::strong_count` will be 2.
            let ud = Rc::clone(&api);
            let ptr = std::mem::transmute::<Mrb, *mut std::ffi::c_void>(ud);
            (*mrb).ud = ptr;

            // Add global extension functions
            // Support for requiring files via `Kernel#require`
            let kernel = CString::new("Kernel").expect("Kernel module");
            let kernel_module = mrb_module_get(mrb, kernel.as_ptr());
            let require_method = CString::new("require").expect("require method");
            let aspec = mrb_args_rest();
            mrb_define_module_function(
                mrb,
                kernel_module,
                require_method.as_ptr(),
                Some(require),
                aspec,
            );

            Ok(api)
        }
    }

    // TODO: Add a benchmark to make sure this function does not leak memory.
    pub unsafe fn from_user_data(mrb: *mut mrb_state) -> Result<Mrb, MrbError> {
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
        let ud = std::mem::transmute::<*mut std::ffi::c_void, Mrb>(ptr);
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
    Convert(Error<Rust, Ruby>),
    New,
    Uninitialized,
}

pub struct MrbApi {
    mrb: *mut mrb_state,
    ctx: *mut mrbc_context,
    data_types: HashMap<String, (CString, mrb_data_type)>,
    file_registry: HashMap<String, Box<RequireFunc>>,
    required_files: HashSet<String>,
}

impl Drop for MrbApi {
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
                let ud = std::mem::transmute::<*mut std::ffi::c_void, Mrb>(ptr);
                // cleanup pointers
                (*self.mrb).ud = std::ptr::null_mut();
                std::mem::drop(ud);
            }

            // Free mrb data structures
            mrbc_context_free(self.mrb, self.ctx);
            mrb_close(self.mrb);
            // Cleanup dangling pointers in `MrbApi`
            self.ctx = std::ptr::null_mut();
            self.mrb = std::ptr::null_mut();
        };
    }
}

impl MrbApi {
    pub fn close(self) {
        drop(self)
    }

    pub fn mrb(&self) -> *mut mrb_state {
        self.mrb
    }

    pub fn ctx(&self) -> *mut mrbc_context {
        self.ctx
    }

    pub fn get_class_cstr<T: AsRef<str>>(&self, class: T) -> Option<&CString> {
        self.data_types
            .get(class.as_ref())
            .map(|class_def| &class_def.0)
    }

    pub fn get_or_create_data_type<T: AsRef<str>>(
        &mut self,
        class: T,
        free: Option<MrbFreeFunc>,
    ) -> &mut mrb_data_type {
        let class = class.as_ref().to_owned();
        &mut self
            .data_types
            .entry(class.clone())
            .or_insert_with(|| {
                let class = CString::new(class.clone())
                    .unwrap_or_else(|_| panic!("class {} to CString", class));
                let data_type = mrb_data_type {
                    struct_name: class.as_ptr(),
                    dfree: free,
                };
                (class, data_type)
            })
            .1
    }

    // TODO: do not expose the mrb_state
    pub unsafe fn inner(&self) -> *mut mrb_state {
        self.mrb
    }

    pub fn def_file<T>(&mut self, filename: T, require_func: RequireFunc)
    where
        T: AsRef<str>,
    {
        self.file_registry
            .insert(filename.as_ref().to_owned(), Box::new(require_func));
    }

    pub fn def_file_for_type<T, F>(&mut self, filename: T)
    where
        T: AsRef<str>,
        F: MrbFile,
    {
        let require_func = |interp: Mrb| F::require(interp);
        self.def_file(filename.as_ref(), require_func);
    }

    pub fn nil(&self) -> Result<Value, MrbError> {
        let nil: Option<Value> = None;
        unsafe { Value::try_from_mrb(self, nil) }.map_err(MrbError::Convert)
    }

    pub fn bool(&self, b: bool) -> Result<Value, MrbError> {
        unsafe { Value::try_from_mrb(self, b) }.map_err(MrbError::Convert)
    }

    pub fn bytes<T: AsRef<[u8]>>(&self, b: T) -> Result<Value, MrbError> {
        unsafe { Value::try_from_mrb(self, b.as_ref()) }.map_err(MrbError::Convert)
    }

    pub fn fixnum(&self, i: Int) -> Result<Value, MrbError> {
        unsafe { Value::try_from_mrb(self, i) }.map_err(MrbError::Convert)
    }

    pub fn string<T: AsRef<str>>(&self, s: T) -> Result<Value, MrbError> {
        unsafe { Value::try_from_mrb(self, s.as_ref()) }.map_err(MrbError::Convert)
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
            let api = interp.borrow_mut();
            let mrb = api.mrb();
            // fake null user data
            (*mrb).ud = std::ptr::null_mut();
            let err = Interpreter::from_user_data(mrb);
            assert_eq!(err.err(), Some(MrbError::Uninitialized));
        }
    }

    #[test]
    fn from_user_data() {
        unsafe {
            let mut interp = Interpreter::create().expect("mrb init");
            let ptr = &mut interp as *mut Mrb as *mut std::ffi::c_void;
            let mrb = interp.borrow_mut().mrb();
            (*mrb).ud = ptr;
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
            let (mrb, ctx) = { (interp.borrow().mrb(), interp.borrow().ctx()) };
            let code = "255";
            let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), ctx);
            assert_eq!(mrb_sys_fixnum_to_cint(result), 255);
        }
    }

    struct F;
    impl MrbFile for F {
        fn require(interp: Mrb) {
            unsafe {
                let borrow = interp.borrow();
                let mrb = borrow.mrb();
                let ctx = borrow.ctx();
                let code = "@i = 255";
                mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), ctx);
            }
        }
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
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            {
                let mut borrow = interp.borrow_mut();
                borrow.def_file_for_type::<_, F>("required-f");
                drop(borrow);
            }
            let result = {
                let (mrb, ctx) = {
                    let borrow = interp.borrow();
                    (borrow.mrb(), borrow.ctx())
                };
                let code = "require 'required-f'";
                Value::new(mrb_load_nstring_cxt(
                    mrb,
                    code.as_ptr() as *const i8,
                    code.len(),
                    ctx,
                ))
            };
            let require_result = bool::try_from_mrb(&interp.borrow(), result);
            assert_eq!(require_result, Ok(true));
            let result = {
                let (mrb, ctx) = {
                    let borrow = interp.borrow();
                    (borrow.mrb(), borrow.ctx())
                };
                let code = "@i";
                Value::new(mrb_load_nstring_cxt(
                    mrb,
                    code.as_ptr() as *const i8,
                    code.len(),
                    ctx,
                ))
            };
            let i_result = i64::try_from_mrb(&interp.borrow(), result);
            assert_eq!(i_result, Ok(255));
            let result = {
                let (mrb, ctx) = {
                    let borrow = interp.borrow();
                    (borrow.mrb(), borrow.ctx())
                };
                let code = "@i = 1000; require 'required-f'";
                Value::new(mrb_load_nstring_cxt(
                    mrb,
                    code.as_ptr() as *const i8,
                    code.len(),
                    ctx,
                ))
            };
            let second_require_result = bool::try_from_mrb(&interp.borrow(), result);
            assert_eq!(second_require_result, Ok(false));
            let result = {
                let (mrb, ctx) = {
                    let borrow = interp.borrow();
                    (borrow.mrb(), borrow.ctx())
                };
                let code = "@i";
                Value::new(mrb_load_nstring_cxt(
                    mrb,
                    code.as_ptr() as *const i8,
                    code.len(),
                    ctx,
                ))
            };
            let second_i_result = i64::try_from_mrb(&interp.borrow(), result);
            assert_eq!(second_i_result, Ok(1000));
            let result = {
                let (mrb, ctx) = {
                    let borrow = interp.borrow();
                    (borrow.mrb(), borrow.ctx())
                };
                let code = "require 'non-existent-f'";
                Value::new(mrb_load_nstring_cxt(
                    mrb,
                    code.as_ptr() as *const i8,
                    code.len(),
                    ctx,
                ))
            };
            let bad_require_result = <Option<bool>>::try_from_mrb(&interp.borrow(), result);
            assert_eq!(bad_require_result, Ok(None));
            let api = interp.borrow();
            let exc = Value::new(mrb_sys_get_current_exception(api.mrb()));
            let exc = String::try_from_mrb(&api, exc);
            let expected = "RuntimeError: cannot load such file -- non-existent-f".to_owned();
            assert_eq!(exc, Ok(expected.to_owned()));
        }
    }
}
