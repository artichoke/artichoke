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
/// variable is read on start-up and is expected to contain a platform-native
/// path separator-delimited list of file system paths.
///
/// This loader will attempt to resolve relative paths in any of the paths given
/// in `RUBYLIB`. Absolute paths are rejected by this loader.
///
/// The `RUBYLIB` environment variable or other sequence of paths is parsed when
/// this loader is created and is immutable.
///
/// This loader resolves files in the search paths in the order the directories
/// appear in the `RUBYLIB` environment variable. Paths earlier in the sequence
/// have higher priority.
///
/// ```no_run
/// use std::ffi::OsStr;
/// use std::path::Path;
/// use mezzaluna_feature_loader::loaders::Rubylib;
///
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
///     Path::new("/home/artichoke"),
/// )?;
/// # Some(())
/// # }
/// # example().unwrap();
/// ```
///
/// [require]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
/// [resolves to the same file]: same_file
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "rubylib")))]
pub struct Rubylib {
    /// Fixed set of paths on the host file system to search for Ruby sources.
    load_path: Box<[PathBuf]>,
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
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// This method returns [`None`] if there are errors resolving the
    /// `RUBYLIB` environment variable, if the `RUBYLIB` environment variable is
    /// not set, if the current working directory cannot be retrieved, or if the
    /// `RUBYLIB` environment variable does not contain any paths.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mezzaluna_feature_loader::loaders::Rubylib;
    ///
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::new()?;
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
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
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// This method returns [`None`] if the current working directory cannot be
    /// retrieved or if the given `rubylib` does not contain any paths.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg(unix)]
    /// use std::ffi::OsStr;
    /// use mezzaluna_feature_loader::loaders::Rubylib;
    ///
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::with_rubylib(OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"))?;
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
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
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// This method returns [`None`] if the given `rubylib` does not contain any
    /// paths or if the given working directory is not an absolute path.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg(unix)]
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use mezzaluna_feature_loader::loaders::Rubylib;
    ///
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::with_rubylib_and_cwd(
    ///     OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
    ///     Path::new("/home/artichoke"),
    /// )?;
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_rubylib_and_cwd(rubylib: &OsStr, cwd: &Path) -> Option<Self> {
        if !cwd.is_absolute() {
            return None;
        }
        if rubylib.is_empty() {
            return None;
        }

        let load_path = env::split_paths(rubylib)
            .map(|load_path| cwd.join(load_path))
            .collect::<Box<[_]>>();

        // If the `RUBYLIB` env variable is empty or otherwise results in no
        // search paths being resolved, return `None` so the `Rubylib` loader is
        // not used.
        if load_path.is_empty() {
            return None;
        }

        Some(Self {
            load_path: dbg!(load_path),
        })
    }

    /// Check whether `path` points to a file in the backing file system and
    /// return a file [`Handle`] if it exists.
    ///
    /// Returns [`Some`] if the file system object pointed to by `path` exists.
    /// This method refuses to resolve absolute paths and will always return
    /// [`None`] for absolute paths. If `path` is relative, it is joined to each
    /// path in the `RUBYLIB` environment variable at the time this loader was
    /// initialized.
    ///
    /// This method is infallible and will return [`None`] for non-existent
    /// paths.
    #[inline]
    #[must_use]
    pub fn resolve_file(&self, path: &Path) -> Option<Handle> {
        // The `Rubylib` loader only loads relative paths in `RUBYLIB`.
        if path.is_absolute() {
            return None;
        }
        for load_path in &*self.load_path {
            let path = load_path.join(path);
            if let Ok(handle) = Handle::from_path(path) {
                return Some(handle);
            }
        }
        None
    }

    /// Return a reference to the loader's current `$LOAD_PATH`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![cfg(unix)]
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use mezzaluna_feature_loader::loaders::Rubylib;
    ///
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::with_rubylib_and_cwd(
    ///     OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
    ///     Path::new("/home/artichoke"),
    /// )?;
    /// assert_eq!(
    ///     loader.load_path(),
    ///     &[
    ///         Path::new("/home/artichoke/src"),
    ///         Path::new("/usr/share/artichoke"),
    ///         Path::new("/home/artichoke/_lib")
    ///     ]
    /// );
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn load_path(&self) -> &[PathBuf] {
        &self.load_path
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::env;
    use std::ffi::OsStr;
    use std::path::Path;

    use super::*;

    #[test]
    fn with_rubylib_env_var() {
        env::remove_var("RUBYLIB");
        let loader = Rubylib::new();
        assert!(loader.is_none());

        env::set_var("RUBYLIB", "");
        let loader = Rubylib::new();
        assert!(loader.is_none());

        env::set_var("RUBYLIB", "/home/artichoke/src:/usr/share/artichoke:_lib");
        let loader = Rubylib::new().unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), &env::current_dir().unwrap().join("_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_load_path_is_set_on_construction() {
        let loader = Rubylib::with_rubylib(OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib")).unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), &env::current_dir().unwrap().join("_lib"));
        assert_eq!(iter.next(), None);

        let loader = Rubylib::with_rubylib_and_cwd(
            OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
            Path::new("/test/xyz"),
        )
        .unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("/test/xyz/_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_relative_cwd_is_err() {
        let loader = Rubylib::with_rubylib_and_cwd(
            OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
            Path::new("xyz"),
        );
        assert!(loader.is_none());
    }

    #[test]
    fn test_env_var_path_is_none() {
        let loader = Rubylib::with_rubylib_and_cwd(OsStr::new(""), Path::new("/test/xyz"));
        assert!(loader.is_none());

        let loader = Rubylib::with_rubylib(OsStr::new(""));
        assert!(loader.is_none());
    }

    #[test]
    fn test_resolve_file_rejects_absolute_paths() {
        let loader = Rubylib::with_rubylib_and_cwd(
            OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
            Path::new("/test/xyz"),
        )
        .unwrap();

        let file = loader.resolve_file(Path::new("/absolute/path/to/source.rb"));
        assert!(file.is_none());
    }

    #[test]
    fn test_resolve_file_returns_none_for_nonexistent_path() {
        let loader = Rubylib::with_rubylib_and_cwd(
            OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
            Path::new("/test/xyz"),
        )
        .unwrap();

        // randomly generated with `python -c 'import secrets; print(secrets.token_urlsafe())'`
        let file = loader.resolve_file(Path::new("aSMZbEQeJbIfEJYtV-sDOxvJuvSvO4arx3nNXVzMRvg.rb"));
        assert!(file.is_none());
    }
}
