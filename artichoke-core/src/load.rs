//! Load Ruby and Rust sources into the VM.

use alloc::borrow::Cow;
use alloc::vec::Vec;

#[cfg(feature = "std")]
type Path = std::path::Path;
#[cfg(not(feature = "std"))]
type Path = str;

use crate::file::File;

/// Load Ruby sources and Rust extensions into an interpreter.
#[allow(clippy::module_name_repetitions)]
pub trait LoadSources {
    /// Concrete type for interpreter.
    type Artichoke;

    /// Concrete type for errors returned from file system IO.
    type Error;

    /// Concrete type for errors returned by `File::require`.
    type Exception;

    /// Add a Rust extension hook to the virtual file system. A stub Ruby file is
    /// added to the file system and [`File::require`] will dynamically define
    /// Ruby items when invoked via `Kernel#require`.
    ///
    /// If `path` is a relative path, the Ruby source is added to the
    /// file system relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the file system. Anscestor directories are
    /// created automatically.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    ///
    /// If writes to the underlying file system fail, an error is returned.
    fn def_file_for_type<P, T>(&mut self, path: P) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: File<Artichoke = Self::Artichoke, Error = Self::Exception>;

    /// Add a Ruby source to the virtual file system.
    ///
    /// If `path` is a relative path, the Ruby source is added to the
    /// file system relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the file system. Anscestor directories are
    /// created automatically.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    ///
    /// If writes to the underlying file system fail, an error is returned.
    fn def_rb_source_file<P, T>(&mut self, path: P, contents: T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: Into<Cow<'static, [u8]>>;

    /// Test for a source file at a path and return the absolute path of the
    /// resolved file.
    ///
    /// Query the underlying virtual file system to check if `path` points to a
    /// source file.
    ///
    /// This function returns [`None`] if `path` does not exist in the virtual
    /// file system.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    fn resolve_source_path<P>(&self, path: P) -> Result<Option<Vec<u8>>, Self::Error>
    where
        P: AsRef<Path>;

    /// Test for a source file at a path.
    ///
    /// Query the underlying virtual file system to check if `path` points to a
    /// source file.
    ///
    /// This function returns `false` if `path` does not exist in the virtual
    /// file system.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    fn source_is_file<P>(&self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>;

    /// Load source located at the given path.
    ///
    /// Query the underlying virtual file system for a source file and load it
    /// onto the interpreter. This loads files with the following steps:
    ///
    /// 1. Retrieve and execute the extension hook, if any.
    /// 2. Read file contents and [`eval`](crate::eval::Eval) them.
    ///
    /// If this function returns without error, the feature specified by `path`
    /// is loaded, but is not added to `$LOADED_FEATURES`. This function is
    /// equivalent to `Kernel#load`.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    ///
    /// If reads to the underlying file system fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    ///
    /// If the souce file at `path` has no contents, an error is returned.
    fn load_source<P>(&mut self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>;

    /// Require source located at the given path.
    ///
    /// Query the underlying virtual file system for a source file and require it
    /// onto the interpreter. This requires files with the following steps:
    ///
    /// 1. Retrieve and execute the extension hook, if any.
    /// 2. Read file contents and [`eval`](crate::eval::Eval) them.
    /// 3. Mark file as required and add to `$LOADED_FEATURES`.
    ///
    /// If this function returns without error, the feature specified by `path`
    /// is loaded and added to `$LOADED_FEATURES`. This function is equivalent
    /// to `Kernel#require`.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    ///
    /// If reads to the underlying file system fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    ///
    /// If the souce file at `path` has no contents, an error is returned.
    fn require_source<P>(&mut self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>;

    /// Retrieve file contents for a source file.
    ///
    /// Query the underlying virtual file system for the file contents of the
    /// source file at `path`.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    ///
    /// If reads to the underlying file system fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    fn read_source_file_contents<P>(&self, path: P) -> Result<Cow<'_, [u8]>, Self::Error>
    where
        P: AsRef<Path>;
}
