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

    /// Concrete type for extension hooks defined by `File::require`.
    type Extension;

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
    /// If the underlying filesystem is inaccessible, an error is returned.
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
    /// If the underlying filesystem is inaccessible, an error is returned.
    ///
    /// If writes to the underlying filesystem fail, an error is returned.
    fn def_rb_source_file<P, T>(&mut self, path: P, contents: T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: Into<Cow<'static, [u8]>>;

    /// Test for a source file at a path.
    ///
    /// Query the underlying virtual filesystem to check if `path` points to a
    /// source file.
    ///
    /// This function returns `false` if `path` does not exist in the virtual
    /// filesystem.
    ///
    /// # Errors
    ///
    /// If the underlying filesystem is inaccessible, an error is returned.
    fn source_is_file<P>(&self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>;

    /// Retrieve the require state for a source file.
    ///
    /// Query the underlying virtual filesystem for the [`State`] of the source
    /// file at `path`.
    ///
    /// # Errors
    ///
    /// If the underlying filesystem is inaccessible, an error is returned.
    ///
    /// If reads to the underlying filesystem fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    fn source_require_state<P>(&self, path: P) -> Result<State, Self::Error>
    where
        P: AsRef<Path>;

    /// Set the require state for a source file.
    ///
    /// Write the source require [`State`] into the underlying virtual
    /// filesystem for the source file at `path`.
    ///
    /// # Errors
    ///
    /// If an invalid state transition occurs, an error is returned.
    ///
    /// If the underlying filesystem is inaccessible, an error is returned.
    ///
    /// If writes to the underlying filesystem fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    fn set_source_require_state<P>(&mut self, path: P, next: State) -> Result<(), Self::Error>
    where
        P: AsRef<Path>;

    /// Retrieve the extension hook for a source file.
    ///
    /// Query the underlying virtual filesystem for the optional extension hook
    /// of the source file at `path`.
    ///
    /// # Errors
    ///
    /// If the underlying filesystem is inaccessible, an error is returned.
    ///
    /// If reads to the underlying filesystem fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    fn source_extension_hook<P>(&self, path: P) -> Result<Option<Self::Extension>, Self::Error>
    where
        P: AsRef<Path>;

    /// Retrieve file contents for a source file.
    ///
    /// Query the underlying virtual filesystem for the file contents of the
    /// source file at `path`.
    ///
    /// # Errors
    ///
    /// If the underlying filesystem is inaccessible, an error is returned.
    ///
    /// If reads to the underlying filesystem fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    fn read_source_file<P>(&self, path: P) -> Result<Cow<'_, [u8]>, Self::Error>
    where
        P: AsRef<Path>;
}

/// States in a source file's `require` lifecycle.
///
/// Source files are by default un-required.
///
/// When `Kernel#require` is invoked with a path that resolves to a source file,
/// it's `require` state transitions to `Required`.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum State {
    /// Default require state, the feature has not been required.
    Default,
    /// The feature has been loaded by `Kernel#require`.
    Required,
}

impl Default for State {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

impl State {
    /// Create a new, default `State`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether this state represents that `Kernel#require` has loaded the feature.
    ///
    /// Returns `true` if `self == State::Required`.
    pub fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }
}
