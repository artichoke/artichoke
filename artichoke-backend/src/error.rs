use std::borrow::Cow;
use std::error;
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

/// Raise implementation for `RubyException` boxed trait objects.
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
    let exc = exception.as_mrb_value(&mut guard);
    let mrb: *mut sys::mrb_state = guard.mrb.as_mut();
    drop(guard);
    if let Some(exc) = exc {
        // Any non-`Copy` objects that we haven't cleaned up at this point will
        // leak, so drop everything.
        drop(exception);
        // `mrb_exc_raise` will call longjmp which will unwind the stack.
        sys::mrb_exc_raise(mrb, exc);
    } else {
        error!("unable to raise {:?}", exception);
        // Any non-`Copy` objects that we haven't cleaned up at this point will
        // leak, so drop everything.
        drop(exception);
        // `mrb_sys_raise` will call longjmp which will unwind the stack.
        sys::mrb_sys_raise(
            mrb,
            "RuntimeError\0".as_ptr().cast::<i8>(),
            "Unable to raise exception".as_ptr().cast::<i8>(),
        );
    }
    // unreachable: `raise` will unwind the stack with longjmp.
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
