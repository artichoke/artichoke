//! Load Ruby and Rust sources into the VM.

use std::borrow::Cow;

use crate::file::File;
use crate::ArtichokeError;

/// Interpreters that implement [`LoadSources`] expose methods for loading Ruby
/// and Rust sources into the VM.
#[allow(clippy::module_name_repetitions)]
pub trait LoadSources {
    /// Concrete type for interpreter.
    type Artichoke;

    /// Add a Rust-backed Ruby source file to the virtual filesystem. A stub
    /// Ruby file is added to the filesystem and [`File::require`] will
    /// dynamically define Ruby items when invoked via `Kernel#require`.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    ///
    /// # Errors
    ///
    /// This method manipulates a filesystem-like interface, which may fail on
    /// write. Implementations should return [`ArtichokeError::Vfs`] to wrap any
    /// [`io::Error`](std::io::Error).
    fn def_file_for_type<T>(&self, filename: &[u8]) -> Result<(), ArtichokeError>
    where
        T: File<Artichoke = Self::Artichoke>;

    /// Add a pure Ruby source file to the virtual filesystem.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    ///
    /// # Errors
    ///
    /// This method manipulates a filesystem-like interface, which may fail on
    /// write. Implementations should return [`ArtichokeError::Vfs`] to wrap any
    /// [`io::Error`](std::io::Error).
    fn def_rb_source_file<T>(&self, filename: &[u8], contents: T) -> Result<(), ArtichokeError>
    where
        T: Into<Cow<'static, [u8]>>;
}
