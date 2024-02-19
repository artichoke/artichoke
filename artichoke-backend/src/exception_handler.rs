use std::borrow::Cow;
use std::error;
use std::fmt;

use bstr::BString;
use scolapasta_string_escape::format_debug_escape_into;

use crate::core::{TryConvertMut, Value as _};
use crate::error::{Error, RubyException};
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

/// Incrementally construct a [`CaughtException`].
///
/// See also [`CaughtException::builder`].
#[derive(Default, Debug)]
pub struct Builder(CaughtException);

impl Builder {
    /// Construct a new, empty `Builder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_value(mut self, value: Value) -> Self {
        self.0.value = value;
        self
    }

    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.0.name = name;
        self
    }

    #[must_use]
    pub fn with_message(mut self, message: Vec<u8>) -> Self {
        self.0.message = message.into();
        self
    }

    #[must_use]
    pub fn finish(self) -> CaughtException {
        self.0
    }
}

/// An `Exception` rescued with [`sys::mrb_protect`].
///
/// `CaughtException` is re-raiseable because it implements [`RubyException`].
#[derive(Default, Debug, Clone)]
pub struct CaughtException {
    value: Value,
    name: String,
    message: BString,
}

impl CaughtException {
    /// Incrementally construct a [`CaughtException`].
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Construct a new `CaughtException`.
    #[must_use]
    pub fn with_value_class_and_message(value: Value, name: String, message: Vec<u8>) -> Self {
        let message = message.into();
        Self { value, name, message }
    }
}

impl fmt::Display for CaughtException {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())?;
        f.write_str(" (")?;
        format_debug_escape_into(&mut f, &self.message())?;
        f.write_str(")")?;
        Ok(())
    }
}

impl error::Error for CaughtException {}

impl RubyException for CaughtException {
    fn message(&self) -> Cow<'_, [u8]> {
        self.message.as_slice().into()
    }

    fn name(&self) -> Cow<'_, str> {
        self.name.as_str().into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let backtrace = self.value.funcall(interp, "backtrace", &[], None).ok()?;
        let backtrace = interp.try_convert_mut(backtrace).ok()?;
        Some(backtrace)
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let _ = interp;
        Some(self.value.inner())
    }
}

impl From<CaughtException> for Box<dyn RubyException> {
    fn from(exc: CaughtException) -> Self {
        Box::new(exc)
    }
}

impl From<CaughtException> for Error {
    fn from(exc: CaughtException) -> Self {
        Self::from(Box::<dyn RubyException>::from(exc))
    }
}

/// Transform a `Exception` Ruby `Value` into an [`Error`].
///
/// # Errors
///
/// This function makes fallible calls into the Ruby VM to resolve the
/// exception. If these calls return an error, that error is returned from this
/// function.
pub fn last_error(interp: &mut Artichoke, exception: Value) -> Result<Error, Error> {
    let mut arena = interp.create_arena_savepoint()?;

    // Clear the current exception from the mruby interpreter so subsequent
    // calls to the mruby VM are not tainted by an error they did not
    // generate.
    //
    // We must clear the pointer at the beginning of this function so we can
    // use the mruby VM to inspect the exception once we turn it into an
    // `mrb_value`. `Value::funcall` handles errors by calling this
    // function, so not clearing the exception results in a stack overflow.

    // Generate exception metadata in by executing the Ruby code:
    //
    // ```ruby
    // clazz = exception.class.name
    // message = exception.message
    // ```

    // Sometimes when hacking on `extn/core` it is possible to enter a
    // crash loop where an exception is captured by this handler, but
    // extracting the exception name or backtrace throws again.
    // Un-commenting the following print statement will at least get you the
    // exception class and message, which should help debugging.
    //
    // ```
    // let message = exception.funcall(&mut arena, "message", &[], None)?;
    // let message = message.try_convert_into_mut::<String>(&mut arena);
    // println!("{:?}, {:?}", exception, message);
    // ```

    let class = exception.funcall(&mut arena, "class", &[], None)?;
    let classname = class.funcall(&mut arena, "name", &[], None)?;
    let classname = classname.try_convert_into_mut::<&str>(&mut arena)?;
    let message = exception.funcall(&mut arena, "message", &[], None)?;
    let message = message.try_convert_into_mut::<&[u8]>(&mut arena)?;

    let exc = CaughtException::builder()
        .with_value(exception)
        .with_name(classname.into())
        .with_message(message.to_vec())
        .finish();
    Ok(Error::from(exc))
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    #[test]
    fn return_exception() {
        let mut interp = interpreter();
        let err = interp.eval(b"raise ArgumentError.new('waffles')").unwrap_err();
        assert_eq!("ArgumentError", err.name().as_ref());
        assert_eq!(b"waffles".as_bstr(), err.message().as_ref().as_bstr());
        let expected_backtrace = b"(eval):1".to_vec();
        let backtrace = bstr::join("\n", err.vm_backtrace(&mut interp).unwrap());
        assert_eq!(backtrace.as_bstr(), expected_backtrace.as_bstr());
    }

    #[test]
    fn return_exception_with_no_backtrace() {
        let mut interp = interpreter();
        let err = interp.eval(b"def bad; (; end").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_ref());
        assert_eq!(b"syntax error".as_bstr(), err.message().as_ref().as_bstr());
        assert_eq!(None, err.vm_backtrace(&mut interp));
    }

    #[test]
    fn raise_does_not_panic_or_segfault() {
        let mut interp = interpreter();
        interp.eval(b"raise 'foo'").unwrap_err();
        interp.eval(b"raise 'foo'").unwrap_err();
        interp.eval(br#"eval("raise 'foo'")"#).unwrap_err();
        interp.eval(br#"eval("raise 'foo'")"#).unwrap_err();
        interp.eval(b"require 'foo'").unwrap_err();
        interp.eval(b"require 'foo'").unwrap_err();
        interp.eval(br#"eval("require 'foo'")"#).unwrap_err();
        interp.eval(br#"eval("require 'foo'")"#).unwrap_err();
        interp.eval(b"Regexp.compile(2)").unwrap_err();
        interp.eval(b"Regexp.compile(2)").unwrap_err();
        #[cfg(feature = "core-regexp")]
        {
            interp.eval(br#"eval("Regexp.compile(2)")"#).unwrap_err();
            interp.eval(br#"eval("Regexp.compile(2)")"#).unwrap_err();
        }
        #[cfg(feature = "stdlib-forwardable")]
        {
            const REQUIRE_TEST: &[u8] = b"\
def fail
  begin
    require 'foo'
  rescue LoadError
    require 'forwardable'
  end
end

fail
";
            interp.eval(REQUIRE_TEST).unwrap();
        }
        let kernel = interp.eval(b"Kernel").unwrap();
        kernel.funcall(&mut interp, "raise", &[], None).unwrap_err();
        kernel.funcall(&mut interp, "raise", &[], None).unwrap_err();
    }
}
