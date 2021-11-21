use std::ffi::OsStr;
use std::path::Path;

use bstr::ByteSlice;

use crate::core::{Eval, LoadSources, Parser, Value as _};
use crate::error::Error;
use crate::exception_handler;
use crate::extn::core::exception::{ArgumentError, Fatal};
use crate::ffi::InterpreterExtractError;
use crate::platform_string::os_str_to_bytes;
use crate::state::parser::Context;
use crate::sys;
use crate::sys::protect;
use crate::value::Value;
use crate::Artichoke;

impl Eval for Artichoke {
    type Value = Value;

    type Error = Error;

    fn eval(&mut self, code: &[u8]) -> Result<Self::Value, Self::Error> {
        trace!("Attempting eval of Ruby source");
        let result = unsafe {
            let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
            let parser = state.parser.as_mut().ok_or_else(InterpreterExtractError::new)?;
            let context: *mut sys::mrbc_context = parser.context_mut();
            self.with_ffi_boundary(|mrb| protect::eval(mrb, context, code))?
        };
        match result {
            Ok(value) => {
                let value = Value::from(value);
                if value.is_unreachable() {
                    // Unreachable values are internal to the mruby interpreter
                    // and interacting with them via the C API is unspecified
                    // and may result in a segfault.
                    //
                    // See: https://github.com/mruby/mruby/issues/4460
                    error!("Fatal eval returned unreachable value");
                    Err(Fatal::from("Unreachable Ruby value").into())
                } else {
                    trace!("Sucessful eval");
                    Ok(self.protect(value))
                }
            }
            Err(exception) => {
                let exception = self.protect(Value::from(exception));
                let debug = exception.inspect(self);
                debug!("Failed eval raised exception: {:?}", debug.as_bstr());
                Err(exception_handler::last_error(self, exception)?)
            }
        }
    }

    fn eval_os_str(&mut self, code: &OsStr) -> Result<Self::Value, Self::Error> {
        let code = os_str_to_bytes(code)?;
        self.eval(code)
    }

    fn eval_file(&mut self, file: &Path) -> Result<Self::Value, Self::Error> {
        let context = Context::new(os_str_to_bytes(file.as_os_str())?.to_vec())
            .ok_or_else(|| ArgumentError::with_message("path name contains null byte"))?;
        self.push_context(context)?;
        let code = self.read_source_file_contents(file)?.into_owned();
        let result = self.eval(code.as_slice());
        self.pop_context()?;
        result
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    #[test]
    fn root_eval_context() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(result, "(eval)");
    }

    #[test]
    fn context_is_restored_after_eval() {
        let mut interp = interpreter().unwrap();
        let context = Context::new(&b"context.rb"[..]).unwrap();
        interp.push_context(context).unwrap();
        interp.eval(b"15").unwrap();
        let context = interp.peek_context().unwrap();
        let filename = context.unwrap().filename();
        assert_eq!(filename.as_bstr(), b"context.rb".as_bstr());
    }

    #[test]
    fn root_context_is_not_pushed_after_eval() {
        let mut interp = interpreter().unwrap();
        interp.eval(b"15").unwrap();
        let context = interp.peek_context().unwrap();
        assert!(context.is_none());
    }

    mod nested {
        use crate::test::prelude::*;

        #[derive(Debug)]
        struct NestedEval;

        unsafe extern "C" fn nested_eval_file(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            unwrap_interpreter!(mrb, to => guard);
            let result = if let Ok(value) = guard.eval(b"__FILE__") {
                value
            } else {
                Value::nil()
            };
            result.inner()
        }

        impl File for NestedEval {
            type Artichoke = Artichoke;

            type Error = Error;

            fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
                let spec = module::Spec::new(interp, "NestedEval", cstr::cstr!("NestedEval"), None)?;
                module::Builder::for_spec(interp, &spec)
                    .add_self_method("file", nested_eval_file, sys::mrb_args_none())?
                    .define()?;
                interp.def_module::<Self>(spec)?;
                Ok(())
            }
        }

        #[test]
        #[should_panic]
        // this test is known broken
        fn eval_context_is_a_stack() {
            let mut interp = interpreter().unwrap();
            interp.def_file_for_type::<_, NestedEval>("nested_eval.rb").unwrap();
            let code = br#"require 'nested_eval'; NestedEval.file"#;
            let result = interp.eval(code).unwrap();
            let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
            assert_eq!(result, "/src/lib/nested_eval.rb");
        }
    }

    #[test]
    fn eval_with_context() {
        let mut interp = interpreter().unwrap();

        let context = Context::new(b"source.rb".as_ref()).unwrap();
        interp.push_context(context).unwrap();
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(result, "source.rb");
        interp.pop_context().unwrap();

        let context = Context::new(b"source.rb".as_ref()).unwrap();
        interp.push_context(context).unwrap();
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(result, "source.rb");
        interp.pop_context().unwrap();

        let context = Context::new(b"main.rb".as_ref()).unwrap();
        interp.push_context(context).unwrap();
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(result, "main.rb");
        interp.pop_context().unwrap();
    }

    #[test]
    fn unparseable_code_returns_err_syntax_error() {
        let mut interp = interpreter().unwrap();
        let err = interp.eval(b"'a").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_ref());
    }

    #[test]
    fn interpreter_is_usable_after_syntax_error() {
        let mut interp = interpreter().unwrap();
        let err = interp.eval(b"'a").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_ref());
        // Ensure interpreter is usable after evaling unparseable code
        let result = interp.eval(b"'a' * 10 ").unwrap();
        let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(result, "a".repeat(10));
    }

    #[test]
    fn file_magic_constant() {
        let file = if cfg!(windows) {
            "c:/artichoke/virtual_root/src/lib/source.rb"
        } else {
            "/artichoke/virtual_root/src/lib/source.rb"
        };
        let mut interp = interpreter().unwrap();
        interp
            .def_rb_source_file("source.rb", &b"def file; __FILE__; end"[..])
            .unwrap();
        let result = interp.eval(b"require 'source'; file").unwrap();
        let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(result, file);
    }

    #[test]
    fn file_not_persistent() {
        let mut interp = interpreter().unwrap();
        interp
            .def_rb_source_file("source.rb", &b"def file; __FILE__; end"[..])
            .unwrap();
        let result = interp.eval(b"require 'source'; __FILE__").unwrap();
        let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(result, "(eval)");
    }

    #[test]
    fn return_syntax_error() {
        let mut interp = interpreter().unwrap();
        interp
            .def_rb_source_file("fail.rb", &b"def bad; 'as'.scan(; end"[..])
            .unwrap();
        let err = interp.eval(b"require 'fail'").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_ref());
    }
}
