use core::ptr::NonNull;
use std::hash::{Hash, Hasher};

use crate::ptr::RcBox;
use crate::Reachable;

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
