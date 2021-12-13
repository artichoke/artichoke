//! A set of loaded Ruby source files based on a [`Vec`] and [`HashSet`].
//!
//! This module exposes an append-only, insertion order-preserving, set-like
//! container for tracking disk and in-memory Ruby sources as they are
//! evaluated on a Ruby interpreter using [`require`] and [`require_relative`].
//!
//! See [`LoadedFeatures`] for more documentation on how to use the types in
//! this module.
//!
//! [`require`]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
//! [`require_relative`]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require_relative

use std::collections::hash_map::RandomState;
use std::collections::hash_set::{self, HashSet};
use std::hash::{BuildHasher, Hash};
use std::iter::FusedIterator;
use std::path::{Path, PathBuf};
use std::slice;

#[cfg(feature = "disk")]
use same_file::Handle;

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
    pub fn disk(handle: Handle, path: PathBuf) -> Self {
        let disk = disk::Feature::with_handle_and_path(handle, path);
        let inner = FeatureType::Disk(disk);
        Self { inner }
    }

    /// Create a new feature from a virtual in-memory path.
    #[must_use]
    pub fn memory(path: PathBuf) -> Self {
        let memory = memory::Feature::with_path(path);
        let inner = FeatureType::Memory(memory);
        Self { inner }
    }

    /// Get the path associated with this feature.
    ///
    /// If a feature is loaded multiple times, the first path used to load it is
    /// returned.
    #[must_use]
    pub fn path(&self) -> &Path {
        match &self.inner {
            #[cfg(feature = "disk")]
            FeatureType::Disk(disk) => disk.path(),
            FeatureType::Memory(memory) => memory.path(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum FeatureType {
    #[cfg(feature = "disk")]
    Disk(disk::Feature),
    Memory(memory::Feature),
}

#[cfg(feature = "disk")]
mod disk {
    use std::hash::{Hash, Hasher};
    use std::path::{Path, PathBuf};

    use same_file::Handle;

    #[derive(Debug)]
    pub struct Feature {
        handle: Handle,
        path: PathBuf,
    }

    impl Feature {
        pub fn with_handle_and_path(handle: Handle, path: PathBuf) -> Self {
            Self { handle, path }
        }

        pub fn path(&self) -> &Path {
            &*self.path
        }
    }

    impl Hash for Feature {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.handle.hash(state);
        }
    }

    impl PartialEq for Feature {
        fn eq(&self, other: &Self) -> bool {
            self.handle == other.handle
        }
    }

    impl Eq for Feature {}
}

mod memory {
    use std::path::{Path, PathBuf};

    #[derive(Debug, Hash, PartialEq, Eq)]
    pub struct Feature {
        path: PathBuf,
    }

    impl Feature {
        pub fn with_path(path: PathBuf) -> Self {
            Self { path }
        }

        pub fn path(&self) -> &Path {
            &*self.path
        }
    }
}

/// A set of all sources loaded by a Ruby interpreter with [`require`] and
/// [`require_relative`].
///
/// In Ruby, when loading files with `require` and `require_relative`, the
/// constants defined in them have global scope. Ruby keeps track of loaded
/// sources in its interpreter state to ensure files are not `require`'d
/// multiple times.
///
/// Ruby refers to files tracked in this way as _features_. The set of loaded
/// features are stored in a global variable called `$"` (which is aliased to
/// `$LOADED_FEATURES`).
///
/// `$LOADED_FEATURES` is an append only set. Disk-based features are
/// deduplicated by their real position on the underlying file system (i.e. their
/// device and inode).
///
/// [`require`]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
/// [`require_relative`]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require_relative
#[derive(Debug)]
pub struct LoadedFeatures<S = RandomState> {
    features: HashSet<Feature, S>,
    paths: Vec<PathBuf>,
}

impl Default for LoadedFeatures {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadedFeatures<RandomState> {
    /// Creates an empty `LoadedFeatures`.
    ///
    /// The set of features is initially created with a capacity of 0, so it
    /// will not allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::LoadedFeatures;
    /// let features = LoadedFeatures::new();
    ///
    /// assert!(features.is_empty());
    /// assert_eq!(features.capacity(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let features = HashSet::new();
        let paths = Vec::new();
        Self { features, paths }
    }

    /// Creates and empty `LoadedFeatures` with the specified capacity.
    ///
    /// The set of features will be able to hold at least `capacity` elements
    /// without reallocating. If `capacity` is 0, the feature set will not
    /// allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::LoadedFeatures;
    /// let features = LoadedFeatures::with_capacity(10);
    ///
    /// assert!(features.capacity() >= 10);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let features = HashSet::with_capacity(capacity);
        let paths = Vec::with_capacity(capacity);
        Self { features, paths }
    }
}

impl<S> LoadedFeatures<S> {
    /// Returns the number of elements the set of features can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::LoadedFeatures;
    /// let features = LoadedFeatures::with_capacity(100);
    ///
    /// assert!(features.capacity() >= 100);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        usize::min(self.features.capacity(), self.paths.capacity())
    }

    /// An iterator visiting all features in insertion order. The iterator
    /// element type is `&'a Path`.
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        let inner = self.paths.iter();
        Iter { inner }
    }

    /// An iterator visiting all features in arbitrary order. The iterator
    /// element type is `&'a Feature`.
    #[must_use]
    pub fn features(&self) -> Features<'_> {
        let inner = self.features.iter();
        Features { inner }
    }

    /// Returns the number of features in the set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Returns true if the set contains no features.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }

    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
        self.features.clear();
    }

    /// Creates a new empty feature set which will use the given hasher to hash
    /// keys.
    ///
    /// The feature set is also created with the default initial capacity.
    ///
    /// Warning: `hasher` is normally randomly generated, and is designed to
    /// allow `LoadedFeatures` to be resistant to attacks that cause many
    /// collisions and very poor performance. Setting it manually using this
    /// function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the `LoadedFeatures` to be useful, see its documentation for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use std::path::PathBuf;
    /// use mezzaluna_feature_loader::LoadedFeatures;
    ///
    /// let s = RandomState::new();
    /// let mut set = LoadedFeatures::with_hasher(s);
    /// set.insert_in_memory_feature(PathBuf::from("set.rb"));
    /// ```
    #[must_use]
    pub fn with_hasher(hasher: S) -> Self {
        let features = HashSet::with_hasher(hasher);
        let paths = Vec::new();
        Self { features, paths }
    }

    /// Creates a new empty feature set with the specified capacity which will
    /// use the given hasher to hash keys.
    ///
    /// The feature set will be able to hold at least `capacity` elements
    /// without reallocating. If `capacity` is 0, the feature set will not
    /// allocate.
    ///
    /// Warning: `hasher` is normally randomly generated, and is designed to
    /// allow `LoadedFeatures` to be resistant to attacks that cause many
    /// collisions and very poor performance. Setting it manually using this
    /// function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the `LoadedFeatures` to be useful, see its documentation for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use std::path::PathBuf;
    /// use mezzaluna_feature_loader::LoadedFeatures;
    ///
    /// let s = RandomState::new();
    /// let mut set = LoadedFeatures::with_capacity_and_hasher(10, s);
    /// set.insert_in_memory_feature(PathBuf::from("set.rb"));
    /// ```
    #[must_use]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        let features = HashSet::with_capacity_and_hasher(capacity, hasher);
        let paths = Vec::with_capacity(capacity);
        Self { features, paths }
    }

    /// Returns a reference to the feature set's [`BuildHasher`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use mezzaluna_feature_loader::LoadedFeatures;
    ///
    /// let s = RandomState::new();
    /// let set = LoadedFeatures::with_hasher(s);
    /// let hasher: &RandomState = set.hasher();
    /// ```
    #[must_use]
    pub fn hasher(&self) -> &S {
        self.features.hasher()
    }
}

impl<S> LoadedFeatures<S>
where
    S: BuildHasher,
{
    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the `LoadedFeatures`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::LoadedFeatures;
    /// let mut features = LoadedFeatures::new();
    /// features.reserve(10);
    /// assert!(features.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.features.reserve(additional);
        self.paths.reserve(additional);
    }

    /// Tries to reserve capacity for at least `additional` more elements to be
    /// inserted in the `LoadedFeatures`. The collection may reserve more space
    /// to avoid frequent reallocations.
    /// After calling `try_reserve`, capacity will be greater than or equal to
    /// `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_feature_loader::LoadedFeatures;
    /// let mut features = LoadedFeatures::new();
    /// features.try_reserve(10).expect("why is this OOMing on 10 bytes?");
    /// assert!(features.capacity() >= 10);
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        self.features.try_reserve(additional)?;
        self.paths.try_reserve(additional)?;

        Ok(())
    }

    /// Shrinks the capacity of the set as much as possible. It will drop down
    /// as much as possible while maintaining the internal rules and possibly
    /// leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use mezzaluna_feature_loader::LoadedFeatures;
    ///
    /// let mut features = LoadedFeatures::with_capacity(100);
    /// features.insert_in_memory_feature(PathBuf::from("set.rb"));
    /// features.insert_in_memory_feature(PathBuf::from("artichoke.rb"));
    /// assert!(features.capacity() >= 100);
    /// features.shrink_to_fit();
    /// assert!(features.capacity() >= 2);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.features.shrink_to_fit();
        self.paths.shrink_to_fit();
    }

    /// Returns true if the set contains a feature.
    ///
    /// Features loaded from disk are compared based on whether they point to
    /// the same file on the underlying file system. Features loaded from memory
    /// are compared by their paths.
    #[must_use]
    pub fn contains(&self, feature: &Feature) -> bool {
        self.features.contains(feature)
    }

    /// Add a feature to the set.
    ///
    /// # Panics
    ///
    /// Panics if the given feature is already loaded.
    pub fn insert(&mut self, feature: Feature) {
        let path = feature.path().to_owned();
        if !self.features.insert(feature) {
            panic!("duplicate feature inserted at {}", path.display());
        }
        self.paths.push(path);
    }

    /// Add a disk feature from its handle loaded from disk and its associated path.
    ///
    /// # Panics
    ///
    /// Panics if the given feature is already loaded.
    #[cfg(feature = "disk")]
    pub fn insert_on_disk_feature(&mut self, handle: Handle, path: PathBuf) {
        let feature = Feature::disk(handle, path);
        self.insert(feature);
    }

    /// Add a memory feature from its associated path.
    ///
    /// # Panics
    ///
    /// Panics if the given feature is already loaded.
    pub fn insert_in_memory_feature(&mut self, path: PathBuf) {
        let feature = Feature::memory(path);
        self.insert(feature);
    }
}

/// An iterator over the feature paths in a `LoadedFeatures`.
///
/// This struct is created by the [`iter`] method on [`LoadedFeatures`]. See its
/// documentation for more.
///
/// [`iter`]: LoadedFeatures::iter
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, PathBuf>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        Some(&*next)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let nth = self.inner.nth(n)?;
        Some(&*nth)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn last(self) -> Option<Self::Item> {
        let last = self.inner.last()?;
        Some(&*last)
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> FusedIterator for Iter<'a> {}

/// An iterator over the features in a `LoadedFeatures`.
///
/// This struct is created by the [`features`] method on [`LoadedFeatures`]. See
/// its documentation for more.
///
/// [`features`]: LoadedFeatures::features
#[derive(Debug, Clone)]
pub struct Features<'a> {
    inner: hash_set::Iter<'a, Feature>,
}

impl<'a> Iterator for Features<'a> {
    type Item = &'a Feature;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> ExactSizeIterator for Features<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> FusedIterator for Features<'a> {}
