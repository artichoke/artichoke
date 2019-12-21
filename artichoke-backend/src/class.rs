use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::def::{EnclosingRubyScope, Free, Method};
use crate::method;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

#[derive(Clone)]
pub struct Builder<'a> {
    interp: &'a Artichoke,
    spec: &'a Spec,
    is_mrb_tt_data: bool,
    super_class: Option<&'a Spec>,
    methods: HashSet<method::Spec>,
}

impl<'a> Builder<'a> {
    pub fn for_spec(interp: &'a Artichoke, spec: &'a Spec) -> Self {
        Self {
            interp,
            spec,
            is_mrb_tt_data: false,
            super_class: None,
            methods: HashSet::default(),
        }
    }

    pub fn value_is_rust_object(mut self) -> Self {
        self.is_mrb_tt_data = true;
        self
    }

    pub fn with_super_class(mut self, super_class: Option<&'a Spec>) -> Self {
        self.super_class = super_class;
        self
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

    pub fn define(self) -> Result<(), ArtichokeError> {
        let mrb = self.interp.0.borrow().mrb;
        let super_class = if let Some(spec) = self.super_class {
            spec.rclass(self.interp)
                .ok_or_else(|| ArtichokeError::NotDefined(Cow::Owned(spec.fqname().into_owned())))?
        } else {
            unsafe { (*mrb).object_class }
        };
        let rclass = if let Some(rclass) = self.spec.rclass(self.interp) {
            rclass
        } else if let Some(scope) = self.spec.enclosing_scope() {
            let scope = scope.rclass(self.interp).ok_or_else(|| {
                ArtichokeError::NotDefined(Cow::Owned(scope.fqname().into_owned()))
            })?;
            unsafe {
                sys::mrb_define_class_under(
                    mrb,
                    scope,
                    self.spec.name_c_str().as_ptr(),
                    super_class,
                )
            }
        } else {
            unsafe { sys::mrb_define_class(mrb, self.spec.name_c_str().as_ptr(), super_class) }
        };
        for method in &self.methods {
            unsafe {
                method.define(self.interp, rclass)?;
            }
        }
        // If a `Spec` defines a `Class` whose isntances own a pointer to a
        // Rust object, mark them as `MRB_TT_DATA`.
        if self.is_mrb_tt_data {
            unsafe {
                sys::mrb_sys_set_instance_tt(rclass, sys::mrb_vtype::MRB_TT_DATA);
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Spec {
    name: Cow<'static, str>,
    cstring: CString,
    data_type: sys::mrb_data_type,
    enclosing_scope: Option<Box<EnclosingRubyScope>>,
}

impl Spec {
    pub fn new<T>(name: T, enclosing_scope: Option<EnclosingRubyScope>, free: Option<Free>) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        let name = name.into();
        let cstring = CString::new(name.as_ref()).expect("name for data type");
        let data_type = sys::mrb_data_type {
            struct_name: cstring.as_ptr(),
            dfree: free,
        };
        Self {
            name,
            cstring,
            data_type,
            enclosing_scope: enclosing_scope.map(Box::new),
        }
    }

    pub fn new_instance(&self, interp: &Artichoke, args: &[Value]) -> Option<Value> {
        let mrb = interp.0.borrow().mrb;
        let rclass = self.rclass(interp)?;
        let args = args.iter().map(Value::inner).collect::<Vec<_>>();
        let arglen = Int::try_from(args.len()).unwrap_or_default();
        let value = unsafe {
            sys::mrb_obj_new(mrb, rclass, arglen, args.as_ptr() as *const sys::mrb_value)
        };
        Some(Value::new(interp, value))
    }

    pub fn value(&self, interp: &Artichoke) -> Option<Value> {
        let rclass = self.rclass(interp)?;
        let module = unsafe { sys::mrb_sys_class_value(rclass) };
        Some(Value::new(interp, module))
    }

    pub fn data_type(&self) -> &sys::mrb_data_type {
        &self.data_type
    }

    pub fn name_c_str(&self) -> &CStr {
        self.cstring.as_c_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn enclosing_scope(&self) -> Option<&EnclosingRubyScope> {
        self.enclosing_scope.as_deref()
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
        if let Some(ref scope) = self.enclosing_scope {
            if let Some(scope) = scope.rclass(interp) {
                if unsafe { sys::mrb_class_defined_under(mrb, scope, self.name_c_str().as_ptr()) }
                    == 0
                {
                    // Enclosing scope exists.
                    // Class is not defined under the enclosing scope.
                    None
                } else {
                    // Enclosing scope exists.
                    // Class is defined under the enclosing scope.
                    Some(unsafe {
                        sys::mrb_class_get_under(mrb, scope, self.name_c_str().as_ptr())
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
            Some(unsafe { sys::mrb_class_get(mrb, self.name_c_str().as_ptr()) })
        }
    }
}

impl fmt::Debug for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)?;
        if self.data_type.dfree.is_some() {
            write!(f, " -- with free func")?;
        }
        Ok(())
    }
}

impl fmt::Display for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::value::Value as _;

    use crate::class;
    use crate::def::EnclosingRubyScope;
    use crate::extn::core::exception::StandardError;
    use crate::extn::core::kernel::Kernel;
    use crate::module;

    #[test]
    fn super_class() {
        struct RustError;

        let interp = crate::interpreter().expect("init");
        let borrow = interp.0.borrow();
        let standard_error = borrow.class_spec::<StandardError>().unwrap();
        let spec = class::Spec::new("RustError", None, None);
        class::Builder::for_spec(&interp, &spec)
            .with_super_class(Some(&standard_error))
            .define()
            .unwrap();
        drop(borrow);
        interp.0.borrow_mut().def_class::<RustError>(spec);

        let result = interp
            .eval(b"RustError.new.is_a?(StandardError)")
            .expect("eval");
        let result = result.try_into::<bool>().expect("convert");
        assert!(result, "RustError instances are instance of StandardError");
        let result = interp.eval(b"RustError < StandardError").expect("eval");
        let result = result.try_into::<bool>().expect("convert");
        assert!(result, "RustError inherits from StandardError");
    }

    #[test]
    fn rclass_for_undef_root_class() {
        let interp = crate::interpreter().expect("init");
        let spec = class::Spec::new("Foo", None, None);
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_undef_nested_class() {
        let interp = crate::interpreter().expect("init");
        let borrow = interp.0.borrow();
        let scope = borrow.module_spec::<Kernel>().unwrap();
        let spec = class::Spec::new("Foo", Some(EnclosingRubyScope::module(scope)), None);
        drop(borrow);
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_root_class() {
        let interp = crate::interpreter().expect("init");
        let borrow = interp.0.borrow();
        let spec = borrow.class_spec::<StandardError>().unwrap();
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_class() {
        let interp = crate::interpreter().expect("init");
        interp
            .eval(b"module Foo; class Bar; end; end")
            .expect("eval");
        let spec = module::Spec::new("Foo", None);
        let spec = class::Spec::new("Bar", Some(EnclosingRubyScope::module(&spec)), None);
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_class_under_class() {
        let interp = crate::interpreter().expect("init");
        interp
            .eval(b"class Foo; class Bar; end; end")
            .expect("eval");
        let spec = class::Spec::new("Foo", None, None);
        let spec = class::Spec::new("Bar", Some(EnclosingRubyScope::class(&spec)), None);
        assert!(spec.rclass(&interp).is_some());
    }
}
