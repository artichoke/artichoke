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
/// # #[cfg(unix)]
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
/// let fixed_loader = Rubylib::with_rubylib(
///     OsStr::new("/home/artichoke/src:/usr/share/artichoke:./_lib"),
/// )?;
/// # Some(())
/// # }
/// # #[cfg(unix)]
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
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// This method returns [`None`] if there are errors resolving the
    /// `RUBYLIB` environment variable, if the `RUBYLIB` environment variable is
    /// not set, or if the given `RUBYLIB` environment variable contains no
    /// non-empty paths.
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
    #[inline]
    #[must_use]
    pub fn new() -> Option<Self> {
        let rubylib = env::var_os("RUBYLIB")?;
        Self::with_rubylib(&rubylib)
    }

    /// Create a new native file system loader that searches the file system for
    /// Ruby sources at the paths specified by the given `rubylib` platform
    /// string. `rubylib` is expected to be a set of file system paths that are
    /// delimited by the platform path separator.
    ///
    /// The resolved load path is immutable.
    ///
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// This method returns [`None`] if the given `rubylib` contains no
    /// non-empty paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use mezzaluna_feature_loader::loaders::Rubylib;
    ///
    /// # #[cfg(unix)]
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::with_rubylib(OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"))?;
    /// # Some(())
    /// # }
    /// # #[cfg(unix)]
    /// # example().unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_rubylib(rubylib: &OsStr) -> Option<Self> {
        let load_path = env::split_paths(rubylib)
            .filter(|p| {
                // Empty paths are filtered out of RUBYLIB:
                !p.as_os_str().is_empty()
            })
            .collect::<Box<[_]>>();

        // If the `RUBYLIB` env variable is empty or otherwise results in no
        // search paths being resolved, return `None` so the `Rubylib` loader is
        // not used.
        if load_path.is_empty() {
            return None;
        }

        Some(Self { load_path })
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
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use mezzaluna_feature_loader::loaders::Rubylib;
    ///
    /// # #[cfg(unix)]
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::with_rubylib(
    ///     OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
    /// )?;
    /// assert_eq!(
    ///     loader.load_path(),
    ///     &[
    ///         Path::new("/home/artichoke/src"),
    ///         Path::new("/usr/share/artichoke"),
    ///         Path::new("_lib")
    ///     ]
    /// );
    /// # Some(())
    /// # }
    /// # #[cfg(unix)]
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
        assert_eq!(iter.next().unwrap(), Path::new("_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_load_path_is_set_on_construction() {
        let loader = Rubylib::with_rubylib(OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib")).unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_rubylib_is_none() {
        let loader = Rubylib::with_rubylib(OsStr::new(""));
        assert!(loader.is_none());
    }

    #[test]
    fn test_empty_rubylib_paths_are_filtered() {
        // ```console
        // $ ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // $ RUBYLIB= ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // $ RUBYLIB=::: ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // ```
        let loader = Rubylib::with_rubylib(OsStr::new(":::::::::::::::::"));
        assert!(loader.is_none());

        let loader =
            Rubylib::with_rubylib(OsStr::new(":::/home/artichoke/src:::/usr/share/artichoke:::_lib:::")).unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_paths_taken_verbatim() {
        // Relative paths are not resolved, duplicates are not removed, paths
        // are not normalized:
        //
        // ```console
        // $ RUBYLIB=.:.:`pwd`:`pwd`:/Users/:/Users ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // .
        // .
        // /Users/lopopolo/dev/artichoke/artichoke
        // /Users/lopopolo/dev/artichoke/artichoke
        // /Users/
        // /Users
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // ```
        let loader = Rubylib::with_rubylib(OsStr::new(
            ".:.:/Users/lopopolo/dev/artichoke/artichoke:/Users/lopopolo/dev/artichoke/artichoke:/Users/:/Users",
        ))
        .unwrap();

        assert_eq!(loader.load_path().len(), 6);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("."));
        assert_eq!(iter.next().unwrap(), Path::new("."));
        assert_eq!(
            iter.next().unwrap(),
            Path::new("/Users/lopopolo/dev/artichoke/artichoke")
        );
        assert_eq!(
            iter.next().unwrap(),
            Path::new("/Users/lopopolo/dev/artichoke/artichoke")
        );
        assert_eq!(iter.next().unwrap(), Path::new("/Users/"));
        assert_eq!(iter.next().unwrap(), Path::new("/Users"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_resolve_file_rejects_absolute_paths() {
        let loader = Rubylib::with_rubylib(OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib")).unwrap();

        let file = loader.resolve_file(Path::new("/absolute/path/to/source.rb"));
        assert!(file.is_none());
    }

    #[test]
    fn test_resolve_file_returns_none_for_nonexistent_path() {
        let loader = Rubylib::with_rubylib(OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib")).unwrap();

        // randomly generated with `python -c 'import secrets; print(secrets.token_urlsafe())'`
        let file = loader.resolve_file(Path::new("aSMZbEQeJbIfEJYtV-sDOxvJuvSvO4arx3nNXVzMRvg.rb"));
        assert!(file.is_none());
    }
}

#[cfg(all(test, windows))]
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

        env::set_var("RUBYLIB", "c:/home/artichoke/src;c:/usr/share/artichoke;_lib");
        let loader = Rubylib::new().unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("c:/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_load_path_is_set_on_construction() {
        let loader = Rubylib::with_rubylib(OsStr::new("c:/home/artichoke/src;c:/usr/share/artichoke;_lib")).unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("c:/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_rubylib_is_none() {
        let loader = Rubylib::with_rubylib(OsStr::new(""));
        assert!(loader.is_none());
    }

    #[test]
    fn test_empty_rubylib_paths_are_filtered() {
        // ```console
        // $ ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // $ RUBYLIB= ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // $ RUBYLIB=::: ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // ```
        let loader = Rubylib::with_rubylib(OsStr::new(";;;;;;;;;;;;;;;;"));
        assert!(loader.is_none());

        let loader = Rubylib::with_rubylib(OsStr::new(
            ";;;c:/home/artichoke/src;;;c:/usr/share/artichoke;;;_lib;;;",
        ))
        .unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("c:/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("_lib"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_paths_taken_verbatim() {
        // Relative paths are not resolved, duplicates are not removed, paths
        // are not normalized:
        //
        // ```console
        // $ RUBYLIB=.:.:`pwd`:`pwd`:/Users/:/Users ruby -e 'puts $:'
        // /usr/local/Cellar/rbenv/1.2.0/rbenv.d/exec/gem-rehash
        // .
        // .
        // /Users/lopopolo/dev/artichoke/artichoke
        // /Users/lopopolo/dev/artichoke/artichoke
        // /Users/
        // /Users
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/site_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby/3.2.0/x86_64-darwin22
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/vendor_ruby
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0
        // /usr/local/var/rbenv/versions/3.2.2/lib/ruby/3.2.0/x86_64-darwin22
        // ```
        let loader = Rubylib::with_rubylib(OsStr::new(
            ".;.;c:/lopopolo/dev/artichoke/artichoke;c:/lopopolo/dev/artichoke/artichoke;c:/var/;c:/var",
        ))
        .unwrap();

        assert_eq!(loader.load_path().len(), 6);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("."));
        assert_eq!(iter.next().unwrap(), Path::new("."));
        assert_eq!(iter.next().unwrap(), Path::new("c:/lopopolo/dev/artichoke/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/lopopolo/dev/artichoke/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/var/"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/var"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_resolve_file_rejects_absolute_paths() {
        let loader = Rubylib::with_rubylib(OsStr::new("c:/home/artichoke/src;c:/usr/share/artichoke;_lib")).unwrap();

        let file = loader.resolve_file(Path::new("c:/absolute/path/to/source.rb"));
        assert!(file.is_none());
    }

    #[test]
    fn test_resolve_file_returns_none_for_nonexistent_path() {
        let loader = Rubylib::with_rubylib(OsStr::new("c:/home/artichoke/src;/usr/share/artichoke;_lib")).unwrap();

        // randomly generated with `python -c 'import secrets; print(secrets.token_urlsafe())'`
        let file = loader.resolve_file(Path::new("aSMZbEQeJbIfEJYtV-sDOxvJuvSvO4arx3nNXVzMRvg.rb"));
        assert!(file.is_none());
    }
}
