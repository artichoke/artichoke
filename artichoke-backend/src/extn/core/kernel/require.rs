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

pub fn load(interp: &mut Artichoke, filename: Value) -> Result<bool, Exception> {
    let filename = filename.implicitly_convert_to_string(interp)?;
    if filename.find_byte(b'\0').is_some() {
        return Err(Exception::from(ArgumentError::new(
            interp,
            "path name contains null byte",
        )));
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let pathbuf;
    let mut path = Path::new(file);
    if path.is_relative() {
        pathbuf = Path::new(RUBY_LOAD_PATH).join(file);
        path = pathbuf.as_path();
    }
    if !interp.source_is_file(path)? {
        let mut message = b"cannot load such file -- ".to_vec();
        message.extend_from_slice(filename);
        return Err(LoadError::new_raw(interp, message).into());
    }
    let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);
    let result = interp.load_source(path);
    let _ = interp.pop_context();
    result
}

pub fn require(
    interp: &mut Artichoke,
    filename: Value,
    base: Option<RelativePath>,
) -> Result<bool, Exception> {
    let filename = filename.implicitly_convert_to_string(interp)?;
    if filename.find_byte(b'\0').is_some() {
        return Err(Exception::from(ArgumentError::new(
            interp,
            "path name contains null byte",
        )));
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let path = Path::new(file);

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
        if interp.source_is_file(&path)? {
            let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
                .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
            interp.push_context(context);
            let result = interp.require_source(&path);
            let _ = interp.pop_context();
            return result;
        } else {
            let path = if let Some(ref base) = base {
                base.join(file)
            } else {
                Path::new(RUBY_LOAD_PATH).join(file)
            };
            if interp.source_is_file(&path)? {
                let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
                    .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
                interp.push_context(context);
                let result = interp.require_source(&path);
                let _ = interp.pop_context();
                return result;
            }
        }
    }
    let path = if let Some(ref base) = base {
        base.join(&file)
    } else {
        Path::new(RUBY_LOAD_PATH).join(file)
    };
    if !interp.source_is_file(&path)? {
        let mut message = b"cannot load such file -- ".to_vec();
        message.extend_from_slice(filename);
        return Err(LoadError::new_raw(interp, message).into());
    }
    let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
        .ok_or_else(|| ArgumentError::new(interp, "path name contains null byte"))?;
    interp.push_context(context);
    let result = interp.require_source(&path);
    let _ = interp.pop_context();
    result
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
