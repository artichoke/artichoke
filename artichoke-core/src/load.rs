//! Load Ruby and Rust sources into the VM.

use std::path::Path;

use crate::file::File;
use crate::ArtichokeError;

/// Interpreters that implement [`LoadSources`] expose methods for loading Ruby
/// and Rust sources into the VM.
#[allow(clippy::module_name_repetitions)]
pub trait LoadSources {
    /// Concrete type object to modify the interpreter once a source is
    /// `require`d.
    type Require;

    /// Add a Rust-backed Ruby source file to the virtual filesystem. A stub
    /// Ruby file is added to the filesystem and `require` will dynamically
    /// define Ruby items when invoked via `Kernel#require`.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_file(&self, filename: Path, require: Self::Require) -> Result<(), ArtichokeError>;

    /// Add a Rust-backed Ruby source file to the virtual filesystem. A stub
    /// Ruby file is added to the filesystem and [`File::require`] will
    /// dynamically define Ruby items when invoked via `Kernel#require`.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_file_for_type<F>(&self, filename: Path) -> Result<(), ArtichokeError>
    where
        F: File;

    /// Add a pure Ruby source file to the virtual filesystem.
    ///
    /// If filename is a relative path, the Ruby source is added to the
    /// filesystem relative to `RUBY_LOAD_PATH`. If the path is absolute, the
    /// file is placed directly on the filesystem. Anscestor directories are
    /// created automatically.
    fn def_rb_source_file(&self, filename: Path, contents: &[u8]) -> Result<(), ArtichokeError>;
}
