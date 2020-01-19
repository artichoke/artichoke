use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::def::Method;
use crate::sys;
use crate::{Artichoke, ArtichokeError};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Type {
    Class,
    Global,
    Instance,
    Module,
}

#[derive(Clone)]
pub struct Spec {
    name: Cow<'static, str>,
    cstring: CString,
    method_type: Type,
    method: Method,
    args: sys::mrb_aspec,
}

impl Spec {
    pub fn new<T>(
        method_type: Type,
        method_name: T,
        method: Method,
        args: sys::mrb_aspec,
    ) -> Result<Self, ArtichokeError>
    where
        T: Into<Cow<'static, str>>,
    {
        let name = method_name.into();
        let method_cstr =
            CString::new(name.as_ref()).map_err(|_| ArtichokeError::InvalidConstantName)?;
        Ok(Self {
            name,
            cstring: method_cstr,
            method_type,
            method,
            args,
        })
    }

    #[must_use]
    pub fn method_type(&self) -> &Type {
        &self.method_type
    }

    #[must_use]
    pub fn method(&self) -> Method {
        self.method
    }

    #[must_use]
    pub fn name_c_str(&self) -> &CStr {
        self.cstring.as_c_str()
    }

    /// Define this method on the class-like pointed to by `into`.
    ///
    /// # Safety
    ///
    /// This method requires that `into` is non-null and points to a valid
    /// [`sys::RClass`].
    ///
    /// This method requires that the [`sys::mrb_state`] has a valid `top_self`
    /// object.
    pub unsafe fn define(
        &self,
        interp: &Artichoke,
        into: *mut sys::RClass,
    ) -> Result<(), ArtichokeError> {
        let mrb = interp.0.borrow().mrb;
        match self.method_type {
            Type::Class => sys::mrb_define_class_method(
                mrb,
                into,
                self.name_c_str().as_ptr(),
                Some(self.method),
                self.args,
            ),
            Type::Global => sys::mrb_define_singleton_method(
                mrb,
                (*mrb).top_self,
                self.name_c_str().as_ptr(),
                Some(self.method),
                self.args,
            ),
            Type::Instance => sys::mrb_define_method(
                mrb,
                into,
                self.name_c_str().as_ptr(),
                Some(self.method),
                self.args,
            ),
            Type::Module => sys::mrb_define_module_function(
                mrb,
                into,
                self.name_c_str().as_ptr(),
                Some(self.method),
                self.args,
            ),
        }
        Ok(())
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
            Type::Class => write!(f, "self method spec -- {}", self.name),
            Type::Global => write!(f, "global method spec -- {}", self.name),
            Type::Instance => write!(f, "instance method spec -- {}", self.name),
            Type::Module => write!(f, "module method spec -- {}", self.name),
        }
    }
}

impl Eq for Spec {}

impl PartialEq for Spec {
    #[must_use]
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
