use core::ptr::{self, NonNull};
use std::collections::hash_set;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::ptr::RcBox;

pub(crate) struct Links<T: ?Sized> {
    pub registry: HashSet<Link<T>>,
}

impl<T: ?Sized> Links<T> {
    pub fn contains(&self, other: &Link<T>) -> bool {
        self.registry.iter().any(|link| link.0 == other.0)
    }

    pub fn insert(&mut self, other: Link<T>) {
        if !(&*self).contains(&other) {
            self.registry.insert(other);
        }
    }

    pub fn clear(&mut self) {
        self.registry.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.registry.is_empty()
    }

    pub fn len(&self) -> usize {
        self.registry.len()
    }

    pub fn iter(&self) -> hash_set::Iter<Link<T>> {
        self.registry.iter()
    }
}

impl<T: ?Sized> Clone for Links<T> {
    fn clone(&self) -> Self {
        Self {
            registry: self.registry.clone(),
        }
    }
}

impl<T: ?Sized> Default for Links<T> {
    fn default() -> Self {
        Self {
            registry: HashSet::default(),
        }
    }
}

pub(crate) struct Link<T: ?Sized>(pub NonNull<RcBox<T>>);

impl<T: ?Sized> Copy for Link<T> {}

impl<T: ?Sized> Clone for Link<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized> PartialEq for Link<T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0.as_ptr(), other.0.as_ptr())
    }
}

impl<T: ?Sized> Eq for Link<T> {}

impl<T: ?Sized> Hash for Link<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
