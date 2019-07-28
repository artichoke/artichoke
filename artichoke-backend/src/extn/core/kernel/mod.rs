use log::trace;
use std::io::{self, Write};
use std::rc::Rc;

use crate::convert::Convert;
use crate::def::{ClassLike, Define};
use crate::eval::{EvalContext, MrbEval};
use crate::extn::core::error::{ArgumentError, LoadError, RubyException, RuntimeError};
use crate::sys;
use crate::value::types::Ruby;
use crate::value::{Value, ValueLike};
use crate::{Mrb, MrbError};

mod args;
pub mod require;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    let warning = interp.borrow_mut().def_module::<Warning>("Warning", None);
    warning
        .borrow_mut()
        .add_method("warn", Warning::warn, sys::mrb_args_req(1));
    warning
        .borrow_mut()
        .add_self_method("warn", Warning::warn, sys::mrb_args_req(1));
    warning.borrow().define(interp).map_err(|_| MrbError::New)?;
    let kernel = interp.borrow_mut().def_module::<Kernel>("Kernel", None);
    kernel
        .borrow_mut()
        .add_method("require", Kernel::require, sys::mrb_args_rest());
    kernel.borrow_mut().add_self_method(
        "require_relative",
        Kernel::require_relative,
        sys::mrb_args_rest(),
    );
    kernel
        .borrow_mut()
        .add_method("print", Kernel::print, sys::mrb_args_rest());
    kernel
        .borrow_mut()
        .add_method("puts", Kernel::puts, sys::mrb_args_rest());
    kernel
        .borrow_mut()
        .add_method("warn", Kernel::warn, sys::mrb_args_rest());
    kernel.borrow().define(interp).map_err(|_| MrbError::New)?;
    interp.eval(include_str!("kernel.rb"))?;
    trace!("Patched Kernel#require onto interpreter");
    Ok(())
}

pub struct Warning;

impl Warning {
    unsafe extern "C" fn warn(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let stderr = sys::mrb_gv_get(mrb, interp.borrow_mut().sym_intern("$stderr"));
        if !sys::mrb_sys_value_is_nil(stderr) {
            let args = args::Rest::extract(&interp);
            let stderr = Value::new(&interp, stderr);
            // TODO: introduce a `unchecked_funcall` to propagate errors.
            let _ = stderr
                .funcall::<Value, _, _>("print", args.map(|args| args.rest).unwrap_or_default());
        }
        sys::mrb_sys_nil_value()
    }
}

pub struct Kernel;

impl Kernel {
    unsafe extern "C" fn require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let args = require::Args::extract(&interp);
        let result = args.and_then(|args| require::method::require(&interp, args));
        match result {
            Ok(req) => {
                let result = if let Some(req) = req.rust {
                    req(Rc::clone(&interp))
                } else {
                    Ok(())
                };
                if result.is_ok() {
                    if let Some(contents) = req.ruby {
                        interp.unchecked_eval_with_context(contents, EvalContext::new(req.file));
                    }
                    Value::convert(&interp, true).inner()
                } else {
                    LoadError::raisef(interp, "cannot load such file -- %S", vec![req.file])
                }
            }
            Err(require::Error::AlreadyRequired) => Value::convert(&interp, false).inner(),
            Err(require::Error::CannotLoad(file)) => {
                LoadError::raisef(interp, "cannot load such file -- %S", vec![file])
            }
            Err(require::Error::Fatal) => RuntimeError::raise(interp, "fatal Kernel#require error"),
            Err(require::Error::NoImplicitConversionToString) => {
                ArgumentError::raise(interp, "No implicit conversion to String")
            }
        }
    }

    unsafe extern "C" fn require_relative(
        mrb: *mut sys::mrb_state,
        _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let args = require::Args::extract(&interp);
        let result = args.and_then(|args| require::method::require_relative(&interp, args));
        match result {
            Ok(req) => {
                let result = if let Some(req) = req.rust {
                    req(Rc::clone(&interp))
                } else {
                    Ok(())
                };
                if result.is_ok() {
                    if let Some(contents) = req.ruby {
                        interp.unchecked_eval_with_context(contents, EvalContext::new(req.file));
                    }
                    Value::convert(&interp, true).inner()
                } else {
                    LoadError::raisef(interp, "cannot load such file -- %S", vec![req.file])
                }
            }
            Err(require::Error::AlreadyRequired) => Value::convert(&interp, false).inner(),
            Err(require::Error::CannotLoad(file)) => {
                LoadError::raisef(interp, "cannot load such file -- %S", vec![file])
            }
            Err(require::Error::Fatal) => RuntimeError::raise(interp, "fatal Kernel#require error"),
            Err(require::Error::NoImplicitConversionToString) => {
                ArgumentError::raise(interp, "No implicit conversion to String")
            }
        }
    }

    unsafe extern "C" fn print(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let args = args::Rest::extract(&interp);

        for value in args.map(|args| args.rest).unwrap_or_default() {
            print!("{}", value.to_s());
        }
        let _ = io::stdout().flush();
        sys::mrb_sys_nil_value()
    }

    unsafe extern "C" fn puts(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        fn do_puts(value: Value) {
            if value.ruby_type() == Ruby::Array {
                if let Ok(array) = value.try_into::<Vec<Value>>() {
                    for value in array {
                        do_puts(value);
                    }
                }
            } else {
                println!("{}", value.to_s());
            }
        }

        let interp = unwrap_interpreter!(mrb);
        let rest = args::Rest::extract(&interp)
            .map(|args| args.rest)
            .unwrap_or_default();

        if rest.is_empty() {
            println!();
        }
        for value in rest {
            do_puts(value);
        }
        sys::mrb_sys_nil_value()
    }

    unsafe extern "C" fn warn(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let args = args::Rest::extract(&interp);

        for value in args.map(|args| args.rest).unwrap_or_default() {
            let mut string = value.to_s();
            if !string.ends_with('\n') {
                string = format!("{}\n", string);
            }
            Warning::warn(mrb, Value::convert(&interp, string).inner());
        }
        sys::mrb_sys_nil_value()
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::TryConvert;
    use crate::eval::MrbEval;
    use crate::file::MrbFile;
    use crate::load::MrbLoadSources;
    use crate::{Mrb, MrbError};

    // Integration test for `Kernel::require`:
    //
    // - require side effects (e.g. ivar set or class def) effect the interpreter
    // - Successful first require returns `true`.
    // - Second require returns `false`.
    // - Second require does not cause require side effects.
    // - Require non-existing file raises and returns `nil`.
    #[test]
    fn require() {
        struct File;

        impl MrbFile for File {
            fn require(interp: Mrb) -> Result<(), MrbError> {
                interp.eval("@i = 255")?;
                Ok(())
            }
        }

        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_file_for_type::<_, File>("file.rb")
            .expect("def file");
        let result = interp.eval("require 'file'").expect("eval");
        let require_result = unsafe { bool::try_convert(&interp, result) };
        assert_eq!(require_result, Ok(true));
        let result = interp.eval("@i").expect("eval");
        let i_result = unsafe { i64::try_convert(&interp, result) };
        assert_eq!(i_result, Ok(255));
        let result = interp.eval("@i = 1000; require 'file'").expect("eval");
        let second_require_result = unsafe { bool::try_convert(&interp, result) };
        assert_eq!(second_require_result, Ok(false));
        let result = interp.eval("@i").expect("eval");
        let second_i_result = unsafe { i64::try_convert(&interp, result) };
        assert_eq!(second_i_result, Ok(1000));
        let result = interp.eval("require 'non-existent-source'").map(|_| ());
        let expected = r#"
(eval):1: cannot load such file -- non-existent-source (LoadError)
(eval):1
            "#;
        assert_eq!(result, Err(MrbError::Exec(expected.trim().to_owned())));
    }

    #[test]
    fn require_absolute_path() {
        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_rb_source_file("/foo/bar/source.rb", "# a source file")
            .expect("def file");
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(unsafe { bool::try_convert(&interp, result).expect("convert") });
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(!unsafe { bool::try_convert(&interp, result).expect("convert") });
    }

    #[test]
    fn require_relative_with_dotted_path() {
        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_rb_source_file("/foo/bar/source.rb", "require_relative '../bar.rb'")
            .expect("def file");
        interp
            .def_rb_source_file("/foo/bar.rb", "# a source file")
            .expect("def file");
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(unsafe { bool::try_convert(&interp, result).expect("convert") });
    }

    #[test]
    fn require_directory() {
        let interp = crate::interpreter().expect("mrb init");
        let result = interp.eval("require '/src'").map(|_| ());
        let expected = r#"
(eval):1: cannot load such file -- /src (LoadError)
(eval):1
        "#;
        assert_eq!(result, Err(MrbError::Exec(expected.trim().to_owned())));
    }

    #[test]
    fn require_path_defined_as_source_then_mrbfile() {
        struct Foo;
        impl MrbFile for Foo {
            fn require(interp: Mrb) -> Result<(), MrbError> {
                interp.eval("module Foo; RUST = 7; end")?;
                Ok(())
            }
        }
        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_rb_source_file("foo.rb", "module Foo; RUBY = 3; end")
            .expect("def");
        interp.def_file_for_type::<_, Foo>("foo.rb").expect("def");
        let result = interp.eval("require 'foo'").expect("eval");
        let result = unsafe { bool::try_convert(&interp, result).expect("convert") };
        assert!(result, "successfully required foo.rb");
        let result = interp.eval("Foo::RUBY + Foo::RUST").expect("eval");
        let result = unsafe { i64::try_convert(&interp, result).expect("convert") };
        assert_eq!(
            result, 10,
            "defined Ruby and Rust sources from single require"
        );
    }

    #[test]
    fn require_path_defined_as_mrbfile_then_source() {
        struct Foo;
        impl MrbFile for Foo {
            fn require(interp: Mrb) -> Result<(), MrbError> {
                interp.eval("module Foo; RUST = 7; end")?;
                Ok(())
            }
        }
        let interp = crate::interpreter().expect("mrb init");
        interp.def_file_for_type::<_, Foo>("foo.rb").expect("def");
        interp
            .def_rb_source_file("foo.rb", "module Foo; RUBY = 3; end")
            .expect("def");
        let result = interp.eval("require 'foo'").expect("eval");
        let result = unsafe { bool::try_convert(&interp, result).expect("convert") };
        assert!(result, "successfully required foo.rb");
        let result = interp.eval("Foo::RUBY + Foo::RUST").expect("eval");
        let result = unsafe { i64::try_convert(&interp, result).expect("convert") };
        assert_eq!(
            result, 10,
            "defined Ruby and Rust sources from single require"
        );
    }

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn kernel_throw_catch() {
        // https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-catch
        let interp = crate::interpreter().expect("mrb init");
        let result = interp
            .eval("catch(1) { 123 }")
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 123);
        let result = interp
            .eval("catch(1) { throw(1, 456) }")
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 456);
        let result = interp
            .eval("catch(1) { throw(1) }")
            .unwrap()
            .try_into::<Option<i64>>()
            .unwrap();
        assert_eq!(result, None);
        let result = interp
            .eval("catch(1) {|x| x + 2 }")
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 3);

        let result = interp
            .eval(
                r#"
catch do |obj_A|
  catch do |obj_B|
    throw(obj_B, 123)
    # puts "This puts is not reached"
  end

  # puts "This puts is displayed"
  456
end
            "#,
            )
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 456);
        let result = interp
            .eval(
                r#"
catch do |obj_A|
  catch do |obj_B|
    throw(obj_A, 123)
    # puts "This puts is still not reached"
  end

  # puts "Now this puts is also not reached"
  456
end
            "#,
            )
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 123);
    }
}
