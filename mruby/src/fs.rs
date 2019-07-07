//! [`Mrb`] virtual filesystem used for storing Ruby sources.

use mruby_vfs::{FakeFileSystem, FileSystem};
use std::path::Path;

use crate::{Mrb, MrbError};

pub const RUBY_LOAD_PATH: &str = "/src/lib";

pub type RequireFunc = fn(Mrb) -> Result<(), MrbError>;

/// Virtual filesystem that wraps a [`mruby_vfs`] [`FakeFileSystem`].
pub struct MrbFilesystem {
    fs: FakeFileSystem<Metadata>,
}

impl MrbFilesystem {
    /// Create a new in memory virtual filesystem.
    ///
    /// Creates a directory at [`RUBY_LOAD_PATH`] for storing Ruby source files.
    /// This path is searched by
    /// [`Kernel::require`](crate::extn::core::kernel::Kernel::require) and
    /// [`Kernel::require_relative`](crate::extn::core::kernel::Kernel::require_relative).
    pub fn new() -> Result<Self, MrbError> {
        let fs = FakeFileSystem::new();
        fs.create_dir_all(RUBY_LOAD_PATH).map_err(MrbError::Vfs)?;
        Ok(Self { fs })
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), MrbError> {
        self.fs.create_dir_all(path.as_ref()).map_err(MrbError::Vfs)
    }

    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.fs.is_file(path.as_ref())
    }

    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, MrbError> {
        self.fs.read_file(path.as_ref()).map_err(MrbError::Vfs)
    }

    pub fn write_file<P, B>(&self, path: P, buf: B) -> Result<(), MrbError>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>,
    {
        self.fs
            .write_file(path.as_ref(), buf.as_ref())
            .map_err(MrbError::Vfs)
    }

    pub fn set_metadata<P: AsRef<Path>>(
        &self,
        path: P,
        metadata: Metadata,
    ) -> Result<(), MrbError> {
        self.fs
            .set_metadata(path.as_ref(), metadata)
            .map_err(MrbError::Vfs)
    }

    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Option<Metadata> {
        self.fs.metadata(path.as_ref())
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
