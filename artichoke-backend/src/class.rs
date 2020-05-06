use std::any::Any;
use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

use crate::def::{ConstantNameError, EnclosingRubyScope, Free, Method, NotDefinedError};
use crate::exception::Exception;
use crate::ffi::InterpreterExtractError;
use crate::method;
use crate::sys;
use crate::Artichoke;

#[derive(Debug)]
pub struct Builder<'a> {
    interp: &'a mut Artichoke,
    spec: &'a Spec,
    is_mrb_tt_data: bool,
    super_class: Option<NonNull<sys::RClass>>,
    methods: HashSet<method::Spec>,
}

impl<'a> Builder<'a> {
    #[must_use]
    pub fn for_spec(interp: &'a mut Artichoke, spec: &'a Spec) -> Self {
        Self {
            interp,
            spec,
            is_mrb_tt_data: false,
            super_class: None,
            methods: HashSet::default(),
        }
    }

    #[must_use]
    pub fn value_is_rust_object(mut self) -> Self {
        self.is_mrb_tt_data = true;
        self
    }

    pub fn with_super_class<T>(mut self, classname: &str) -> Result<Self, Exception>
    where
        T: Any,
    {
        let state = self.interp.state.as_ref().ok_or(InterpreterExtractError)?;
        let spec = state
            .classes
            .get::<T>()
            .ok_or_else(|| NotDefinedError::super_class(String::from(classname)))?;
        let rclass = unsafe {
            let mrb = self.interp.mrb.as_mut();
            spec.rclass(mrb)
                .ok_or_else(|| NotDefinedError::super_class("super class"))?
        };
        self.super_class = Some(rclass);
        Ok(self)
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
        let spec = method::Spec::new(method::Type::Instance, name, method, args)?;
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
        let spec = method::Spec::new(method::Type::Class, name, method, args)?;
        self.methods.insert(spec);
        Ok(self)
    }

    pub fn define(self) -> Result<(), NotDefinedError> {
        let mrb = unsafe { self.interp.mrb.as_mut() };
        let mut super_class = if let Some(super_class) = self.super_class {
            super_class
        } else {
            let rclass = mrb.object_class;
            NonNull::new(rclass).ok_or_else(|| NotDefinedError::super_class("Object"))?
        };
        let mut rclass = if let Some(rclass) = self.spec.rclass(mrb) {
            rclass
        } else if let Some(scope) = self.spec.enclosing_scope() {
            let mut scope_rclass = scope
                .rclass(mrb)
                .ok_or_else(|| NotDefinedError::enclosing_scope(scope.fqname().into_owned()))?;
            let rclass = unsafe {
                sys::mrb_define_class_under(
                    mrb,
                    scope_rclass.as_mut(),
                    self.spec.name_c_str().as_ptr(),
                    super_class.as_mut(),
                )
            };
            NonNull::new(rclass)
                .ok_or_else(|| NotDefinedError::class(self.spec.name.as_ref().to_owned()))?
        } else {
            let rclass = unsafe {
                sys::mrb_define_class(mrb, self.spec.name_c_str().as_ptr(), super_class.as_mut())
            };
            NonNull::new(rclass)
                .ok_or_else(|| NotDefinedError::class(self.spec.name.as_ref().to_owned()))?
        };
        for method in &self.methods {
            unsafe {
                method.define(self.interp, rclass.as_mut());
            }
        }
        // If a `Spec` defines a `Class` whose isntances own a pointer to a
        // Rust object, mark them as `MRB_TT_DATA`.
        if self.is_mrb_tt_data {
            unsafe {
                sys::mrb_sys_set_instance_tt(rclass.as_mut(), sys::mrb_vtype::MRB_TT_DATA);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Spec {
    name: Cow<'static, str>,
    cstring: CString,
    data_type: sys::mrb_data_type,
    enclosing_scope: Option<Box<EnclosingRubyScope>>,
}

impl Spec {
    pub fn new<T>(
        name: T,
        enclosing_scope: Option<EnclosingRubyScope>,
        free: Option<Free>,
    ) -> Result<Self, ConstantNameError>
    where
        T: Into<Cow<'static, str>>,
    {
        let name = name.into();
        if let Ok(cstring) = CString::new(name.as_ref()) {
            let data_type = sys::mrb_data_type {
                struct_name: cstring.as_ptr(),
                dfree: free,
            };
            Ok(Self {
                name,
                cstring,
                data_type,
                enclosing_scope: enclosing_scope.map(Box::new),
            })
        } else {
            Err(ConstantNameError::new(name))
        }
    }

    #[must_use]
    pub fn data_type(&self) -> &sys::mrb_data_type {
        &self.data_type
    }

    #[must_use]
    pub fn name_c_str(&self) -> &CStr {
        self.cstring.as_c_str()
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
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
                let defined_under = unsafe {
                    sys::mrb_class_defined_under(mrb, scope.as_mut(), self.name_c_str().as_ptr())
                };
                if defined_under == 0 {
                    // Enclosing scope exists.
                    // Class is not defined under the enclosing scope.
                    None
                } else {
                    // Enclosing scope exists.
                    // Class is defined under the enclosing scope.
                    NonNull::new(unsafe {
                        sys::mrb_class_get_under(mrb, scope.as_mut(), self.name_c_str().as_ptr())
                    })
                }
            } else {
                // Enclosing scope does not exist.
                None
            }
        } else if unsafe { sys::mrb_class_defined(mrb, self.cstring.as_ptr()) } == 0 {
            // Class does not exist in root scope.
            None
        } else {
            // Class exists in root scope.
            NonNull::new(unsafe { sys::mrb_class_get(mrb, self.name_c_str().as_ptr()) })
        }
    }
}

impl fmt::Display for Spec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "artichoke class spec -- {}", self.fqname())
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
    use crate::extn::core::exception::StandardError;
    use crate::extn::core::kernel::Kernel;
    use crate::test::prelude::*;

    struct RustError;

    #[test]
    fn super_class() {
        let mut interp = crate::interpreter().unwrap();
        let spec = class::Spec::new("RustError", None, None).unwrap();
        class::Builder::for_spec(&mut interp, &spec)
            .with_super_class::<StandardError>("StandardError")
            .unwrap()
            .define()
            .unwrap();
        interp.def_class::<RustError>(spec).unwrap();

        let result = interp.eval(b"RustError.new.is_a?(StandardError)").unwrap();
        let result = result.try_into::<bool>(&interp).unwrap();
        assert!(result, "RustError instances are instance of StandardError");

        let result = interp.eval(b"RustError < StandardError").unwrap();
        let result = result.try_into::<bool>(&interp).unwrap();
        assert!(result, "RustError inherits from StandardError");
    }

    #[test]
    fn rclass_for_undef_root_class() {
        let mut interp = crate::interpreter().unwrap();
        let spec = class::Spec::new("Foo", None, None).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_none());
    }

    #[test]
    fn rclass_for_undef_nested_class() {
        let mut interp = crate::interpreter().unwrap();
        let scope = interp.module_spec::<Kernel>().unwrap().unwrap();
        let spec = class::Spec::new("Foo", Some(EnclosingRubyScope::module(scope)), None).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_none());
    }

    #[test]
    fn rclass_for_nested_class() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(b"module Foo; class Bar; end; end").unwrap();
        let spec = module::Spec::new(&mut interp, "Foo", None).unwrap();
        let spec = class::Spec::new("Bar", Some(EnclosingRubyScope::module(&spec)), None).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_some());
    }

    #[test]
    fn rclass_for_nested_class_under_class() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(b"class Foo; class Bar; end; end").unwrap();
        let spec = class::Spec::new("Foo", None, None).unwrap();
        let spec = class::Spec::new("Bar", Some(EnclosingRubyScope::class(&spec)), None).unwrap();
        let rclass = spec.rclass(unsafe { interp.mrb.as_mut() });
        assert!(rclass.is_some());
    }
}
