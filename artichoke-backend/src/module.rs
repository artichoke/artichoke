use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashSet;
use std::convert::AsRef;
use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::def::{EnclosingRubyScope, Method};
use crate::method;
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

#[derive(Clone)]
pub struct Builder<'a> {
    interp: &'a Artichoke,
    spec: &'a Spec,
    methods: HashSet<method::Spec>,
}

impl<'a> Builder<'a> {
    pub fn for_spec(interp: &'a Artichoke, spec: &'a Spec) -> Self {
        Self {
            interp,
            spec,
            methods: HashSet::default(),
        }
    }

    pub fn add_method(mut self, name: &str, method: Method, args: sys::mrb_aspec) -> Self {
        let spec = method::Spec::new(method::Type::Instance, name, method, args);
        self.methods.insert(spec);
        self
    }

    pub fn add_self_method(mut self, name: &str, method: Method, args: sys::mrb_aspec) -> Self {
        let spec = method::Spec::new(method::Type::Class, name, method, args);
        self.methods.insert(spec);
        self
    }

    pub fn add_module_method(mut self, name: &str, method: Method, args: sys::mrb_aspec) -> Self {
        let spec = method::Spec::new(method::Type::Module, name, method, args);
        self.methods.insert(spec);
        self
    }

    pub fn define(self) -> Result<(), ArtichokeError> {
        let mrb = self.interp.0.borrow().mrb;
        let rclass = if let Some(rclass) = self.spec.rclass(self.interp) {
            rclass
        } else if let Some(scope) = self.spec.enclosing_scope() {
            let scope = scope.rclass(self.interp).ok_or_else(|| {
                ArtichokeError::NotDefined(Cow::Owned(scope.fqname().into_owned()))
            })?;
            unsafe { sys::mrb_define_module_under(mrb, scope, self.spec.name_c_str().as_ptr()) }
        } else {
            unsafe { sys::mrb_define_module(mrb, self.spec.name_c_str().as_ptr()) }
        };
        for method in self.methods {
            unsafe {
                method.define(self.interp, rclass)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Spec {
    name: Cow<'static, str>,
    sym: Cell<sys::mrb_sym>,
    cstring: CString,
    enclosing_scope: Option<Box<EnclosingRubyScope>>,
}

impl Spec {
    pub fn new<T>(name: T, enclosing_scope: Option<EnclosingRubyScope>) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        let name = name.into();
        let cstring = CString::new(name.as_ref()).expect("name for data type");
        Self {
            name,
            cstring,
            sym: Cell::default(),
            enclosing_scope: enclosing_scope.map(Box::new),
        }
    }

    pub fn value(&self, interp: &Artichoke) -> Option<Value> {
        let rclass = self.rclass(interp)?;
        let module = unsafe { sys::mrb_sys_module_value(rclass) };
        Some(Value::new(interp, module))
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn name_c_str(&self) -> &CStr {
        self.cstring.as_c_str()
    }

    pub fn enclosing_scope(&self) -> Option<&EnclosingRubyScope> {
        self.enclosing_scope.as_ref().map(Box::as_ref)
    }

    pub fn fqname(&self) -> Cow<'_, str> {
        if let Some(scope) = self.enclosing_scope() {
            Cow::Owned(format!("{}::{}", scope.fqname(), self.name()))
        } else {
            match &self.name {
                Cow::Borrowed(name) => Cow::Borrowed(name),
                Cow::Owned(name) => Cow::Borrowed(name.as_str()),
            }
        }
    }

    pub fn rclass(&self, interp: &Artichoke) -> Option<*mut sys::RClass> {
        let mrb = interp.0.borrow().mrb;
        if self.sym.get() == 0 {
            let sym = interp
                .0
                .borrow_mut()
                .sym_intern(self.name.as_bytes().to_vec());
            self.sym.set(sym);
        }
        if let Some(ref scope) = self.enclosing_scope {
            if let Some(scope) = scope.rclass(interp) {
                let defined = unsafe {
                    sys::mrb_const_defined_at(
                        mrb,
                        sys::mrb_sys_obj_value(scope as *mut c_void),
                        self.sym.get(),
                    )
                };
                if defined == 0 {
                    // Enclosing scope exists and module is NOT defined under
                    // the enclosing scope.
                    None
                } else {
                    // Enclosing scope exists module IS defined under the
                    // enclosing scope.
                    Some(unsafe {
                        sys::mrb_module_get_under(mrb, scope, self.name_c_str().as_ptr())
                    })
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
                    self.sym.get(),
                )
            };
            if defined == 0 {
                // Module does NOT exist in root scop.
                None
            } else {
                // Module exists in root scope.
                Some(unsafe { sys::mrb_module_get(mrb, self.name_c_str().as_ptr()) })
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

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;

    use crate::class;
    use crate::def::EnclosingRubyScope;
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
        let scope = EnclosingRubyScope::module(&scope);
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
            .eval(b"module Foo; module Bar; end; end")
            .expect("eval");
        let scope = Spec::new("Foo", None);
        let scope = EnclosingRubyScope::module(&scope);
        let spec = Spec::new("Bar", Some(scope));
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_module_under_class() {
        let interp = crate::interpreter().expect("init");
        interp
            .eval(b"class Foo; module Bar; end; end")
            .expect("eval");
        let scope = class::Spec::new("Foo", None, None);
        let scope = EnclosingRubyScope::class(&scope);
        let spec = Spec::new("Bar", Some(scope));
        assert!(spec.rclass(&interp).is_some());
    }
}
