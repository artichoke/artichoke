use crate::exception::{CaughtException, Exception};
use crate::gc::MrbGarbageCollection;
use crate::value::Value;
use crate::{Artichoke, ValueLike};

/// Transform a `Exception` Ruby `Value` into an [`Exception`].
///
/// # Errors
///
/// This function makes funcalls on the interpreter which are fallible.
pub fn last_error(interp: &Artichoke, exception: Value) -> Result<Exception, Exception> {
    let _arena = interp.create_arena_savepoint();
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

    // Sometimes when hacking on extn/core it is possible to enter a
    // crash loop where an exception is captured by this handler, but
    // extracting the exception name or backtrace throws again.
    // Uncommenting the folllowing print statement will at least get you
    // the exception class and message, which should help debugging.
    //
    // println!("{:?}", exception);

    let class = exception.funcall::<Value>("class", &[], None)?;
    let classname = class.funcall::<&str>("name", &[], None)?;
    let message = exception.funcall::<&[u8]>("message", &[], None)?;

    let exception = CaughtException::new(exception, classname, message);
    debug!("Extracted exception from interpreter: {}", exception);
    Ok(Exception::from(exception))
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn return_exception() {
        let mut interp = crate::interpreter().expect("init");
        let err = interp
            .eval(b"raise ArgumentError.new('waffles')")
            .unwrap_err();
        assert_eq!("ArgumentError", err.name().as_str());
        assert_eq!(&b"waffles"[..], err.message());
        assert_eq!(
            Some(vec![Vec::from(&b"(eval):1"[..])]),
            err.vm_backtrace(&mut interp)
        );
    }

    #[test]
    fn return_exception_with_no_backtrace() {
        let mut interp = crate::interpreter().expect("init");
        let err = interp.eval(b"def bad; (; end").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
        assert_eq!(&b"syntax error"[..], err.message());
        assert_eq!(None, err.vm_backtrace(&mut interp));
    }

    #[test]
    fn raise_does_not_panic_or_segfault() {
        let mut interp = crate::interpreter().expect("init");
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
