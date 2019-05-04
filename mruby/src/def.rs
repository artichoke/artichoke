use std::ffi::{c_void, CString};
use std::fmt;
use std::rc::Rc;

use crate::class;
use crate::interpreter::{Mrb, MrbError};
use crate::module;
use crate::sys;

// Types
pub type Free = unsafe extern "C" fn(mrb: *mut sys::mrb_state, data: *mut c_void);
pub type Method =
    unsafe extern "C" fn(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Parent {
    Class { spec: Rc<class::Spec> },
    Module { spec: Rc<module::Spec> },
}

impl Parent {
    pub fn rclass(&self, interp: Mrb) -> *mut sys::RClass {
        match self {
            Parent::Class { spec } => spec.rclass(interp),
            Parent::Module { spec } => spec.rclass(interp),
        }
    }
}

/// `Define` trait allows a type to install classes, modules, and
/// methods into an mruby interpreter.
pub trait Define
where
    Self: ClassLike,
{
    fn define(&self, interp: Mrb) -> Result<*mut sys::RClass, MrbError>;
}

/// `ClassLike` trait unifies `class::Spec` and `module::Spec`.
pub trait ClassLike
where
    Self: fmt::Debug + fmt::Display,
{
    fn add_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec);

    fn add_self_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec);

    fn cstring(&self) -> &CString;

    fn name(&self) -> &str;

    fn parent(&self) -> Option<Parent>;

    fn rclass(&self, interp: Mrb) -> *mut sys::RClass;
}
