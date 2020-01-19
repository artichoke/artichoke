use std::ffi::c_void;
use std::fmt;

use crate::extn::core::exception::RubyException;
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::{Artichoke, ArtichokeError};

/// Metadata about a Ruby exception.
#[derive(Debug, Clone)]
#[must_use]
pub struct Exception {
    value: Value,
    name: String,
    message: Vec<u8>,
}

impl Exception {
    pub fn new(value: Value, name: &str, message: &[u8]) -> Self {
        Self {
            value,
            name: name.to_owned(),
            message: message.to_vec(),
        }
    }
}

impl RubyException for Exception {
    fn box_clone(&self) -> Box<dyn RubyException> {
        Box::new(self.clone())
    }

    fn message(&self) -> &[u8] {
        self.message.as_slice()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn rclass(&self) -> Option<*mut sys::RClass> {
        let inner = self.value.inner();
        let rclass = unsafe { sys::mrb_sys_class_ptr(inner) };
        Some(rclass)
    }
}

impl From<Exception> for Box<dyn RubyException> {
    fn from(exc: Exception) -> Self {
        Box::new(exc)
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}

/// Extract the last exception thrown on the interpreter.
#[allow(clippy::module_name_repetitions)]
pub trait ExceptionHandler {
    /// Extract the last thrown exception on the artichoke interpreter if there
    /// is one.
    ///
    /// If there is an error, return [`LastError::Some`], which contains the
    /// exception class name, message, and optional backtrace.
    fn last_error(&self) -> Result<Option<Exception>, ArtichokeError>;
}

impl ExceptionHandler for Artichoke {
    fn last_error(&self) -> Result<Option<Exception>, ArtichokeError> {
        let _arena = self.create_arena_savepoint();
        let mrb = self.0.borrow().mrb;
        let exc = unsafe {
            let exc = (*mrb).exc;
            // Clear the current exception from the mruby interpreter so
            // subsequent calls to the mruby VM are not tainted by an error they
            // did not generate.
            //
            // We must do this at the beginning of `current_exception` so we can
            // use the mruby VM to inspect the exception once we turn it into an
            // `mrb_value`. `ValueLike::funcall` handles errors by calling this
            // function, so not clearing the exception results in a stack
            // overflow.
            (*mrb).exc = std::ptr::null_mut();
            exc
        };
        if exc.is_null() {
            trace!("No last error present");
            return Ok(None);
        }
        // Generate exception metadata in by executing the following Ruby code:
        //
        // ```ruby
        // clazz = exception.class.name
        // message = exception.message
        // ```
        let exception = Value::new(self, unsafe { sys::mrb_sys_obj_value(exc as *mut c_void) });
        // Sometimes when hacking on extn/core it is possible to enter a crash
        // loop where an exception is captured by this handler, but extracting
        // the exception name or backtrace throws again. Uncommenting the
        // folllowing print statement will at least get you the exception class
        // and message, which should help debugging.
        //
        // println!("{:?}", exception);
        let classname = exception
            .funcall::<Value>("class", &[], None)
            .and_then(|exception| exception.funcall::<&str>("name", &[], None))?;
        let message = exception.funcall::<&[u8]>("message", &[], None)?;
        let exception = Exception::new(exception, classname, message);
        debug!("Extracted exception from interpreter: {}", exception);
        Ok(Some(exception))
    }
}

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;

    use crate::exception::Exception;
    use crate::value::{Value, ValueLike};
    use crate::ArtichokeError;

    #[test]
    fn return_exception() {
        let interp = crate::interpreter().expect("init");
        let result = interp
            .eval(b"raise ArgumentError.new('waffles')")
            .map(|_| ());
        let expected = Exception::new(
            "ArgumentError",
            "waffles",
            Some(vec!["(eval):1".to_owned()]),
            "(eval):1: waffles (ArgumentError)",
        );
        assert_eq!(result, Err(ArtichokeError::Exec(expected.to_string())));
    }

    #[test]
    fn return_exception_with_no_backtrace() {
        let interp = crate::interpreter().expect("init");
        let result = interp.eval(b"def bad; (; end").map(|_| ());
        let expected = Exception::new("SyntaxError", "waffles", None, "SyntaxError: syntax error");
        assert_eq!(result, Err(ArtichokeError::Exec(expected.to_string())));
    }

    #[test]
    fn raise_does_not_panic_or_segfault() {
        let interp = crate::interpreter().expect("init");
        let _ = interp.eval(br#"raise 'foo'"#);
        let _ = interp.eval(br#"raise 'foo'"#);
        let _ = interp.eval(br#"eval(b"raise 'foo'""#);
        let _ = interp.eval(br#"eval(b"raise 'foo'""#);
        let _ = interp.eval(br#"require 'foo'"#);
        let _ = interp.eval(br#"require 'foo'"#);
        let _ = interp.eval(br#"eval(b"require 'foo'""#);
        let _ = interp.eval(br#"eval(b"require 'foo'""#);
        let _ = interp.eval(br#"Regexp.compile(2)"#);
        let _ = interp.eval(br#"Regexp.compile(2)"#);
        let _ = interp.eval(br#"eval(b"Regexp.compile(2)""#);
        let _ = interp.eval(br#"eval(b"Regexp.compile(2)""#);
        let _ = interp.eval(
            br#"
def fail
  begin
    require 'foo'
  rescue LoadError
    require 'forwardable'
  end
end

fail
            "#,
        );
        let kernel = interp.eval(br#"Kernel"#).unwrap();
        let _ = kernel.funcall::<Value>("raise", &[], None);
        let _ = kernel.funcall::<Value>("raise", &[], None);
    }
}
