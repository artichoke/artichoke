//! # Ruby Exception Hierarchy
//!
//! The built-in subclasses of
//! [`Exception`](https://ruby-doc.org/core-2.6.3/Exception.html) are:
//!
//! - `NoMemoryError`
//! - `ScriptError`
//!   - `LoadError`
//!   - `NotImplementedError`
//!   - `SyntaxError`
//! - `SecurityError`
//! - `SignalException`
//!   - `Interrupt`
//! - `StandardError` -- default for `rescue`
//!   - `ArgumentError`
//!     - `UncaughtThrowError`
//!   - `EncodingError`
//!   - `FiberError`
//!   - `IOError`
//!     - `EOFError`
//!   - `IndexError`
//!     - `KeyError`
//!     - `StopIteration`
//!   - `LocalJumpError`
//!   - `NameError`
//!     - `NoMethodError`
//!   - `RangeError`
//!     - `FloatDomainError`
//!   - `RegexpError`
//!   - `RuntimeError` -- default for `raise`
//!     - `FrozenError`
//!   - `SystemCallError`
//!     - `Errno::*`
//!   - `ThreadError`
//!   - `TypeError`
//!   - `ZeroDivisionError`
//! - `SystemExit`
//! - `SystemStackError`
//! - `fatal` -- impossible to rescue

use bstr::{BStr, ByteSlice};
use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let exception_spec = class::Spec::new("Exception", None, None)?;
    class::Builder::for_spec(interp, &exception_spec).define()?;
    interp.def_class::<Exception>(exception_spec)?;

    let nomemory_spec = class::Spec::new("NoMemoryError", None, None)?;
    class::Builder::for_spec(interp, &nomemory_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<NoMemoryError>(nomemory_spec)?;

    let script_spec = class::Spec::new("ScriptError", None, None)?;
    class::Builder::for_spec(interp, &script_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<ScriptError>(script_spec)?;

    let load_spec = class::Spec::new("LoadError", None, None)?;
    class::Builder::for_spec(interp, &load_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<LoadError>(load_spec)?;

    let notimplemented_spec = class::Spec::new("NotImplementedError", None, None)?;
    class::Builder::for_spec(interp, &notimplemented_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<NotImplementedError>(notimplemented_spec)?;

    let syntax_spec = class::Spec::new("SyntaxError", None, None)?;
    class::Builder::for_spec(interp, &syntax_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<SyntaxError>(syntax_spec)?;

    let security_spec = class::Spec::new("SecurityError", None, None)?;
    class::Builder::for_spec(interp, &security_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SecurityError>(security_spec)?;

    let signal_spec = class::Spec::new("SignalException", None, None)?;
    class::Builder::for_spec(interp, &signal_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SignalException>(signal_spec)?;

    let interrupt_spec = class::Spec::new("Interrupt", None, None)?;
    class::Builder::for_spec(interp, &interrupt_spec)
        .with_super_class::<SignalException, _>("SignalException")?
        .define()?;
    interp.def_class::<Interrupt>(interrupt_spec)?;

    // Default for `rescue`.
    let standard_spec = class::Spec::new("StandardError", None, None)?;
    class::Builder::for_spec(interp, &standard_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<StandardError>(standard_spec)?;

    let argument_spec = class::Spec::new("ArgumentError", None, None)?;
    class::Builder::for_spec(interp, &argument_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ArgumentError>(argument_spec)?;

    let uncaughthrow_spec = class::Spec::new("UncaughtThrowError", None, None)?;
    class::Builder::for_spec(interp, &uncaughthrow_spec)
        .with_super_class::<ArgumentError, _>("ArgumentError")?
        .define()?;
    interp.def_class::<UncaughtThrowError>(uncaughthrow_spec)?;

    let encoding_spec = class::Spec::new("EncodingError", None, None)?;
    class::Builder::for_spec(interp, &encoding_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<EncodingError>(encoding_spec)?;

    let fiber_spec = class::Spec::new("FiberError", None, None)?;
    class::Builder::for_spec(interp, &fiber_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<FiberError>(fiber_spec)?;

    let io_spec = class::Spec::new("IOError", None, None)?;
    class::Builder::for_spec(interp, &io_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<IOError>(io_spec)?;

    let eof_spec = class::Spec::new("EOFError", None, None)?;
    class::Builder::for_spec(interp, &eof_spec)
        .with_super_class::<IOError, _>("IOError")?
        .define()?;
    interp.def_class::<EOFError>(eof_spec)?;

    let index_spec = class::Spec::new("IndexError", None, None)?;
    class::Builder::for_spec(interp, &index_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<IndexError>(index_spec)?;

    let key_spec = class::Spec::new("KeyError", None, None)?;
    class::Builder::for_spec(interp, &key_spec)
        .with_super_class::<IndexError, _>("IndexError")?
        .define()?;
    interp.def_class::<KeyError>(key_spec)?;

    let stopiteration_spec = class::Spec::new("StopIteration", None, None)?;
    class::Builder::for_spec(interp, &stopiteration_spec)
        .with_super_class::<IndexError, _>("IndexError")?
        .define()?;
    interp.def_class::<StopIteration>(stopiteration_spec)?;

    let localjump_spec = class::Spec::new("LocalJumpError", None, None)?;
    class::Builder::for_spec(interp, &localjump_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<LocalJumpError>(localjump_spec)?;

    let name_spec = class::Spec::new("NameError", None, None)?;
    class::Builder::for_spec(interp, &name_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<NameError>(name_spec)?;

    let nomethod_spec = class::Spec::new("NoMethodError", None, None)?;
    class::Builder::for_spec(interp, &nomethod_spec)
        .with_super_class::<NameError, _>("NameError")?
        .define()?;
    interp.def_class::<NoMethodError>(nomethod_spec)?;

    let range_spec = class::Spec::new("RangeError", None, None)?;
    class::Builder::for_spec(interp, &range_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RangeError>(range_spec)?;

    let floatdomain_spec = class::Spec::new("FloatDomainError", None, None)?;
    class::Builder::for_spec(interp, &floatdomain_spec)
        .with_super_class::<RangeError, _>("RangeError")?
        .define()?;
    interp.def_class::<FloatDomainError>(floatdomain_spec)?;

    let regexp_spec = class::Spec::new("RegexpError", None, None)?;
    class::Builder::for_spec(interp, &regexp_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RegexpError>(regexp_spec)?;

    // Default `Exception` type for `raise`.
    let runtime_spec = class::Spec::new("RuntimeError", None, None)?;
    class::Builder::for_spec(interp, &runtime_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RuntimeError>(runtime_spec)?;

    let frozen_spec = class::Spec::new("FrozenError", None, None)?;
    class::Builder::for_spec(interp, &frozen_spec)
        .with_super_class::<RuntimeError, _>("RuntimeError")?
        .define()?;
    interp.def_class::<FrozenError>(frozen_spec)?;

    let systemcall_spec = class::Spec::new("SystemCallError", None, None)?;
    class::Builder::for_spec(interp, &systemcall_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<SystemCallError>(systemcall_spec)?;

    let thread_spec = class::Spec::new("ThreadError", None, None)?;
    class::Builder::for_spec(interp, &thread_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ThreadError>(thread_spec)?;

    let type_spec = class::Spec::new("TypeError", None, None)?;
    class::Builder::for_spec(interp, &type_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<TypeError>(type_spec)?;

    let zerodivision_spec = class::Spec::new("ZeroDivisionError", None, None)?;
    class::Builder::for_spec(interp, &zerodivision_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ZeroDivisionError>(zerodivision_spec)?;

    let systemexit_spec = class::Spec::new("SystemExit", None, None)?;
    class::Builder::for_spec(interp, &systemexit_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SystemExit>(systemexit_spec)?;

    let systemstack_spec = class::Spec::new("SystemStackError", None, None)?;
    class::Builder::for_spec(interp, &systemstack_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SystemStackError>(systemstack_spec)?;

    let fatal_spec = class::Spec::new("fatal", None, None)?;
    class::Builder::for_spec(interp, &fatal_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<Fatal>(fatal_spec)?;

    let _ = interp.eval(&include_bytes!("exception.rb")[..])?;
    trace!("Patched Exception onto interpreter");
    trace!("Patched core exception hierarchy onto interpreter");
    Ok(())
}

macro_rules! ruby_exception_impl {
    ($exception:ident) => {
        #[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $exception {
            message: Cow<'static, BStr>,
        }

        impl $exception {
            #[must_use]
            pub fn new() -> Self {
                // `Exception`s initialized via `raise RuntimeError` or
                // `RuntimeError.new` have `message` equal to the `Exception`
                // class name.
                let message = Cow::Borrowed(stringify!($exception).as_bytes().into());
                Self { message }
            }
        }

        impl From<String> for $exception {
            fn from(message: String) -> Self {
                let message = Cow::Owned(message.into_bytes().into());
                Self { message }
            }
        }

        impl From<&'static str> for $exception {
            fn from(message: &'static str) -> Self {
                let message = Cow::Borrowed(message.as_bytes().into());
                Self { message }
            }
        }

        impl From<Cow<'static, str>> for $exception {
            fn from(message: Cow<'static, str>) -> Self {
                let message = match message {
                    Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes().into()),
                    Cow::Owned(s) => Cow::Owned(s.into_bytes().into()),
                };
                Self { message }
            }
        }

        impl From<Vec<u8>> for $exception {
            fn from(message: Vec<u8>) -> Self {
                let message = Cow::Owned(message.into());
                Self { message }
            }
        }

        impl From<&'static [u8]> for $exception {
            fn from(message: &'static [u8]) -> Self {
                let message = message.as_bstr().into();
                Self { message }
            }
        }

        impl From<Cow<'static, [u8]>> for $exception {
            fn from(message: Cow<'static, [u8]>) -> Self {
                let message = match message {
                    Cow::Borrowed(s) => Cow::Borrowed(s.into()),
                    Cow::Owned(s) => Cow::Owned(s.into()),
                };
                Self { message }
            }
        }

        impl From<$exception> for Error {
            fn from(exception: $exception) -> Error {
                Error::from(Box::<dyn RubyException>::from(exception))
            }
        }

        impl From<Box<$exception>> for Error {
            fn from(exception: Box<$exception>) -> Error {
                Error::from(Box::<dyn RubyException>::from(exception))
            }
        }

        impl From<$exception> for Box<dyn RubyException> {
            fn from(exception: $exception) -> Box<dyn RubyException> {
                Box::new(exception)
            }
        }

        impl From<Box<$exception>> for Box<dyn RubyException> {
            fn from(exception: Box<$exception>) -> Box<dyn RubyException> {
                exception
            }
        }

        impl RubyException for $exception {
            fn message(&self) -> Cow<'_, [u8]> {
                Cow::Borrowed(self.message.as_ref())
            }

            fn name(&self) -> Cow<'_, str> {
                stringify!($exception).into()
            }

            fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
                let _ = interp;
                None
            }

            fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
                let message = interp.convert_mut(self.message());
                let value = interp.new_instance::<Self>(&[message]).ok().flatten()?;
                Some(value.inner())
            }
        }

        impl fmt::Display for $exception {
            fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.name())?;
                f.write_str(" (")?;
                string::format_unicode_debug_into(&mut f, &self.message())
                    .map_err(string::WriteError::into_inner)?;
                f.write_str(")")?;
                Ok(())
            }
        }

        impl error::Error for $exception {}
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
// ruby_exception_impl!(Errno::*);
ruby_exception_impl!(ThreadError);
ruby_exception_impl!(TypeError);
ruby_exception_impl!(ZeroDivisionError);
ruby_exception_impl!(SystemExit);
ruby_exception_impl!(SystemStackError);
// Fatal interpreter error. Impossible to rescue.
ruby_exception_impl!(Fatal);

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    struct Run;

    unsafe extern "C" fn run_run(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let mut interp = unwrap_interpreter!(mrb);
        let guard = Guard::new(&mut interp);
        let exc = RuntimeError::from("something went wrong");
        error::raise(guard, exc)
    }

    impl File for Run {
        type Artichoke = Artichoke;

        type Error = Error;

        fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
            let spec = class::Spec::new("Run", None, None).unwrap();
            class::Builder::for_spec(interp, &spec)
                .add_self_method("run", run_run, sys::mrb_args_none())?
                .define()?;
            interp.def_class::<Self>(spec)?;
            Ok(())
        }
    }

    #[test]
    fn raise() {
        let mut interp = interpreter().unwrap();
        Run::require(&mut interp).unwrap();
        let err = interp.eval(b"Run.run").unwrap_err();
        assert_eq!("RuntimeError", err.name().as_ref());
        assert_eq!(&b"something went wrong"[..], err.message().as_ref());
        assert_eq!(
            Some(vec![Vec::from(&b"(eval):1"[..])]),
            err.vm_backtrace(&mut interp)
        );
    }
}
