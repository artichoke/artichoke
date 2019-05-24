use log::trace;
use mruby_vfs::FileSystem;
use std::path::PathBuf;

use crate::file::MrbFile;
use crate::interpreter::{Mrb, MrbApi, RUBY_LOAD_PATH};
use crate::state::VfsMetadata;
use crate::MrbError;

pub trait MrbLoadSources
where
    Self: MrbApi,
{
    /// Add a Rust-backed Ruby source file to the virtual filesystem. A stub
    /// Ruby file is added to the filesystem and `require` will dynamically
    /// define Ruby items when invoked via `Kernel#require`.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to [`RUBY_LOAD_PATH`]. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_file<T>(
        &self,
        filename: T,
        require: fn(Self) -> Result<(), MrbError>,
    ) -> Result<(), MrbError>
    where
        T: AsRef<str>;

    /// Add a Rust-backed Ruby source file to the virtual filesystem. A stub
    /// Ruby file is added to the filesystem and [`MrbFile::require`] will
    /// dynamically define Ruby items when invoked via `Kernel#require`.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to [`RUBY_LOAD_PATH`]. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_file_for_type<T, F>(&self, filename: T) -> Result<(), MrbError>
    where
        T: AsRef<str>,
        F: MrbFile;

    /// Add a pure Ruby source file to the virtual filesystem.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to [`RUBY_LOAD_PATH`]. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_rb_source_file<T, F>(&self, filename: T, contents: F) -> Result<(), MrbError>
    where
        T: AsRef<str>,
        F: AsRef<[u8]>;

    /// Normalize path of a Ruby source to be relative to `RUBY_LOAD_PATH`
    /// unless the path is absolute.
    fn normalize_source_path<T>(&self, filename: T) -> PathBuf
    where
        T: AsRef<str>,
    {
        let mut path = PathBuf::from(filename.as_ref());
        if path.is_relative() {
            path = PathBuf::from(RUBY_LOAD_PATH).join(filename.as_ref());
        }
        path
    }
}

impl MrbLoadSources for Mrb {
    fn def_file<T>(
        &self,
        filename: T,
        require: fn(Self) -> Result<(), MrbError>,
    ) -> Result<(), MrbError>
    where
        T: AsRef<str>,
    {
        let api = self.borrow();
        let path = self.normalize_source_path(filename.as_ref());
        if let Some(parent) = path.parent() {
            api.vfs.create_dir_all(parent).map_err(MrbError::Vfs)?;
        }
        if !api.vfs.is_file(&path) {
            let contents = format!("# virtual source file -- {:?}", &path);
            api.vfs.write_file(&path, contents).map_err(MrbError::Vfs)?;
        }
        let mut metadata = api.vfs.metadata(&path).unwrap_or_else(VfsMetadata::new);
        metadata.require = Some(require);
        api.vfs
            .set_metadata(&path, metadata)
            .map_err(MrbError::Vfs)?;
        trace!(
            "Added rust-backed ruby source file with require func -- {:?}",
            &path
        );
        Ok(())
    }

    fn def_file_for_type<T, F>(&self, filename: T) -> Result<(), MrbError>
    where
        T: AsRef<str>,
        F: MrbFile,
    {
        self.def_file(filename.as_ref(), F::require)
    }

    fn def_rb_source_file<T, F>(&self, filename: T, contents: F) -> Result<(), MrbError>
    where
        T: AsRef<str>,
        F: AsRef<[u8]>,
    {
        let api = self.borrow();
        let path = self.normalize_source_path(filename.as_ref());
        if let Some(parent) = path.parent() {
            api.vfs.create_dir_all(parent).map_err(MrbError::Vfs)?;
        }
        api.vfs
            .write_file(&path, contents.as_ref())
            .map_err(MrbError::Vfs)?;
        let metadata = api.vfs.metadata(&path).unwrap_or_else(VfsMetadata::new);
        api.vfs
            .set_metadata(&path, metadata)
            .map_err(MrbError::Vfs)?;
        trace!("Added pure ruby source file -- {:?}", &path);
        Ok(())
    }
}
