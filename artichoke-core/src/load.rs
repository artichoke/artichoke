//! Load Ruby and Rust sources into the VM.

use std::borrow::Cow;
use std::error;
use std::path::Path;

use crate::file::File;

/// Load Ruby sources and Rust extensions into an interpreter.
#[allow(clippy::module_name_repetitions)]
pub trait LoadSources {
    /// Concrete type for interpreter.
    type Artichoke;

    /// Concrete type for errors returned from filesystem IO.
    type Error: error::Error;

    /// Concrete type for errors returned by `File::require`.
    type Exception: error::Error;

    /// Add a Rust extension hook to the virtual filesystem. A stub Ruby file is
    /// added to the filesystem and [`File::require`] will dynamically define
    /// Ruby items when invoked via `Kernel#require`.
    ///
    /// If `path` is a relative path, the Ruby source is added to the
    /// filesystem relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    ///
    /// # Errors
    ///
    /// If writes to the underlying filesystem fail, an error is returned.
    fn def_file_for_type<P, T>(&mut self, path: P) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: File<Artichoke = Self::Artichoke, Error = Self::Exception>;

    /// Add a Ruby source to the virtual filesystem.
    ///
    /// If `path` is a relative path, the Ruby source is added to the
    /// filesystem relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    ///
    /// # Errors
    ///
    /// If writes to the underlying filesystem fail, an error is returned.
    fn def_rb_source_file<P, T>(&mut self, path: P, contents: T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: Into<Cow<'static, [u8]>>;
}
