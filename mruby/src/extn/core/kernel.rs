use log::trace;
use mruby_vfs::FileSystem;
use path_abs::PathAbs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::convert::FromMrb;
use crate::def::{ClassLike, Define};
use crate::eval::{EvalContext, MrbEval};
use crate::extn::core::error::{LoadError, RubyException};
use crate::interpreter::{Mrb, MrbApi, RUBY_LOAD_PATH};
use crate::state::VfsMetadata;
use crate::sys;
use crate::value::types::Ruby;
use crate::value::{Value, ValueLike};
use crate::MrbError;

mod args;

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
        let interp = interpreter_or_raise!(mrb);
        let stderr = sys::mrb_gv_get(mrb, interp.borrow_mut().sym_intern("$stderr"));
        if !sys::mrb_sys_value_is_nil(stderr) {
            let args = unwrap_or_raise!(interp, args::Rest::extract(&interp), interp.nil().inner());
            let stderr = Value::new(&interp, stderr);
            unwrap_value_or_raise!(interp, stderr.funcall::<Value, _, _>("print", args.rest));
        }
        interp.nil().inner()
    }
}

pub struct Kernel;

impl Kernel {
    unsafe fn require_impl(interp: &Mrb, filename: &str, base: &str) -> sys::mrb_value {
        // Track whether any iterations of the loop successfully required some
        // Ruby sources.
        let mut success = false;
        let mut path = PathBuf::from(filename);
        if path.is_relative() {
            path = PathBuf::from(base);
        }
        let files = vec![path.join(format!("{}.rb", filename)), path.join(filename)];
        for path in files {
            // canonicalize path (remove '.' and '..' components).
            let path = match PathAbs::new(path) {
                Ok(path) => path,
                Err(_) => continue,
            };
            let is_file = {
                let api = interp.borrow();
                api.vfs.is_file(path.as_path())
            };
            if !is_file {
                // If no paths are files in the VFS, then the require does
                // nothing.
                continue;
            }
            let metadata = {
                let api = interp.borrow();
                api.vfs
                    .metadata(path.as_path())
                    .unwrap_or_else(VfsMetadata::new)
            };
            // If a file is already required, short circuit.
            if metadata.is_already_required() {
                return interp.bool(false).inner();
            }
            let context = if let Some(filename) = path.as_path().to_str() {
                EvalContext::new(filename)
            } else {
                EvalContext::new("(require)")
            };
            // Require Rust MrbFile first because an MrbFile may define classes
            // and module with `MrbLoadSources` and Ruby files can require
            // arbitrary other files, including some child sources that may
            // depend on these module definitions. This behavior is enforced
            // with a test in crate mruby-gems. See mruby-gems/src/lib.rs.
            if let Some(require) = metadata.require {
                // dynamic, Rust-backed `MrbFile` require
                interp.push_context(context.clone());
                unwrap_or_raise!(interp, require(Rc::clone(interp)), interp.nil().inner());
                interp.pop_context();
            }
            let contents = {
                let api = interp.borrow();
                api.vfs.read_file(path.as_path())
            };
            if let Ok(contents) = contents {
                unwrap_value_or_raise!(
                    Rc::clone(interp),
                    interp.eval_with_context(contents, context)
                );
            } else {
                // this branch should be unreachable because the `Mrb`
                // interpreter is not `Send` so it can only be owned and
                // accessed by one thread.
                return LoadError::raise(&interp, filename);
            }
            let metadata = metadata.mark_required();
            let api = interp.borrow();
            unwrap_or_raise!(
                interp,
                api.vfs.set_metadata(path.as_path(), metadata),
                interp.nil().inner()
            );
            success = true;
            trace!(
                r#"Successful require of "{}" at {:?} on {:?}"#,
                filename,
                path,
                api
            );
        }
        if success {
            interp.bool(success).inner()
        } else {
            LoadError::raise(&interp, filename)
        }
    }

    unsafe extern "C" fn require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::Require::extract(&interp),
            interp.nil().inner()
        );
        Self::require_impl(&interp, args.filename.as_str(), RUBY_LOAD_PATH)
    }

    unsafe extern "C" fn require_relative(
        mrb: *mut sys::mrb_state,
        _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::Require::extract(&interp),
            interp.nil().inner()
        );
        let base = interp
            .peek_context()
            .and_then(|context| {
                PathBuf::from(context.filename)
                    .parent()
                    .and_then(Path::to_str)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| RUBY_LOAD_PATH.to_owned());
        Self::require_impl(&interp, args.filename.as_str(), base.as_str())
    }

    unsafe extern "C" fn print(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, args::Rest::extract(&interp), interp.nil().inner());

        for value in args.rest {
            print!("{}", value.to_s());
        }
        interp.nil().inner()
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

        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, args::Rest::extract(&interp), interp.nil().inner());

        if args.rest.is_empty() {
            println!();
        }
        for value in args.rest {
            do_puts(value);
        }
        interp.nil().inner()
    }

    unsafe extern "C" fn warn(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, args::Rest::extract(&interp), interp.nil().inner());

        for value in args.rest {
            let mut string = value.to_s();
            if !string.ends_with('\n') {
                string = format!("{}\n", string);
            }
            Warning::warn(mrb, Value::from_mrb(&interp, string).inner());
        }
        interp.nil().inner()
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::TryFromMrb;
    use crate::eval::MrbEval;
    use crate::file::MrbFile;
    use crate::interpreter::{Interpreter, Mrb};
    use crate::load::MrbLoadSources;
    use crate::MrbError;

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

        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_file_for_type::<_, File>("file.rb")
            .expect("def file");
        let result = interp.eval("require 'file'").expect("eval");
        let require_result = unsafe { bool::try_from_mrb(&interp, result) };
        assert_eq!(require_result, Ok(true));
        let result = interp.eval("@i").expect("eval");
        let i_result = unsafe { i64::try_from_mrb(&interp, result) };
        assert_eq!(i_result, Ok(255));
        let result = interp.eval("@i = 1000; require 'file'").expect("eval");
        let second_require_result = unsafe { bool::try_from_mrb(&interp, result) };
        assert_eq!(second_require_result, Ok(false));
        let result = interp.eval("@i").expect("eval");
        let second_i_result = unsafe { i64::try_from_mrb(&interp, result) };
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
        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_rb_source_file("/foo/bar/source.rb", "# a source file")
            .expect("def file");
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(unsafe { bool::try_from_mrb(&interp, result).expect("convert") });
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(!unsafe { bool::try_from_mrb(&interp, result).expect("convert") });
    }

    #[test]
    fn require_relative_with_dotted_path() {
        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_rb_source_file("/foo/bar/source.rb", "require_relative '../bar.rb'")
            .expect("def file");
        interp
            .def_rb_source_file("/foo/bar.rb", "# a source file")
            .expect("def file");
        let result = interp.eval("require '/foo/bar/source.rb'").expect("value");
        assert!(unsafe { bool::try_from_mrb(&interp, result).expect("convert") });
    }

    #[test]
    fn require_directory() {
        let interp = Interpreter::create().expect("mrb init");
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
        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_rb_source_file("foo.rb", "module Foo; RUBY = 3; end")
            .expect("def");
        interp.def_file_for_type::<_, Foo>("foo.rb").expect("def");
        let result = interp.eval("require 'foo'").expect("eval");
        let result = unsafe { bool::try_from_mrb(&interp, result).expect("convert") };
        assert!(result, "successfully required foo.rb");
        let result = interp.eval("Foo::RUBY + Foo::RUST").expect("eval");
        let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
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
        let interp = Interpreter::create().expect("mrb init");
        interp.def_file_for_type::<_, Foo>("foo.rb").expect("def");
        interp
            .def_rb_source_file("foo.rb", "module Foo; RUBY = 3; end")
            .expect("def");
        let result = interp.eval("require 'foo'").expect("eval");
        let result = unsafe { bool::try_from_mrb(&interp, result).expect("convert") };
        assert!(result, "successfully required foo.rb");
        let result = interp.eval("Foo::RUBY + Foo::RUST").expect("eval");
        let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(
            result, 10,
            "defined Ruby and Rust sources from single require"
        );
    }

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn kernel_throw_catch() {
        // https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-catch
        let interp = Interpreter::create().expect("mrb init");
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
