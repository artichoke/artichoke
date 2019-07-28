use std::convert::AsRef;
use std::ffi::CString;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::def::Method;
use crate::sys;
use crate::ArtichokeError;
use crate::Mrb;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Type {
    Class,
    Global,
    Instance,
    Module,
}

pub struct Spec {
    name: String,
    cstring: CString,
    method_type: Type,
    method: Method,
    args: sys::mrb_aspec,
}

impl Spec {
    pub fn new<T>(method_type: Type, method_name: T, method: Method, args: sys::mrb_aspec) -> Self
    where
        T: AsRef<str>,
    {
        let method_cstr = CString::new(method_name.as_ref()).expect("method name");
        Self {
            name: method_name.as_ref().to_owned(),
            cstring: method_cstr,
            method_type,
            method,
            args,
        }
    }

    pub fn method_type(&self) -> &Type {
        &self.method_type
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn cstring(&self) -> &CString {
        &self.cstring
    }

    pub unsafe fn define(
        &self,
        interp: &Mrb,
        into: *mut sys::RClass,
    ) -> Result<(), ArtichokeError> {
        let mrb = interp.borrow().mrb;
        match self.method_type {
            Type::Class => {
                sys::mrb_define_class_method(
                    mrb,
                    into,
                    self.cstring().as_ptr(),
                    Some(self.method),
                    self.args,
                );
                Ok(())
            }
            Type::Global => {
                sys::mrb_define_singleton_method(
                    mrb,
                    (*mrb).top_self,
                    self.cstring().as_ptr(),
                    Some(self.method),
                    self.args,
                );
                Ok(())
            }
            Type::Instance => {
                sys::mrb_define_method(
                    mrb,
                    into,
                    self.cstring().as_ptr(),
                    Some(self.method),
                    self.args,
                );
                Ok(())
            }
            Type::Module => {
                sys::mrb_define_module_function(
                    mrb,
                    into,
                    self.cstring().as_ptr(),
                    Some(self.method),
                    self.args,
                );
                Ok(())
            }
        }
    }
}

impl fmt::Debug for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.method_type() {
            Type::Class => write!(f, "mruby self method spec -- {}", self.name),
            Type::Global => write!(f, "mruby global method spec -- {}", self.name),
            Type::Instance => write!(f, "mruby instance method spec -- {}", self.name),
            Type::Module => write!(f, "mruby module method spec -- {}", self.name),
        }
    }
}

impl Eq for Spec {}

impl PartialEq for Spec {
    fn eq(&self, other: &Self) -> bool {
        self.method_type == other.method_type && self.name == other.name
    }
}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.method_type.hash(state);
    }
}
