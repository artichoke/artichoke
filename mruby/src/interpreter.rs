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
        let api = MrbApi::new()?;
        Ok(Rc::new(RefCell::new(api)))
    }

    pub unsafe fn from_user_data(mrb: *mut mrb_state) -> Result<*mut Mrb, MrbError> {
        if mrb.is_null() {
            return Err(MrbError::Uninitialized);
        }
        let mrb = (*mrb).ud as *mut _ as *mut Mrb;
        if mrb.is_null() {
            return Err(MrbError::Uninitialized);
        }
        Ok(mrb)
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
    data_types: HashMap<String, (CString, mrb_data_type)>,
}

impl Drop for MrbApi {
    fn drop(&mut self) {
        unsafe { mrb_close(self.mrb) };
    }
}

impl MrbApi {
    fn new() -> Result<Self, MrbError> {
        let mrb = unsafe { mrb_open() };
        if mrb.is_null() {
            Err(MrbError::New)
        } else {
            Ok(Self {
                mrb,
                data_types: HashMap::new(),
            })
        }
    }

    pub fn close(self) {
        drop(self)
    }

    pub fn mrb(&self) -> *mut mrb_state {
        self.mrb
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
            let err = Interpreter::from_user_data(mrb);
            assert_eq!(err.err(), Some(MrbError::Uninitialized));
        }
    }

    #[test]
    fn from_user_data() {
        unsafe {
            let mut interp = Interpreter::create().expect("mrb init");
            let ptr = &mut interp as *mut _ as *mut std::ffi::c_void;
            let mrb = interp.borrow_mut().mrb();
            (*mrb).ud = ptr;
            let res = Interpreter::from_user_data(mrb);
            assert!(res.is_ok());
        }
    }
}
