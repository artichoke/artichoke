//! # Ruby Exception Hierarchy
//!
//! The built-in subclasses of
//! [`Exception`](https://ruby-doc.org/core-2.6.3/Exception.html) are:
//!
//! - `NoMemoryError`
//! - `ScriptError`
//!   - `LoadError`
//!   - `NotImplementedError`
//!   - `SyntaxError`
//! - `SecurityError`
//! - `SignalException`
//!   - `Interrupt`
//! - `StandardError` -- default for `rescue`
//!   - `ArgumentError`
//!     - `UncaughtThrowError`
//!   - `EncodingError`
//!   - `FiberError`
//!   - `IOError`
//!     - `EOFError`
//!   - `IndexError`
//!     - `KeyError`
//!     - `StopIteration`
//!   - `LocalJumpError`
//!   - `NameError`
//!     - `NoMethodError`
//!   - `RangeError`
//!     - `FloatDomainError`
//!   - `RegexpError`
//!   - `RuntimeError` -- default for `raise`
//!     - `FrozenError`
//!   - `SystemCallError`
//!     - `Errno::*`
//!   - `ThreadError`
//!   - `TypeError`
//!   - `ZeroDivisionError`
//! - `SystemExit`
//! - `SystemStackError`
//! - `fatal` -- impossible to rescue

use std::ffi::CString;
use std::rc::Rc;

use crate::convert::Convert;
use crate::def::{ClassLike, Define};
use crate::eval::Eval;
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.eval(include_bytes!("exception.rb").as_ref())?;
    let exception = interp
        .0
        .borrow_mut()
        .def_class::<Exception>("Exception", None, None);
    let scripterror = interp
        .0
        .borrow_mut()
        .def_class::<ScriptError>("ScriptError", None, None);
    scripterror
        .borrow_mut()
        .with_super_class(Rc::clone(&exception));
    scripterror
        .borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;
    let loaderror = interp
        .0
        .borrow_mut()
        .def_class::<LoadError>("LoadError", None, None);
    loaderror
        .borrow_mut()
        .with_super_class(Rc::clone(&scripterror));
    loaderror
        .borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;
    interp
        .0
        .borrow_mut()
        .def_class::<ArgumentError>("ArgumentError", None, None);
    interp
        .0
        .borrow_mut()
        .def_class::<FrozenError>("FrozenError", None, None);
    interp
        .0
        .borrow_mut()
        .def_class::<IndexError>("IndexError", None, None);
    interp
        .0
        .borrow_mut()
        .def_class::<RangeError>("RangeError", None, None);
    interp
        .0
        .borrow_mut()
        .def_class::<RuntimeError>("RuntimeError", None, None);
    interp
        .0
        .borrow_mut()
        .def_class::<SyntaxError>("SyntaxError", None, None);
    interp
        .0
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
#[allow(clippy::module_name_repetitions)]
pub trait RubyException: 'static + Sized {
    /// Raise the `Exception` defined with this type with a message.
    ///
    /// **Warning**: This function calls [`sys::mrb_sys_raise`] which modifies
    /// the stack with `longjmp`. mruby expects raise to be called at some stack
    /// frame below an eval. If this is not the case, mruby will segfault.
    unsafe fn raise(interp: Artichoke, message: &'static str) -> sys::mrb_value {
        Self::raisef::<Value>(interp, message, vec![])
    }

    unsafe fn raisef<V>(interp: Artichoke, message: &'static str, format: Vec<V>) -> sys::mrb_value
    where
        Artichoke: Convert<V, Value>,
    {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Artichoke` to
        // get access to the underlying `ArtichokeState`.
        let mrb = interp.0.borrow().mrb;

        let spec = if let Some(spec) = interp.0.borrow().class_spec::<Self>() {
            spec
        } else {
            return sys::mrb_sys_nil_value();
        };
        let borrow = spec.borrow();
        let eclass = if let Some(rclass) = borrow.rclass(&interp) {
            rclass
        } else {
            warn!("unable to raise {}", borrow.name());
            return sys::mrb_sys_nil_value();
        };
        // message is a &'static str so it should never leak
        let message = CString::new(message);
        let message_cstring = if let Ok(message) = message {
            message
        } else {
            warn!("unable to raise {}", borrow.name());
            return sys::mrb_sys_nil_value();
        };
        let message_ptr = message_cstring.as_ptr();

        let mut formatargs = format
            .into_iter()
            .map(|item| interp.convert(item).inner())
            .collect::<Vec<_>>();
        // `mrb_sys_raise` will call longjmp which will unwind the stack.
        // Anything we haven't cleaned up at this point will leak, so drop
        // everything.
        drop(borrow);
        drop(spec);
        drop(interp);
        match formatargs.len() {
            0 => {
                drop(formatargs);
                sys::mrb_raise(mrb, eclass, message_ptr)
            }
            1 => {
                let arg1 = formatargs.remove(0);
                drop(formatargs);
                sys::mrb_raisef(mrb, eclass, message_ptr, arg1)
            }
            2 => {
                let arg1 = formatargs.remove(0);
                let arg2 = formatargs.remove(0);
                drop(formatargs);
                sys::mrb_raisef(mrb, eclass, message_ptr, arg1, arg2)
            }
            _ => panic!("unsupported raisef format arg count. See mruby/src/extn/core/error.rs."),
        }
        unreachable!("mrb_raise will unwind the stack with longjmp");
    }
}

pub struct Exception;

impl RubyException for Exception {}

pub struct ScriptError;

impl RubyException for ScriptError {}

pub struct LoadError;

impl RubyException for LoadError {}

pub struct ArgumentError;

impl RubyException for ArgumentError {}

pub struct FrozenError;

impl RubyException for FrozenError {}

pub struct IndexError;

impl RubyException for IndexError {}

pub struct RangeError;

impl RubyException for RangeError {}

pub struct RuntimeError;

impl RubyException for RuntimeError {}

pub struct SyntaxError;

impl RubyException for SyntaxError {}

pub struct TypeError;

impl RubyException for TypeError {}

#[cfg(test)]
mod tests {
    use crate::def::{ClassLike, Define};
    use crate::eval::Eval;
    use crate::exception::Exception;
    use crate::extn::core::exception::{RubyException, RuntimeError};
    use crate::file::File;
    use crate::sys;
    use crate::{Artichoke, ArtichokeError};

    struct Run;

    impl Run {
        unsafe extern "C" fn run(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            let interp = unwrap_interpreter!(mrb);
            RuntimeError::raise(interp, "something went wrong")
        }
    }

    impl File for Run {
        fn require(interp: Artichoke) -> Result<(), ArtichokeError> {
            let spec = interp.0.borrow_mut().def_class::<Self>("Run", None, None);
            spec.borrow_mut()
                .add_self_method("run", Self::run, sys::mrb_args_none());
            spec.borrow().define(&interp)?;
            Ok(())
        }
    }

    #[test]
    fn raise() {
        let interp = crate::interpreter().expect("init");
        Run::require(interp.clone()).unwrap();
        let value = interp.eval("Run.run").map(|_| ());
        let expected = Exception::new(
            "RuntimeError",
            "something went wrong",
            Some(vec!["(eval):1".to_owned()]),
            "(eval):1: something went wrong (RuntimeError)",
        );
        assert_eq!(value, Err(ArtichokeError::Exec(expected.to_string())));
    }
}
