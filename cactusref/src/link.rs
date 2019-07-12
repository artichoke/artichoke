use core::ptr::NonNull;
use std::hash::{Hash, Hasher};

use crate::{CactusBox, Reachable};

pub(crate) struct CactusLinkRef<T: ?Sized + Reachable>(pub NonNull<CactusBox<T>>);

impl<T: ?Sized + Reachable> CactusLinkRef<T> {
    #[inline]
    pub fn value(&self) -> &T {
        unsafe { &self.0.as_ref().value }
    }
}

impl<T: ?Sized + Reachable> Copy for CactusLinkRef<T> {}

impl<T: ?Sized + Reachable> Clone for CactusLinkRef<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized + Reachable> Hash for CactusLinkRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let object_id = self.value().object_id();
        object_id.hash(state);
    }
}

impl<T: ?Sized + Reachable> PartialEq for CactusLinkRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value().object_id() == other.value().object_id()
    }
}

impl<T: ?Sized + Reachable> Eq for CactusLinkRef<T> {}
