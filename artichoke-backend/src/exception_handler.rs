use artichoke_core::value::Value as _;
use std::ffi::c_void;
use std::mem;
use std::ptr::{self, NonNull};

use crate::exception::{CaughtException, Exception};
// use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

/// Extract the last exception thrown on the interpreter.
pub trait ExceptionHandler {
    /// Extract the last thrown exception on the artichoke interpreter if there
    /// is one.
    ///
    /// If there is an error, return an [`Exception`], which contains the
    /// exception class name, message, and optional backtrace.
    fn last_error(&mut self) -> Result<Option<Exception>, Exception>;
}

impl ExceptionHandler for Artichoke {
    fn last_error(&mut self) -> Result<Option<Exception>, Exception> {
        // TODO: fix arena
        // let _arena = self.create_arena_savepoint();
        // Clear the current exception from the mruby interpreter so subsequent
        // calls to the mruby VM are not tainted by an error they did not
        // generate. We must do this at the beginning of `current_exception` so
        // we can use the mruby VM to inspect the exception once we turn it into
        // an `mrb_value`. `Value::funcall` handles errors by calling this
        // function, so not clearing the exception results in a stack overflow.
        let exc = mem::replace(&mut self.mrb_mut().exc, ptr::null_mut());
        let exc = if let Some(exc) = NonNull::new(exc) {
            exc
        } else {
            trace!("No last error present");
            return Ok(None);
        };
        // Generate exception metadata in by executing the following Ruby code:
        //
        // ```ruby
        // clazz = exception.class.name
        // message = exception.message
        // ```
        let exception = Value::new(self, unsafe {
            sys::mrb_sys_obj_value(exc.as_ptr() as *mut c_void)
        });
        // Sometimes when hacking on extn/core it is possible to enter a crash
        // loop where an exception is captured by this handler, but extracting
        // the exception name or backtrace throws again.
        // Uncommenting the
        // folllowing print statement will at least get you the exception class
        // and message, which should help debugging.
        //
        println!("{}", exception.to_s_debug(self));
        let classname = exception
            .funcall::<Value>(self, "class", &[], None)
            .and_then(|exception| exception.funcall::<&str>(self, "name", &[], None))?;
        let message = exception.funcall::<&[u8]>(self, "message", &[], None)?;
        let exception = CaughtException::new(exception, classname, message);
        debug!("Extracted exception from interpreter: {}", exception);
        Ok(Some(Exception::from(exception)))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn return_exception() {
        let interp = crate::interpreter().expect("init");
        let err = interp
            .eval(b"raise ArgumentError.new('waffles')")
            .unwrap_err();
        assert_eq!("ArgumentError", err.name().as_str());
        assert_eq!(&b"waffles"[..], err.message());
        assert_eq!(
            Some(vec![Vec::from(&b"(eval):1"[..])]),
            err.backtrace(&interp)
        );
    }

    #[test]
    fn return_exception_with_no_backtrace() {
        let interp = crate::interpreter().expect("init");
        let err = interp.eval(b"def bad; (; end").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
        assert_eq!(&b"syntax error"[..], err.message());
        assert_eq!(None, err.backtrace(&interp));
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
