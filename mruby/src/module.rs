use std::collections::HashSet;
use std::convert::AsRef;
use std::ffi::{c_void, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::def::{ClassLike, Define, Method, Parent};
use crate::interpreter::Mrb;
use crate::method;
use crate::sys;
use crate::MrbError;

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

    fn rclass(&self, interp: Mrb) -> Option<*mut sys::RClass> {
        let mrb = interp.borrow().mrb;
        if let Some(ref parent) = self.parent {
            if let Some(parent) = parent.rclass(interp) {
                let defined = unsafe {
                    sys::mrb_const_defined_at(
                        mrb,
                        sys::mrb_sys_obj_value(parent as *mut c_void),
                        sys::mrb_intern_cstr(mrb, self.cstring.as_ptr()),
                    )
                };
                if defined == 0 {
                    // parent exists and module is NOT defined under parent
                    None
                } else {
                    // parent exists module is defined under parent
                    Some(unsafe { sys::mrb_module_get_under(mrb, parent, self.cstring().as_ptr()) })
                }
            } else {
                // parent does not exist
                None
            }
        } else {
            let defined = unsafe {
                sys::mrb_const_defined_at(
                    mrb,
                    sys::mrb_sys_obj_value((*mrb).object_class as *mut c_void),
                    sys::mrb_intern_cstr(mrb, self.cstring.as_ptr()),
                )
            };
            if defined == 0 {
                // class does NOT exist in root namespace
                None
            } else {
                // module exists in root namespace
                Some(unsafe { sys::mrb_module_get(mrb, self.cstring().as_ptr()) })
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
            let parent = parent
                .rclass(Rc::clone(&interp))
                .ok_or_else(|| MrbError::NotDefined(parent.fqname()))?;
            unsafe { sys::mrb_define_module_under(mrb, parent, self.cstring().as_ptr()) }
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::class;
    use crate::def::{ClassLike, Parent};
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::module::Spec;

    #[test]
    fn rclass_for_undef_root_module() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = Spec::new("Foo", None);
        assert!(spec.rclass(Rc::clone(&interp)).is_none());
    }

    #[test]
    fn rclass_for_undef_nested_module() {
        let interp = Interpreter::create().expect("mrb init");
        let parent = Spec::new("Kernel", None);
        let parent = Parent::Module {
            spec: Rc::new(RefCell::new(parent)),
        };
        let spec = Spec::new("Foo", Some(parent));
        assert!(spec.rclass(Rc::clone(&interp)).is_none());
    }

    #[test]
    fn rclass_for_root_module() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = Spec::new("Kernel", None);
        assert!(spec.rclass(Rc::clone(&interp)).is_some());
    }

    #[test]
    fn rclass_for_nested_module() {
        let interp = Interpreter::create().expect("mrb init");
        interp
            .eval("module Foo; module Bar; end; end")
            .expect("eval");
        let parent = Spec::new("Foo", None);
        let parent = Parent::Module {
            spec: Rc::new(RefCell::new(parent)),
        };
        let spec = Spec::new("Bar", Some(parent));
        assert!(spec.rclass(Rc::clone(&interp)).is_some());
    }

    #[test]
    fn rclass_for_nested_module_under_class() {
        let interp = Interpreter::create().expect("mrb init");
        interp
            .eval("class Foo; module Bar; end; end")
            .expect("eval");
        let parent = class::Spec::new("Foo", None, None);
        let parent = Parent::Class {
            spec: Rc::new(RefCell::new(parent)),
        };
        let spec = Spec::new("Bar", Some(parent));
        assert!(spec.rclass(Rc::clone(&interp)).is_some());
    }
}
