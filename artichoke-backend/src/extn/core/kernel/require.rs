//! [`Kernel#require`](https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require)

use bstr::ByteSlice;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::extn::prelude::*;
use crate::ffi;
use crate::fs::RUBY_LOAD_PATH;
use crate::state::parser::Context;
use crate::Parser;

const RUBY_EXTENSION: &str = "rb";

pub fn load(interp: &mut Artichoke, filename: Value) -> Result<Value, Exception> {
    let filename = filename.implicitly_convert_to_string(interp)?;
    if filename.find_byte(b'\0').is_some() {
        return Err(Exception::from(ArgumentError::new(
            interp,
            "path name contains null byte",
        )));
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let path = if Path::new(&file).is_relative() {
        Path::new(RUBY_LOAD_PATH).join(&file)
    } else {
        PathBuf::from(&file)
    };
    let is_file = {
        let api = interp.0.borrow();
        api.vfs.is_file(path.as_path())
    };
    if !is_file {
        return Err(Exception::from(load_error(interp, filename)?));
    }
    let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);

    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    let hook = interp.0.borrow().vfs.get_extension(path.as_path());
    if let Some(hook) = hook {
        // dynamic, Rust-backed `File` require
        if let Err(err) = hook(interp) {
            let _ = interp.pop_context();
            return Err(err);
        }
    }
    let contents = {
        let api = interp.0.borrow();
        api.vfs.read_file(path.as_path()).map(<[_]>::to_vec)
    };
    if let Ok(contents) = contents {
        let _ = interp.eval(contents.as_slice())?;
    }
    let _ = interp.pop_context();
    let mut logged_filename = String::new();
    string::format_unicode_debug_into(&mut logged_filename, filename)?;
    trace!(r#"Successful load of "{}" at {:?}"#, logged_filename, path,);
    Ok(interp.convert(true))
}

pub fn require(
    interp: &mut Artichoke,
    filename: Value,
    base: Option<RelativePath>,
) -> Result<Value, Exception> {
    let filename = filename.implicitly_convert_to_string(interp)?;
    if filename.find_byte(b'\0').is_some() {
        return Err(Exception::from(ArgumentError::new(
            interp,
            "path name contains null byte",
        )));
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let path = Path::new(&file);

    if path.is_relative() && path.extension() != Some(OsStr::new(RUBY_EXTENSION)) {
        let mut with_rb_ext = Vec::with_capacity(filename.len() + 3);
        with_rb_ext.extend_from_slice(filename);
        with_rb_ext.extend_from_slice(b".rb");
        let rb_ext = ffi::bytes_to_os_str(with_rb_ext.as_slice())?;
        let path = if let Some(ref base) = base {
            base.join(rb_ext)
        } else {
            Path::new(RUBY_LOAD_PATH).join(rb_ext)
        };
        let is_file = {
            let api = interp.0.borrow();
            api.vfs.is_file(path.as_path())
        };
        if is_file {
            // If a file is already required, short circuit.
            if interp.0.borrow().vfs.is_required(path.as_path()) {
                return Ok(interp.convert(false));
            }
            let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
                .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
            interp.push_context(context);
            // Require Rust File first because an File may define classes and
            // module with `LoadSources` and Ruby files can require arbitrary
            // other files, including some child sources that may depend on these
            // module definitions.
            let hook = interp.0.borrow().vfs.get_extension(path.as_path());
            if let Some(hook) = hook {
                // dynamic, Rust-backed `File` require
                if let Err(err) = hook(interp) {
                    let _ = interp.pop_context();
                    return Err(err);
                }
            }
            let contents = {
                let api = interp.0.borrow();
                api.vfs.read_file(path.as_path()).map(<[_]>::to_vec)
            };
            if let Ok(contents) = contents {
                let _ = interp.eval(contents.as_slice())?;
            }
            let _ = interp.pop_context();
            if interp
                .0
                .borrow_mut()
                .vfs
                .mark_required(path.as_path())
                .is_err()
            {
                return Err(Exception::from(load_error(interp, b"internal error")?));
            }
            let mut logged_filename = String::new();
            string::format_unicode_debug_into(&mut logged_filename, filename)?;
            trace!(
                r#"Successful require of "{}" at {:?}"#,
                logged_filename,
                path,
            );
            return Ok(interp.convert(true));
        } else {
            let path = if let Some(ref base) = base {
                base.join(&file)
            } else {
                Path::new(RUBY_LOAD_PATH).join(&file)
            };
            let is_file = {
                let api = interp.0.borrow();
                api.vfs.is_file(path.as_path())
            };
            if is_file {
                // If a file is already required, short circuit.
                if interp.0.borrow().vfs.is_required(path.as_path()) {
                    return Ok(interp.convert(false));
                }
                let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
                    .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
                interp.push_context(context);
                // Require Rust File first because an File may define classes and
                // module with `LoadSources` and Ruby files can require arbitrary
                // other files, including some child sources that may depend on these
                // module definitions.
                let hook = interp.0.borrow().vfs.get_extension(path.as_path());
                if let Some(hook) = hook {
                    // dynamic, Rust-backed `File` require
                    if let Err(err) = hook(interp) {
                        let _ = interp.pop_context();
                        return Err(err);
                    }
                }
                let contents = {
                    let api = interp.0.borrow();
                    api.vfs.read_file(path.as_path()).map(<[_]>::to_vec)
                };
                if let Ok(contents) = contents {
                    let _ = interp.eval(contents.as_slice())?;
                }
                let _ = interp.pop_context();
                if interp
                    .0
                    .borrow_mut()
                    .vfs
                    .mark_required(path.as_path())
                    .is_err()
                {
                    return Err(Exception::from(load_error(interp, b"internal error")?));
                }
                let mut logged_filename = String::new();
                string::format_unicode_debug_into(&mut logged_filename, filename)?;
                trace!(
                    r#"Successful require of "{}" at {:?}"#,
                    logged_filename,
                    path,
                );
                return Ok(interp.convert(true));
            }
        }
    }
    let path = if let Some(ref base) = base {
        base.join(&file)
    } else {
        Path::new(RUBY_LOAD_PATH).join(&file)
    };
    if !interp.0.borrow().vfs.is_file(path.as_path()) {
        return Err(Exception::from(load_error(interp, filename)?));
    }
    // If a file is already required, short circuit.
    if interp.0.borrow().vfs.is_required(path.as_path()) {
        return Ok(interp.convert(false));
    }
    let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);
    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    let hook = interp.0.borrow().vfs.get_extension(path.as_path());
    if let Some(hook) = hook {
        // dynamic, Rust-backed `File` require
        if let Err(err) = hook(interp) {
            let _ = interp.pop_context();
            return Err(err);
        }
    }
    let contents = {
        let api = interp.0.borrow();
        api.vfs.read_file(path.as_path()).map(<[_]>::to_vec)
    };
    if let Ok(contents) = contents {
        let _ = interp.eval(contents.as_slice())?;
    }
    let _ = interp.pop_context();
    if interp
        .0
        .borrow_mut()
        .vfs
        .mark_required(path.as_path())
        .is_err()
    {
        return Err(Exception::from(load_error(interp, b"internal error")?));
    }
    let mut logged_filename = String::new();
    string::format_unicode_debug_into(&mut logged_filename, filename)?;
    trace!(
        r#"Successful require of "{}" at {:?}"#,
        logged_filename,
        path,
    );
    Ok(interp.convert(true))
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RelativePath(PathBuf);

impl RelativePath {
    pub fn new<T>(path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        Self(path.into())
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.0.join(path.as_ref())
    }

    pub fn try_from_interp(interp: &mut Artichoke) -> Result<Self, Exception> {
        let borrow = interp.0.borrow();
        // TODO(GH-468): Use `Parser::peek_context`.
        let context = borrow
            .parser
            .peek_context()
            .ok_or_else(|| Fatal::new(interp, "relative require with no context stack"))?;
        let path = ffi::bytes_to_os_str(context.filename())?;
        let path = Path::new(path);
        if let Some(base) = path.parent() {
            Ok(Self::new(base))
        } else {
            Ok(Self::new("/"))
        }
    }
}

fn load_error(interp: &Artichoke, filename: &[u8]) -> Result<LoadError, Exception> {
    let mut message = String::from("cannot load such file -- ");
    string::format_unicode_debug_into(&mut message, filename)?;
    Ok(LoadError::new(interp, message))
}
