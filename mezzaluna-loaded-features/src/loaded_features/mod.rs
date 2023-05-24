//! A set to track loaded Ruby source paths based on a [`Vec`] and [`HashSet`].
//!
//! Ruby tracks which files and native extensions have been [required] in a
//! global variable called `$LOADED_FEATURES`, which is aliased to `$"`.
//! `$LOADED_FEATURES` is an `Array` of paths which point to these Ruby sources
//! and native extensions.
//!
//! This module exposes an append-only, insertion order-preserving, set-like
//! container for tracking disk and in-memory Ruby sources as they are
//! evaluated on a Ruby interpreter using [`require`] and [`require_relative`].
//!
//! See [`LoadedFeatures`] for more documentation on how to use the types in
//! this module.
//!
//! [required]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
//! [`require`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
//! [`require_relative`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require_relative

use core::hash::BuildHasher;
use std::collections::hash_map::RandomState;
use std::collections::hash_set::HashSet;
use std::collections::TryReserveError;
use std::path::PathBuf;

mod iter;

pub use iter::{Features, Iter};

use crate::Feature;

/// A set of all sources loaded by a Ruby interpreter with [`require`] and
/// [`require_relative`].
///
/// In Ruby, when loading files with `require` and `require_relative`, the
/// constants defined in them have global scope. Ruby keeps track of loaded
/// sources in its interpreter state to ensure files are not `require`'d
/// multiple times.
///
/// Ruby refers to files tracked in this way as _features_. The set of loaded
/// features are stored in a global variable called `$LOADED_FEATURES`, which is
/// aliased to `$"`.
///
/// `$LOADED_FEATURES` is an append only set. Disk-based features are
/// deduplicated by their real position on the underlying file system (i.e.
/// their device and inode).
///
/// Ruby uses a feature's presence in the loaded features set to determine
/// whether a require has side effects (i.e. a file can be required multiple
/// times but is only evaluated once).
///
/// # Examples
///
/// ```
/// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
///
/// let mut features = LoadedFeatures::new();
/// features.insert(Feature::with_in_memory_path("/src/_lib/test.rb".into()));
/// features.insert(Feature::with_in_memory_path("set.rb".into()));
/// features.insert(Feature::with_in_memory_path("artichoke.rb".into()));
///
/// for f in features.features() {
///     println!("Loaded feature at: {}", f.path().display());
/// }
///
/// features.shrink_to_fit();
/// ```
///
/// [`require`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require
/// [`require_relative`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require_relative
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
    /// use mezzaluna_loaded_features::LoadedFeatures;
    ///
    /// let features = LoadedFeatures::new();
    /// assert!(features.is_empty());
    /// assert_eq!(features.capacity(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let features = HashSet::new();
        let paths = Vec::new();
        Self { features, paths }
    }

    /// Creates an empty `LoadedFeatures` with the specified capacity.
    ///
    /// The set of features will be able to hold at least `capacity` elements
    /// without reallocating. If `capacity` is 0, the feature set will not
    /// allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::LoadedFeatures;
    ///
    /// let features = LoadedFeatures::with_capacity(10);
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
    /// use mezzaluna_loaded_features::LoadedFeatures;
    ///
    /// let features = LoadedFeatures::with_capacity(100);
    /// assert!(features.capacity() >= 100);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        usize::min(self.features.capacity(), self.paths.capacity())
    }

    /// An iterator visiting all features in insertion order. The iterator
    /// element type is `&'a Path`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::new();
    /// features.insert(Feature::with_in_memory_path("/src/_lib/test.rb".into()));
    /// features.insert(Feature::with_in_memory_path("set.rb".into()));
    /// features.insert(Feature::with_in_memory_path("artichoke.rb".into()));
    ///
    /// for path in features.iter() {
    ///     println!("Loaded feature at: {}", path.display());
    /// }
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        let inner = self.paths.iter();
        Iter { inner }
    }

    /// An iterator visiting all features in arbitrary order. The iterator
    /// element type is `&'a Feature`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::new();
    /// features.insert(Feature::with_in_memory_path("/src/_lib/test.rb".into()));
    /// features.insert(Feature::with_in_memory_path("set.rb".into()));
    /// features.insert(Feature::with_in_memory_path("artichoke.rb".into()));
    ///
    /// for f in features.features() {
    ///     println!("Loaded feature at: {}", f.path().display());
    /// }
    /// ```
    #[must_use]
    pub fn features(&self) -> Features<'_> {
        let inner = self.features.iter();
        Features { inner }
    }

    /// Returns the number of features in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::new();
    /// assert_eq!(features.len(), 0);
    ///
    /// features.insert(Feature::with_in_memory_path("/src/_lib/test.rb".into()));
    /// assert_eq!(features.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Returns true if the set contains no features.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::new();
    /// assert!(features.is_empty());
    ///
    /// features.insert(Feature::with_in_memory_path("/src/_lib/test.rb".into()));
    /// assert!(!features.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
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
    ///
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let s = RandomState::new();
    /// let mut features = LoadedFeatures::with_hasher(s);
    /// features.insert(Feature::with_in_memory_path("set.rb".into()));
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
    ///
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let s = RandomState::new();
    /// let mut features = LoadedFeatures::with_capacity_and_hasher(10, s);
    /// features.insert(Feature::with_in_memory_path("set.rb".into()));
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
    ///
    /// use mezzaluna_loaded_features::LoadedFeatures;
    ///
    /// let s = RandomState::new();
    /// let features = LoadedFeatures::with_hasher(s);
    /// let hasher: &RandomState = features.hasher();
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
    /// use mezzaluna_loaded_features::LoadedFeatures;
    ///
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
    /// to avoid frequent reallocations. After calling `try_reserve`, capacity
    /// will be greater than or equal to `self.len() + additional`. Does nothing
    /// if capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::LoadedFeatures;
    ///
    /// let mut features = LoadedFeatures::new();
    /// features
    ///     .try_reserve(10)
    ///     .expect("why is this OOMing on 10 features?");
    /// assert!(features.capacity() >= 10);
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
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
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::with_capacity(100);
    /// features.insert(Feature::with_in_memory_path("set.rb".into()));
    /// features.insert(Feature::with_in_memory_path("artichoke.rb".into()));
    ///
    /// assert!(features.capacity() >= 100);
    /// features.shrink_to_fit();
    /// assert!(features.capacity() >= 2);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.features.shrink_to_fit();
        self.paths.shrink_to_fit();
    }

    /// Shrinks the capacity of the set with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the
    /// supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::with_capacity(100);
    /// features.insert(Feature::with_in_memory_path("set.rb".into()));
    /// features.insert(Feature::with_in_memory_path("artichoke.rb".into()));
    ///
    /// assert!(features.capacity() >= 100);
    /// features.shrink_to(2);
    /// assert!(features.capacity() >= 2);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.features.shrink_to(min_capacity);
        self.paths.shrink_to(min_capacity);
    }

    /// Returns true if the set contains a feature.
    ///
    /// Features loaded from disk are compared based on whether they point to
    /// the same file on the underlying file system. Features loaded from memory
    /// are compared by their paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::new();
    /// let set_feature = Feature::with_in_memory_path("set.rb".into());
    ///
    /// assert!(!features.contains(&set_feature));
    ///
    /// features.insert(set_feature);
    /// assert_eq!(features.len(), 1);
    /// ```
    #[must_use]
    pub fn contains(&self, feature: &Feature) -> bool {
        self.features.contains(feature)
    }

    /// Add a feature to the set.
    ///
    /// # Panics
    ///
    /// Panics if the given feature is already loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_loaded_features::{Feature, LoadedFeatures};
    ///
    /// let mut features = LoadedFeatures::new();
    /// let set_feature = Feature::with_in_memory_path("set.rb".into());
    /// features.insert(set_feature);
    ///
    /// assert_eq!(features.len(), 1);
    /// ```
    pub fn insert(&mut self, feature: Feature) {
        let path = feature.path().to_owned();
        let feature_was_not_loaded = self.features.insert(feature);
        assert!(
            feature_was_not_loaded,
            "duplicate feature inserted at {}",
            path.display()
        );
        self.paths.push(path);
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{Feature, LoadedFeatures};

    #[test]
    #[should_panic(expected = "duplicate feature inserted at set.rb")]
    fn duplicate_memory_insert_panics() {
        let mut features = LoadedFeatures::new();
        features.insert(Feature::with_in_memory_path("set.rb".into()));
        features.insert(Feature::with_in_memory_path("set.rb".into()));
    }

    #[test]
    fn insert_multiple_memort_features() {
        let mut features = LoadedFeatures::new();
        features.insert(Feature::with_in_memory_path("set.rb".into()));
        features.insert(Feature::with_in_memory_path("hash.rb".into()));
        features.insert(Feature::with_in_memory_path("artichoke.rb".into()));

        assert_eq!(features.len(), 3);

        let paths = features.iter().collect::<Vec<_>>();
        assert_eq!(paths.len(), 3);
        assert_eq!(
            paths,
            &[Path::new("set.rb"), Path::new("hash.rb"), Path::new("artichoke.rb")]
        );
    }

    #[test]
    #[should_panic(expected = "duplicate feature inserted at Cargo.toml")]
    fn duplicate_disk_insert_panics() {
        use same_file::Handle;

        let mut features = LoadedFeatures::new();
        loop {
            let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
            let handle = Handle::from_path(&path).unwrap();
            features.insert(Feature::with_handle_and_path(
                handle,
                path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().to_owned(),
            ));
        }
    }

    // ```shell
    // $ echo 'puts __FILE__' > a.rb
    // $ irb
    // [3.2.2] > require './a.rb'
    // /Users/lopopolo/dev/artichoke/artichoke/a.rb
    // => true
    // [3.2.2] > require '../artichoke/a.rb'
    // => false
    // ```
    #[test]
    #[should_panic(expected = "duplicate feature inserted at src/../Cargo.toml")]
    fn duplicate_disk_insert_with_different_path_panics() {
        use same_file::Handle;

        let mut features = LoadedFeatures::new();

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let handle = Handle::from_path(&path).unwrap();
        features.insert(Feature::with_handle_and_path(
            handle,
            path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().to_owned(),
        ));

        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("..")
            .join("Cargo.toml");
        let handle = Handle::from_path(&path).unwrap();
        features.insert(Feature::with_handle_and_path(
            handle,
            path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().to_owned(),
        ));
    }

    #[test]
    fn insert_multiple_disk_features() {
        use same_file::Handle;

        let mut features = LoadedFeatures::new();

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let handle = Handle::from_path(&path).unwrap();
        features.insert(Feature::with_handle_and_path(
            handle,
            path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().to_owned(),
        ));

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("LICENSE");
        let handle = Handle::from_path(&path).unwrap();
        features.insert(Feature::with_handle_and_path(
            handle,
            path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().to_owned(),
        ));

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");
        let handle = Handle::from_path(&path).unwrap();
        features.insert(Feature::with_handle_and_path(
            handle,
            path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().to_owned(),
        ));

        assert_eq!(features.len(), 3);

        let paths = features.iter().collect::<Vec<_>>();
        assert_eq!(paths.len(), 3);
        assert_eq!(
            paths,
            &[Path::new("Cargo.toml"), Path::new("LICENSE"), Path::new("README.md")]
        );
    }

    #[test]
    fn iter_yields_paths_in_insertion_order() {
        let mut features = LoadedFeatures::new();
        features.insert(Feature::with_in_memory_path("a.rb".into()));
        features.insert(Feature::with_in_memory_path("b.rb".into()));
        features.insert(Feature::with_in_memory_path("c.rb".into()));
        features.insert(Feature::with_in_memory_path("d.rb".into()));
        features.insert(Feature::with_in_memory_path("e.rb".into()));
        features.insert(Feature::with_in_memory_path("f.rb".into()));
        features.insert(Feature::with_in_memory_path("g.rb".into()));

        assert_eq!(features.len(), 7);

        let paths = features.iter().collect::<Vec<_>>();
        assert_eq!(paths.len(), 7);
        assert_eq!(
            paths,
            &[
                Path::new("a.rb"),
                Path::new("b.rb"),
                Path::new("c.rb"),
                Path::new("d.rb"),
                Path::new("e.rb"),
                Path::new("f.rb"),
                Path::new("g.rb"),
            ]
        );
    }

    #[test]
    fn features_iter_yields_all_features() {
        let mut features = LoadedFeatures::new();
        features.insert(Feature::with_in_memory_path("a.rb".into()));
        features.insert(Feature::with_in_memory_path("b.rb".into()));
        features.insert(Feature::with_in_memory_path("c.rb".into()));
        features.insert(Feature::with_in_memory_path("d.rb".into()));
        features.insert(Feature::with_in_memory_path("e.rb".into()));
        features.insert(Feature::with_in_memory_path("f.rb".into()));
        features.insert(Feature::with_in_memory_path("g.rb".into()));

        assert_eq!(features.len(), 7);

        let mut feats = features.features().collect::<Vec<_>>();
        assert_eq!(feats.len(), 7);

        feats.sort_unstable_by_key(|f| f.path());
        let paths = feats.into_iter().map(Feature::path).collect::<Vec<_>>();
        assert_eq!(
            paths,
            &[
                Path::new("a.rb"),
                Path::new("b.rb"),
                Path::new("c.rb"),
                Path::new("d.rb"),
                Path::new("e.rb"),
                Path::new("f.rb"),
                Path::new("g.rb"),
            ]
        );
    }
}
