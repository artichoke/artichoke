//! A Ruby source loader that resolves sources from an in-memory virtual file
//! system.

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A Ruby source code loader that loads sources directly from an in-memory
/// virtual file system.
///
/// The memory loader has a hard-coded load path and only supports loading paths
/// that are relative to or absolute within this loader's [load path].
///
/// # Examples
///
/// ```
/// use mezzaluna_feature_loader::loaders::Memory;
///
/// let mut loader = Memory::new();
/// ```
///
/// [load path]: Self::load_path
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    sources: HashMap<PathBuf, Cow<'static, [u8]>>,
}

impl Memory {
    /// Create a new in-memory file system loader that loads sources that are
    /// registered with the feature loading system from a hash map.
    ///
    /// In-memory sources can be registered during interpreter boot or from
    /// native extensions.
    ///
    /// This loader has a fixed [load path] that may vary across targets.
    ///
    /// This source loader does NOT grant access to the host file system. The
    /// `Memory` loader does not support native extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::loaders::Memory;
    ///
    /// let mut loader = Memory::new();
    /// assert_eq!(loader.capacity(), 0);
    /// ```
    ///
    /// [load path]: Self::load_path
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    /// Create a new in-memory file system loader that loads sources that are
    /// registered with the feature loading system from a hash map with the
    /// given capacity.
    ///
    /// In-memory sources can be registered during interpreter boot or from
    /// native extensions.
    ///
    /// This loader has a fixed [load path] that may vary across targets.
    ///
    /// This source loader does NOT grant access to the host file system. The
    /// `Memory` loader does not support native extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::loaders::Memory;
    ///
    /// let mut loader = Memory::with_capacity(10);
    /// assert!(loader.capacity() >= 10);
    /// ```
    ///
    /// [load path]: Self::load_path
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sources: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of sources the loader can hold without reallocating.
    ///
    /// This number is a lower bound; the memory loader might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::loaders::Memory;
    ///
    /// let mut loader = Memory::with_capacity(10);
    /// assert!(loader.capacity() >= 10);
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.sources.capacity()
    }

    /// Check whether `path` points to a file in the backing file system and
    /// return a byte slice of file contents if it exists.
    ///
    /// Returns [`Some`] if the in-memory file system object pointed to by
    /// `path` exists.
    ///
    /// This method is infallible and will return [`None`] for non-existent
    /// paths or if the given path is outside of this loader's [load path].
    ///
    /// [load path]: Self::load_path
    #[inline]
    #[must_use]
    pub fn resolve_file(&self, path: &Path) -> Option<&[u8]> {
        // Absolute paths do not need to be resolved against the load paths.
        if let Ok(path) = path.strip_prefix(self.load_path()) {
            if let Some(bytes) = self.sources.get(path) {
                return Some(&**bytes);
            }
            return None;
        }
        let bytes = self.sources.get(path)?;
        Some(&**bytes)
    }

    /// Insert byte content into the in-memory feature store at the given path.
    ///
    /// # Panics
    ///
    /// If the given path is an absolute path outside of this loader's [load
    /// path], this function will panic.
    ///
    /// If the given path has already been inserted into the in-memory file
    /// system, this function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use mezzaluna_feature_loader::loaders::Memory;
    ///
    /// const STRSCAN: &[u8] = b"class StringScanner; end";
    ///
    /// let mut loader = Memory::new();
    /// loader.put_file_bytes(PathBuf::from("strscan.rb"), STRSCAN);
    ///
    /// let content = loader.resolve_file(Path::new("strscan.rb"));
    /// assert_eq!(content, Some(STRSCAN));
    /// ```
    ///
    /// [load path]: Self::load_path
    #[inline]
    pub fn put_file_bytes<T>(&mut self, mut path: PathBuf, content: T)
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let content = content.into();
        let load_path = self.load_path();
        if path.is_absolute() {
            path = path
                .strip_prefix(load_path)
                .expect("In-memory feature loader given absolute path outside of load path")
                .to_path_buf();
        }
        match self.sources.entry(path) {
            Entry::Occupied(entry) => panic!(
                "Duplicate insert into in-memory feature loader at '{}'",
                entry.key().display()
            ),
            Entry::Vacant(entry) => entry.insert(content),
        };
    }

    /// Insert string content into the in-memory feature store at the given
    /// path.
    ///
    /// # Panics
    ///
    /// If the given path is an absolute path outside of this loader's [load
    /// path], this function will panic.
    ///
    /// If the given path has already been inserted into the in-memory file
    /// system, this function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use mezzaluna_feature_loader::loaders::Memory;
    ///
    /// const STRSCAN: &str = "class StringScanner; end";
    ///
    /// let mut loader = Memory::new();
    /// loader.put_file_str(PathBuf::from("strscan.rb"), STRSCAN);
    ///
    /// let content = loader.resolve_file(Path::new("strscan.rb"));
    /// assert_eq!(content, Some(STRSCAN.as_bytes()));
    /// ```
    ///
    /// [load path]: Self::load_path
    #[inline]
    pub fn put_file_str<T>(&mut self, path: PathBuf, content: T)
    where
        T: Into<Cow<'static, str>>,
    {
        let content = match content.into() {
            Cow::Borrowed(data) => Cow::Borrowed(data.as_bytes()),
            Cow::Owned(data) => Cow::Owned(data.into_bytes()),
        };
        self.put_file_bytes(path, content);
    }

    /// Return a reference to the loader's current `$LOAD_PATH`.
    ///
    /// # Platform Compatibility
    ///
    /// On Windows systems, this method returns a different value than all other
    /// platforms.
    ///
    /// # Examples
    ///
    /// On non-Windows systems, this method returns:
    ///
    /// ```
    /// use std::path::Path;
    /// use mezzaluna_feature_loader::loaders::Memory;
    ///
    /// let loader = Memory::new();
    /// # #[cfg(not(windows))]
    /// assert_eq!(
    ///     loader.load_path(),
    ///     Path::new("/artichoke/virtual_root/src/lib")
    /// );
    /// ```
    ///
    /// On Windows, this method returns a different path:
    ///
    /// ```
    /// use std::path::Path;
    /// use mezzaluna_feature_loader::loaders::Memory;
    ///
    /// let loader = Memory::new();
    /// # #[cfg(windows)]
    /// assert_eq!(
    ///     loader.load_path(),
    ///     Path::new("c:/artichoke/virtual_root/src/lib")
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn load_path(&self) -> &'static Path {
        if cfg!(windows) {
            Path::new("c:/artichoke/virtual_root/src/lib")
        } else {
            Path::new("/artichoke/virtual_root/src/lib")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    #[cfg(windows)]
    fn test_windows_load_path() {
        let loader = Memory::new();
        assert_eq!(loader.load_path(), Path::new("c:/artichoke/virtual_root/src/lib"));
    }

    #[test]
    #[cfg(not(windows))]
    fn test_not_windows_load_path() {
        let loader = Memory::new();
        assert_eq!(loader.load_path(), Path::new("/artichoke/virtual_root/src/lib"));
    }
}
