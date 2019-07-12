#![feature(allocator_api, box_into_raw_non_null, core_intrinsics, dropck_eyepatch)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;

use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::{self, NonNull};
use itertools::Itertools;
use std::alloc::{Alloc, Global, Layout};
use std::borrow;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::intrinsics::abort;

mod link;

use link::CactusLinkRef;

pub type ObjectId = usize;

pub unsafe trait Reachable {
    fn object_id(&self) -> ObjectId;

    fn can_reach(&self, object_id: ObjectId) -> bool;
}

trait CactusBoxPtr<T: Reachable> {
    fn inner(&self) -> &CactusBox<T>;

    #[inline]
    fn strong(&self) -> usize {
        self.inner().strong.get()
    }

    #[inline]
    fn inc_strong(&self) {
        // We want to abort on overflow instead of dropping the value.
        // nevertheless, we insert an abort here to hint LLVM at
        // an otherwise missed optimization.
        if self.strong() == 0 || self.strong() == usize::max_value() {
            unsafe {
                abort();
            }
        }
        self.inner().strong.set(self.strong() + 1);
    }

    #[inline]
    fn dec_strong(&self) {
        self.inner().strong.set(self.strong() - 1);
    }

    #[inline]
    fn weak(&self) -> usize {
        self.inner().weak.get()
    }

    #[inline]
    fn inc_weak(&self) {
        // We want to abort on overflow instead of dropping the value.
        // The reference count will never be zero when this is called;
        // nevertheless, we insert an abort here to hint LLVM at
        // an otherwise missed optimization.
        if self.weak() == 0 || self.weak() == usize::max_value() {
            unsafe {
                abort();
            }
        }
        self.inner().weak.set(self.weak() + 1);
    }

    #[inline]
    fn dec_weak(&self) {
        self.inner().weak.set(self.weak() - 1);
    }
}

impl<T: Reachable> CactusBoxPtr<T> for CactusRef<T> {
    fn inner(&self) -> &CactusBox<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Reachable> CactusBoxPtr<T> for CactusBox<T> {
    fn inner(&self) -> &Self {
        self
    }
}

struct CactusBox<T: Reachable> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
    links: RefCell<HashSet<CactusLinkRef<T>>>,
}

pub struct CactusRef<T: Reachable> {
    ptr: NonNull<CactusBox<T>>,
    phantom: PhantomData<T>,
}

impl<T: Reachable> CactusRef<T> {
    pub fn new(value: T) -> Self {
        Self {
            // there is an implicit weak pointer owned by all the strong
            // pointers, which ensures that the weak destructor never frees
            // the allocation while the strong destructor is running, even
            // if the weak pointer is stored inside the strong one.
            ptr: Box::into_raw_non_null(Box::new(CactusBox {
                strong: Cell::new(1),
                weak: Cell::new(1),
                value,
                links: RefCell::new(HashSet::default()),
            })),
            phantom: PhantomData,
        }
    }

    pub fn adopt(this: &Self, other: &Self) {
        let other_id = other.inner().value.object_id();
        let mut links = this.inner().links.borrow_mut();
        if this.inner().value.object_id() != other_id && !links.contains(&CactusLinkRef(other.ptr))
        {
            other.inc_strong();
            links.insert(CactusLinkRef(other.ptr));
        }
    }

    pub fn downgrade(this: &Self) -> CactusWeakRef<T> {
        this.inc_weak();
        // Make sure we do not create a dangling Weak
        debug_assert!(!is_dangling(this.ptr));
        CactusWeakRef { ptr: this.ptr }
    }
}

unsafe impl<#[may_dangle] T: Reachable> Drop for CactusRef<T> {
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
            // determine if the nodes can mutually reach each other. If two
            // nodes can mutually reach each other, they participate in a cycle.
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

impl<T: Reachable> Deref for CactusRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T: Reachable> Clone for CactusRef<T> {
    fn clone(&self) -> Self {
        self.inc_strong();
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T: Reachable + Default> Default for CactusRef<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Reachable + fmt::Display> fmt::Display for CactusRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner().value, f)
    }
}

impl<T: Reachable + fmt::Debug> fmt::Debug for CactusRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner().value, f)
    }
}

impl<T: Reachable> From<T> for CactusRef<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

pub(crate) fn is_dangling<T: ?Sized>(ptr: NonNull<T>) -> bool {
    let address = ptr.as_ptr() as *mut () as usize;
    address == usize::max_value()
}

pub struct CactusWeakRef<T>
where
    T: Reachable,
{
    // This is a `NonNull` to allow optimizing the size of this type in enums,
    // but it is not necessarily a valid pointer.
    // `Weak::new` sets this to `usize::MAX` so that it doesn’t need
    // to allocate space on the heap.  That's not a value a real pointer
    // will ever have because RcBox has alignment at least 2.
    ptr: NonNull<CactusBox<T>>,
}

impl<T: Reachable> CactusWeakRef<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::new(usize::max_value() as *mut CactusBox<T>).expect("MAX is not 0"),
        }
    }

    pub fn upgrade(&self) -> Option<CactusRef<T>> {
        let inner = self.inner()?;
        if inner.strong() == 0 {
            None
        } else {
            inner.inc_strong();
            Some(CactusRef {
                ptr: self.ptr,
                phantom: PhantomData,
            })
        }
    }

    pub fn strong_count(&self) -> usize {
        if let Some(inner) = self.inner() {
            inner.strong()
        } else {
            0
        }
    }

    #[inline]
    fn inner(&self) -> Option<&CactusBox<T>> {
        if is_dangling(self.ptr) {
            None
        } else {
            Some(unsafe { self.ptr.as_ref() })
        }
    }

    #[inline]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }
}

impl<T: Reachable> Drop for CactusWeakRef<T> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner() {
            inner.dec_weak();
            // the weak count starts at 1, and will only go to zero if all
            // the strong pointers have disappeared.
            if inner.weak() == 0 {
                unsafe {
                    Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                }
            }
        }
    }
}

impl<T: Reachable> Clone for CactusWeakRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        if let Some(inner) = self.inner() {
            inner.inc_weak()
        }
        Self { ptr: self.ptr }
    }
}

impl<T: Reachable + fmt::Debug> fmt::Debug for CactusWeakRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Weak)")
    }
}

impl<T: Reachable> Default for CactusWeakRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Reachable> borrow::Borrow<T> for CactusRef<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: Reachable> AsRef<T> for CactusRef<T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}
