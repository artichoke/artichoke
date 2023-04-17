#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::manual_let_else)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! A registry for "type spec" values that uses types as keys.
//!
//! This data structure is used for associating data type metadata with a Rust
//! type which can be used to ensure the lifetime of the associated metadata.
//!
//! # Example: `mrb_data_type`
//!
//! In the mruby C API, custom data types define a `mrb_data_type` struct which
//! contains the custom data type's module name and free function. The C API
//! requires that this struct live at least as long as the `mrb_state`.
//! Typically, the `mrb_data_type` is `static`.
//!
//! ```c
//! static const struct mrb_data_type mrb_time_type = { "Time", mrb_free };
//! ```
//!
//! The registry resembles an append-only [`HashMap`].
//!
//! The registry stores values behind a [`Box`] pointer to ensure pointers to
//! the interior of the spec, like [`CString`](std::ffi::CString) fields, are
//! not invalidated as the underlying storage reallocates.

use std::any::{self, Any, TypeId};
use std::collections::hash_map::{HashMap, RandomState, Values};
use std::collections::TryReserveError;
use std::fmt;
use std::hash::BuildHasher;
use std::iter::FusedIterator;

/// An iterator of all type specs stored in the [`Registry`].
///
/// See the [`type_specs`] method for more details.
///
/// [`type_specs`]: Registry::type_specs
#[derive(Debug, Clone)]
pub struct TypeSpecs<'a, T>(Values<'a, TypeId, Box<T>>);

impl<'a, T> ExactSizeIterator for TypeSpecs<'a, T> {}

impl<'a, T> FusedIterator for TypeSpecs<'a, T> {}

impl<'a, T> Iterator for TypeSpecs<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.next()?;
        Some(value.as_ref())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

/// A registry for "type spec" values that uses types as keys.
///
/// This data structure is used for associating data type metadata with a Rust
/// type which can be used to ensure the lifetime of the associated metadata.
///
/// # Example: `mrb_data_type`
///
/// In the mruby C API, custom data types define a `mrb_data_type` struct which
/// contains the custom data type's module name and free function. The C API
/// requires that this struct live at least as long as the `mrb_state`.
/// Typically, the `mrb_data_type` is `static`.
///
/// ```c
/// static const struct mrb_data_type mrb_time_type = { "Time", mrb_free };
/// ```
///
/// The registry resembles an append-only [`HashMap`].
///
/// The registry stores values behind a [`Box`] pointer to ensure pointers to
/// the interior of the spec, like [`CString`] fields, are not invalidated as
/// the underlying storage reallocates.
///
/// [`CString`]: std::ffi::CString
#[derive(Default, Debug)]
pub struct Registry<T, S = RandomState>(HashMap<TypeId, Box<T>, S>);

impl<T, S> PartialEq for Registry<T, S>
where
    T: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T, S> Eq for Registry<T, S>
where
    T: Eq,
    S: BuildHasher,
{
}

impl<'a, T, S> IntoIterator for &'a Registry<T, S> {
    type Item = &'a T;
    type IntoIter = TypeSpecs<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.type_specs()
    }
}

impl<T> Registry<T, RandomState> {
    /// Construct a new, empty registry.
    ///
    /// The registry is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    /// let mut reg: Registry<&'static str> = Registry::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Construct a new registry with at least the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(10);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
}

impl<T, S> Registry<T, S> {
    /// Construct a new registry with the given `hash_builder`.
    ///
    /// The created registry has the default initial capacity.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and is designed
    /// to allow registries to be resistant to attacks that cause many collisions
    /// and very poor performance. Setting it manually using this function can
    /// expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the registry to be useful, see its documentation for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let s = RandomState::new();
    /// let mut reg = Registry::with_hasher(s);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// ```
    #[must_use]
    pub fn with_hasher(hash_builder: S) -> Self {
        Self(HashMap::with_hasher(hash_builder))
    }

    /// Construct a new registry with at least the specified capacity, using
    /// `hasher` to hash the types.
    ///
    /// The registry will be able to hold at least `capacity` elements without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, the registry will not allocate.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and is designed
    /// to allow registries to be resistant to attacks that cause many collisions
    /// and very poor performance. Setting it manually using this function can
    /// expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the registry to be useful, see its documentation for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let s = RandomState::new();
    /// let mut reg = Registry::with_capacity_and_hasher(10, s);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// ```
    #[must_use]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self(HashMap::with_capacity_and_hasher(capacity, hash_builder))
    }

    /// Returns the number of type specs the registry can hold without
    /// reallocating.
    ///
    /// This number is a lower bound; the registry might be able to hold more,
    /// but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    /// let reg: Registry<&'static str> = Registry::with_capacity(100);
    /// assert!(reg.capacity() >= 100);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// An iterator of all type specs stored in the registry in arbitrary order.
    /// The iterator element type is `&'a T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(10);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// reg.insert::<Vec<u8>>(Box::new("String"));
    ///
    /// for spec in reg.type_specs() {
    ///     println!("{spec}");
    /// }
    /// ```
    #[must_use]
    pub fn type_specs(&self) -> TypeSpecs<'_, T> {
        TypeSpecs(self.0.values())
    }

    /// Returns the number of type specs in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(10);
    /// assert_eq!(reg.len(), 0);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// assert_eq!(reg.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the registry does not contain any type specs.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(10);
    /// assert!(reg.is_empty());
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// assert!(!reg.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to the registry's [`BuildHasher`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let s = RandomState::new();
    /// let reg: Registry<&'static str> = Registry::with_hasher(s);
    /// let hasher: &RandomState = reg.hasher();
    /// ```
    #[must_use]
    pub fn hasher(&self) -> &S {
        self.0.hasher()
    }
}

impl<T, S> Registry<T, S>
where
    T: fmt::Debug,
    S: BuildHasher,
{
    /// Returns true if the registry contains a type spec for the specified
    /// type.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(10);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// assert_eq!(reg.contains::<i32>(), true);
    /// assert_eq!(reg.contains::<Vec<u8>>(), false);
    /// ```
    #[must_use]
    pub fn contains<K>(&self) -> bool
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        self.0.contains_key(&key)
    }

    /// Inserts a type-type spec pair into the registry.
    ///
    /// This operation will only succeed if `K` has never been inserted into the
    /// registry.
    ///
    /// # Panics
    ///
    /// If `insert` has previously been called with type `K`, this function will
    /// panic. The registry is append-only and does not allow mutations.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(10);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// assert_eq!(reg.is_empty(), false);
    /// ```
    pub fn insert<K>(&mut self, spec: Box<T>)
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        if let Some(old_spec) = self.0.insert(key, spec) {
            panic!(
                "Attempted duplicate insert of {}. Registry is append-only. Previous spec: {:?}",
                any::type_name::<K>(),
                old_spec
            );
        }
    }

    /// Returns a reference to the type spec corresponding to the type key.
    ///
    /// If the type `K` has not been registered, [`None`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(10);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// assert_eq!(reg.get::<i32>(), Some(&"Numeric"));
    /// assert_eq!(reg.get::<Vec<u8>>(), None);
    /// ```
    #[must_use]
    pub fn get<K>(&self) -> Option<&T>
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        let value = self.0.get(&key)?;
        Some(value)
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the registry. The collection may reserve more space to speculatively
    /// avoid frequent reallocations. After calling `reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::new();
    /// reg.reserve(10);
    /// assert!(reg.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Tries to reserve capacity for at least `additional` more elements to be
    /// inserted in the registry. The collection may reserve more space to
    /// speculatively avoid frequent reallocations. After calling `try_reserve`,
    /// capacity will be greater than or equal to `self.len() + additional` if
    /// it returns `Ok(())`. Does nothing if capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::new();
    /// reg.try_reserve(10).expect("cannot OOM the doctest harness");
    /// assert!(reg.capacity() >= 10);
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }

    /// Shrinks the capacity of the registry as much as possible. It will drop
    /// down as much as possible while maintaining the internal rules and
    /// possibly leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(100);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// reg.insert::<Vec<u8>>(Box::new("String"));
    /// assert!(reg.capacity() >= 100);
    /// reg.shrink_to_fit();
    /// assert!(reg.capacity() >= 2);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Shrinks the capacity of the registry with a lower limit. It will drop
    /// down no lower than the supplied limit while maintaining the internal
    /// rules and possibly leaving some space in accordance with the resize
    /// policy.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use mezzaluna_type_registry::Registry;
    ///
    /// let mut reg: Registry<&'static str> = Registry::with_capacity(100);
    /// reg.insert::<i32>(Box::new("Numeric"));
    /// reg.insert::<Vec<u8>>(Box::new("String"));
    /// assert!(reg.capacity() >= 100);
    /// reg.shrink_to(10);
    /// assert!(reg.capacity() >= 10);
    /// reg.shrink_to(0);
    /// assert!(reg.capacity() >= 2);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity);
    }
}

#[cfg(test)]
mod tests {
    use super::Registry;

    #[test]
    #[should_panic = "Attempted duplicate insert of i32. Registry is append-only. Previous spec: \"Numeric\""]
    fn registry_panics_on_duplicate_insert() {
        let mut reg = Registry::new();
        reg.insert::<i32>(Box::new("Numeric"));
        reg.insert::<i32>(Box::new("Integer"));
    }
}
