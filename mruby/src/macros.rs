//! This module defines macros for working with interpreters and `Value`s. This
//! source module is included first in `lib.rs`, which means the macros are
//! available to all modules within the mruby crate in addition to being
//! exported.

/// Extract an [`Mrb`](interpreter::Mrb) instance from the userdata on a
/// [`sys::mrb_state`]. If there is an error when extracting the Rust wrapper
/// around the interpreter, attempt to raise a `RuntimeError` and return `nil`.
///
/// This macro is `unsafe` and assumes it is being called from an `extern "C" fn`
/// that is embedded in an mruby class, module, or function definition.
#[macro_export]
macro_rules! interpreter_or_raise {
    ($mrb:expr) => {
        match $crate::interpreter::Interpreter::from_user_data($mrb) {
            std::result::Result::Err(err) => {
                // Unable to retrieve interpreter from user data pointer in
                // `mrb_state`.
                let eclass = std::ffi::CString::new("RuntimeError");
                let message = std::ffi::CString::new(format!("{}", err));
                if let (std::result::Result::Ok(eclass), std::result::Result::Ok(message)) =
                    (eclass, message)
                {
                    $crate::sys::mrb_sys_raise($mrb, eclass.as_ptr(), message.as_ptr());
                }
                // must call the sys function directly because we could not
                // extract an interp.
                return $crate::sys::mrb_sys_nil_value();
            }
            std::result::Result::Ok(interpreter) => interpreter,
        }
    };
}

/// Unwrap a `Result` or raise a `RuntimeError` and return `default`.
///
/// This macro is `unsafe` and assumes it is being called from an `extern "C" fn`
/// that is embedded in an mruby class, module, or function definition.
#[macro_export]
macro_rules! unwrap_or_raise {
    ($interp:expr, $result:expr, $onerr:expr) => {
        match $result {
            std::result::Result::Err(err) => {
                // There was a TypeError converting to the desired Rust type.
                let eclass = std::ffi::CString::new("RuntimeError");
                let message = std::ffi::CString::new(format!("{}", err));
                if let (std::result::Result::Ok(eclass), std::result::Result::Ok(message)) =
                    (eclass, message)
                {
                    $crate::sys::mrb_sys_raise(
                        $interp.borrow().mrb,
                        eclass.as_ptr(),
                        message.as_ptr(),
                    );
                }
                return $onerr;
            }
            std::result::Result::Ok(value) => value,
        }
    };
}

/// Unwrap a `Result<Value>` and return the inner [`sys::mrb_value`] or raise a
/// `RuntimeError` and return `nil`.
///
/// This macro is `unsafe` and assumes it is being called from an `extern "C" fn`
/// that is embedded in an mruby class, module, or function definition.
#[macro_export]
macro_rules! unwrap_value_or_raise {
    ($interp:expr, $result:expr) => {
        unwrap_or_raise!(
            $interp,
            $result,
            $crate::interpreter::MrbApi::nil(&$interp).inner()
        )
        .inner()
    };
}

/// Lookup a [`class::Spec`] for a Rust type `T`. If the spec does not exist,
/// raise on the interpreter and return `nil`.
///
/// This macro is `unsafe` and assumes it is being called from an `extern "C" fn`
/// that is embedded in an mruby class, module, or function definition.
#[macro_export]
macro_rules! class_spec_or_raise {
    ($interp:expr, $type:ty) => {
        if let Some(spec) = $interp.borrow().class_spec::<$type>() {
            spec
        } else {
            // The class spec does not exist or has not been deifned with
            // `State::def_class` yet.
            let eclass = std::ffi::CString::new("RuntimeError");
            let message = std::ffi::CString::new("Uninitialized Class");
            if let (std::result::Result::Ok(eclass), std::result::Result::Ok(message)) =
                (eclass, message)
            {
                $crate::sys::mrb_sys_raise($interp.borrow().mrb, eclass.as_ptr(), message.as_ptr());
            }
            return $crate::interpreter::MrbApi::nil(&$interp).inner();
        }
    };
}

/// Lookup a [`module::Spec`] for a Rust type `T`. If the spec does not exist,
/// raise on the interpreter and return `nil`.
///
/// This macro is `unsafe` and assumes it is being called from an `extern "C" fn`
/// that is embedded in an mruby class, module, or function definition.
#[macro_export]
macro_rules! module_spec_or_raise {
    ($interp:expr, $type:ty) => {
        if let Some(spec) = $interp.borrow().module_spec::<$type>() {
            spec
        } else {
            // The module spec does not exist or has not been deifned with
            // `State::def_module` yet.
            let eclass = std::ffi::CString::new("RuntimeError");
            let message = std::ffi::CString::new("Uninitialized Module");
            if let (std::result::Result::Ok(eclass), std::result::Result::Ok(message)) =
                (eclass, message)
            {
                $crate::sys::mrb_sys_raise($interp.borrow().mrb, eclass.as_ptr(), message.as_ptr());
            }
            return $crate::interpreter::MrbApi::nil(&$interp).inner();
        }
    };
}
