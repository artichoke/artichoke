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
///
/// **Warning**: Calling [`raise`](RubyException::raise) on an interpreter from
/// outside the mruby VM call stack will result in a segfault. Raise should only
/// be called from Rust functions that are exposed on the mruby interpreter via
/// [`class::Spec`](crate::class::Spec) and
/// [`module::Spec`](crate::module::Spec).
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::def::{ClassLike, Define};
    use crate::eval::MrbEval;
    use crate::exception::Exception;
    use crate::extn::core::error::{RubyException, RuntimeError};
    use crate::file::MrbFile;
    use crate::interpreter::{Interpreter, Mrb};
    use crate::sys;
    use crate::MrbError;

    struct Run;

    impl Run {
        unsafe extern "C" fn run(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            let interp = interpreter_or_raise!(mrb);
            RuntimeError::raise(&interp, "something went wrong")
        }
    }

    impl MrbFile for Run {
        fn require(interp: Mrb) -> Result<(), MrbError> {
            let spec = interp.borrow_mut().def_class::<Self>("Run", None, None);
            spec.borrow_mut()
                .add_self_method("run", Self::run, sys::mrb_args_none());
            spec.borrow().define(&interp)?;
            Ok(())
        }
    }

    #[test]
    fn raise() {
        let interp = Interpreter::create().expect("mrb init");
        Run::require(Rc::clone(&interp)).unwrap();
        let value = interp.eval("Run.run").map(|_| ());
        let expected = Exception::new(
            "RuntimeError",
            "something went wrong",
            Some(vec!["(eval):1".to_owned()]),
            "(eval):1: something went wrong (RuntimeError)",
        );
        assert_eq!(value, Err(MrbError::Exec(expected.to_string())));
    }
}
