//! [`Kernel#require`](https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require)

use std::mem;
use std::rc::Rc;

use crate::convert::TryFromMrb;
use crate::eval::{EvalContext, MrbEval};
use crate::extn::core::error::{LoadError, RubyException};
use crate::fs::RequireFunc;
use crate::sys;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    AlreadyRequired,
    CannotLoad(String),
    Fatal,
    NoImplicitConversionToString,
}

pub struct Require {
    pub file: String,
    pub rust: Option<RequireFunc>,
    pub ruby: Option<Vec<u8>>,
}

impl Require {
    pub unsafe fn require(self, interp: Mrb) -> sys::mrb_value {
        let context = EvalContext::new(self.file.as_str());
        // Require Rust MrbFile first because an MrbFile may define classes and
        // module with `MrbLoadSources` and Ruby files can require arbitrary
        // other files, including some child sources that may depend on these
        // module definitions.
        if let Some(require) = self.rust {
            // dynamic, Rust-backed `MrbFile` require
            interp.push_context(context.clone());
            let response = require(Rc::clone(&interp));
            interp.pop_context();
            if response.is_err() {
                let file = self.file.clone();
                // LoadError::raise will unwind the stack with longjmp. Drop
                // everything we can to avoid a leak.
                drop(context);
                drop(self);
                return LoadError::raisef(interp, "cannot load such file -- %s", vec![file]);
            }
        }
        if let Some(contents) = self.ruby {
            // We need to be sure we don't leak anything by unwinding past
            // this point. This likely requires a significant refactor to
            // require_impl.
            interp.unchecked_eval_with_context(contents, context);
        }
        sys::mrb_sys_true_value()
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub file: String,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o\0";

    pub unsafe fn extract(interp: &Mrb) -> Result<Self, Error> {
        let mut string = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            string.as_mut_ptr(),
        );
        let string = string.assume_init();
        if let Ok(file) = String::try_from_mrb(interp, Value::new(interp, string)) {
            Ok(Self { file })
        } else {
            Err(Error::NoImplicitConversionToString)
        }
    }
}

pub mod method {
    use log::trace;
    use path_abs::PathAbs;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    use crate::eval::MrbEval;
    use crate::fs::RUBY_LOAD_PATH;
    use crate::Mrb;

    use super::{Args, Error, Require};

    pub fn require(interp: &Mrb, args: Args) -> Result<Require, Error> {
        require_impl(interp, args, RUBY_LOAD_PATH)
    }

    pub fn require_relative(interp: &Mrb, args: Args) -> Result<Require, Error> {
        let context = interp
            .peek_context()
            .ok_or_else(|| Error::CannotLoad(args.file.clone()))?;
        let base = PathBuf::from(context.filename)
            .parent()
            .and_then(Path::to_str)
            .map(str::to_owned)
            .ok_or_else(|| Error::CannotLoad(args.file.clone()))?;
        require_impl(interp, args, base.as_str())
    }

    fn require_impl(interp: &Mrb, args: Args, base: &str) -> Result<Require, Error> {
        let interp = Rc::clone(interp);
        // Track whether any iterations of the loop successfully required some
        // Ruby sources.
        let mut path = PathBuf::from(args.file.as_str());
        let files = if path.is_relative() {
            path = PathBuf::from(base);
            let mut files = Vec::with_capacity(2);
            if !args.file.ends_with(".rb") {
                files.push(path.join(format!("{}.rb", args.file.as_str())))
            }
            files.push(path.join(args.file.as_str()));
            files
        } else {
            vec![path.join(args.file.as_str())]
        };
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
                api.vfs.metadata(path.as_path()).unwrap_or_default()
            };
            // If a file is already required, short circuit.
            if metadata.is_already_required() {
                return Err(Error::AlreadyRequired);
            }
            let file = if let Some(filename) = path.as_path().to_str() {
                filename
            } else {
                "(require)"
            };
            // Require Rust MrbFile first because an MrbFile may define classes
            // and module with `MrbLoadSources` and Ruby files can require
            // arbitrary other files, including some child sources that may
            // depend on these module definitions.
            let contents = {
                let api = interp.borrow();
                api.vfs.read_file(path.as_path())
            };
            let require = Require {
                file: file.to_owned(),
                rust: metadata.require,
                ruby: contents.ok(),
            };
            let metadata = metadata.mark_required();
            let borrow = interp.borrow();
            borrow
                .vfs
                .set_metadata(path.as_path(), metadata)
                .map_err(|_| Error::Fatal)?;
            trace!(
                r#"Successful require of "{}" at {:?} on {:?}"#,
                args.file,
                path,
                borrow
            );
            return Ok(require);
        }
        Err(Error::CannotLoad(args.file))
    }
}
