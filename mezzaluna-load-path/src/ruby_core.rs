use std::path::Path;

/// A Ruby load path builder that prepares paths for in-memory Ruby Core and
/// Ruby Standard Library sources.
///
/// Artichoke embeds Ruby sources and native extensions into an in-memory file
/// system. This in-memory file system is addressed at a virtual mount point,
/// which must be located within the VM `$LOAD_PATH` so they may be loaded by
/// the [require] subsystem.
///
/// Like the site directories in MRI, these paths are the lowest priority load
/// paths and should appear at the end of the `$LOAD_PATH`.
///
/// Paths earlier in the sequence returned from [`load_path`] have higher
/// priority.
///
/// ```no_run
/// use mezzaluna_load_path::RubyCore;
///
/// let core_loader = RubyCore::new();
/// // Load path contains 2 entries: one for Ruby Core sources and one for
/// // Ruby Standard Library sources.
/// assert_eq!(core_loader.load_path().len(), 2);
/// ```
///
/// [require]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
/// [`load_path`]: Self::load_path
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RubyCore {
    _private: (),
}

impl RubyCore {
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
    /// use mezzaluna_load_path::RubyCore;
    ///
    /// let loader = RubyCore::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Return a reference to the load path for sources in the Ruby Core
    /// library.
    ///
    /// Features in Ruby Core have the lowest priority, so the returned path
    /// should appear last in `$LOAD_PATH`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use mezzaluna_load_path::RubyCore;
    ///
    /// let loader = RubyCore::new();
    ///
    /// if cfg!(windows) {
    ///     assert_eq!(
    ///         loader.core_load_path(),
    ///         Path::new("c:/artichoke/virtual_root/site/core/lib"),
    ///     );
    /// } else {
    ///     assert_eq!(
    ///         loader.core_load_path(),
    ///         Path::new("/artichoke/virtual_root/site/core/lib"),
    ///     );
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn core_load_path(self) -> &'static Path {
        if cfg!(windows) {
            Path::new("c:/artichoke/virtual_root/site/core/lib")
        } else {
            Path::new("/artichoke/virtual_root/site/core/lib")
        }
    }

    /// Return a reference to the load path for sources in the Ruby Standard
    /// Library.
    ///
    /// Features in Ruby Standard Library have low priority, so the returned
    /// path should appear second to last in `$LOAD_PATH` (only ahead of the
    /// [core load path]).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use mezzaluna_load_path::RubyCore;
    ///
    /// let loader = RubyCore::new();
    ///
    /// if cfg!(windows) {
    ///     assert_eq!(
    ///         loader.stdlib_load_path(),
    ///         Path::new("c:/artichoke/virtual_root/site/stdlib/lib"),
    ///     );
    /// } else {
    ///     assert_eq!(
    ///         loader.stdlib_load_path(),
    ///         Path::new("/artichoke/virtual_root/site/stdlib/lib"),
    ///     );
    /// }
    /// ```
    ///
    /// [core load path]: Self::core_load_path
    #[inline]
    #[must_use]
    pub fn stdlib_load_path(self) -> &'static Path {
        if cfg!(windows) {
            Path::new("c:/artichoke/virtual_root/site/stdlib/lib")
        } else {
            Path::new("/artichoke/virtual_root/site/stdlib/lib")
        }
    }

    /// Return a reference to the paths in `$LOAD_PATH` parsed by this builder.
    ///
    /// Because the site paths have the lowest priority when loading
    /// features, the returned paths should appear last in `$LOAD_PATH`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_load_path::RubyCore;
    ///
    /// let loader = RubyCore::new();
    /// assert_eq!(
    ///     loader.load_path(),
    ///     [loader.stdlib_load_path(), loader.core_load_path()],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn load_path(self) -> [&'static Path; 2] {
        [self.stdlib_load_path(), self.core_load_path()]
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    #[allow(dead_code)]
    fn test_apis_are_const() {
        const LOADER: RubyCore = RubyCore::new();
    }

    #[test]
    fn test_load_path_has_two_entries() {
        let loader = RubyCore::new();
        assert_eq!(loader.load_path().len(), 2);
    }

    #[test]
    fn test_load_path_has_stdlib_first() {
        let loader = RubyCore::new();
        assert_eq!(loader.load_path().first().copied().unwrap(), loader.stdlib_load_path());
    }

    #[test]
    fn test_load_path_has_core_last() {
        let loader = RubyCore::new();
        assert_eq!(loader.load_path().last().copied().unwrap(), loader.core_load_path());
    }

    #[test]
    fn test_all_paths_are_non_empty() {
        let loader = RubyCore::new();
        assert!(!loader.core_load_path().as_os_str().is_empty());
        assert!(!loader.stdlib_load_path().as_os_str().is_empty());
        assert!(loader.load_path().iter().copied().all(|p| !p.as_os_str().is_empty()));
    }

    #[test]
    fn test_all_paths_are_absolute() {
        let loader = RubyCore::new();
        assert!(loader.core_load_path().is_absolute());
        assert!(loader.stdlib_load_path().is_absolute());
        assert!(loader.load_path().iter().copied().all(Path::is_absolute));
    }
}
