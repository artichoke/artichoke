use core::ptr::NonNull;
use std::hash::{Hash, Hasher};

use crate::{CactusBox, Reachable};

pub(crate) struct CactusLinkRef<T: Reachable>(pub NonNull<CactusBox<T>>);

impl<T: Reachable> CactusLinkRef<T> {
    #[inline]
    pub fn value(&self) -> &T {
        unsafe { &self.0.as_ref().value }
    }
}

impl<T: Reachable> Copy for CactusLinkRef<T> {}

impl<T: Reachable> Clone for CactusLinkRef<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: Reachable> Hash for CactusLinkRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let object_id = self.value().object_id();
        object_id.hash(state);
    }
}

impl<T: Reachable> PartialEq for CactusLinkRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value().object_id() == other.value().object_id()
    }
}

impl<T: Reachable> Eq for CactusLinkRef<T> {}
