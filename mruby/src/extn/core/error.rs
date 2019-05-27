use log::warn;
use std::ffi::CString;
use std::rc::Rc;

use crate::def::{ClassLike, Define};
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    let exception = interp
        .borrow_mut()
        .def_class::<Exception>("Exception", None, None);
    let scripterror = interp
        .borrow_mut()
        .def_class::<ScriptError>("ScriptError", None, None);
    scripterror
        .borrow_mut()
        .with_super_class(Rc::clone(&exception));
    scripterror
        .borrow()
        .define(interp)
        .map_err(|_| MrbError::New)?;
    let loaderror = interp
        .borrow_mut()
        .def_class::<LoadError>("LoadError", None, None);
    loaderror
        .borrow_mut()
        .with_super_class(Rc::clone(&scripterror));
    loaderror
        .borrow()
        .define(interp)
        .map_err(|_| MrbError::New)?;
    interp
        .borrow_mut()
        .def_class::<ArgumentError>("ArgumentError", None, None);
    Ok(())
}

pub struct Exception;

#[allow(clippy::module_name_repetitions)]
pub struct ScriptError;

#[allow(clippy::module_name_repetitions)]
pub struct LoadError;

impl LoadError {
    pub fn raise(interp: &Mrb, file: &str) -> sys::mrb_value {
        let spec = if let Some(spec) = interp.borrow().class_spec::<Self>() {
            spec
        } else {
            return interp.bool(false).inner();
        };
        let message = format!("cannot load such file -- {}", file);
        let msg = CString::new(message).expect("error message");
        unsafe {
            sys::mrb_sys_raise(
                interp.borrow().mrb,
                spec.borrow().cstring().as_ptr() as *const i8,
                msg.as_ptr(),
            )
        };
        warn!("Failed require '{}' on {:?}", file, interp.borrow());
        interp.bool(false).inner()
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ArgumentError;

impl ArgumentError {
    pub fn raise(interp: &Mrb, message: &str) -> sys::mrb_value {
        let spec = if let Some(spec) = interp.borrow().class_spec::<Self>() {
            spec
        } else {
            return interp.nil().inner();
        };
        let msg = CString::new(message).expect("error message");
        unsafe {
            sys::mrb_sys_raise(
                interp.borrow().mrb,
                spec.borrow().cstring().as_ptr() as *const i8,
                msg.as_ptr(),
            )
        };
        warn!("ArgumentError '{}' on {:?}", message, interp.borrow());
        interp.nil().inner()
    }
}
