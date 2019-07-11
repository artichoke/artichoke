#![feature(allocator_api, box_into_raw_non_null, core_intrinsics, dropck_eyepatch)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;

use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::{self, NonNull};
use std::alloc::{Alloc, Global, Layout};
use std::borrow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::intrinsics::abort;

pub type ObjectId = usize;

pub unsafe trait Reachable {
    fn object_id(&self) -> ObjectId;

    fn can_reach(&self, object_id: ObjectId) -> bool;
}

trait CactusBoxPtr<T> {
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

impl<T> CactusBoxPtr<T> for CactusBox<T> {
    fn inner(&self) -> &Self {
        self
    }
}

struct CactusBox<T> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
    links: RefCell<HashMap<ObjectId, NonNull<Self>>>,
}

pub struct CactusRef<T>
where
    T: Reachable,
{
    ptr: NonNull<CactusBox<T>>,
    phantom: PhantomData<T>,
}

impl<T> CactusRef<T>
where
    T: Reachable,
{
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
                links: RefCell::new(HashMap::default()),
            })),
            phantom: PhantomData,
        }
    }

    pub fn adopt(this: &Self, other: &Self) {
        let other_id = other.inner().value.object_id();
        let mut links = this.inner().links.borrow_mut();
        if this.inner().value.object_id() != other_id && !links.contains_key(&other_id) {
            other.inc_strong();
            let ptr = unsafe {
                NonNull::new_unchecked(other.inner() as *const CactusBox<T> as *mut CactusBox<T>)
            };
            links.insert(other_id, ptr);
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

            trace!("drop on {}", self.inner().value.object_id());
            let no_links = self.inner().links.borrow().is_empty();
            if no_links {
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
            } else {
                // Trace the links to determine a clique that includes self.
                let mut cycle_owned_refs = vec![self.ptr];
                let mut strong_counts_in_cycle = HashMap::new();
                loop {
                    let mut new_refs = vec![];
                    for item in &cycle_owned_refs {
                        let links = item.as_ref().links.borrow();
                        for (id, obj) in links.iter() {
                            if !cycle_owned_refs
                                .iter()
                                .any(|item| item.as_ref().value.object_id() == *id)
                            {
                                new_refs.push(*obj);
                            }
                        }
                    }
                    if new_refs.is_empty() {
                        break;
                    }
                    cycle_owned_refs.extend(new_refs);
                }
                // Iterate over the items in the clique and for each pair of nodes,
                // find nodes that can reach each other. These nodes form a cycle.
                let mut cycle_participants = vec![];
                for left in cycle_owned_refs.clone() {
                    for right in cycle_owned_refs.clone() {
                        if left.as_ref().value.object_id() == right.as_ref().value.object_id() {
                            continue;
                        }
                        let left_reaches_right = left
                            .as_ref()
                            .value
                            .can_reach(right.as_ref().value.object_id());
                        let right_reaches_left = right
                            .as_ref()
                            .value
                            .can_reach(left.as_ref().value.object_id());
                        let right_is_new_cycle_participant = cycle_participants
                            .iter()
                            .find(|item: &&NonNull<CactusBox<T>>| {
                                item.as_ref().value.object_id() == right.as_ref().value.object_id()
                            })
                            .is_none();
                        if left_reaches_right
                            && right_reaches_left
                            && right_is_new_cycle_participant
                        {
                            trace!(
                                "ObjectId<{}> and ObjectId<{}> are in a cycle",
                                left.as_ref().value.object_id(),
                                right.as_ref().value.object_id()
                            );
                            cycle_participants.push(right);
                            let count = *strong_counts_in_cycle
                                .get(&right.as_ref().value.object_id())
                                .unwrap_or(&0);
                            strong_counts_in_cycle
                                .insert(right.as_ref().value.object_id(), count + 1);
                        }
                    }
                }
                let mut cycle_ids = cycle_participants
                    .iter()
                    .map(|item| item.as_ref().value.object_id())
                    .collect::<Vec<_>>();
                cycle_ids.sort();
                let cycle_has_external_owners = cycle_participants.iter().any(|item| {
                    let object_id = item.as_ref().value.object_id();
                    let cycle_strong_count = strong_counts_in_cycle[&object_id];
                    item.as_ref().strong() > cycle_strong_count
                });
                if !cycle_participants.is_empty() && !cycle_has_external_owners {
                    debug!("CactusRef cycle detected with object ids: {:?}", cycle_ids);
                    // break the cycle and remove all links
                    for item in &cycle_participants {
                        let mut links = item.as_ref().links.borrow_mut();
                        for other in &cycle_participants {
                            let other_id = other.as_ref().value.object_id();
                            let _ = links.remove(&other_id);
                        }
                    }
                    for mut obj in cycle_participants {
                        debug!(
                            "CactusRef dropping cycle participant ObjectId<{}>",
                            obj.as_ref().value.object_id()
                        );
                        // destroy the contained object
                        ptr::drop_in_place(obj.as_mut());
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
