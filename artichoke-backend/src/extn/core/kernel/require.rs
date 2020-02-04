//! [`Kernel#require`](https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require)

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::extn::prelude::*;
use crate::ffi;
use crate::fs::RUBY_LOAD_PATH;
use crate::state::parser::Context;
use crate::Parser;

const RUBY_EXTENSION: &str = "rb";

pub fn load(interp: &mut Artichoke, filename: Value) -> Result<Value, Exception> {
    let ruby_type = filename.pretty_name();
    let filename = if let Ok(filename) = filename.clone().try_into::<&[u8]>() {
        filename
    } else if let Ok(filename) = filename.funcall::<&[u8]>("to_str", &[], None) {
        filename
    } else {
        return Err(Exception::from(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", ruby_type),
        )));
    };
    if memchr::memchr(b'\0', filename).is_some() {
        return Err(Exception::from(ArgumentError::new(
            interp,
            "path name contains null byte",
        )));
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let path = if Path::new(&file).is_relative() {
        let base = ffi::bytes_to_os_str(RUBY_LOAD_PATH.as_bytes())?;
        Path::new(&base).join(&file)
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
    let metadata = {
        let api = interp.0.borrow();
        api.vfs.metadata(path.as_path()).unwrap_or_default()
    };
    // Require Rust File first because an File may define classes
    // and module with `LoadSources` and Ruby files can require
    // arbitrary other files, including some child sources that may
    // depend on these module definitions.
    let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.into_owned())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);

    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    if let Some(require) = metadata.require {
        // dynamic, Rust-backed `File` require
        if require(interp).is_err() {
            let _ = interp.pop_context();
            return Err(Exception::from(load_error(interp, filename)?));
        }
    }
    let contents = {
        let api = interp.0.borrow();
        api.vfs.read_file(path.as_path())
    };
    if let Ok(contents) = contents {
        let _ = interp.eval(contents.as_slice())?;
    }
    let _ = interp.pop_context();
    let mut logged_filename = String::new();
    string::escape_unicode(&mut logged_filename, filename)?;
    trace!(r#"Successful load of "{}" at {:?}"#, logged_filename, path,);
    Ok(interp.convert(true))
}

pub fn require(
    interp: &mut Artichoke,
    filename: Value,
    base: Option<&Path>,
) -> Result<Value, Exception> {
    let ruby_type = filename.pretty_name();
    let filename = if let Ok(filename) = filename.clone().try_into::<&[u8]>() {
        filename
    } else if let Ok(filename) = filename.funcall::<&[u8]>("to_str", &[], None) {
        filename
    } else {
        return Err(Exception::from(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", ruby_type),
        )));
    };
    if memchr::memchr(b'\0', filename).is_some() {
        return Err(Exception::from(ArgumentError::new(
            interp,
            "path name contains null byte",
        )));
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let path = Path::new(&file);

    if path.is_relative() && path.extension() != Some(OsStr::new(RUBY_EXTENSION)) {
        let mut with_rb_ext = Vec::with_capacity(filename.len() + 3);
        with_rb_ext.extend(filename.iter());
        with_rb_ext.extend(b".rb".iter());
        let rb_ext = ffi::bytes_to_os_str(with_rb_ext.as_slice())?;
        let path = if let Some(base) = base {
            base.join(rb_ext)
        } else {
            let base = ffi::bytes_to_os_str(RUBY_LOAD_PATH.as_bytes())?;
            Path::new(&base).join(rb_ext)
        };
        let is_file = {
            let api = interp.0.borrow();
            api.vfs.is_file(path.as_path())
        };
        if is_file {
            let metadata = {
                let api = interp.0.borrow();
                api.vfs.metadata(path.as_path()).unwrap_or_default()
            };
            // If a file is already required, short circuit.
            if metadata.is_already_required() {
                return Ok(interp.convert(false));
            }
            // Require Rust File first because an File may define classes
            // and module with `LoadSources` and Ruby files can require
            // arbitrary other files, including some child sources that may
            // depend on these module definitions.
            let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.into_owned())
                .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
            interp.push_context(context);
            // Require Rust File first because an File may define classes and
            // module with `LoadSources` and Ruby files can require arbitrary
            // other files, including some child sources that may depend on these
            // module definitions.
            if let Some(require) = metadata.require {
                // dynamic, Rust-backed `File` require
                if require(interp).is_err() {
                    let _ = interp.pop_context();
                    return Err(Exception::from(load_error(interp, filename)?));
                }
            }
            let contents = {
                let api = interp.0.borrow();
                api.vfs.read_file(path.as_path())
            };
            if let Ok(contents) = contents {
                let _ = interp.eval(contents.as_slice())?;
            }
            let _ = interp.pop_context();
            let metadata = metadata.mark_required();
            let borrow = interp.0.borrow();
            borrow
                .vfs
                .set_metadata(path.as_path(), metadata)
                .map_err(|_| {
                    Fatal::new(
                        interp,
                        "Unable to set require metadata in the Artichoke virtual filesystem",
                    )
                })?;
            let mut logged_filename = String::new();
            string::escape_unicode(&mut logged_filename, filename)?;
            trace!(
                r#"Successful require of "{}" at {:?}"#,
                logged_filename,
                path,
            );
            return Ok(interp.convert(true));
        } else {
            let path = if let Some(base) = base {
                base.join(&file)
            } else {
                let base = ffi::bytes_to_os_str(RUBY_LOAD_PATH.as_bytes())?;
                Path::new(&base).join(&file)
            };
            let is_file = {
                let api = interp.0.borrow();
                api.vfs.is_file(path.as_path())
            };
            if is_file {
                let metadata = {
                    let api = interp.0.borrow();
                    api.vfs.metadata(path.as_path()).unwrap_or_default()
                };
                // If a file is already required, short circuit.
                if metadata.is_already_required() {
                    return Ok(interp.convert(false));
                }
                // Require Rust File first because an File may define classes
                // and module with `LoadSources` and Ruby files can require
                // arbitrary other files, including some child sources that may
                // depend on these module definitions.
                let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.into_owned())
                    .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
                interp.push_context(context);
                // Require Rust File first because an File may define classes and
                // module with `LoadSources` and Ruby files can require arbitrary
                // other files, including some child sources that may depend on these
                // module definitions.
                if let Some(require) = metadata.require {
                    // dynamic, Rust-backed `File` require
                    if require(interp).is_err() {
                        let _ = interp.pop_context();
                        return Err(Exception::from(load_error(interp, filename)?));
                    }
                }
                let contents = {
                    let api = interp.0.borrow();
                    api.vfs.read_file(path.as_path())
                };
                if let Ok(contents) = contents {
                    let _ = interp.eval(contents.as_slice())?;
                }
                let _ = interp.pop_context();
                let metadata = metadata.mark_required();
                let borrow = interp.0.borrow();
                borrow
                    .vfs
                    .set_metadata(path.as_path(), metadata)
                    .map_err(|_| {
                        Fatal::new(
                            interp,
                            "Unable to set require metadata in the Artichoke virtual filesystem",
                        )
                    })?;
                let mut logged_filename = String::new();
                string::escape_unicode(&mut logged_filename, filename)?;
                trace!(
                    r#"Successful require of "{}" at {:?}"#,
                    logged_filename,
                    path,
                );
                return Ok(interp.convert(true));
            }
        }
    }
    let path = if let Some(base) = base {
        base.join(&file)
    } else {
        let base = ffi::bytes_to_os_str(RUBY_LOAD_PATH.as_bytes())?;
        Path::new(&base).join(&file)
    };
    let is_file = {
        let api = interp.0.borrow();
        api.vfs.is_file(path.as_path())
    };
    if !is_file {
        return Err(Exception::from(load_error(interp, filename)?));
    }
    let metadata = {
        let api = interp.0.borrow();
        api.vfs.metadata(path.as_path()).unwrap_or_default()
    };
    // If a file is already required, short circuit.
    if metadata.is_already_required() {
        return Ok(interp.convert(false));
    }
    // Require Rust File first because an File may define classes
    // and module with `LoadSources` and Ruby files can require
    // arbitrary other files, including some child sources that may
    // depend on these module definitions.
    let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.into_owned())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);
    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    if let Some(require) = metadata.require {
        // dynamic, Rust-backed `File` require
        if require(interp).is_err() {
            let _ = interp.pop_context();
            return Err(Exception::from(load_error(interp, filename)?));
        }
    }
    let contents = {
        let api = interp.0.borrow();
        api.vfs.read_file(path.as_path())
    };
    if let Ok(contents) = contents {
        let _ = interp.eval(contents.as_slice())?;
    }
    let _ = interp.pop_context();
    {
        let metadata = metadata.mark_required();
        let borrow = interp.0.borrow();
        borrow
            .vfs
            .set_metadata(path.as_path(), metadata)
            .map_err(|_| {
                Fatal::new(
                    interp,
                    "Unable to set require metadata in the Artichoke virtual filesystem",
                )
            })?;
    }
    let mut logged_filename = String::new();
    string::escape_unicode(&mut logged_filename, filename)?;
    trace!(
        r#"Successful require of "{}" at {:?}"#,
        logged_filename,
        path,
    );
    Ok(interp.convert(true))
}

#[allow(clippy::module_name_repetitions)]
pub fn require_relative(interp: &mut Artichoke, file: Value) -> Result<Value, Exception> {
    let current = {
        let borrow = interp.0.borrow();
        // TODO: GH-468 - Use `Parser::peek_context`.
        let context = borrow
            .parser
            .peek_context()
            .ok_or_else(|| Fatal::new(interp, "relative require with no context stack"))?;
        ffi::bytes_to_os_str(context.filename())?.into_owned()
    };
    let base = if let Some(base) = Path::new(current.as_os_str()).parent() {
        base
    } else {
        Path::new("/")
    };
    require(interp, file, Some(base))
}

fn load_error(interp: &Artichoke, filename: &[u8]) -> Result<LoadError, Exception> {
    let mut message = String::from("cannot load such file -- ");
    string::escape_unicode(&mut message, filename)?;
    Ok(LoadError::new(interp, message))
}
