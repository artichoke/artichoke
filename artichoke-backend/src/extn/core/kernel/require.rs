//! [`Kernel#require`](https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require)

use bstr::BStr;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::eval::Context;
use crate::extn::prelude::*;
use crate::fs::{self, RUBY_LOAD_PATH};

const RUBY_EXTENSION: &str = "rb";

pub fn load(interp: &mut Artichoke, filename: Value) -> Result<Value, Exception> {
    let ruby_type = filename.pretty_name(interp);
    let filename = if let Ok(filename) = filename.try_into::<&[u8]>(interp) {
        filename
    } else if let Ok(filename) = filename.funcall::<&[u8]>(interp, "to_str", &[], None) {
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
    let file: &Path = fs::bytes_to_osstr(interp, filename)?.as_ref();
    let path = if file.is_relative() {
        let base: &Path = fs::bytes_to_osstr(interp, RUBY_LOAD_PATH.as_bytes())?.as_ref();
        base.join(file)
    } else {
        file.to_path_buf()
    };
    let is_file = interp.vfs().is_file(path.as_path());
    if !is_file {
        let filestr = format!("{:?}", <&BStr>::from(filename));
        return Err(Exception::from(LoadError::new(
            interp,
            format!(
                "cannot load such file -- {:?}",
                &filestr[1..filestr.len() - 1]
            ),
        )));
    }
    let metadata = interp.vfs().metadata(path.as_path()).unwrap_or_default();
    // Require Rust File first because an File may define classes
    // and module with `LoadSources` and Ruby files can require
    // arbitrary other files, including some child sources that may
    // depend on these module definitions.
    let context = Context::new(fs::osstr_to_bytes(interp, path.as_os_str())?.to_vec())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);
    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    if let Some(require) = metadata.require {
        // dynamic, Rust-backed `File` require
        if require(interp).is_err() {
            interp.pop_context();
            let filestr = format!("{:?}", <&BStr>::from(filename));
            return Err(Exception::from(LoadError::new(
                interp,
                format!(
                    "cannot load such file -- {:?}",
                    &filestr[1..filestr.len() - 1]
                ),
            )));
        }
    }
    let contents = interp.vfs().read_file(path.as_path());
    if let Ok(contents) = contents {
        let _ = interp.eval(contents.as_slice())?;
    }
    interp.pop_context();
    trace!(
        r#"Successful load of "{:?}" at {:?}"#,
        <&BStr>::from(filename),
        path,
    );
    Ok(interp.convert(true))
}

pub fn require(
    interp: &mut Artichoke,
    filename: Value,
    base: Option<&Path>,
) -> Result<Value, Exception> {
    let ruby_type = filename.pretty_name(interp);
    let filename = if let Ok(filename) = filename.try_into::<&[u8]>(interp) {
        filename
    } else if let Ok(filename) = filename.funcall::<&[u8]>(interp, "to_str", &[], None) {
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
    let file: &Path = fs::bytes_to_osstr(interp, filename)?.as_ref();

    if file.is_relative() && file.extension() != Some(OsStr::new(RUBY_EXTENSION)) {
        let relative_base: &Path = if let Some(base) = base {
            base
        } else {
            fs::bytes_to_osstr(interp, RUBY_LOAD_PATH.as_bytes())?.as_ref()
        };
        let mut with_rb_ext = Vec::with_capacity(filename.len() + 3);
        with_rb_ext.extend(filename.iter());
        with_rb_ext.extend(b".rb".iter());
        let rb_ext = fs::bytes_to_osstr(interp, with_rb_ext.as_slice())?;
        let path = relative_base.join(rb_ext);
        let is_file = interp.vfs().is_file(path.as_path());
        if is_file {
            let metadata = interp.vfs().metadata(path.as_path()).unwrap_or_default();
            // If a file is already required, short circuit.
            if metadata.is_already_required() {
                return Ok(interp.convert(false));
            }
            // Require Rust File first because an File may define classes
            // and module with `LoadSources` and Ruby files can require
            // arbitrary other files, including some child sources that may
            // depend on these module definitions.
            let context = Context::new(fs::osstr_to_bytes(interp, path.as_os_str())?.to_vec())
                .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
            interp.push_context(context);
            // Require Rust File first because an File may define classes and
            // module with `LoadSources` and Ruby files can require arbitrary
            // other files, including some child sources that may depend on these
            // module definitions.
            if let Some(require) = metadata.require {
                // dynamic, Rust-backed `File` require
                if require(interp).is_err() {
                    interp.pop_context();
                    let filestr = format!("{:?}", <&BStr>::from(filename));
                    return Err(Exception::from(LoadError::new(
                        interp,
                        format!(
                            "cannot load such file -- {:?}",
                            &filestr[1..filestr.len() - 1]
                        ),
                    )));
                }
            }
            let contents = interp.vfs().read_file(path.as_path());
            if let Ok(contents) = contents {
                let _ = interp.eval(contents.as_slice())?;
            }
            interp.pop_context();
            let metadata = metadata.mark_required();
            interp
                .vfs_mut()
                .set_metadata(path.as_path(), metadata)
                .map_err(|_| {
                    Fatal::new(
                        interp,
                        "Unable to set require metadata in the Artichoke virtual filesystem",
                    )
                })?;
            trace!(
                r#"Successful require of {:?} at {:?}"#,
                <&BStr>::from(filename),
                path,
            );
            return Ok(interp.convert(true));
        } else {
            let path = relative_base.join(file);
            let is_file = interp.vfs().is_file(path.as_path());
            if is_file {
                let metadata = interp.vfs().metadata(path.as_path()).unwrap_or_default();
                // If a file is already required, short circuit.
                if metadata.is_already_required() {
                    return Ok(interp.convert(false));
                }
                // Require Rust File first because an File may define classes
                // and module with `LoadSources` and Ruby files can require
                // arbitrary other files, including some child sources that may
                // depend on these module definitions.
                let context = Context::new(fs::osstr_to_bytes(interp, path.as_os_str())?.to_vec())
                    .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
                interp.push_context(context);
                // Require Rust File first because an File may define classes and
                // module with `LoadSources` and Ruby files can require arbitrary
                // other files, including some child sources that may depend on these
                // module definitions.
                if let Some(require) = metadata.require {
                    // dynamic, Rust-backed `File` require
                    if require(interp).is_err() {
                        interp.pop_context();
                        let filestr = format!("{:?}", <&BStr>::from(filename));
                        return Err(Exception::from(LoadError::new(
                            interp,
                            format!(
                                "cannot load such file -- {:?}",
                                &filestr[1..filestr.len() - 1]
                            ),
                        )));
                    }
                }
                let contents = interp.vfs().read_file(path.as_path());
                if let Ok(contents) = contents {
                    let _ = interp.eval(contents.as_slice())?;
                }
                interp.pop_context();
                let metadata = metadata.mark_required();
                interp
                    .vfs_mut()
                    .set_metadata(path.as_path(), metadata)
                    .map_err(|_| {
                        Fatal::new(
                            interp,
                            "Unable to set require metadata in the Artichoke virtual filesystem",
                        )
                    })?;
                trace!(
                    r#"Successful require of {:?} at {:?}"#,
                    <&BStr>::from(filename),
                    path,
                );
                return Ok(interp.convert(true));
            }
        }
    }
    let relative_base: &Path = if let Some(base) = base {
        base
    } else {
        let path = fs::bytes_to_osstr(interp, RUBY_LOAD_PATH.as_bytes())?;
        Path::new(path)
    };
    let path = relative_base.join(file);
    let is_file = interp.vfs().is_file(path.as_path());
    if !is_file {
        let filestr = format!("{:?}", <&BStr>::from(filename));
        return Err(Exception::from(LoadError::new(
            interp,
            format!(
                "cannot load such file -- {}",
                &filestr[1..filestr.len() - 1]
            ),
        )));
    }
    let metadata = interp.vfs().metadata(path.as_path()).unwrap_or_default();
    // If a file is already required, short circuit.
    if metadata.is_already_required() {
        return Ok(interp.convert(false));
    }
    // Require Rust File first because an File may define classes
    // and module with `LoadSources` and Ruby files can require
    // arbitrary other files, including some child sources that may
    // depend on these module definitions.
    let context = Context::new(fs::osstr_to_bytes(interp, path.as_os_str())?.to_vec())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);
    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    if let Some(require) = metadata.require {
        // dynamic, Rust-backed `File` require
        if require(interp).is_err() {
            interp.pop_context();
            let filestr = format!("{:?}", <&BStr>::from(filename));
            return Err(Exception::from(LoadError::new(
                interp,
                format!(
                    "cannot load such file -- {}",
                    &filestr[1..filestr.len() - 1]
                ),
            )));
        }
    }
    let contents = interp.vfs().read_file(path.as_path());
    if let Ok(contents) = contents {
        let _ = interp.eval(contents.as_slice())?;
    }
    interp.pop_context();
    let metadata = metadata.mark_required();
    interp
        .vfs_mut()
        .set_metadata(path.as_path(), metadata)
        .map_err(|_| {
            Fatal::new(
                interp,
                "Unable to set require metadata in the Artichoke virtual filesystem",
            )
        })?;
    trace!(
        r#"Successful require of "{:?}" at {:?}"#,
        <&BStr>::from(filename),
        path,
    );
    Ok(interp.convert(true))
}

#[allow(clippy::module_name_repetitions)]
pub fn require_relative(interp: &mut Artichoke, file: Value) -> Result<Value, Exception> {
    let context = interp
        .peek_context()
        .ok_or_else(|| Fatal::new(interp, "relative require with no context stack"))?;
    let current = fs::bytes_to_osstr(interp, context.filename())?;
    let base = if let Some(base) = Path::new(current).parent() {
        base.to_path_buf()
    } else {
        PathBuf::from("/")
    };
    require(interp, file, Some(base.as_path()))
}
