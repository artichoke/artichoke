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
    interp
        .borrow_mut()
        .def_class::<RuntimeError>("RuntimeError", None, None);
    Ok(())
}

/// Raise implementation for `Exception` structs.
pub trait RubyException: 'static + Sized {
    /// Raise the `Exception` defined with this type with a message.
    fn raise(interp: &Mrb, message: &str) -> sys::mrb_value {
        let spec = if let Some(spec) = interp.borrow().class_spec::<Self>() {
            spec
        } else {
            return interp.nil().inner();
        };
        let message = Self::message(message);
        if let Ok(msg) = CString::new(message.as_str()) {
            unsafe {
                sys::mrb_sys_raise(
                    interp.borrow().mrb,
                    spec.borrow().cstring().as_ptr() as *const i8,
                    msg.as_ptr(),
                )
            };
            warn!(
                "raised {} '{}' on {:?}",
                spec.borrow().name(),
                message,
                interp.borrow()
            );
        } else {
            warn!(
                "unable to raise {} with message {}",
                spec.borrow().name(),
                message
            );
        }
        interp.nil().inner()
    }

    fn message(message: &str) -> String {
        message.to_owned()
    }
}

pub struct Exception;

impl RubyException for Exception {}

#[allow(clippy::module_name_repetitions)]
pub struct ScriptError;

impl RubyException for ScriptError {}

#[allow(clippy::module_name_repetitions)]
pub struct LoadError;

impl RubyException for LoadError {
    fn message(message: &str) -> String {
        format!("cannot load such file -- {}", message)
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ArgumentError;

impl RubyException for ArgumentError {}

#[allow(clippy::module_name_repetitions)]
pub struct RuntimeError;

impl RubyException for RuntimeError {}
