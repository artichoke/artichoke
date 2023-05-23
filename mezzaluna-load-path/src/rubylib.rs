use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

/// A Ruby load path builder that reads from the `RUBYLIB` environment variable.
///
/// MRI Ruby allows manipulating the [require] search path by setting the
/// `RUBYLIB` environment variable before launching the Ruby CLI. The `RUBYLIB`
/// variable is read on start-up and is expected to contain a platform-native
/// path separator-delimited list of file system paths.
///
/// The `RUBYLIB` environment variable or other sequence of paths is parsed when
/// this loader is created and is immutable. This builder is intended to be
/// called during interpreter boot.
///
/// Paths earlier in the sequence returned from [`load_path`] have higher
/// priority.
///
/// ```no_run
/// use std::ffi::OsStr;
/// use std::path::Path;
/// use mezzaluna_load_path::Rubylib;
///
/// # #[cfg(unix)]
/// # fn example() -> Option<()> {
/// // Grab the load paths from the `RUBYLIB` environment variable. If the
/// // variable is empty or unset, `None` is returned.
/// let env_loader = Rubylib::new()?;
///
/// // Search `/home/artichoke/src` first, only attempting to search
/// // `/usr/share/artichoke` if no file is found in `/home/artichoke/src`.
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
/// [`load_path`]: Self::load_path
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "rubylib")))]
pub struct Rubylib {
    /// Fixed set of paths on the host file system to search for Ruby sources.
    ///
    /// These load paths are loaded once and are immutable once loaded.
    load_path: Box<[PathBuf]>,
}

impl Rubylib {
    /// Create a new load path builder that reads from the `RUBYLIB` environment
    /// variable.
    ///
    /// The `RUBYLIB` environment variable is read only once at the time this
    /// method is called. The resolved load path is immutable.
    ///
    /// This method returns [`None`] if there are errors resolving the
    /// `RUBYLIB` environment variable, if the `RUBYLIB` environment variable is
    /// not set, or if the given `RUBYLIB` environment variable only contains
    /// empty paths.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mezzaluna_load_path::Rubylib;
    ///
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::new()?;
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[must_use]
    pub fn new() -> Option<Self> {
        let rubylib = env::var_os("RUBYLIB")?;
        Self::with_rubylib(&rubylib)
    }

    /// Create a new load path builder that reads from the given [`OsStr`].
    ///
    /// The `rubylib` platform string given to this method is expected to be a
    /// [path string] of file system paths that are delimited by the platform
    /// path separator.
    ///
    /// The resolved load path is immutable.
    ///
    /// This method returns [`None`] if the given `rubylib` argument only
    /// contains empty paths.
    /// non-empty paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use mezzaluna_load_path::Rubylib;
    ///
    /// # #[cfg(unix)]
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::with_rubylib(OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"))?;
    /// # Some(())
    /// # }
    /// # #[cfg(unix)]
    /// # example().unwrap();
    /// ```
    ///
    /// An empty path string returns [`None`].
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use mezzaluna_load_path::Rubylib;
    ///
    /// let loader = Rubylib::with_rubylib(OsStr::new(""));
    /// assert!(loader.is_none());
    ///
    /// # #[cfg(unix)]
    /// let loader = Rubylib::with_rubylib(OsStr::new("::::"));
    /// # #[cfg(unix)]
    /// assert!(loader.is_none());
    /// ```
    ///
    /// [path string]: env::split_paths
    #[must_use]
    pub fn with_rubylib(rubylib: &OsStr) -> Option<Self> {
        // Empty paths are filtered out of RUBYLIB.
        //
        // `std::env::split_paths` yields empty paths as of Rust 1.69.0.
        // See: https://github.com/rust-lang/rust/issues/111832
        let load_path = env::split_paths(rubylib)
            .filter(|p| !p.as_os_str().is_empty())
            .collect::<Box<[_]>>();

        // If the `RUBYLIB` env variable is empty or otherwise results in no
        // search paths being resolved, return `None` so the `Rubylib` loader is
        // not used.
        if load_path.is_empty() {
            return None;
        }

        Some(Self { load_path })
    }

    /// Return a reference to the paths in `$LOAD_PATH` parsed by this builder.
    ///
    /// Because the paths in `RUBYLIB` have the highest priority when loading
    /// features, the returned paths should appear first in `$LOAD_PATH`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use mezzaluna_load_path::Rubylib;
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
    ///         Path::new("_lib"),
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

    /// Consume this loader and return its `$LOAD_PATH`.
    ///
    /// Because the paths in `RUBYLIB` have the highest priority when loading
    /// features, the returned paths should appear first in `$LOAD_PATH`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use mezzaluna_load_path::Rubylib;
    ///
    /// # #[cfg(unix)]
    /// # fn example() -> Option<()> {
    /// let loader = Rubylib::with_rubylib(
    ///     OsStr::new("/home/artichoke/src:/usr/share/artichoke:_lib"),
    /// )?;
    ///
    /// let load_paths = loader.into_load_path();
    /// assert_eq!(
    ///     load_paths,
    ///     [
    ///         Path::new("/home/artichoke/src"),
    ///         Path::new("/usr/share/artichoke"),
    ///         Path::new("_lib"),
    ///     ]
    /// );
    /// # Some(())
    /// # }
    /// # #[cfg(unix)]
    /// # example().unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn into_load_path(self) -> Vec<PathBuf> {
        self.load_path.into()
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 3);

        let mut iter = load_path.iter();
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 3);

        let mut iter = load_path.iter();
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 3);

        let mut iter = load_path.iter();
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 6);

        let mut iter = load_path.iter();
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 3);

        let mut iter = load_path.iter();
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 3);

        let mut iter = load_path.iter();
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 3);

        let mut iter = load_path.iter();
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

        let load_path = loader.into_load_path();
        assert_eq!(load_path.len(), 6);

        let mut iter = load_path.iter();
        assert_eq!(iter.next().unwrap(), Path::new("."));
        assert_eq!(iter.next().unwrap(), Path::new("."));
        assert_eq!(iter.next().unwrap(), Path::new("c:/lopopolo/dev/artichoke/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/lopopolo/dev/artichoke/artichoke"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/var/"));
        assert_eq!(iter.next().unwrap(), Path::new("c:/var"));
        assert_eq!(iter.next(), None);
    }
}
