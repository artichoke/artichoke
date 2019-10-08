#![allow(warnings)]
use std::ffi::c_void;
use std::fmt;

use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::{Artichoke, ArtichokeError};

/// Metadata about a Ruby exception.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Exception {
    /// The result of calling `exception.class.name`.
    pub class: String,
    /// The result of calling `exception.message`.
    pub message: String,
    /// The result of calling `exception.backtrace`.
    ///
    /// Some exceptions, like `SyntaxError` which is thrown directly by the
    /// artichoke VM, do not have backtraces, so this field is optional.
    pub backtrace: Option<Vec<String>>,
    /// The result of calling `exception.inspect`.
    pub inspect: String,
}

impl Exception {
    pub fn new(class: &str, message: &str, backtrace: Option<Vec<String>>, inspect: &str) -> Self {
        Self {
            class: class.to_owned(),
            message: message.to_owned(),
            backtrace,
            inspect: inspect.to_owned(),
        }
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inspect)?;
        if let Some(ref backtrace) = self.backtrace {
            for frame in backtrace {
                write!(f, "\n{}", frame)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LastError {
    Some(Exception),
    None,
    UnableToExtract(ArtichokeError),
}

/// Extract the last exception thrown on the interpreter.
#[allow(clippy::module_name_repetitions)]
pub trait ExceptionHandler {
    /// Extract the last thrown exception on the artichoke interpreter if there
    /// is one.
    ///
    /// If there is an error, return [`LastError::Some`], which contains the
    /// exception class name, message, and optional backtrace.
    fn last_error(&self) -> LastError;
}

impl ExceptionHandler for Artichoke {
    fn last_error(&self) -> LastError {
        let _arena = self.create_arena_savepoint();
        let mrb = { self.0.borrow().mrb };
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
            return LastError::None;
        }
        // Generate exception metadata in by executing the following Ruby code:
        //
        // ```ruby
        // clazz = exception.class.name
        // message = exception.message
        // backtrace = exception.backtrace
        // ```
        let exception = Value::new(self, unsafe { sys::mrb_sys_obj_value(exc as *mut c_void) });
        let class = exception
            .funcall::<Value>("class", &[], None)
            .and_then(|exception| exception.funcall::<String>("name", &[], None));
        let class = match class {
            Ok(class) => class,
            Err(err) => return LastError::UnableToExtract(err),
        };
        let message = match exception.funcall::<String>("message", &[], None) {
            Ok(message) => message,
            Err(err) => return LastError::UnableToExtract(err),
        };
        let backtrace = match exception.funcall::<Option<Vec<String>>>("backtrace", &[], None) {
            Ok(backtrace) => backtrace,
            Err(err) => return LastError::UnableToExtract(err),
        };
        let inspect = match exception.funcall::<String>("inspect", &[], None) {
            Ok(inspect) => inspect,
            Err(err) => return LastError::UnableToExtract(err),
        };
        let exception = Exception {
            class,
            message,
            backtrace,
            inspect,
        };
        debug!("Extracted exception from interpreter: {}", exception);
        LastError::Some(exception)
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::exception::Exception;
    use crate::value::{Value, ValueLike};
    use crate::ArtichokeError;

    #[test]
    fn return_exception() {
        let interp = crate::interpreter().expect("init");
        let result = interp
            .eval("raise ArgumentError.new('waffles')")
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
        let result = interp.eval("def bad; (; end").map(|_| ());
        let expected = Exception::new("SyntaxError", "waffles", None, "SyntaxError: syntax error");
        assert_eq!(result, Err(ArtichokeError::Exec(expected.to_string())));
    }

    #[test]
    fn raise_does_not_panic_or_segfault() {
        let interp = crate::interpreter().expect("init");
        let _ = interp.eval(r#"raise 'foo'"#);
        let _ = interp.eval(r#"raise 'foo'"#);
        let _ = interp.eval(r#"eval "raise 'foo'""#);
        let _ = interp.eval(r#"eval "raise 'foo'""#);
        let _ = interp.eval(r#"require 'foo'"#);
        let _ = interp.eval(r#"require 'foo'"#);
        let _ = interp.eval(r#"eval "require 'foo'""#);
        let _ = interp.eval(r#"eval "require 'foo'""#);
        let _ = interp.eval(r#"Regexp.compile(2)"#);
        let _ = interp.eval(r#"Regexp.compile(2)"#);
        let _ = interp.eval(r#"eval "Regexp.compile(2)""#);
        let _ = interp.eval(r#"eval "Regexp.compile(2)""#);
        let _ = interp.eval(
            r#"
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
        let kernel = interp.eval(r#"Kernel"#).unwrap();
        let _ = kernel.funcall::<Value>("raise", &[], None);
        let _ = kernel.funcall::<Value>("raise", &[], None);
    }
}
