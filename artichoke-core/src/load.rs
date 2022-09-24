//! Load Ruby and Rust sources into the VM.

use alloc::borrow::Cow;
use alloc::vec::Vec;

#[cfg(feature = "std")]
type Path = std::path::Path;
#[cfg(not(feature = "std"))]
type Path = str;

use crate::file::File;

/// The side effect from a call to [`Kernel#require`].
///
/// In Ruby, `require` is stateful. All required sources are tracked in a global
/// interpreter state accessible as `$"` and `$LOADED_FEATURES`.
///
/// The first time a file is required, it is parsed and executed by the
/// interpreter. If the file executes without raising an error, the file is
/// successfully required and Rust callers can expect a [`Required::Success`]
/// variant. Files that are successfully required are added to the interpreter's
/// set of loaded features.
///
/// If the file raises an exception as it is required, Rust callers can expect
/// an `Err` variant. The file is not added to the set of loaded features.
///
/// If the file has previously been required such that [`Required::Success`] has
/// been returned, all subsequent calls to require the file will return
/// [`Required::AlreadyRequired`].
///
/// See the documentation of [`require_source`] for more details.
///
/// [`Kernel#require`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
/// [`require_source`]: LoadSources::require_source
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Required {
    /// [`Kernel#require`] succeeded at requiring the file.
    ///
    /// If this variant is returned, this is the first time the given file has
    /// been required in the interpreter.
    ///
    /// This variant has value `true` when converting to a Boolean as returned
    /// by `Kernel#require`.
    ///
    /// [`Kernel#require`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
    Success,
    /// [`Kernel#require`] did not require the file because it has already been
    /// required.
    ///
    /// If this variant is returned, this is at least the second time the given
    /// file has been required. Interpreters guarantee that files are only
    /// required once. To load a source multiple times, see [`load_source`] and
    /// [`Kernel#load`].
    ///
    /// This variant has value `false` when converting to a Boolean as returned
    /// by `Kernel#require`.
    ///
    /// [`Kernel#require`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
    /// [`load_source`]: LoadSources::load_source
    /// [`Kernel#load`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-load
    AlreadyRequired,
}

impl From<Required> for bool {
    /// Convert a [`Required`] enum into a [`bool`] as returned by
    /// [`Kernel#require`].
    ///
    /// [`Kernel#require`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
    fn from(req: Required) -> Self {
        match req {
            Required::Success => true,
            Required::AlreadyRequired => false,
        }
    }
}

/// The side effect from a call to [`Kernel#load`].
///
/// In Ruby, `load` is stateless. All sources passed to `load` are loaded for
/// every method call.
///
/// Each time a file is loaded, it is parsed and executed by the
/// interpreter. If the file executes without raising an error, the file is
/// successfully loaded and Rust callers can expect a [`Loaded::Success`]
/// variant.
///
/// If the file raises an exception as it is required, Rust callers can expect
/// an `Err` variant. The file is not added to the set of loaded features.
///
/// See the documentation of [`load_source`] for more details.
///
/// [`Kernel#load`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-load
/// [`load_source`]: LoadSources::load_source
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Loaded {
    /// [`Kernel#load`] succeeded at loading the file.
    ///
    /// This variant has value `true` when converting to a Boolean as returned
    /// by `Kernel#load`.
    ///
    /// [`Kernel#load`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-load
    Success,
}

impl From<Loaded> for bool {
    /// Convert a [`Loaded`] enum into a [`bool`] as returned by
    /// [`Kernel#load`].
    ///
    /// [`Kernel#load`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-load
    fn from(loaded: Loaded) -> Self {
        let Loaded::Success = loaded;
        true
    }
}

/// Load Ruby sources and Rust extensions into an interpreter.
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
    /// file is placed directly on the file system. Ancestor directories are
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
    /// file is placed directly on the file system. Ancestor directories are
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
    /// If the source file at `path` has no contents, an error is returned.
    fn load_source<P>(&mut self, path: P) -> Result<Loaded, Self::Error>
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
    /// Implementations should ensure that this method returns
    /// [`Ok(Required::Success)`][success] at most once. Subsequent `Ok(_)`
    /// return values should include [`Required::AlreadyRequired`]. See the
    /// documentation of [`Required`] for more details.
    ///
    /// # Errors
    ///
    /// If the underlying file system is inaccessible, an error is returned.
    ///
    /// If reads to the underlying file system fail, an error is returned.
    ///
    /// If `path` does not point to a source file, an error is returned.
    ///
    /// If the source file at `path` has no contents, an error is returned.
    ///
    /// [success]: Required::Success
    fn require_source<P>(&mut self, path: P) -> Result<Required, Self::Error>
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
