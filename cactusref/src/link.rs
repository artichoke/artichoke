use core::ptr::NonNull;
use core::slice;
use std::hash::{Hash, Hasher};

use crate::ptr::RcBox;
use crate::Reachable;

#[derive(Clone)]
pub(crate) struct Links<T: ?Sized + Reachable> {
    pub registry: Vec<Link<T>>,
}

impl<T: ?Sized + Reachable> Links<T> {
    pub fn contains(&self, other: &Link<T>) -> bool {
        self.registry.iter().any(|link| link.0 == other.0)
    }

    pub fn insert(&mut self, other: Link<T>) {
        if !(&*self).contains(&other) {
            self.registry.push(other);
        }
    }

    pub fn clear(&mut self) {
        self.registry.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.registry.is_empty()
    }

    pub fn iter(&self) -> slice::Iter<Link<T>> {
        self.registry.iter()
    }
}

impl<T: ?Sized + Reachable> Default for Links<T> {
    fn default() -> Self {
        Self {
            registry: Vec::default(),
        }
    }
}

pub(crate) struct Link<T: ?Sized + Reachable>(pub NonNull<RcBox<T>>);

impl<T: ?Sized + Reachable> Link<T> {
    #[inline]
    pub fn value(&self) -> &T {
        unsafe { &self.0.as_ref().value }
    }
}

impl<T: ?Sized + Reachable> Copy for Link<T> {}

impl<T: ?Sized + Reachable> Clone for Link<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized + Reachable> Hash for Link<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let object_id = self.value().object_id();
        object_id.hash(state);
    }
}

impl<T: ?Sized + Reachable> PartialEq for Link<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value().object_id() == other.value().object_id()
    }
}

impl<T: ?Sized + Reachable> Eq for Link<T> {}
