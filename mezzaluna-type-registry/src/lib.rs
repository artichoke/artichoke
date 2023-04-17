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
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Construct a new registry with the given `capacity`.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
}

impl<T, S> Registry<T, S> {
    /// Construct a new registry with the given `hash_builder`.
    #[must_use]
    pub fn with_hasher(hash_builder: S) -> Self {
        Self(HashMap::with_hasher(hash_builder))
    }

    /// Construct a new registry with the given `capacity` and `hash_builder`.
    #[must_use]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self(HashMap::with_capacity_and_hasher(capacity, hash_builder))
    }

    /// Returns the number of type specs the registry can hold without
    /// reallocating.
    ///
    /// This number is a lower bound; the registry might be able to hold more,
    /// but is guaranteed to be able to hold at least this many.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// An iterator of all type specs stored in the registry in arbitrary order.
    #[must_use]
    pub fn type_specs(&self) -> TypeSpecs<'_, T> {
        TypeSpecs(self.0.values())
    }

    /// Returns the number of type specs in the registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the registry does not contain any type specs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to the registry's [`BuildHasher`].
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
    #[must_use]
    pub fn get<K>(&self) -> Option<&T>
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        let value = self.0.get(&key)?;
        Some(value)
    }

    /// Reserves `capacity` for at least additional more elements to be inserted
    /// in the `Registry`. The collection may reserve more space to avoid
    /// frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Tries to reserve capacity for at least additional more elements to be
    /// inserted in the `Registry`. The collection may reserve more space to
    /// avoid frequent reallocations.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }

    /// Shrinks the capacity of the map as much as possible. It will drop down
    /// as much as possible while maintaining the internal rules and possibly
    /// leaving some space in accordance with the resize policy.
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Shrinks the capacity of the registry with a lower bound.
    /// The capacity will remain at least as large as both the length and the
    /// supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity);
    }
}
