use crate::core::Value as _;
use crate::exception::{CaughtException, Exception};
use crate::gc::MrbGarbageCollection;
use crate::value::Value;
use crate::Artichoke;

/// Transform a `Exception` Ruby `Value` into an [`Exception`].
///
/// # Errors
///
/// This function makes funcalls on the interpreter which are fallible.
pub fn last_error(interp: &mut Artichoke, exception: Value) -> Result<Exception, Exception> {
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

    // Sometimes when hacking on extn/core it is possible to enter a
    // crash loop where an exception is captured by this handler, but
    // extracting the exception name or backtrace throws again.
    // Uncommenting the folllowing print statement will at least get you
    // the exception class and message, which should help debugging.
    //
    // let message = exception.funcall(&mut arena, "message", &[], None)?;
    // let message = message.try_into_mut::<String>(&mut arena);
    // println!("{:?}, {:?}", exception, message);

    let class = exception.funcall(&mut arena, "class", &[], None)?;
    let classname = class.funcall(&mut arena, "name", &[], None)?;
    let classname = classname.try_into_mut::<&str>(&mut arena)?;
    let message = exception.funcall(&mut arena, "message", &[], None)?;
    let message = message.try_into_mut::<&[u8]>(&mut arena)?;

    let exception = CaughtException::new(exception, String::from(classname), message.to_vec());
    debug!("Extracted exception from interpreter: {}", exception);
    Ok(Exception::from(exception))
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn return_exception() {
        let mut interp = interpreter().unwrap();
        let err = interp
            .eval(b"raise ArgumentError.new('waffles')")
            .unwrap_err();
        assert_eq!("ArgumentError", err.name().as_ref());
        assert_eq!(&b"waffles"[..], err.message().as_ref());
        assert_eq!(
            Some(vec![Vec::from(&b"(eval):1"[..])]),
            err.vm_backtrace(&mut interp)
        );
    }

    #[test]
    fn return_exception_with_no_backtrace() {
        let mut interp = interpreter().unwrap();
        let err = interp.eval(b"def bad; (; end").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_ref());
        assert_eq!(&b"syntax error"[..], err.message().as_ref());
        assert_eq!(None, err.vm_backtrace(&mut interp));
    }

    #[test]
    fn raise_does_not_panic_or_segfault() {
        let mut interp = interpreter().unwrap();
        let _ = interp.eval(br#"raise 'foo'"#).unwrap_err();
        let _ = interp.eval(br#"raise 'foo'"#).unwrap_err();
        let _ = interp.eval(br#"eval("raise 'foo'")"#).unwrap_err();
        let _ = interp.eval(br#"eval("raise 'foo'")"#).unwrap_err();
        let _ = interp.eval(br#"require 'foo'"#).unwrap_err();
        let _ = interp.eval(br#"require 'foo'"#).unwrap_err();
        let _ = interp.eval(br#"eval("require 'foo'")"#).unwrap_err();
        let _ = interp.eval(br#"eval("require 'foo'")"#).unwrap_err();
        let _ = interp.eval(br#"Regexp.compile(2)"#).unwrap_err();
        let _ = interp.eval(br#"Regexp.compile(2)"#).unwrap_err();
        let _ = interp.eval(br#"eval("Regexp.compile(2)")"#).unwrap_err();
        let _ = interp.eval(br#"eval("Regexp.compile(2)")"#).unwrap_err();
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
        let _ = kernel.funcall(&mut interp, "raise", &[], None).unwrap_err();
        let _ = kernel.funcall(&mut interp, "raise", &[], None).unwrap_err();
    }
}
