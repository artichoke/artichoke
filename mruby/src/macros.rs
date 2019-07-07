// This module defines macros for working with interpreters and `Value`s. This
// source module is included first in `lib.rs`, which means the macros are
// available to all modules within the mruby crate in addition to being
// exported.

/// Extract an [`Mrb`](interpreter::Mrb) instance from the userdata on a
/// [`sys::mrb_state`].
///
/// If there is an error when extracting the Rust wrapper around the
/// interpreter, attempt to raise a `RuntimeError` and return `nil`.
///
/// This macro is `unsafe`.
#[macro_export]
macro_rules! interpreter_or_raise {
    ($mrb:expr) => {
        match $crate::ffi::from_user_data($mrb) {
            std::result::Result::Err(err) => {
                // Unable to retrieve interpreter from user data pointer in
                // `mrb_state`.
                let eclass = std::ffi::CString::new("RuntimeError");
                let message = std::ffi::CString::new(format!("{}", err));
                if let (std::result::Result::Ok(eclass), std::result::Result::Ok(message)) =
                    (eclass, message)
                {
                    // must call the sys function directly because we could not
                    // extract an interp.
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
