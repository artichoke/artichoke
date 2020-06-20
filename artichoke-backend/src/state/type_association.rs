//! A [`HashMap`] that uses Rust types as keys.
//!
//! Register data associated with a type (e.g. struct, enum, or primitive).
//!
//! `TypeAssociation` stores values in a [`Box`] to ensure pointers to values
//! in the registry are not invalidated.

use std::any::{Any, TypeId};
use std::collections::hash_map::{RandomState, Values};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::iter::FusedIterator;

/// An iterator of all associated values stored in the [`TypeAssociation`].
#[derive(Debug, Clone)]
pub struct AssociatedValues<'a, T>(Values<'a, TypeId, Box<T>>);

impl<'a, T> ExactSizeIterator for AssociatedValues<'a, T> {}

impl<'a, T> FusedIterator for AssociatedValues<'a, T> {}

impl<'a, T> Iterator for AssociatedValues<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.next()?;
        Some(value.as_ref())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }
}

/// A [`HashMap`] that uses Rust types as keys.
///
/// Associate data with a type (e.g. struct, enum, or primitive).
///
/// `TypeAssociation` is append-only; values cannot be removed or modified, only
/// replaced.
///
/// Artichoke uses this module to associate Rust implementations of Ruby Core
/// and standard library with their underlying [class spec](crate::class::Spec)
/// or [module spec](crate::module::Spec).
#[derive(Default, Debug)]
pub struct TypeAssociation<T, S = RandomState>(HashMap<TypeId, Box<T>, S>);

impl<T, S> PartialEq for TypeAssociation<T, S>
where
    T: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T, S> Eq for TypeAssociation<T, S>
where
    T: Eq,
    S: BuildHasher,
{
}

impl<'a, T, S> IntoIterator for &'a TypeAssociation<T, S> {
    type Item = &'a T;
    type IntoIter = AssociatedValues<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.associated_values()
    }
}

impl<T> TypeAssociation<T, RandomState> {
    /// Construct a new, empty `TypeAssociation`.
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Construct a new `TypeAssociation` with the given `capacity`.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
}

impl<T, S> TypeAssociation<T, S> {
    /// Construct a new `TypeAssociation` with the given `hash_builder`.
    #[must_use]
    pub fn with_hasher(hash_builder: S) -> Self {
        Self(HashMap::with_hasher(hash_builder))
    }

    /// Construct a new `TypeAssociation` with the given `capacity` and
    /// `hash_builder`.
    #[must_use]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self(HashMap::with_capacity_and_hasher(capacity, hash_builder))
    }

    /// Returns the number of elements the registry can hold without
    /// reallocating.
    ///
    /// This number is a lower bound; the `TypeAssociation` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// An iterator of all associated values stored in the [`TypeAssociation`] in
    /// arbitrary order.
    #[must_use]
    pub fn associated_values(&self) -> AssociatedValues<'_, T> {
        AssociatedValues(self.0.values())
    }

    /// Returns the number of elements in the registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the registry contains no elements.
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

impl<T, S> TypeAssociation<T, S>
where
    S: BuildHasher,
{
    /// Returns true if the registry contains a value for the specified type.
    #[must_use]
    pub fn contains<K>(&self) -> bool
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        self.0.contains_key(&key)
    }

    /// Inserts a type-value pair into the map.
    ///
    /// If the registry did not have this key present, [`None`] is returned.
    ///
    /// If the registry did have this key present, the value is updated, and the
    /// old value is returned.
    pub fn insert<K>(&mut self, spec: Box<T>) -> Option<Box<T>>
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        self.0.insert(key, spec)
    }

    /// Returns a reference to the value corresponding to the type key.
    #[must_use]
    pub fn get<K>(&self) -> Option<&T>
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        let value = self.0.get(&key)?;
        Some(value.as_ref())
    }

    /// Reserves `capacity` for at least additional more elements to be inserted
    /// in the `TypeAssociation`. The collection may reserve more space to avoid
    /// frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    /// Shrinks the capacity of the map as much as possible. It will drop down
    /// as much as possible while maintaining the internal rules and possibly
    /// leaving some space in accordance with the resize policy.
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
}
