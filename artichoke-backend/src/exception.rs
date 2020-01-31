use std::error;
use std::fmt;

use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ValueLike};

#[derive(Debug)]
pub struct Exception(Box<dyn RubyException>);

impl RubyException for Exception {
    fn box_clone(&self) -> Box<dyn RubyException> {
        self.0.box_clone()
    }

    fn message(&self) -> &[u8] {
        self.0.message()
    }

    /// Class name of the `Exception`.
    fn name(&self) -> String {
        self.0.name()
    }

    fn backtrace(&self, interp: &Artichoke) -> Option<Vec<Vec<u8>>> {
        self.0.backtrace(interp)
    }

    fn as_mrb_value(&self, interp: &Artichoke) -> Option<sys::mrb_value> {
        self.0.as_mrb_value(interp)
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for Exception {
    fn description(&self) -> &str {
        "Ruby Exception thrown on Artichoke VM"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl From<Box<dyn RubyException>> for Exception {
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
/// called in the entrypoint into Rust from mruby.
pub unsafe fn raise(interp: Artichoke, exception: impl RubyException + fmt::Debug) -> ! {
    let exc = if let Some(exc) = exception.as_mrb_value(&interp) {
        exc
    } else {
        error!("unable to raise {:?}", exception);
        panic!("unable to raise {:?}", exception);
    };
    // `mrb_sys_raise` will call longjmp which will unwind the stack.
    // Any non-`Copy` objects that we haven't cleaned up at this point will
    // leak, so drop everything.
    let mrb = Artichoke::into_raw(interp);
    drop(exception);

    sys::mrb_exc_raise(mrb, exc);
    unreachable!("mrb_exc_raise will unwind the stack with longjmp");
}

/// Polymorphic exception type that corresponds to Ruby's `Exception`.
///
/// All types that implement `RubyException` can be raised with
/// [`exception::raise`](raise). Rust code can re-raise a trait object to
/// propagate exceptions from native code back into the interpreter.
#[allow(clippy::module_name_repetitions)]
pub trait RubyException
where
    Self: 'static,
{
    /// Clone `self` and return a new boxed trait object.
    fn box_clone(&self) -> Box<dyn RubyException>;

    /// Message of the `Exception`.
    ///
    /// This value is a byte slice since Ruby `String`s are equivalent to
    /// `Vec<u8>`.
    fn message(&self) -> &[u8];

    /// Class name of the `Exception`.
    fn name(&self) -> String;

    /// Optional backtrace specified by a `Vec` of frames.
    fn backtrace(&self, interp: &Artichoke) -> Option<Vec<Vec<u8>>>;

    /// Return a raiseable [`sys::mrb_value`].
    fn as_mrb_value(&self, interp: &Artichoke) -> Option<sys::mrb_value>;
}

impl RubyException for Box<dyn RubyException> {
    fn box_clone(&self) -> Box<dyn RubyException> {
        self.as_ref().box_clone()
    }

    fn message(&self) -> &[u8] {
        self.as_ref().message()
    }

    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn backtrace(&self, interp: &Artichoke) -> Option<Vec<Vec<u8>>> {
        self.as_ref().backtrace(interp)
    }

    fn as_mrb_value(&self, interp: &Artichoke) -> Option<sys::mrb_value> {
        self.as_ref().as_mrb_value(interp)
    }
}

impl fmt::Debug for Box<dyn RubyException> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}

impl fmt::Display for Box<dyn RubyException> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}

impl error::Error for Box<dyn RubyException> {
    fn description(&self) -> &str {
        "Ruby Exception thrown on Artichoke VM"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Debug for &dyn RubyException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}

impl fmt::Display for &dyn RubyException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}

impl error::Error for &dyn RubyException {
    fn description(&self) -> &str {
        "Ruby Exception thrown on Artichoke VM"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

/// An `Exception` rescued with [`sys::mrb_protect`].
///
/// `CaughtException` is re-raiseable because it implements [`RubyException`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub(crate) struct CaughtException {
    value: Value,
    name: String,
    message: Vec<u8>,
}

impl CaughtException {
    /// Construct a new `CaughtException`.
    pub fn new(value: Value, name: &str, message: &[u8]) -> Self {
        Self {
            value,
            name: name.to_owned(),
            message: message.to_vec(),
        }
    }
}

impl RubyException for CaughtException {
    fn box_clone(&self) -> Box<dyn RubyException> {
        Box::new(self.clone())
    }

    fn message(&self) -> &[u8] {
        self.message.as_slice()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn backtrace(&self, interp: &Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        self.value.funcall("backtrace", &[], None).ok()
    }

    fn as_mrb_value(&self, interp: &Artichoke) -> Option<sys::mrb_value> {
        let _ = interp;
        Some(self.value.inner())
    }
}

#[allow(clippy::use_self)]
impl From<CaughtException> for Box<dyn RubyException> {
    fn from(exc: CaughtException) -> Self {
        Box::new(exc)
    }
}

impl From<CaughtException> for Exception {
    fn from(exc: CaughtException) -> Self {
        Self(Box::new(exc))
    }
}

impl fmt::Display for CaughtException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}
