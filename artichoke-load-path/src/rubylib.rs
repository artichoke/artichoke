//! A Ruby source loader that resolves sources relative to paths given in a
//! `RUBYLIB` environment variable.

use std::collections::hash_map::{Entry, HashMap};
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use same_file::Handle;

/// A Ruby source code loader that searches in paths given by the `RUBYLIB`
/// environment variable.
///
/// MRI Ruby allows manipulating the [require] search path by setting the
/// `RUBYLIB` environment variable before launching the Ruby CLI. The `RUBYLIB`
/// variable is read on startup and is expected to contain a platform-native
/// path separator-delimited list of file system paths.
///
/// This loader will attempt to resolve relative paths in any of the paths given
/// in `RUBYLIB`. Absolute paths are rejected by this loader.
///
/// This loader tracks the files it has loaded, which MRI refers to as "loaded
/// features". This loader deduplicates loaded features be detecting whether the
/// given path [resolves to the same file] as a previously loaded path.
///
/// The `RUBYLIB` environment variable or other sequence of paths is parsed when
/// this loader is created and is immutable.
///
/// This loader resolves files in the search paths in the order the directories
/// appear in the `RUBYLIB` environment variable. Paths earlier in the sequence
/// have higher priority.
///
/// ```no_run
/// # use std::ffi::OsStr;
/// # use std::path::Path;
/// # use artichoke_load_path::Rubylib;
/// # fn example() -> Option<()> {
/// // Grab the load paths from the `RUBYLIB` environment variable. If the
/// // variable is empty or unset, `None` is returned.
/// //
/// // Relative paths in `RUBYLIB` are resolved relative to the current process's
/// // current working directory.
/// let env_loader = Rubylib::new()?;
///
/// // Search `/home/artichoke/src` first, only attempting to search
/// // `/usr/share/artichoke` if no file is found in `/home/artichoke/src`.
/// //
/// // The relative path `./_lib` is resolved relative to the given working
/// // directory.
/// let fixed_loader = Rubylib::with_rubylib_and_cwd(
///     OsStr::new("/home/artichoke/src:/usr/share/artichoke:./_lib"),
///     Path::new("/home/artichoke")
/// )?;
/// # Some(())
/// # }
/// # example().unwrap();
/// ```
///
/// [require]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
/// [resolves to the same file]: same_file
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "rubylib-native-file-system-loader")))]
pub struct Rubylib {
    /// Fixed set of paths on the host file system to search for Ruby sources.
    load_paths: Box<[PathBuf]>,
    loaded_features: HashMap<Handle, PathBuf>,
}

impl Rubylib {
    /// Create a new native file system loader that searches the file system for
    /// Ruby sources at the paths specified by the `RUBYLIB` environment
    /// variable.
    ///
    /// The `RUBYLIB` environment variable is resolved once at the time this
    /// method is called and the resolved load path is immutable.
    ///
    /// If any of the paths in the `RUBYLIB` environment variable are not
    /// absolute paths, they are absolutized relative to the current process's
    /// [current working directory] at the time this method is called.
    ///
    /// This source loader grants access to the host file system. The Rubylib
    /// loader does not support native extensions.
    ///
    /// This method returns [`None`] if there are errors resolving the
    /// `RUBYLIB` environment variable, if the `RUBYLIB` environment variable is
    /// not set, if the current working directory cannot be retrieved, or if the
    /// `RUBYLIB` environment variable does not contain any paths.
    ///
    /// [current working directory]: env::current_dir
    #[inline]
    #[must_use]
    pub fn new() -> Option<Self> {
        let rubylib = env::var_os("RUBYLIB")?;
        let cwd = env::current_dir().ok()?;
        Self::with_rubylib_and_cwd(&rubylib, &cwd)
    }

    /// Create a new native file system loader that searches the file system for
    /// Ruby sources at the paths specified by the given `rubylib` platform
    /// string. `rubylib` is expected to be a set of file system paths that are
    /// delimited by the platform path separator.
    ///
    /// The resolved load path is immutable.
    ///
    /// If any of the paths in the given `rubylib` are not absolute paths, they
    /// are absolutized relative to the current process's [current working
    /// directory] at the time this method is called.
    ///
    /// This source loader grants access to the host file system. The Rubylib
    /// loader does not support native extensions.
    ///
    /// This method returns [`None`] if the current working directory cannot be
    /// retrieved or if the given `rubylib` does not contain any paths.
    ///
    /// [current working directory]: env::current_dir
    #[inline]
    #[must_use]
    pub fn with_rubylib(rubylib: &OsStr) -> Option<Self> {
        let cwd = env::current_dir().ok()?;
        Self::with_rubylib_and_cwd(rubylib, &cwd)
    }

    /// Create a new native file system loader that searches the file system for
    /// Ruby sources at the paths specified by the given `rubylib` platform
    /// string. `rubylib` is expected to be a set of file system paths that are
    /// delimited by the platform path separator.
    ///
    /// The resolved load path is immutable.
    ///
    /// If any of the paths in the given `rubylib` are not absolute paths, they
    /// are absolutized relative to the given current working directory at the
    /// time this method is called.
    ///
    /// This source loader grants access to the host file system. The Rubylib
    /// loader does not support native extensions.
    ///
    /// This method returns [`None`] if the given `rubylib` does not contain any
    /// paths.
    #[inline]
    #[must_use]
    pub fn with_rubylib_and_cwd(rubylib: &OsStr, cwd: &Path) -> Option<Self> {
        let load_paths = env::split_paths(rubylib)
            .map(|load_path| cwd.join(&load_path))
            .collect::<Vec<_>>();

        // If the `RUBYLIB` env variable is empty or otherwise results in no
        // search paths being resolved, return `None` so the Rubylib loader is
        // not used.
        if load_paths.is_empty() {
            return None;
        }
        // Using a boxed slice ensures that the load path is immutable and it
        // saves a little bit of memory.
        let load_paths = load_paths.into_boxed_slice();
        // Individual source files that the Ruby interpreter requires and loads
        // are called "features" and are exposed in a virtual global variable
        // called `$LOADED_FEATURES` or `$"`.
        let loaded_features = HashMap::new();

        Some(Self {
            load_paths,
            loaded_features,
        })
    }

    /// Check whether `path` points to a file in the virtual file system and
    /// return the absolute path if it exists.
    ///
    /// Returns [`Some`] if the file system object pointed to by `path` exists.
    /// If `path` is relative, it is joined to each path in the `RUBYLIB`
    /// environment variable at the time this loader was initialized.
    ///
    /// This method is infallible and will return [`None`] for non-existent
    /// paths.
    ///
    /// [converted to a byte vec]: crate::ConvertBytesError
    #[inline]
    #[must_use]
    pub fn resolve_file(&self, path: &Path) -> Option<PathBuf> {
        // The Rubylib loader only loads relative paths in `RUBYLIB`.
        if path.is_absolute() {
            return None;
        }
        for load_path in &*self.load_paths {
            let path = load_path.join(path);
            if File::open(&path).is_ok() {
                return Some(path);
            }
        }
        None
    }

    /// Check whether `path` points to a file in the virtual file system.
    ///
    /// Returns `true` if the file system object pointed to by `path` exists and
    /// is a readable file.  If `path` is relative, it is absolutized relative
    /// to each path in the `RUBYLIB` environment variable at the time this
    /// loader was initialized.
    ///
    /// This method is infallible and will return `false` for non-existent
    /// paths.
    #[inline]
    #[must_use]
    pub fn is_file(&self, path: &Path) -> bool {
        // The Rubylib loader only loads relative paths in `RUBYLIB`.
        if path.is_absolute() {
            return false;
        }
        for load_path in &*self.load_paths {
            let path = load_path.join(path);
            if File::open(path).is_ok() {
                return true;
            }
        }
        false
    }

    /// Read file contents for the file at `path`.
    ///
    /// Returns a byte vec of complete file contents. If `path` is relative, it
    /// is absolutized relative to each path in the `RUBYLIB` environment
    /// variable at the time this loader was initialized.
    ///
    /// # Errors
    ///
    /// If `path` does not exist, an [`io::Error`] with error kind
    /// [`io::ErrorKind::NotFound`] is returned.
    #[inline]
    pub fn read_file(&self, path: &Path) -> io::Result<Vec<u8>> {
        // The Rubylib loader only loads relative paths in `RUBYLIB`.
        if path.is_absolute() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Only relative paths can be loaded from RUBYLIB",
            ));
        }
        for load_path in &*self.load_paths {
            let path = load_path.join(path);
            if let Ok(contents) = fs::read(path) {
                return Ok(contents);
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "path not found in RUBYLIB load paths",
        ))
    }

    /// Check whether a file at `path` has been required already.
    ///
    /// Returns `true` if the path is a loaded feature, false otherwise. If
    /// `path` is relative, it is absolutized relative to each path in the
    /// `RUBYLIB` environment variable at the time this loader was initialized.
    ///
    /// This method is infallible and will return `false` for non-existent
    /// paths.
    #[inline]
    #[must_use]
    pub fn is_required(&self, path: &Path) -> Option<bool> {
        // The Rubylib loader only loads relative paths in `RUBYLIB`.
        if path.is_absolute() {
            return None;
        }
        for load_path in &*self.load_paths {
            let path = load_path.join(path);
            if let Ok(handle) = Handle::from_path(&path) {
                return Some(self.loaded_features.contains_key(&handle));
            }
        }
        None
    }

    /// Mark a source at `path` as required on the interpreter.
    ///
    /// This metadata is used by [`Kernel#require`] and friends to enforce that
    /// Ruby sources are only loaded into the interpreter once to limit side
    /// effects.
    ///
    /// If `path` is relative, it is absolutized relative to each path in the
    /// `RUBYLIB` environment variable at the time this loader was initialized.
    ///
    /// This method is infallible and will return `false` for non-existent
    /// paths.
    ///
    /// # Errors
    ///
    /// If `path` does not exist, an [`io::Error`] with error kind
    /// [`io::ErrorKind::NotFound`] is returned.
    ///
    /// [`Kernel#require`]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
    #[inline]
    pub fn mark_required(&mut self, path: &Path) -> io::Result<()> {
        for load_path in &*self.load_paths {
            let path = load_path.join(path);
            if let Ok(handle) = Handle::from_path(&path) {
                match self.loaded_features.entry(handle) {
                    Entry::Occupied(_) => {
                        return Err(io::Error::new(io::ErrorKind::Other, "file is already required"))
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(path);
                        return Ok(());
                    }
                }
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "file not found in RUBYLIB load path",
        ))
    }
}
