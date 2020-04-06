use std::ffi::OsStr;

use crate::exception::Exception;
use crate::exception_handler;
use crate::extn::core::exception::Fatal;
use crate::ffi;
use crate::sys::{self, protect};
use crate::value::Value;
use crate::{Artichoke, Eval};

impl Eval for Artichoke {
    type Value = Value;

    type Error = Exception;

    fn eval(&mut self, code: &[u8]) -> Result<Self::Value, Self::Error> {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Artichoke` to
        // get access to the underlying `ArtichokeState`.
        let mrb = self.0.borrow().mrb;
        let context = self.0.borrow_mut().parser.context_mut() as *mut _;

        trace!("Evaling code on {}", sys::mrb_sys_state_debug(mrb));
        match unsafe { protect::eval(mrb, context, code) } {
            Ok(value) => {
                let value = Value::new(self, value);
                if value.is_unreachable() {
                    // Unreachable values are internal to the mruby interpreter
                    // and interacting with them via the C API is unspecified
                    // and may result in a segfault.
                    //
                    // See: https://github.com/mruby/mruby/issues/4460
                    Err(Exception::from(Fatal::new(self, "Unreachable Ruby value")))
                } else {
                    Ok(value)
                }
            }
            Err(exception) => {
                let exception = Value::new(self, exception);
                Err(exception_handler::last_error(self, exception)?)
            }
        }
    }

    fn eval_os_str(&mut self, code: &OsStr) -> Result<Self::Value, Self::Error> {
        let code = ffi::os_str_to_bytes(code)?;
        self.eval(&code)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn root_eval_context() {
        let mut interp = crate::interpreter().unwrap();
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_into::<&str>(&mut interp).unwrap();
        assert_eq!(result, "(eval)");
    }

    #[test]
    fn context_is_restored_after_eval() {
        let mut interp = crate::interpreter().unwrap();
        let context = Context::new(&b"context.rb"[..]).unwrap();
        interp.push_context(context);
        let _ = interp.eval(b"15").unwrap();
        assert_eq!(
            // TODO(GH-468): Use `Parser::peek_context`.
            interp.0.borrow().parser.peek_context().unwrap().filename(),
            &b"context.rb"[..]
        );
    }

    #[test]
    fn root_context_is_not_pushed_after_eval() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(b"15").unwrap();
        // TODO(GH-468): Use `Parser::peek_context`.
        assert!(interp.0.borrow().parser.peek_context().is_none());
    }

    mod nested {
        use crate::test::prelude::*;

        #[derive(Debug)]
        struct NestedEval;

        unsafe extern "C" fn nested_eval_file(
            mrb: *mut sys::mrb_state,
            _slf: sys::mrb_value,
        ) -> sys::mrb_value {
            let mut interp = unwrap_interpreter!(mrb);
            if let Ok(value) = interp.eval(b"__FILE__") {
                value.inner()
            } else {
                interp.convert(None::<Value>).inner()
            }
        }

        impl File for NestedEval {
            type Artichoke = Artichoke;

            type Error = Exception;

            fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
                let spec = module::Spec::new(interp, "NestedEval", None)?;
                module::Builder::for_spec(interp, &spec)
                    .add_self_method("file", nested_eval_file, sys::mrb_args_none())?
                    .define()?;
                interp.0.borrow_mut().def_module::<Self>(spec);
                Ok(())
            }
        }

        #[test]
        #[should_panic]
        // this test is known broken
        fn eval_context_is_a_stack() {
            let mut interp = crate::interpreter().unwrap();
            interp
                .def_file_for_type::<NestedEval>(b"nested_eval.rb")
                .unwrap();
            let code = br#"require 'nested_eval'; NestedEval.file"#;
            let result = interp.eval(code).unwrap();
            let result = result.try_into::<&str>(&mut interp).unwrap();
            assert_eq!(result, "/src/lib/nested_eval.rb");
        }
    }

    #[test]
    fn eval_with_context() {
        let mut interp = crate::interpreter().unwrap();

        interp.push_context(Context::new(b"source.rb".as_ref()).unwrap());
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_into::<&str>(&mut interp).unwrap();
        assert_eq!(result, "source.rb");
        interp.pop_context();

        interp.push_context(Context::new(b"source.rb".as_ref()).unwrap());
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_into::<&str>(&mut interp).unwrap();
        assert_eq!(result, "source.rb");
        interp.pop_context();

        interp.push_context(Context::new(b"main.rb".as_ref()).unwrap());
        let result = interp.eval(b"__FILE__").unwrap();
        let result = result.try_into::<&str>(&mut interp).unwrap();
        assert_eq!(result, "main.rb");
        interp.pop_context();
    }

    #[test]
    fn unparseable_code_returns_err_syntax_error() {
        let mut interp = crate::interpreter().unwrap();
        let err = interp.eval(b"'a").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
    }

    #[test]
    fn interpreter_is_usable_after_syntax_error() {
        let mut interp = crate::interpreter().unwrap();
        let err = interp.eval(b"'a").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
        // Ensure interpreter is usable after evaling unparseable code
        let result = interp.eval(b"'a' * 10 ").unwrap();
        let result = result.try_into::<&str>(&mut interp).unwrap();
        assert_eq!(result, "a".repeat(10));
    }

    #[test]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn file_magic_constant() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_rb_source_file(b"source.rb", &b"def file; __FILE__; end"[..])
            .unwrap();
        let result = interp.eval(b"require 'source'; file").unwrap();
        let result = result.try_into::<&str>(&mut interp).unwrap();
        assert_eq!(result, "/src/lib/source.rb");
    }

    #[test]
    fn file_not_persistent() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_rb_source_file(b"source.rb", &b"def file; __FILE__; end"[..])
            .unwrap();
        let result = interp.eval(b"require 'source'; __FILE__").unwrap();
        let result = result.try_into::<&str>(&mut interp).unwrap();
        assert_eq!(result, "(eval)");
    }

    #[test]
    fn return_syntax_error() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_rb_source_file(b"fail.rb", &b"def bad; 'as'.scan(; end"[..])
            .unwrap();
        let err = interp.eval(b"require 'fail'").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
    }
}
