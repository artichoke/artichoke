//! [`Artichoke`] virtual filesystem used for storing Ruby sources.

use artichoke_vfs::{FakeFileSystem, FileSystem};
use std::ffi::OsStr;
use std::io;
use std::path::Path;

use crate::extn::core::exception::RubyException;
use crate::{Artichoke, ArtichokeError};

pub mod abs;

pub const RUBY_LOAD_PATH: &str = "/src/lib";

pub type RequireFunc = fn(Artichoke) -> Result<(), ArtichokeError>;

/// Virtual filesystem that wraps a [`artichoke_vfs`] [`FakeFileSystem`].
pub struct Filesystem {
    fs: FakeFileSystem<Metadata>,
}

impl Filesystem {
    /// Create a new in memory virtual filesystem.
    ///
    /// Creates a directory at [`RUBY_LOAD_PATH`] for storing Ruby source files.
    /// This path is searched by
    /// [`Kernel::require`](crate::extn::core::kernel::Kernel::require) and
    /// [`Kernel::require_relative`](crate::extn::core::kernel::Kernel::require_relative).
    pub fn new() -> Result<Self, ArtichokeError> {
        let fs = FakeFileSystem::new();
        fs.create_dir_all(RUBY_LOAD_PATH)
            .map_err(ArtichokeError::Vfs)?;
        Ok(Self { fs })
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), ArtichokeError> {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = abs::PathAbs::new(path.as_ref(), cwd)
            .map_err(io::Error::from)
            .map_err(ArtichokeError::Vfs)?;
        self.fs
            .create_dir_all(path.as_path())
            .map_err(ArtichokeError::Vfs)
    }

    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.fs
            .current_dir()
            .and_then(|cwd| abs::PathAbs::new(path.as_ref(), cwd).map_err(io::Error::from))
            .map(|path| self.fs.is_file(path.as_path()))
            .unwrap_or_default()
    }

    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, ArtichokeError> {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = abs::PathAbs::new(path.as_ref(), cwd)
            .map_err(io::Error::from)
            .map_err(ArtichokeError::Vfs)?;
        self.fs
            .read_file(path.as_path())
            .map_err(ArtichokeError::Vfs)
    }

    pub fn write_file<P, B>(&self, path: P, buf: B) -> Result<(), ArtichokeError>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>,
    {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = abs::PathAbs::new(path.as_ref(), cwd)
            .map_err(io::Error::from)
            .map_err(ArtichokeError::Vfs)?;
        self.fs
            .write_file(path.as_path(), buf.as_ref())
            .map_err(ArtichokeError::Vfs)
    }

    pub fn set_metadata<P: AsRef<Path>>(
        &self,
        path: P,
        metadata: Metadata,
    ) -> Result<(), ArtichokeError> {
        let cwd = self.fs.current_dir().map_err(ArtichokeError::Vfs)?;
        let path = abs::PathAbs::new(path.as_ref(), cwd)
            .map_err(io::Error::from)
            .map_err(ArtichokeError::Vfs)?;
        self.fs
            .set_metadata(path.as_path(), metadata)
            .map_err(ArtichokeError::Vfs)
    }

    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Option<Metadata> {
        let cwd = self.fs.current_dir().ok()?;
        let path = abs::PathAbs::new(path.as_ref(), cwd).ok()?;
        self.fs.metadata(path.as_path())
    }
}

#[derive(Clone, Debug)]
pub struct Metadata {
    pub require: Option<RequireFunc>,
    already_required: bool,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_required(self) -> Self {
        Self {
            require: self.require,
            already_required: true,
        }
    }

    pub fn is_already_required(&self) -> bool {
        self.already_required
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            require: None,
            already_required: false,
        }
    }
}

#[cfg(unix)]
pub fn osstr_to_bytes<'a>(
    interp: &Artichoke,
    value: &'a OsStr,
) -> Result<&'a [u8], Box<dyn RubyException>> {
    use std::os::unix::ffi::OsStrExt;

    let _ = interp;
    Ok(value.as_bytes())
}

#[cfg(not(unix))]
pub fn osstr_to_bytes<'a>(
    interp: &Artichoke,
    value: &'a OsStr,
) -> Result<&'a [u8], Box<dyn RubyException>> {
    use crate::extn::core::exception::Fatal;

    if let Ok(converted) = value.as_ref().to_str() {
        Ok(converted.as_bytes())
    } else {
        Err(Box::new(Fatal::new(
            interp,
            // TODO: Add a ticket number to this message.
            "non UTF-8 ENV keys and values are not yet supported on this Artichoke platform",
        )))
    }
}

#[cfg(unix)]
pub fn bytes_to_osstr<'a>(
    interp: &Artichoke,
    value: &'a [u8],
) -> Result<&'a OsStr, Box<dyn RubyException>> {
    use std::os::unix::ffi::OsStrExt;

    let _ = interp;
    Ok(OsStr::from_bytes(value))
}

#[cfg(not(unix))]
pub fn bytes_to_osstr<'a>(
    interp: &Artichoke,
    value: &'a [u8],
) -> Result<&'a OsStr, Box<dyn RubyException>> {
    use crate::extn::core::exception::Fatal;

    if let Ok(converted) = str::from_utf8(value) {
        Ok(OsStr::from(converted))
    } else {
        Err(Box::new(Fatal::new(
            interp,
            // TODO: Add a ticket number to this message.
            "non UTF-8 ENV keys and values are not yet supported on this Artichoke platform",
        )))
    }
}
