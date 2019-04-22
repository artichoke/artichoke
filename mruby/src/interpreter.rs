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
    pub fn new() -> Result<Mrb, MrbError> {
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

#[derive(Clone, Debug)]
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
                let class =
                    CString::new(class.clone()).expect(&format!("class {} to CString", class));
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

    //pub fn bool(&self, b: bool) -> Result<Value, MrbError> {
    //    unsafe { Value::try_from_mrb(self, b) }.map_err(MrbError::Convert)
    //}

    //pub fn bytes<T: AsRef<[u8]>>(&self, b: T) -> Result<Value, MrbError> {
    //    unsafe { Value::try_from_mrb(self, b.as_ref()) }.map_err(MrbError::Convert)
    //}

    pub fn fixnum(&self, i: Int) -> Result<Value, MrbError> {
        unsafe { Value::try_from_mrb(self, i) }.map_err(MrbError::Convert)
    }

    //pub fn string<T: AsRef<str>>(&self, s: T) -> Result<Value, MrbError> {
    //    unsafe { Value::try_from_mrb(self, s.as_ref()) }.map_err(MrbError::Convert)
    //}
}
