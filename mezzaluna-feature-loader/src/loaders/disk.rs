//! A Ruby source loader that resolves sources from the host file system.

use core::mem;
use std::env;
use std::path::{Path, PathBuf};

use same_file::Handle;
use scolapasta_path::is_explicit_relative;

/// A Ruby source code loader that loads sources directly from disk and resolves
/// relative paths with the Ruby `$LOAD_PATH`.
///
/// MRI Ruby allows manipulating the [require] search path by modifying the
/// `$LOAD_PATH` global, or its alias `$:`, at runtime.
///
/// MRI Ruby allows requiring sources with relative, [explicit relative] or
/// absolute paths.
///
/// Relative paths are paths of the form `json/pure` which do not begin with
/// either a file system root like `/` or `C:\` or an explicit relative
/// directory marker like `..` or `.`. These relative paths are resolved
/// relative to the Ruby load path.
///
/// # Examples
///
/// ```
/// use std::ffi::OsStr;
/// use std::path::{Path, PathBuf};
/// use mezzaluna_feature_loader::loaders::Disk;
///
/// # fn example() -> Option<()> {
/// // Search `/home/artichoke/src` first, only attempting to search
/// // `/usr/share/artichoke` if no file is found in `/home/artichoke/src`.
/// //
/// // The relative path `./_lib` is resolved relative to the given working
/// // directory.
/// let loader = Disk::with_load_path_and_cwd(
///     [
///         PathBuf::from("/home/artichoke/src"),
///         PathBuf::from("/usr/share/artichoke"),
///         PathBuf::from("_lib"),
///     ],
///     Path::new("/home/artichoke"),
/// )?;
/// # Some(())
/// # }
/// # example().unwrap();
/// ```
///
/// [require]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
/// [explicit relative]: is_explicit_relative
/// [resolves to the same file]: same_file
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "disk")))]
pub struct Disk {
    load_path: Vec<PathBuf>,
}

impl Disk {
    /// Create a new native file system loader that searches the file system for
    /// Ruby sources with an empty `$LOAD_PATH`.
    ///
    /// A disk loader with an empty `$LOAD_PATH` can only load sources by
    /// absolute paths or relative to the process's [current working directory]
    /// if an [explicit relative path] is given.
    ///
    /// The resolved load paths are mutable; `$LOAD_PATH` can be modified at
    /// runtime by Ruby code as the VM executes. See [`load_path`] and
    /// [`set_load_path`] for reading and modifying a disk loader's load path.
    ///
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::loaders::Disk;
    ///
    /// let loader = Disk::new();
    /// ```
    ///
    /// [`load_path`]: Self::load_path
    /// [`set_load_path`]: Self::set_load_path
    /// [current working directory]: env::current_dir
    /// [explicit relative path]: is_explicit_relative
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { load_path: Vec::new() }
    }

    /// Create a new native file system loader that searches the file system for
    /// Ruby sources at the paths specified by the given `$LOAD_PATH` from the
    /// Ruby global variable.
    ///
    /// The resolved load paths are mutable; `$LOAD_PATH` can be modified at
    /// runtime by Ruby code as the VM executes. See [`load_path`] and
    /// [`set_load_path`] for reading and modifying a disk loader's load path.
    ///
    /// If any of the paths in the `$LOAD_PATH` global variable are not absolute
    /// paths, they are absolutized relative to the current process's [current
    /// working directory] at the time the load path is set or modified.
    ///
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// This method returns [`None`] if the current working directory cannot be
    /// retrieved.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::path::{Path, PathBuf};
    /// use mezzaluna_feature_loader::loaders::Disk;
    ///
    /// # fn example() -> Option<()> {
    /// // Search `/home/artichoke/src` first, only attempting to search
    /// // `/usr/share/artichoke` if no file is found in `/home/artichoke/src`.
    /// //
    /// // The relative path `./_lib` is resolved relative to the current working
    /// // directory.
    /// let loader = Disk::with_load_path(
    ///     [
    ///         PathBuf::from("/home/artichoke/src"),
    ///         PathBuf::from("/usr/share/artichoke"),
    ///         PathBuf::from("_lib"),
    ///     ]
    /// )?;
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
    /// [`load_path`]: Self::load_path
    /// [`set_load_path`]: Self::set_load_path
    /// [current working directory]: env::current_dir
    #[inline]
    #[must_use]
    pub fn with_load_path(load_path: impl IntoIterator<Item = PathBuf>) -> Option<Self> {
        let cwd = env::current_dir().ok()?;
        Self::with_load_path_and_cwd(load_path, &cwd)
    }

    /// Create a new native file system loader that searches the file system for
    /// Ruby sources at the paths specified by the given `load_path` platform
    /// strings.
    ///
    /// The resolved load paths are mutable; `$LOAD_PATH` can be modified at
    /// runtime by Ruby code as the VM executes. See [`load_path`] and
    /// [`set_load_path`] for reading and modifying a disk loader's load path.
    ///
    /// If any of the paths in the `$LOAD_PATH` global variable are not absolute
    /// paths, they are absolutized relative to the current process's [current
    /// working directory] at the time the load path is set or modified.
    ///
    /// This source loader grants access to the host file system. This loader
    /// does not support native extensions.
    ///
    /// This method returns [`None`] if the given `cwd` is not an absolute path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::path::{Path, PathBuf};
    /// use mezzaluna_feature_loader::loaders::Disk;
    ///
    /// # fn example() -> Option<()> {
    /// // Search `/home/artichoke/src` first, only attempting to search
    /// // `/usr/share/artichoke` if no file is found in `/home/artichoke/src`.
    /// //
    /// // The relative path `./_lib` is resolved relative to the given working
    /// // directory.
    /// let loader = Disk::with_load_path_and_cwd(
    ///     [
    ///         PathBuf::from("/home/artichoke/src"),
    ///         PathBuf::from("/usr/share/artichoke"),
    ///         PathBuf::from("_lib"),
    ///     ],
    ///     Path::new("/home/artichoke"),
    /// )?;
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
    /// [`load_path`]: Self::load_path
    /// [`set_load_path`]: Self::set_load_path
    /// [current working directory]: env::current_dir
    #[inline]
    #[must_use]
    pub fn with_load_path_and_cwd(load_path: impl IntoIterator<Item = PathBuf>, cwd: &Path) -> Option<Self> {
        if !cwd.is_absolute() {
            return None;
        }

        let load_path = load_path
            .into_iter()
            .map(|load_path| cwd.join(load_path))
            .collect::<Vec<_>>();

        Some(Self { load_path })
    }

    /// Check whether `path` points to a file in the backing file system and
    /// return a file [`Handle`] if it exists.
    ///
    /// Returns [`Some`] if the file system object pointed to by `path` exists.
    /// If `path` is relative, it is joined to each path in the `$LOAD_PATH`
    /// environment variable at the time this loader was initialized. If `path`
    /// is an [explicit relative path], it is joined with the [current working
    /// directory].
    ///
    /// This method is infallible and will return [`None`] for non-existent
    /// paths or if the current working directory cannot be resolved when given
    /// an explicit relative path.
    ///
    /// [explicit relative path]: is_explicit_relative
    /// [current working directory]: env::current_dir
    #[inline]
    #[must_use]
    pub fn resolve_file(&self, path: &Path) -> Option<Handle> {
        // Absolute paths do not need to be resolved against the load paths.
        if path.is_absolute() {
            if let Ok(handle) = Handle::from_path(path) {
                return Some(handle);
            }
            return None;
        }

        // Explicit relative paths are loaded from the current directory only.
        if is_explicit_relative(path) {
            let cwd = env::current_dir().ok()?;
            let path = cwd.join(path);
            if let Ok(handle) = Handle::from_path(path) {
                return Some(handle);
            }
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
    /// use std::path::{Path, PathBuf};
    /// use mezzaluna_feature_loader::loaders::Disk;
    ///
    /// # fn example() -> Option<()> {
    /// let loader = Disk::with_load_path_and_cwd(
    ///     [
    ///         PathBuf::from("/home/artichoke/src"),
    ///         PathBuf::from("/usr/share/artichoke"),
    ///         PathBuf::from("_lib"),
    ///     ],
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

    /// Replace the loader's current `$LOAD_PATH`.
    ///
    /// This method returns the loader's previous `$LOAD_PATH`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use mezzaluna_feature_loader::loaders::Disk;
    ///
    /// # fn example() -> Option<()> {
    /// let mut loader = Disk::with_load_path_and_cwd(
    ///     [
    ///         PathBuf::from("/home/artichoke/src"),
    ///         PathBuf::from("/usr/share/artichoke"),
    ///         PathBuf::from("_lib"),
    ///     ],
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
    ///
    /// let old_load_path =
    ///     loader.set_load_path([PathBuf::from("libpath")], Path::new("/home/app"));
    /// assert_eq!(
    ///     old_load_path,
    ///     &[
    ///         Path::new("/home/artichoke/src"),
    ///         Path::new("/usr/share/artichoke"),
    ///         Path::new("/home/artichoke/_lib")
    ///     ]
    /// );
    /// assert_eq!(loader.load_path(), [Path::new("/home/app/libpath")]);
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[inline]
    pub fn set_load_path(&mut self, load_path: impl IntoIterator<Item = PathBuf>, cwd: &Path) -> Vec<PathBuf> {
        let load_path = load_path.into_iter().map(|load_path| cwd.join(load_path)).collect();
        mem::replace(&mut self.load_path, load_path)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn test_load_path_is_set_on_construction() {
        let loader = Disk::new();
        assert!(loader.load_path().is_empty());

        let loader = Disk::with_load_path([
            PathBuf::from("/home/artichoke/src"),
            PathBuf::from("/usr/share/artichoke"),
            PathBuf::from("_lib"),
        ])
        .unwrap();

        assert_eq!(loader.load_path().len(), 3);

        let mut iter = loader.load_path().iter();
        assert_eq!(iter.next().unwrap(), Path::new("/home/artichoke/src"));
        assert_eq!(iter.next().unwrap(), Path::new("/usr/share/artichoke"));
        assert_eq!(iter.next().unwrap(), &env::current_dir().unwrap().join("_lib"));
        assert_eq!(iter.next(), None);

        let loader = Disk::with_load_path_and_cwd(
            [
                PathBuf::from("/home/artichoke/src"),
                PathBuf::from("/usr/share/artichoke"),
                PathBuf::from("_lib"),
            ],
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
        let loader = Disk::with_load_path_and_cwd(
            [
                PathBuf::from("/home/artichoke/src"),
                PathBuf::from("/usr/share/artichoke"),
                PathBuf::from("_lib"),
            ],
            Path::new("xyz"),
        );
        assert!(loader.is_none());
    }

    #[test]
    fn test_empty_load_path_is_some() {
        let loader = Disk::with_load_path_and_cwd([], Path::new("/test/xyz")).unwrap();
        assert!(loader.load_path().is_empty());

        let loader = Disk::with_load_path([]).unwrap();
        assert!(loader.load_path().is_empty());
    }
}
