use std::borrow::Cow;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::hint;

use crate::sys;
use crate::{Artichoke, Guard};

#[derive(Debug)]
pub struct Error(Box<dyn RubyException>);

impl RubyException for Error {
    fn message(&self) -> Cow<'_, [u8]> {
        self.0.message()
    }

    /// Class name of the `Exception`.
    fn name(&self) -> Cow<'_, str> {
        self.0.name()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        self.0.vm_backtrace(interp)
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        self.0.as_mrb_value(interp)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for Error {}

impl From<Box<dyn RubyException>> for Error {
    fn from(exc: Box<dyn RubyException>) -> Self {
        Self(exc)
    }
}

static RUNTIME_ERROR_CSTR: &CStr = qed::const_cstr_from_str!("RuntimeError\0");
static UNABLE_TO_RAISE_MESSAGE: &CStr = qed::const_cstr_from_str!("Unable to raise exception\0");

/// Raise implementation for [`RubyException`] boxed trait objects.
///
/// # Safety
///
/// This function unwinds the stack with `longjmp`, which will ignore all Rust
/// landing pads for panics and exit routines for cleaning up borrows. Callers
/// should ensure that only [`Copy`] items are alive in the current stack frame.
///
/// Because this precondition must hold for all frames between the caller and
/// the closest [`sys::mrb_protect`] landing pad, this function should only be
/// called in the entry point into Rust from mruby.
pub unsafe fn raise<T>(mut guard: Guard<'_>, exception: T) -> !
where
    T: RubyException + fmt::Debug,
{
    // Convert the `RubyException` into a raisable boxed Ruby value.
    let exc = exception.as_mrb_value(&mut guard);

    // Pull out the raw pointer to the `mrb_state` so we can drop down to raw
    // `MRB_API` functions.
    let mrb: *mut sys::mrb_state = guard.mrb.as_mut();

    // Ensure the Artichoke `State` is moved back into the `mrb_state`.
    drop(guard);

    if let Some(exc) = exc {
        // Any non-`Copy` objects that we haven't cleaned up at this point will
        // leak, so drop everything.
        drop(exception);

        // `mrb_exc_raise` will call longjmp which will unwind the stack.
        sys::mrb_exc_raise(mrb, exc);

        // SAFETY: This line is unreachable because `raise` will unwind the
        // stack with `longjmp` when calling `sys::mrb_exc_raise` in the
        // preceding line.
        hint::unreachable_unchecked()
    }

    // Being unable to turn the given exception into an `mrb_value` is a bug, so
    // log loudly to stderr and attempt to fallback to a runtime error.
    emit_fatal_warning!("Unable to raise exception: {:?}", exception);

    // Any non-`Copy` objects that we haven't cleaned up at this point will
    // leak, so drop everything.
    drop(exception);

    // `mrb_sys_raise` will call longjmp which will unwind the stack.
    sys::mrb_sys_raise(mrb, RUNTIME_ERROR_CSTR.as_ptr(), UNABLE_TO_RAISE_MESSAGE.as_ptr());

    // SAFETY: This line is unreachable because `raise` will unwind the stack
    // with `longjmp` when calling `sys::mrb_exc_raise` in the preceding line.
    hint::unreachable_unchecked()
}

/// Polymorphic exception type that corresponds to Ruby's `Exception`.
///
/// All types that implement `RubyException` can be raised with
/// [`error::raise`](raise). Rust code can re-raise a trait object to
/// propagate exceptions from native code back into the interpreter.
#[allow(clippy::module_name_repetitions)]
pub trait RubyException: error::Error + 'static {
    /// Message of the `Exception`.
    ///
    /// This value is a byte slice since Ruby `String`s are equivalent to
    /// `Vec<u8>`.
    fn message(&self) -> Cow<'_, [u8]>;

    /// Class name of the `Exception`.
    fn name(&self) -> Cow<'_, str>;

    /// Optional backtrace specified by a `Vec` of frames.
    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>>;

    /// Return a raise-able [`sys::mrb_value`].
    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value>;
}

impl RubyException for Box<dyn RubyException> {
    fn message(&self) -> Cow<'_, [u8]> {
        self.as_ref().message()
    }

    fn name(&self) -> Cow<'_, str> {
        self.as_ref().name()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        self.as_ref().vm_backtrace(interp)
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        self.as_ref().as_mrb_value(interp)
    }
}

impl error::Error for Box<dyn RubyException> {}
