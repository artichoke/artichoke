use log::trace;
use mruby_vfs::FileSystem;
use std::ffi::{CStr, CString};
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use crate::def::{ClassLike, Define};
use crate::eval::{EvalContext, MrbEval};
use crate::extn::core::error::LoadError;
use crate::interpreter::{Mrb, MrbApi, RUBY_LOAD_PATH};
use crate::state::VfsMetadata;
use crate::sys;
use crate::MrbError;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    let kernel = interp.borrow_mut().def_module::<Kernel>("Kernel", None);
    kernel
        .borrow_mut()
        .add_self_method("require", require, sys::mrb_args_rest());
    kernel.borrow().define(interp).map_err(|_| MrbError::New)?;
    trace!("Patched Kernel#require onto interpreter");
    Ok(())
}

pub struct Kernel;

extern "C" fn require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let interp = unsafe { interpreter_or_raise!(mrb) };
    // Extract required filename from arguments
    let name = unsafe {
        let name = mem::uninitialized::<*const std::os::raw::c_char>();
        let argspec = CString::new(sys::specifiers::CSTRING).expect("argspec");
        sys::mrb_get_args(mrb, argspec.as_ptr(), &name);
        match CStr::from_ptr(name).to_str() {
            Ok(name) => name.to_owned(),
            Err(err) => {
                let eclass = CString::new("ArgumentError");
                let message = CString::new(format!("{}", err));
                if let (Ok(eclass), Ok(message)) = (eclass, message) {
                    sys::mrb_sys_raise(interp.borrow().mrb, eclass.as_ptr(), message.as_ptr());
                }
                return interp.nil().inner();
            }
        }
    };

    // track whether any iterations of the loop successfully required a file
    let mut success = false;
    let mut path = PathBuf::from(&name);
    if path.is_relative() {
        path = PathBuf::from(RUBY_LOAD_PATH);
    }
    let files = vec![path.join(&name), path.join(format!("{}.rb", name))];
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
        // If a file is already required, short circuit
        if metadata.is_already_required() {
            return interp.bool(false).inner();
        }
        let context = if let Some(filename) = &path.to_str() {
            EvalContext::new(filename)
        } else {
            EvalContext::new("(require)")
        };
        // Require Rust MrbFile first because an MrbFile may define classes and
        // module with `MrbLoadSources` and Ruby files can require arbitrary
        // other files, including some child sources that may depend on these
        // module definitions. This behavior is enforced with a test in crate
        // mruby-gems. See mruby-gems/src/lib.rs.
        if let Some(require) = metadata.require {
            // dynamic, Rust-backed `MrbFile` require
            interp.push_context(context.clone());
            unsafe { unwrap_or_raise!(interp, require(Rc::clone(&interp)), interp.nil().inner()) };
            interp.pop_context();
        }
        let contents = {
            let api = interp.borrow();
            api.vfs.read_file(&path)
        };
        if let Ok(contents) = contents {
            unsafe {
                unwrap_value_or_raise!(interp, interp.eval_with_context(contents, context));
            }
        } else {
            // this branch should be unreachable because the `Mrb` interpreter
            // is not `Send` so it can only be owned and accessed by one thread.
            return LoadError::raise(&interp, &name);
        }
        let metadata = metadata.mark_required();
        unsafe {
            let api = interp.borrow();
            unwrap_or_raise!(
                interp,
                api.vfs.set_metadata(&path, metadata),
                interp.nil().inner()
            );
        }
        success = true;
        trace!(
            r#"Successful require of "{}" at {:?} on {:?}"#,
            name,
            path,
            interp.borrow()
        );
    }
    if success {
        interp.bool(success).inner()
    } else {
        LoadError::raise(&interp, &name)
    }
}
