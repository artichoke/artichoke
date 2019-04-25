use mruby_sys::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::AsRef;
use std::ffi::CString;
use std::rc::Rc;

use crate::convert::*;
use crate::value::*;

pub type Mrb = Rc<RefCell<MrbApi>>;
pub type MrbFreeFunc = unsafe extern "C" fn(mrb: *mut mrb_state, arg1: *mut ::std::ffi::c_void);

pub struct Interpreter;

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
            }));

            // Clone the smart pointer that wraps the API and store it in the
            // user data of the mrb interpreter. After this operation,
            // `Rc::strong_count` will be 2.
            let ud = Rc::clone(&api);
            let ptr = std::mem::transmute::<Mrb, *mut std::ffi::c_void>(ud);
            (*mrb).ud = ptr;

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
}
