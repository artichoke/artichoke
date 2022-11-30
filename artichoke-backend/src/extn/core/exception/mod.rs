//! Ruby error handling types.
//!
//! This module implements the [`Exception`] class from Ruby Core. It is a
//! collection of error types that the interpreter uses to unwind the stack in
//! the event of an error with [`Kernel#raise`].
//!
//! You can use these types by accessing them in the interpreter. The types are
//! globally available in the root namespace.
//!
//! ```ruby
//! RuntimeError.new
//!
//! raise ArgumentError, "missing semicolon"
//! ```
//!
//! This module implements the core exception types with [`spinoso-exception`]
//! and re-exports these types.
//!
//! [`Exception`]: https://ruby-doc.org/core-3.1.2/Exception.html
//! [`Kernel#raise`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-raise
//! [`spinoso-exception`]: spinoso_exception

use std::borrow::Cow;

#[doc(inline)]
pub use spinoso_exception::core::*;

use crate::extn::prelude::*;

pub(in crate::extn) mod mruby;

/// Implement traits to convert Spinoso exceptions to artichoke-backend error
/// types.
///
/// This macro implements the artichoke-backend trait [`RubyException`]. This
/// trait differs from [`RubyException` in spinoso-exception] by having
/// additional APIs for gluing to the mruby VM, such as [backtraces].
///
/// [`RubyException` in spinoso-exception]: spinoso_exception::RubyException
/// [backtraces]: RubyException::vm_backtrace
macro_rules! ruby_exception_impl {
    ($exc:ty) => {
        impl From<$exc> for Error {
            fn from(exception: $exc) -> Error {
                let err: Box<dyn RubyException> = Box::new(exception);
                Self::from(err)
            }
        }

        impl RubyException for $exc {
            fn message(&self) -> Cow<'_, [u8]> {
                Cow::Borrowed(Self::message(self))
            }

            fn name(&self) -> Cow<'_, str> {
                Cow::Borrowed(Self::name(self))
            }

            fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
                let _ = interp;
                None
            }

            fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
                let message = interp.try_convert_mut(self.message()).ok()?;
                let value = interp.new_instance::<Self>(&[message]).ok().flatten()?;
                Some(value.inner())
            }
        }
    };
}

ruby_exception_impl!(Exception);
ruby_exception_impl!(NoMemoryError);
ruby_exception_impl!(ScriptError);
ruby_exception_impl!(LoadError);
ruby_exception_impl!(NotImplementedError);
ruby_exception_impl!(SyntaxError);
ruby_exception_impl!(SecurityError);
ruby_exception_impl!(SignalException);
ruby_exception_impl!(Interrupt);
// Default for `rescue`.
ruby_exception_impl!(StandardError);
ruby_exception_impl!(ArgumentError);
ruby_exception_impl!(UncaughtThrowError);
ruby_exception_impl!(EncodingError);
ruby_exception_impl!(FiberError);
ruby_exception_impl!(IOError);
ruby_exception_impl!(EOFError);
ruby_exception_impl!(IndexError);
ruby_exception_impl!(KeyError);
ruby_exception_impl!(StopIteration);
ruby_exception_impl!(LocalJumpError);
ruby_exception_impl!(NameError);
ruby_exception_impl!(NoMethodError);
ruby_exception_impl!(RangeError);
ruby_exception_impl!(FloatDomainError);
ruby_exception_impl!(RegexpError);
// Default `Exception` type for `raise`.
ruby_exception_impl!(RuntimeError);
ruby_exception_impl!(FrozenError);
ruby_exception_impl!(SystemCallError);
// TODO: Implement `Errno` family of exceptions.
ruby_exception_impl!(ThreadError);
ruby_exception_impl!(TypeError);
ruby_exception_impl!(ZeroDivisionError);
ruby_exception_impl!(SystemExit);
ruby_exception_impl!(SystemStackError);
// Fatal interpreter error. Impossible to rescue.
ruby_exception_impl!(Fatal);

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    struct Run;

    unsafe extern "C-unwind" fn run_run(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        unwrap_interpreter!(mrb, to => guard);
        let exc = RuntimeError::with_message("something went wrong");
        error::raise(guard, exc)
    }

    impl File for Run {
        type Artichoke = Artichoke;

        type Error = Error;

        fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
            let spec = class::Spec::new("Run", qed::const_cstr_from_str!("Run\0"), None, None).unwrap();
            class::Builder::for_spec(interp, &spec)
                .add_self_method("run", run_run, sys::mrb_args_none())?
                .define()?;
            interp.def_class::<Self>(spec)?;
            Ok(())
        }
    }

    #[test]
    fn raise() {
        let mut interp = interpreter();
        Run::require(&mut interp).unwrap();
        let err = interp.eval(b"Run.run").unwrap_err();
        assert_eq!("RuntimeError", err.name().as_ref());
        assert_eq!(b"something went wrong".as_bstr(), err.message().as_ref().as_bstr());
        let expected_backtrace = b"(eval):1:in run\n(eval):1".to_vec();
        let actual_backtrace = bstr::join("\n", err.vm_backtrace(&mut interp).unwrap());
        assert_eq!(expected_backtrace.as_bstr(), actual_backtrace.as_bstr());
    }
}
