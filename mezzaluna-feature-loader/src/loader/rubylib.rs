//! A Ruby source loader that resolves sources relative to paths given in a
//! `RUBYLIB` environment variable.

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use same_file::Handle;

/// A Ruby source code loader that searches in paths given by the `RUBYLIB`
/// environment variable.
///
/// MRI Ruby allows manipulating the [require] search path by setting the
/// `RUBYLIB` environment variable before launching the Ruby CLI. The `RUBYLIB`
/// variable is read on startup and is expected to contain a platform-native
/// path separator-delimited list of filesystem paths.
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
/// # use mezzaluna_feature_loader::Rubylib;
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
///     "/home/artichoke/src:/usr/share/artichoke:./_lib",
///     "/home/artichoke"
/// )?;
/// # Some(())
/// # }
/// # example().unwrap();
/// ```
///
/// [require]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
/// [resolves to the same file]: same_file
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "rubylib")))]
pub struct Rubylib {
    /// Fixed set of paths on the host filesystem to search for Ruby sources.
    load_paths: Box<[PathBuf]>,
}

impl Rubylib {
    /// Create a new native filesystem loader that searches the filesystem for
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
    /// This source loader grants access to the host filesystem. The Rubylib
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
        Self::with_rubylib_and_cwd(rubylib, cwd)
    }

    /// Create a new native filesystem loader that searches the filesystem for
    /// Ruby sources at the paths specified by the given `rubylib` platform
    /// string. `rubylib` is expected to be a set of filesystem paths that are
    /// delimited by the platform path separator.
    ///
    /// The resolved load path is immutable.
    ///
    /// If any of the paths in the given `rubylib` are not absolute paths, they
    /// are absolutized relative to the current process's [current working
    /// directory] at the time this method is called.
    ///
    /// This source loader grants access to the host filesystem. The Rubylib
    /// loader does not support native extensions.
    ///
    /// This method returns [`None`] if the current working directory cannot be
    /// retrieved or if the given `rubylib` does not contain any paths.
    ///
    /// [current working directory]: env::current_dir
    #[inline]
    #[must_use]
    pub fn with_rubylib<T>(rubylib: T) -> Option<Self>
    where
        T: AsRef<OsStr>,
    {
        let cwd = env::current_dir().ok()?;
        Self::with_rubylib_and_cwd(rubylib, cwd)
    }

    /// Create a new native filesystem loader that searches the filesystem for
    /// Ruby sources at the paths specified by the given `rubylib` platform
    /// string. `rubylib` is expected to be a set of filesystem paths that are
    /// delimited by the platform path separator.
    ///
    /// The resolved load path is immutable.
    ///
    /// If any of the paths in the given `rubylib` are not absolute paths, they
    /// are absolutized relative to the given current working directory at the
    /// time this method is called.
    ///
    /// This source loader grants access to the host filesystem. The Rubylib
    /// loader does not support native extensions.
    ///
    /// This method returns [`None`] if the given `rubylib` does not contain any
    /// paths.
    #[inline]
    #[must_use]
    pub fn with_rubylib_and_cwd<T, U>(rubylib: T, cwd: U) -> Option<Self>
    where
        T: AsRef<OsStr>,
        U: AsRef<OsStr>,
    {
        let cwd = Path::new(&cwd);
        let load_paths = env::split_paths(&rubylib)
            .map(|load_path| {
                if load_path.is_absolute() {
                    load_path
                } else {
                    cwd.join(&load_path)
                }
            })
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

        Some(Self { load_paths })
    }

    /// Check whether `path` points to a file in the virtual filesystem and
    /// return the absolute path if it exists.
    ///
    /// Returns [`Some`] if the filesystem object pointed to by `path` exists.
    /// If `path` is relative, it is joined to each path in the `RUBYLIB`
    /// environment variable at the time this loader was initialized.
    ///
    /// This method is infallible and will return [`None`] for non-existent
    /// paths.
    #[inline]
    #[must_use]
    pub fn resolve_file(&self, path: &Path) -> Option<Handle> {
        // The Rubylib loader only loads relative paths in `RUBYLIB`.
        if path.is_absolute() {
            return None;
        }
        for load_path in &*self.load_paths {
            let path = load_path.join(path);
            if let Ok(handle) = Handle::from_path(&path) {
                return Some(handle);
            }
        }
        None
    }
}