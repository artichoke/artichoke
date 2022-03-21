use std::path::{Path, PathBuf};

#[cfg(feature = "disk")]
use same_file::Handle;

#[cfg(feature = "disk")]
mod disk;
mod memory;

#[derive(Debug, Hash, PartialEq, Eq)]
enum FeatureType {
    #[cfg(feature = "disk")]
    Disk(disk::Feature),
    Memory(memory::Feature),
}

impl FeatureType {
    #[cfg(feature = "disk")]
    #[cfg_attr(docsrs, doc(cfg(feature = "disk")))]
    pub fn with_handle_and_path(handle: Handle, path: PathBuf) -> Self {
        let inner = disk::Feature::with_handle_and_path(handle, path);
        Self::Disk(inner)
    }

    pub fn with_in_memory_path(path: PathBuf) -> Self {
        let inner = memory::Feature::with_path(path);
        Self::Memory(inner)
    }

    pub fn path(&self) -> &Path {
        match self {
            #[cfg(feature = "disk")]
            Self::Disk(inner) => inner.path(),
            Self::Memory(inner) => inner.path(),
        }
    }
}

/// A Ruby source ("feature") that has been loaded into an interpreter.
///
/// Features can either be loaded from disk or from memory.
///
/// Features are identified by the (potentially relative) path used when loading
/// the file for the first time. Features loaded from disk are deduplicated
/// by their real position on the underlying file system (i.e. their device and
/// inode).
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Feature {
    inner: FeatureType,
}

impl Feature {
    /// Create a new feature from a file handle and path.
    #[must_use]
    #[cfg(feature = "disk")]
    #[cfg_attr(docsrs, doc(cfg(feature = "disk")))]
    pub fn with_handle_and_path(handle: Handle, path: PathBuf) -> Self {
        let inner = FeatureType::with_handle_and_path(handle, path);
        Self { inner }
    }

    /// Create a new feature from a virtual in-memory path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// use mezzaluna_feature_loader::Feature;
    ///
    /// let feature = Feature::with_in_memory_path("/src/_lib/test.rb".into());
    /// assert_eq!(feature.path(), Path::new("/src/_lib/test.rb"));
    /// ```
    #[must_use]
    pub fn with_in_memory_path(path: PathBuf) -> Self {
        let inner = FeatureType::with_in_memory_path(path);
        Self { inner }
    }

    /// Get the path associated with this feature.
    ///
    /// The path returned by this method is not guaranteed to be the same as
    /// the path returned by [`LoadedFeatures::features`] since features may
    /// be deduplicated by their physical location in the underlying loaders.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// use mezzaluna_feature_loader::Feature;
    ///
    /// let feature = Feature::with_in_memory_path("/src/_lib/test.rb".into());
    /// assert_eq!(feature.path(), Path::new("/src/_lib/test.rb"));
    /// ```
    ///
    /// [`LoadedFeatures::features`]: crate::LoadedFeatures::features
    #[must_use]
    pub fn path(&self) -> &Path {
        self.inner.path()
    }
}
