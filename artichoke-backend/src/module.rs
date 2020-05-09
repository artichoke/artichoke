use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::AsRef;
use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

use crate::core::Intern;
use crate::def::{ConstantNameError, EnclosingRubyScope, Method, NotDefinedError};
use crate::method;
use crate::sys;
use crate::Artichoke;

#[derive(Debug)]
pub struct Builder<'a> {
    interp: &'a mut Artichoke,
    spec: &'a Spec,
    methods: HashSet<method::Spec>,
}

impl<'a> Builder<'a> {
    #[must_use]
    pub fn for_spec(interp: &'a mut Artichoke, spec: &'a Spec) -> Self {
        Self {
            interp,
            spec,
            methods: HashSet::default(),
        }
    }

    pub fn add_method<T>(
        mut self,
        name: T,
        method: Method,
        args: sys::mrb_aspec,
    ) -> Result<Self, ConstantNameError>
    where
        T: Into<Cow<'static, str>>,
    {
        let spec = method::Spec::new(method::Type::Instance, name.into(), method, args)?;
        self.methods.insert(spec);
        Ok(self)
    }

    pub fn add_self_method<T>(
        mut self,
        name: T,
        method: Method,
        args: sys::mrb_aspec,
    ) -> Result<Self, ConstantNameError>
    where
        T: Into<Cow<'static, str>>,
    {
        let spec = method::Spec::new(method::Type::Class, name.into(), method, args)?;
        self.methods.insert(spec);
        Ok(self)
    }

    pub fn add_module_method<T>(
        mut self,
        name: T,
        method: Method,
        args: sys::mrb_aspec,
    ) -> Result<Self, ConstantNameError>
    where
        T: Into<Cow<'static, str>>,
    {
        let spec = method::Spec::new(method::Type::Module, name.into(), method, args)?;
        self.methods.insert(spec);
        Ok(self)
    }

    pub fn define(self) -> Result<(), NotDefinedError> {
        let mrb = unsafe { self.interp.mrb.as_mut() };
        let mut rclass = if let Some(rclass) = self.spec.rclass(mrb) {
            rclass
        } else if let Some(scope) = self.spec.enclosing_scope() {
            let mut scope_rclass = scope
                .rclass(mrb)
                .ok_or_else(|| NotDefinedError::enclosing_scope(scope.fqname().into_owned()))?;
            let rclass = unsafe {
                sys::mrb_define_module_under(
                    mrb,
                    scope_rclass.as_mut(),
                    self.spec.name_c_str().as_ptr(),
                )
            };
            NonNull::new(rclass)
                .ok_or_else(|| NotDefinedError::module(self.spec.name.as_ref().to_owned()))?
        } else {
            let rclass = unsafe { sys::mrb_define_module(mrb, self.spec.name_c_str().as_ptr()) };
            NonNull::new(rclass)
                .ok_or_else(|| NotDefinedError::module(self.spec.name.as_ref().to_owned()))?
        };
        for method in self.methods {
            unsafe {
                method.define(self.interp, rclass.as_mut());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Spec {
    name: Cow<'static, str>,
    sym: sys::mrb_sym,
    cstring: CString,
    enclosing_scope: Option<Box<EnclosingRubyScope>>,
}

impl Spec {
    pub fn new<T>(
        interp: &mut Artichoke,
        name: T,
        enclosing_scope: Option<EnclosingRubyScope>,
    ) -> Result<Self, ConstantNameError>
    where
        T: Into<Cow<'static, str>>,
    {
        let name = name.into();
        if let Ok(cstring) = CString::new(name.as_ref()) {
            let sym = match name {
                Cow::Borrowed(name) => interp.intern_symbol(name.as_bytes()),
                Cow::Owned(ref name) => interp.intern_symbol(name.clone().into_bytes()),
            };
            Ok(Self {
                name,
                cstring,
                sym,
                enclosing_scope: enclosing_scope.map(Box::new),
            })
        } else {
            Err(ConstantNameError::new(name))
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn name_c_str(&self) -> &CStr {
        self.cstring.as_c_str()
    }

    #[must_use]
    pub fn enclosing_scope(&self) -> Option<&EnclosingRubyScope> {
        self.enclosing_scope.as_deref()
    }

    #[must_use]
    pub fn fqname(&self) -> Cow<'_, str> {
        if let Some(scope) = self.enclosing_scope() {
            let mut fqname = String::from(scope.fqname());
            fqname.push_str("::");
            fqname.push_str(self.name.as_ref());
            fqname.into()
        } else {
            self.name.as_ref().into()
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn rclass(&self, mrb: *mut sys::mrb_state) -> Option<NonNull<sys::RClass>> {
        if let Some(ref scope) = self.enclosing_scope {
            if let Some(mut scope) = scope.rclass(mrb) {
                let defined = unsafe {
                    sys::mrb_const_defined_at(
                        mrb,
                        sys::mrb_sys_obj_value(scope.cast::<c_void>().as_mut()),
                        self.sym,
                    )
                };
                if defined == 0 {
                    // Enclosing scope exists and module is NOT defined under
                    // the enclosing scope.
                    None
                } else {
                    // Enclosing scope exists module IS defined under the
                    // enclosing scope.
                    NonNull::new(unsafe {
                        sys::mrb_module_get_under(mrb, scope.as_mut(), self.name_c_str().as_ptr())
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
                    self.sym,
                )
            };
            if defined == 0 {
                // Module does NOT exist in root scop.
                None
            } else {
                // Module exists in root scope.
                NonNull::new(unsafe { sys::mrb_module_get(mrb, self.name_c_str().as_ptr()) })
            }
        }
    }
}

impl fmt::Display for Spec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        self.fqname() == other.fqname()
    }
}

#[cfg(test)]
mod tests {
    use crate::module::Spec;
    use crate::test::prelude::*;

    #[test]
    fn rclass_for_undef_root_module() {
        let mut interp = crate::interpreter().unwrap();
        let spec = Spec::new(&mut interp, "Foo", None).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_none());
    }

    #[test]
    fn rclass_for_undef_nested_module() {
        let mut interp = crate::interpreter().unwrap();
        let scope = Spec::new(&mut interp, "Kernel", None).unwrap();
        let scope = EnclosingRubyScope::module(&scope);
        let spec = Spec::new(&mut interp, "Foo", Some(scope)).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_none());
    }

    #[test]
    fn rclass_for_root_module() {
        let mut interp = crate::interpreter().unwrap();
        let spec = Spec::new(&mut interp, "Kernel", None).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_some());
    }

    #[test]
    fn rclass_for_nested_module() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(b"module Foo; module Bar; end; end").unwrap();
        let scope = Spec::new(&mut interp, "Foo", None).unwrap();
        let scope = EnclosingRubyScope::module(&scope);
        let spec = Spec::new(&mut interp, "Bar", Some(scope)).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_some());
    }

    #[test]
    fn rclass_for_nested_module_under_class() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(b"class Foo; module Bar; end; end").unwrap();
        let scope = class::Spec::new("Foo", None, None).unwrap();
        let scope = EnclosingRubyScope::class(&scope);
        let spec = Spec::new(&mut interp, "Bar", Some(scope)).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_some());
    }
}
