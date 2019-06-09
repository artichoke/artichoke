use log::trace;
use mruby_vfs::FileSystem;
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use crate::convert::TryFromMrb;
use crate::def::{ClassLike, Define};
use crate::eval::{EvalContext, MrbEval};
use crate::extn::core::error::{LoadError, RubyException};
use crate::interpreter::{Mrb, MrbApi, RUBY_LOAD_PATH};
use crate::state::VfsMetadata;
use crate::sys;
use crate::value::Value;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    let kernel = interp.borrow_mut().def_module::<Kernel>("Kernel", None);
    kernel
        .borrow_mut()
        .add_self_method("require", Kernel::require, sys::mrb_args_rest());
    kernel.borrow().define(interp).map_err(|_| MrbError::New)?;
    trace!("Patched Kernel#require onto interpreter");
    Ok(())
}

pub struct Kernel;

impl Kernel {
    unsafe extern "C" fn require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        struct Args {
            filename: String,
        }

        impl Args {
            unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
                let inner = mem::uninitialized::<sys::mrb_value>();
                let mut argspec = vec![];
                argspec
                    .write_all(sys::specifiers::OBJECT.as_bytes())
                    .map_err(|_| MrbError::ArgSpec)?;
                argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
                sys::mrb_get_args(interp.borrow().mrb, argspec.as_ptr() as *const i8, &inner);
                let filename = Value::new(interp, inner);
                let filename =
                    String::try_from_mrb(&interp, filename).map_err(MrbError::ConvertToRust)?;
                Ok(Self { filename })
            }
        }

        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, Args::extract(&interp), interp.nil().inner());

        // Track whether any iterations of the loop successfully required some
        // Ruby sources.
        let mut success = false;
        let mut path = PathBuf::from(&args.filename);
        if path.is_relative() {
            path = PathBuf::from(RUBY_LOAD_PATH);
        }
        let files = vec![
            path.join(&args.filename),
            path.join(format!("{}.rb", args.filename)),
        ];
        for path in files {
            let is_file = {
                let api = interp.borrow();
                api.vfs.is_file(&path)
            };
            if !is_file {
                // If no paths are files in the VFS, then the require does
                // nothing.
                continue;
            }
            let metadata = {
                let api = interp.borrow();
                api.vfs.metadata(&path).unwrap_or_else(VfsMetadata::new)
            };
            // If a file is already required, short circuit.
            if metadata.is_already_required() {
                return interp.bool(false).inner();
            }
            let context = if let Some(filename) = &path.to_str() {
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
                unwrap_or_raise!(interp, require(Rc::clone(&interp)), interp.nil().inner());
                interp.pop_context();
            }
            let contents = {
                let api = interp.borrow();
                api.vfs.read_file(&path)
            };
            if let Ok(contents) = contents {
                unwrap_value_or_raise!(interp, interp.eval_with_context(contents, context));
            } else {
                // this branch should be unreachable because the `Mrb`
                // interpreter is not `Send` so it can only be owned and
                // accessed by one thread.
                return LoadError::raise(&interp, &args.filename);
            }
            let metadata = metadata.mark_required();
            let api = interp.borrow();
            unwrap_or_raise!(
                interp,
                api.vfs.set_metadata(&path, metadata),
                interp.nil().inner()
            );
            success = true;
            trace!(
                r#"Successful require of "{}" at {:?} on {:?}"#,
                args.filename,
                path,
                api
            );
        }
        if success {
            interp.bool(success).inner()
        } else {
            LoadError::raise(&interp, &args.filename)
        }
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
}
