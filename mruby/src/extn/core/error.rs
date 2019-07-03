use log::warn;
use std::ffi::{c_void, CString};
use std::rc::Rc;

use crate::def::{ClassLike, Define};
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::MrbError;

struct ProtectArgs {
    e_class: String,
    message: String,
}

impl ProtectArgs {
    fn new(e_class: &str, message: &str) -> Self {
        Self {
            e_class: e_class.to_owned(),
            message: message.to_owned(),
        }
    }
}

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
        .def_class::<IndexError>("IndexError", None, None);
    interp
        .borrow_mut()
        .def_class::<RuntimeError>("RuntimeError", None, None);
    interp
        .borrow_mut()
        .def_class::<SyntaxError>("SyntaxError", None, None);
    interp
        .borrow_mut()
        .def_class::<TypeError>("TypeError", None, None);
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
    ///
    /// **Warning**: This function calls [`sys::mrb_sys_raise`] which modifies
    /// the stack with `longjmp`. mruby expects raise to be called at some stack
    /// frame below an eval. If this is not the case, mruby will segfault.
    fn raise(interp: &Mrb, message: &str) -> sys::mrb_value {
        unsafe extern "C" fn run_protected(
            mrb: *mut sys::mrb_state,
            data: sys::mrb_value,
        ) -> sys::mrb_value {
            let ptr = sys::mrb_sys_cptr_ptr(data);
            let args = Rc::from_raw(ptr as *const ProtectArgs);

            let e_class_cstring = CString::new(args.e_class.as_str());
            let e_class = if let Ok(e_class) = e_class_cstring {
                e_class
            } else {
                warn!(
                    "unable to raise {} with message {}",
                    args.e_class, args.message
                );
                return sys::mrb_sys_nil_value();
            };
            let message_cstring = CString::new(args.message.as_str());
            let message = if let Ok(message) = message_cstring {
                message
            } else {
                warn!(
                    "unable to raise {} with message {}",
                    args.e_class, args.message
                );
                return sys::mrb_sys_nil_value();
            };
            sys::mrb_sys_raise(mrb, e_class.as_ptr(), message.as_ptr());
            warn!("raised {} with message {}", args.e_class, args.message);
            sys::mrb_sys_nil_value()
        }
        unsafe extern "C" fn run_ensure(
            _mrb: *mut sys::mrb_state,
            _data: sys::mrb_value,
        ) -> sys::mrb_value {
            sys::mrb_sys_nil_value()
        }
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Mrb` to
        // get access to the underlying `MrbState`.
        let (mrb, _ctx) = {
            let borrow = interp.borrow();
            (borrow.mrb, borrow.ctx)
        };

        let spec = if let Some(spec) = interp.borrow().class_spec::<Self>() {
            spec
        } else {
            return interp.nil().inner();
        };
        let message = Self::message(message);
        let args = Rc::new(ProtectArgs::new(spec.borrow().name(), message.as_str()));
        unsafe {
            let data = sys::mrb_sys_cptr_value(mrb, Rc::into_raw(args) as *mut c_void);
            let value = sys::mrb_ensure(
                mrb,
                Some(run_protected),
                data,
                Some(run_ensure),
                sys::mrb_sys_nil_value(),
            );
            sys::mrb_sys_raise_current_exception(mrb);
            value
        }
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
pub struct IndexError;

impl RubyException for IndexError {}

#[allow(clippy::module_name_repetitions)]
pub struct RuntimeError;

impl RubyException for RuntimeError {}

#[allow(clippy::module_name_repetitions)]
pub struct SyntaxError;

impl RubyException for SyntaxError {}

#[allow(clippy::module_name_repetitions)]
pub struct TypeError;

impl RubyException for TypeError {}

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
