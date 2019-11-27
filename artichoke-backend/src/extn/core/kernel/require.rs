//! [`Kernel#require`](https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require)

use artichoke_core::eval::Eval;
use artichoke_core::value::Value as _;
use bstr::BStr;
use std::ffi::OsStr;
use std::path::Path;

use crate::convert::Convert;
use crate::eval::Context;
use crate::extn::core::exception::{ArgumentError, Fatal, LoadError, RubyException, TypeError};
use crate::fs::{self, RUBY_LOAD_PATH};
use crate::value::Value;
use crate::Artichoke;

const RUBY_EXTENSION: &str = "rb";

pub fn load(interp: &Artichoke, filename: Value) -> Result<Value, Box<dyn RubyException>> {
    let ruby_type = filename.pretty_name();
    let filename = if let Ok(filename) = filename.clone().try_into::<&[u8]>() {
        filename
    } else if let Ok(filename) = filename.funcall::<&[u8]>("to_str", &[], None) {
        filename
    } else {
        return Err(Box::new(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", ruby_type),
        )));
    };
    if memchr::memchr(b'\0', filename).is_some() {
        return Err(Box::new(ArgumentError::new(
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
    let is_file = {
        let api = interp.0.borrow();
        api.vfs.is_file(path.as_path())
    };
    if !is_file {
        let filestr = format!("{:?}", <&BStr>::from(filename));
        return Err(Box::new(LoadError::new(
            interp,
            format!(
                "cannot load such file -- {:?}",
                &filestr[1..filestr.len() - 1]
            ),
        )));
    }
    let metadata = {
        let api = interp.0.borrow();
        api.vfs.metadata(path.as_path()).unwrap_or_default()
    };
    // Require Rust File first because an File may define classes
    // and module with `LoadSources` and Ruby files can require
    // arbitrary other files, including some child sources that may
    // depend on these module definitions.
    let context = Context::new(filename.to_vec());
    interp.push_context(context);
    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    if let Some(require) = metadata.require {
        // dynamic, Rust-backed `File` require
        if require(interp.clone()).is_err() {
            interp.pop_context();
            let filestr = format!("{:?}", <&BStr>::from(filename));
            return Err(Box::new(LoadError::new(
                interp,
                format!(
                    "cannot load such file -- {:?}",
                    &filestr[1..filestr.len() - 1]
                ),
            )));
        }
    }
    let contents = {
        let api = interp.0.borrow();
        api.vfs.read_file(path.as_path())
    };
    if let Ok(contents) = contents {
        // We need to be sure we don't leak anything by unwinding past
        // this point. This likely requires a significant refactor to
        // require_impl.
        interp.unchecked_eval(contents.as_slice());
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
    interp: &Artichoke,
    filename: Value,
    base: Option<&Path>,
) -> Result<Value, Box<dyn RubyException>> {
    let ruby_type = filename.pretty_name();
    let filename = if let Ok(filename) = filename.clone().try_into::<&[u8]>() {
        filename
    } else if let Ok(filename) = filename.funcall::<&[u8]>("to_str", &[], None) {
        filename
    } else {
        return Err(Box::new(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", ruby_type),
        )));
    };
    if memchr::memchr(b'\0', filename).is_some() {
        return Err(Box::new(ArgumentError::new(
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
            let context = Context::new(fs::osstr_to_bytes(interp, path.as_os_str())?.to_vec());
            interp.push_context(context.clone());
            // Require Rust File first because an File may define classes and
            // module with `LoadSources` and Ruby files can require arbitrary
            // other files, including some child sources that may depend on these
            // module definitions.
            if let Some(require) = metadata.require {
                // dynamic, Rust-backed `File` require
                if require(interp.clone()).is_err() {
                    interp.pop_context();
                    let filestr = format!("{:?}", <&BStr>::from(filename));
                    return Err(Box::new(LoadError::new(
                        interp,
                        format!(
                            "cannot load such file -- {:?}",
                            &filestr[1..filestr.len() - 1]
                        ),
                    )));
                }
            }
            let contents = {
                let api = interp.0.borrow();
                api.vfs.read_file(path.as_path())
            };
            if let Ok(contents) = contents {
                // We need to be sure we don't leak anything by unwinding past
                // this point. This likely requires a significant refactor to
                // require_impl.
                interp.unchecked_eval(contents.as_slice());
            }
            interp.pop_context();
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
            trace!(
                r#"Successful require of {:?} at {:?}"#,
                <&BStr>::from(filename),
                path,
            );
            return Ok(interp.convert(true));
        } else {
            let path = relative_base.join(file);
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
                let context = Context::new(fs::osstr_to_bytes(interp, path.as_os_str())?.to_vec());
                interp.push_context(context);
                // Require Rust File first because an File may define classes and
                // module with `LoadSources` and Ruby files can require arbitrary
                // other files, including some child sources that may depend on these
                // module definitions.
                if let Some(require) = metadata.require {
                    // dynamic, Rust-backed `File` require
                    if require(interp.clone()).is_err() {
                        interp.pop_context();
                        let filestr = format!("{:?}", <&BStr>::from(filename));
                        return Err(Box::new(LoadError::new(
                            interp,
                            format!(
                                "cannot load such file -- {:?}",
                                &filestr[1..filestr.len() - 1]
                            ),
                        )));
                    }
                }
                let contents = {
                    let api = interp.0.borrow();
                    api.vfs.read_file(path.as_path())
                };
                if let Ok(contents) = contents {
                    // We need to be sure we don't leak anything by unwinding past
                    // this point. This likely requires a significant refactor to
                    // require_impl.
                    interp.unchecked_eval(contents.as_slice());
                }
                interp.pop_context();
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
    let is_file = {
        let api = interp.0.borrow();
        api.vfs.is_file(path.as_path())
    };
    if !is_file {
        let filestr = format!("{:?}", <&BStr>::from(filename));
        return Err(Box::new(LoadError::new(
            interp,
            format!(
                "cannot load such file -- {}",
                &filestr[1..filestr.len() - 1]
            ),
        )));
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
    let context = Context::new(fs::osstr_to_bytes(interp, path.as_os_str())?.to_vec());
    interp.push_context(context);
    // Require Rust File first because an File may define classes and
    // module with `LoadSources` and Ruby files can require arbitrary
    // other files, including some child sources that may depend on these
    // module definitions.
    if let Some(require) = metadata.require {
        // dynamic, Rust-backed `File` require
        if require(interp.clone()).is_err() {
            interp.pop_context();
            let filestr = format!("{:?}", <&BStr>::from(filename));
            return Err(Box::new(LoadError::new(
                interp,
                format!(
                    "cannot load such file -- {}",
                    &filestr[1..filestr.len() - 1]
                ),
            )));
        }
    }
    let contents = {
        let api = interp.0.borrow();
        api.vfs.read_file(path.as_path())
    };
    if let Ok(contents) = contents {
        // We need to be sure we don't leak anything by unwinding past
        // this point. This likely requires a significant refactor to
        // require_impl.
        interp.unchecked_eval(contents.as_slice());
    }
    interp.pop_context();
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
    trace!(
        r#"Successful require of "{:?}" at {:?}"#,
        <&BStr>::from(filename),
        path,
    );
    Ok(interp.convert(true))
}

#[allow(clippy::module_name_repetitions)]
pub fn require_relative(interp: &Artichoke, file: Value) -> Result<Value, Box<dyn RubyException>> {
    let context = interp
        .peek_context()
        .ok_or_else(|| Fatal::new(interp, "relative require with no context stack"))?;
    let current = fs::bytes_to_osstr(interp, context.filename.as_ref())?;
    let base = if let Some(base) = Path::new(current).parent() {
        base
    } else {
        Path::new("/")
    };
    require(interp, file, Some(base))
}
