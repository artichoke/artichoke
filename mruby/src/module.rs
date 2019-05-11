use std::collections::HashSet;
use std::convert::AsRef;
use std::ffi::CString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::def::{ClassLike, Define, Method, Parent};
use crate::interpreter::{Mrb, MrbError};
use crate::method;
use crate::sys;

pub struct Spec {
    name: String,
    cstring: CString,
    methods: HashSet<method::Spec>,
    parent: Option<Parent>,
}

impl Spec {
    pub fn new<T>(name: T, parent: Option<Parent>) -> Self
    where
        T: AsRef<str>,
    {
        let cstr = CString::new(name.as_ref()).expect("name for data type");
        Self {
            name: name.as_ref().to_owned(),
            cstring: cstr,
            methods: HashSet::new(),
            parent,
        }
    }
}

impl ClassLike for Spec {
    fn add_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec) {
        let spec = method::Spec::new(method::Type::Instance, name, method, args);
        self.methods.insert(spec);
    }

    fn add_self_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec) {
        let spec = method::Spec::new(method::Type::Module, name, method, args);
        self.methods.insert(spec);
    }

    fn cstring(&self) -> &CString {
        &self.cstring
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn parent(&self) -> Option<Parent> {
        self.parent.clone()
    }

    fn rclass(&self, interp: Mrb) -> *mut sys::RClass {
        let mrb = interp.borrow().mrb;
        if let Some(ref parent) = self.parent {
            unsafe {
                sys::mrb_module_get_under(mrb, (*parent).rclass(interp), self.cstring().as_ptr())
            }
        } else {
            unsafe { sys::mrb_module_get(mrb, self.cstring().as_ptr()) }
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
        write!(f, "mruby module spec -- {}", self.fqname())
    }
}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
        self.parent().hash(state);
    }
}

impl Eq for Spec {}

impl PartialEq for Spec {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Define for Spec {
    fn define(&self, interp: &Mrb) -> Result<*mut sys::RClass, MrbError> {
        let mrb = interp.borrow().mrb;
        let rclass = if let Some(ref parent) = self.parent {
            unsafe {
                sys::mrb_define_module_under(
                    mrb,
                    parent.rclass(Rc::clone(&interp)),
                    self.cstring().as_ptr(),
                )
            }
        } else {
            unsafe { sys::mrb_define_module(mrb, self.cstring().as_ptr()) }
        };
        for method in &self.methods {
            unsafe {
                method.define(&interp, rclass)?;
            }
        }
        Ok(rclass)
    }
}
