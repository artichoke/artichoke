use log::trace;
use mruby_vfs::FileSystem;
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use crate::convert::TryFromMrb;
use crate::def::{ClassLike, Define};
use crate::eval::{EvalContext, MrbEval};
use crate::extn::core::error::LoadError;
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
