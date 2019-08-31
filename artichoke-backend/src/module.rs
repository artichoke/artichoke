use std::collections::HashSet;
use std::convert::AsRef;
use std::ffi::{c_void, CString};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::def::{ClassLike, Define, EnclosingRubyScope, Method};
use crate::method;
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub struct Spec {
    name: String,
    cstring: CString,
    methods: HashSet<method::Spec>,
    enclosing_scope: Option<EnclosingRubyScope>,
}

impl Spec {
    pub fn new<T>(name: T, enclosing_scope: Option<EnclosingRubyScope>) -> Self
    where
        T: AsRef<str>,
    {
        let cstr = CString::new(name.as_ref()).expect("name for data type");
        Self {
            name: name.as_ref().to_owned(),
            cstring: cstr,
            methods: HashSet::new(),
            enclosing_scope,
        }
    }

    pub fn value(&self, interp: &Artichoke) -> Option<Value> {
        let rclass = self.rclass(interp)?;
        let module = unsafe { sys::mrb_sys_module_value(rclass) };
        Some(Value::new(interp, module))
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

    fn enclosing_scope(&self) -> Option<EnclosingRubyScope> {
        self.enclosing_scope.clone()
    }

    fn rclass(&self, interp: &Artichoke) -> Option<*mut sys::RClass> {
        let mrb = interp.0.borrow().mrb;
        if let Some(ref scope) = self.enclosing_scope {
            if let Some(scope) = scope.rclass(interp) {
                let defined = unsafe {
                    sys::mrb_const_defined_at(
                        mrb,
                        sys::mrb_sys_obj_value(scope as *mut c_void),
                        interp.0.borrow_mut().sym_intern(self.name.as_str()),
                    )
                };
                if defined == 0 {
                    // Enclosing scope exists and module is NOT defined under
                    // the enclosing scope.
                    None
                } else {
                    // Enclosing scope exists module IS defined under the
                    // enclosing scope.
                    Some(unsafe { sys::mrb_module_get_under(mrb, scope, self.cstring().as_ptr()) })
                }
            } else {
                // Enclosing scope does not exist.
                None
            }
        } else {
            let defined = unsafe {
                sys::mrb_const_defined_at(
                    mrb,
                    sys::mrb_sys_obj_value((*mrb).object_class as *mut c_void),
                    interp.0.borrow_mut().sym_intern(self.name.as_str()),
                )
            };
            if defined == 0 {
                // Module does NOT exist in root scop.
                None
            } else {
                // Module exists in root scope.
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
        write!(f, "artichoke module spec -- {}", self.fqname())
    }
}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
        self.enclosing_scope().hash(state);
    }
}

impl Eq for Spec {}

impl PartialEq for Spec {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Define for Spec {
    fn define(&self, interp: &Artichoke) -> Result<*mut sys::RClass, ArtichokeError> {
        let mrb = interp.0.borrow().mrb;
        let rclass = if let Some(rclass) = self.rclass(interp) {
            rclass
        } else if let Some(ref scope) = self.enclosing_scope {
            let scope = scope
                .rclass(interp)
                .ok_or_else(|| ArtichokeError::NotDefined(scope.fqname()))?;
            unsafe { sys::mrb_define_module_under(mrb, scope, self.cstring().as_ptr()) }
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
    use crate::def::{ClassLike, EnclosingRubyScope};
    use crate::eval::Eval;
    use crate::module::Spec;

    #[test]
    fn rclass_for_undef_root_module() {
        let interp = crate::interpreter().expect("init");
        let spec = Spec::new("Foo", None);
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_undef_nested_module() {
        let interp = crate::interpreter().expect("init");
        let scope = Spec::new("Kernel", None);
        let scope = EnclosingRubyScope::module(Rc::new(RefCell::new(scope)));
        let spec = Spec::new("Foo", Some(scope));
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_root_module() {
        let interp = crate::interpreter().expect("init");
        let spec = Spec::new("Kernel", None);
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_module() {
        let interp = crate::interpreter().expect("init");
        interp
            .eval("module Foo; module Bar; end; end")
            .expect("eval");
        let scope = Spec::new("Foo", None);
        let scope = EnclosingRubyScope::module(Rc::new(RefCell::new(scope)));
        let spec = Spec::new("Bar", Some(scope));
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_module_under_class() {
        let interp = crate::interpreter().expect("init");
        interp
            .eval("class Foo; module Bar; end; end")
            .expect("eval");
        let scope = class::Spec::new("Foo", None, None);
        let scope = EnclosingRubyScope::class(Rc::new(RefCell::new(scope)));
        let spec = Spec::new("Bar", Some(scope));
        assert!(spec.rclass(&interp).is_some());
    }
}
