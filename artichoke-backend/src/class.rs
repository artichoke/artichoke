use std::any::Any;
use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

use crate::def::{ConstantNameError, EnclosingRubyScope, Free, Method, NotDefinedError};
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::method;
use crate::sys;
use crate::Artichoke;

mod registry;

pub use registry::Registry;

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

    pub fn with_super_class<T, U>(mut self, classname: U) -> Result<Self, Error>
    where
        T: Any,
        U: Into<Cow<'static, str>>,
    {
        let state = self.interp.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let rclass = if let Some(spec) = state.classes.get::<T>() {
            spec.rclass()
        } else {
            return Err(NotDefinedError::super_class(classname.into()).into());
        };
        let rclass = unsafe { self.interp.with_ffi_boundary(|mrb| rclass.resolve(mrb))? };
        if let Some(rclass) = rclass {
            self.super_class = Some(rclass);
            Ok(self)
        } else {
            Err(NotDefinedError::super_class(classname.into()).into())
        }
    }

    pub fn add_method<T>(mut self, name: T, method: Method, args: sys::mrb_aspec) -> Result<Self, ConstantNameError>
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

    pub fn define(self) -> Result<(), NotDefinedError> {
        use sys::mrb_vtype::MRB_TT_DATA;

        let name = self.spec.name_c_str().as_ptr();

        let mut super_class = if let Some(super_class) = self.super_class {
            super_class
        } else {
            // SAFETY: Although this direct access of the `mrb` property on the
            // interpreter does not go through `Artichoke::with_ffi_boundary`, no
            // `MRB_API` functions are called, which means it is not required to
            // re-box the Artichoke `State` into the `mrb_state->ud` pointer.
            //
            // This code only performs a memory access to read a field from the
            // `mrb_state`.
            let rclass = unsafe { self.interp.mrb.as_ref().object_class };
            NonNull::new(rclass).ok_or_else(|| NotDefinedError::super_class("Object"))?
        };

        let rclass = self.spec.rclass();
        let rclass = unsafe { self.interp.with_ffi_boundary(|mrb| rclass.resolve(mrb)) };

        let mut rclass = if let Ok(Some(rclass)) = rclass {
            rclass
        } else if let Some(enclosing_scope) = self.spec.enclosing_scope() {
            let scope = unsafe { self.interp.with_ffi_boundary(|mrb| enclosing_scope.rclass(mrb)) };
            if let Ok(Some(mut scope)) = scope {
                let rclass = unsafe {
                    self.interp.with_ffi_boundary(|mrb| {
                        sys::mrb_define_class_under(mrb, scope.as_mut(), name, super_class.as_mut())
                    })
                };
                let rclass = rclass.map_err(|_| NotDefinedError::class(self.spec.name()))?;
                NonNull::new(rclass).ok_or_else(|| NotDefinedError::class(self.spec.name()))?
            } else {
                return Err(NotDefinedError::enclosing_scope(enclosing_scope.fqname().into_owned()));
            }
        } else {
            let rclass = unsafe {
                self.interp
                    .with_ffi_boundary(|mrb| sys::mrb_define_class(mrb, name, super_class.as_mut()))
            };
            let rclass = rclass.map_err(|_| NotDefinedError::class(self.spec.name()))?;
            NonNull::new(rclass).ok_or_else(|| NotDefinedError::class(self.spec.name()))?
        };

        for method in &self.methods {
            unsafe {
                method.define(self.interp, rclass.as_mut())?;
            }
        }

        // If a `Spec` defines a `Class` whose instances own a pointer to a
        // Rust object, mark them as `MRB_TT_DATA`.
        if self.is_mrb_tt_data {
            unsafe {
                sys::mrb_sys_set_instance_tt(rclass.as_mut(), MRB_TT_DATA);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rclass {
    name: &'static CStr,
    enclosing_scope: Option<EnclosingRubyScope>,
}

impl Rclass {
    #[must_use]
    pub const fn new(name: &'static CStr, enclosing_scope: Option<EnclosingRubyScope>) -> Self {
        Self { name, enclosing_scope }
    }

    /// Resolve a type's [`sys::RClass`] using its enclosing scope and name.
    ///
    /// # Safety
    ///
    /// This function must be called within an [`Artichoke::with_ffi_boundary`]
    /// closure because the FFI APIs called in this function may require access
    /// to the Artichoke [`State`](crate::state::State).
    pub unsafe fn resolve(&self, mrb: *mut sys::mrb_state) -> Option<NonNull<sys::RClass>> {
        let class_name = self.name.as_ptr();
        if let Some(ref scope) = self.enclosing_scope {
            // short circuit if enclosing scope does not exist.
            let mut scope = scope.rclass(mrb)?;
            let is_defined_under = sys::mrb_class_defined_under(mrb, scope.as_mut(), class_name);
            if is_defined_under {
                // Enclosing scope exists.
                // Class is defined under the enclosing scope.
                let class = sys::mrb_class_get_under(mrb, scope.as_mut(), class_name);
                NonNull::new(class)
            } else {
                // Enclosing scope exists.
                // Class is not defined under the enclosing scope.
                None
            }
        } else {
            let is_defined = sys::mrb_class_defined(mrb, class_name);
            if is_defined {
                // Class exists in root scope.
                let class = sys::mrb_class_get(mrb, class_name);
                NonNull::new(class)
            } else {
                // Class does not exist in root scope.
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct Spec {
    name: Cow<'static, str>,
    name_cstr: &'static CStr,
    data_type: Box<sys::mrb_data_type>,
    enclosing_scope: Option<EnclosingRubyScope>,
}

impl Spec {
    pub fn new<T>(
        name: T,
        name_cstr: &'static CStr,
        enclosing_scope: Option<EnclosingRubyScope>,
        free: Option<Free>,
    ) -> Result<Self, ConstantNameError>
    where
        T: Into<Cow<'static, str>>,
    {
        let name = name.into();
        // SAFETY: The constructed `mrb_data_type` has `'static` lifetime:
        //
        // - `name_cstr` is `&'static` so it will outlive the `data_type`.
        // - `Spec` does not offer mutable access to these fields.
        let data_type = sys::mrb_data_type {
            struct_name: name_cstr.as_ptr(),
            dfree: free,
        };
        let data_type = Box::new(data_type);
        Ok(Self {
            name,
            name_cstr,
            data_type,
            enclosing_scope,
        })
    }

    #[must_use]
    pub fn data_type(&self) -> *const sys::mrb_data_type {
        self.data_type.as_ref()
    }

    #[must_use]
    pub fn name(&self) -> Cow<'static, str> {
        match &self.name {
            Cow::Borrowed(name) => Cow::Borrowed(name),
            Cow::Owned(name) => name.clone().into(),
        }
    }

    #[must_use]
    pub fn name_c_str(&self) -> &'static CStr {
        self.name_cstr
    }

    #[must_use]
    pub fn enclosing_scope(&self) -> Option<&EnclosingRubyScope> {
        self.enclosing_scope.as_ref()
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

    #[must_use]
    pub fn rclass(&self) -> Rclass {
        Rclass::new(self.name_cstr, self.enclosing_scope.clone())
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
    use spinoso_exception::StandardError;

    use crate::extn::core::kernel::Kernel;
    use crate::test::prelude::*;

    struct RustError;

    #[test]
    fn super_class() {
        let mut interp = interpreter();
        let spec = class::Spec::new("RustError", qed::const_cstr_from_str!("RustError\0"), None, None).unwrap();
        class::Builder::for_spec(&mut interp, &spec)
            .with_super_class::<StandardError, _>("StandardError")
            .unwrap()
            .define()
            .unwrap();
        interp.def_class::<RustError>(spec).unwrap();

        let result = interp.eval(b"RustError.new.is_a?(StandardError)").unwrap();
        let result = result.try_convert_into::<bool>(&interp).unwrap();
        assert!(result, "RustError instances are instance of StandardError");

        let result = interp.eval(b"RustError < StandardError").unwrap();
        let result = result.try_convert_into::<bool>(&interp).unwrap();
        assert!(result, "RustError inherits from StandardError");
    }

    #[test]
    fn rclass_for_undef_root_class() {
        let mut interp = interpreter();
        let spec = class::Spec::new("Foo", qed::const_cstr_from_str!("Foo\0"), None, None).unwrap();
        let rclass = unsafe { interp.with_ffi_boundary(|mrb| spec.rclass().resolve(mrb)) }.unwrap();
        assert!(rclass.is_none());
    }

    #[test]
    fn rclass_for_undef_nested_class() {
        let mut interp = interpreter();
        let scope = interp.module_spec::<Kernel>().unwrap().unwrap();
        let spec = class::Spec::new(
            "Foo",
            qed::const_cstr_from_str!("Foo\0"),
            Some(EnclosingRubyScope::module(scope)),
            None,
        )
        .unwrap();
        let rclass = unsafe { interp.with_ffi_boundary(|mrb| spec.rclass().resolve(mrb)) }.unwrap();
        assert!(rclass.is_none());
    }

    #[test]
    fn rclass_for_nested_class() {
        let mut interp = interpreter();
        interp.eval(b"module Foo; class Bar; end; end").unwrap();
        let spec = module::Spec::new(&mut interp, "Foo", qed::const_cstr_from_str!("Foo\0"), None).unwrap();
        let spec = class::Spec::new(
            "Bar",
            qed::const_cstr_from_str!("Bar\0"),
            Some(EnclosingRubyScope::module(&spec)),
            None,
        )
        .unwrap();
        let rclass = unsafe { interp.with_ffi_boundary(|mrb| spec.rclass().resolve(mrb)) }.unwrap();
        assert!(rclass.is_some());
    }

    #[test]
    fn rclass_for_nested_class_under_class() {
        let mut interp = interpreter();
        interp.eval(b"class Foo; class Bar; end; end").unwrap();
        let spec = class::Spec::new("Foo", qed::const_cstr_from_str!("Foo\0"), None, None).unwrap();
        let spec = class::Spec::new(
            "Bar",
            qed::const_cstr_from_str!("Bar\0"),
            Some(EnclosingRubyScope::class(&spec)),
            None,
        )
        .unwrap();
        let rclass = unsafe { interp.with_ffi_boundary(|mrb| spec.rclass().resolve(mrb)) }.unwrap();
        assert!(rclass.is_some());
    }
}
