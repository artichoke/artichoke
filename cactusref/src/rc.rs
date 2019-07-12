use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::Deref;
use core::pin::Pin;
use core::ptr::{self, NonNull};
use itertools::Itertools;
use std::alloc::{Alloc, Global, Layout};
use std::borrow;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;

use crate::link::CactusLinkRef;
use crate::ptr::{is_dangling, RcBox, RcBoxPtr};
use crate::{Reachable, Weak};

pub struct Rc<T: ?Sized + Reachable> {
    pub(crate) ptr: NonNull<RcBox<T>>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: ?Sized> !Send for Rc<T> {}
impl<T: ?Sized> !Sync for Rc<T> {}

impl<T: Reachable> Rc<T> {
    pub fn new(value: T) -> Self {
        Self {
            // there is an implicit weak pointer owned by all the strong
            // pointers, which ensures that the weak destructor never frees
            // the allocation while the strong destructor is running, even
            // if the weak pointer is stored inside the strong one.
            ptr: Box::into_raw_non_null(Box::new(RcBox {
                strong: Cell::new(1),
                weak: Cell::new(1),
                links: RefCell::new(HashSet::default()),
                value: Box::new(value),
            })),
            phantom: PhantomData,
        }
    }

    pub fn pin(value: T) -> Pin<Self> {
        unsafe { Pin::new_unchecked(Self::new(value)) }
    }

    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        if Self::strong_count(&this) == 1 {
            unsafe {
                let val = ptr::read(&*this); // copy the contained object

                // Indicate to Weaks that they can't be promoted by decrementing
                // the strong count, and then remove the implicit "strong weak"
                // pointer while also handling drop logic by just crafting a
                // fake Weak.
                this.dec_strong();
                let _weak = Weak { ptr: this.ptr };
                mem::forget(this);
                Ok(val)
            }
        } else {
            Err(this)
        }
    }
}

impl<T: ?Sized + Reachable> Rc<T> {
    pub fn adopt(this: &Self, other: &Self) {
        let other_id = other.inner().value.object_id();
        let mut links = this.inner().links.borrow_mut();
        if this.inner().value.object_id() != other_id && !links.contains(&CactusLinkRef(other.ptr))
        {
            other.inc_strong();
            links.insert(CactusLinkRef(other.ptr));
        }
    }

    pub fn downgrade(this: &Self) -> Weak<T> {
        this.inc_weak();
        // Make sure we do not create a dangling Weak
        debug_assert!(!is_dangling(this.ptr));
        Weak { ptr: this.ptr }
    }

    pub fn weak_count(this: &Self) -> usize {
        this.weak() - 1
    }

    pub fn strong_count(this: &Self) -> usize {
        this.strong()
    }

    pub(crate) fn is_unique(this: &Self) -> bool {
        Self::weak_count(this) == 0 && Self::strong_count(this) == 1
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if Self::is_unique(this) {
            unsafe { Some(&mut this.ptr.as_mut().value) }
        } else {
            None
        }
    }

    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }
}

impl<T: ?Sized + Clone + Reachable> Rc<T> {
    #[inline]
    pub fn make_mut(this: &mut Self) -> &mut T {
        if Self::strong_count(this) != 1 {
            // Gotta clone the data, there are other Rcs
            *this = Self::new((**this).clone())
        } else if Self::weak_count(this) != 0 {
            // Can just steal the data, all that's left is Weaks
            unsafe {
                let mut swap = Self::new(ptr::read(&*this.ptr.as_ref().value));
                mem::swap(this, &mut swap);
                swap.dec_strong();
                // Remove implicit strong-weak ref (no need to craft a fake
                // Weak here -- we know other Weaks can clean up for us)
                swap.dec_weak();
                mem::forget(swap);
            }
        }
        // This unsafety is ok because we're guaranteed that the pointer
        // returned is the *only* pointer that will ever be returned to T. Our
        // reference count is guaranteed to be 1 at this point, and we required
        // the `Rc<T>` itself to be `mut`, so we're returning the only
        // possible reference to the inner value.
        unsafe { &mut this.ptr.as_mut().value }
    }
}

impl<T: ?Sized + PartialEq + Reachable> PartialEq for Rc<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: ?Sized + Eq + Reachable> Eq for Rc<T> {}

impl<T: ?Sized + PartialOrd + Reachable> PartialOrd for Rc<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        **self < **other
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        **self <= **other
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        **self > **other
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        **self >= **other
    }
}

impl<T: ?Sized + Ord + Reachable> Ord for Rc<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: ?Sized + Hash + Reachable> Hash for Rc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}
unsafe impl<#[may_dangle] T: ?Sized + Reachable> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            self.dec_strong();

            // If links is empty, the object is either not in a cycle or part of
            // a cycle that has been link busted for deallocation.
            if self.inner().links.borrow().is_empty() {
                if self.strong() == 0 {
                    // destroy the contained object
                    ptr::drop_in_place(self.ptr.as_mut());

                    // remove the implicit "strong weak" pointer now that we've
                    // destroyed the contents.
                    self.dec_weak();

                    if self.weak() == 0 {
                        Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                    }
                }
                return;
            }
            // Perform a breadth first search over all of the links to determine
            // the clique of refs that self can reach.
            let mut clique = HashSet::new();
            clique.insert(CactusLinkRef(self.ptr));
            let mut strong_counts_in_cycle = HashMap::new();
            loop {
                let size = clique.len();
                for item in clique.clone() {
                    let links = item.0.as_ref().links.borrow();
                    for link in links.iter() {
                        clique.insert(*link);
                    }
                }
                // BFS has found no new refs in the clique.
                if size == clique.len() {
                    break;
                }
            }
            // Iterate over the items in the clique. For each pair of nodes,
            // find nodes that can reach each other. These nodes form a cycle.
            let mut cycle = HashSet::new();
            for (left, right) in clique
                .iter()
                .cartesian_product(clique.iter())
                .filter(|(left, right)| left != right)
            {
                let left_reaches_right = left.value().can_reach(right.value().object_id());
                let right_reaches_left = right.value().can_reach(left.value().object_id());
                let is_new = !cycle.iter().any(|item: &CactusLinkRef<T>| *item == *right);
                if left_reaches_right && right_reaches_left && is_new {
                    cycle.insert(*right);
                    let count = *strong_counts_in_cycle.get(&right).unwrap_or(&0);
                    strong_counts_in_cycle.insert(right, count + 1);
                }
            }
            let cycle_has_external_owners = cycle.iter().any(|item| {
                let cycle_strong_count = strong_counts_in_cycle[item];
                item.0.as_ref().strong() > cycle_strong_count
            });
            if !cycle.is_empty() && !cycle_has_external_owners {
                let ids = cycle
                    .iter()
                    .map(|item| item.value().object_id())
                    .collect::<HashSet<_>>();
                debug!("orphaned cycle detected with object ids: {:?}", ids);
                // Break the cycle and remove all links to prevent loops when
                // dropping cycle refs.
                for (left, right) in cycle
                    .iter()
                    .cartesian_product(cycle.iter())
                    .filter(|(left, right)| left != right)
                {
                    let mut links = left.0.as_ref().links.borrow_mut();
                    links.remove(right);
                }
                for mut obj in cycle {
                    debug!("dropping cycle participant {{{}}}", obj.value().object_id());
                    // destroy the contained object
                    ptr::drop_in_place(obj.0.as_mut());
                }

                // remove the implicit "strong weak" pointer now that we've
                // destroyed the contents.
                self.dec_weak();

                if self.weak() == 0 {
                    Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                }
            }
        }
    }
}

impl<T: ?Sized + Reachable> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T: ?Sized + Reachable> Clone for Rc<T> {
    fn clone(&self) -> Self {
        self.inc_strong();
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized + Reachable + Default> Default for Rc<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized + Reachable + fmt::Display> fmt::Display for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner().value, f)
    }
}

impl<T: ?Sized + Reachable + fmt::Debug> fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner().value, f)
    }
}

impl<T: Reachable> From<T> for Rc<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T: ?Sized + Reachable> borrow::Borrow<T> for Rc<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized + Reachable> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}
