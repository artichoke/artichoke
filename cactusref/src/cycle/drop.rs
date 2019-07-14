use core::ptr;
use std::alloc::{Alloc, Global, Layout};

use crate::cycle::DetectCycles;
use crate::link::Link;
use crate::ptr::RcBoxPtr;
use crate::Rc;

unsafe impl<#[may_dangle] T: ?Sized> Drop for Rc<T> {
    /// Drops the [`Rc`].
    ///
    /// This will decrement the strong reference count. If the strong reference
    /// count reaches zero then the only other references (if any) are
    /// [`Weak`](crate::Weak), so we `drop` the inner value.
    ///
    /// If this `Rc` has adopted any other `Rc`s, drop will trace the reachable
    /// object graph and detect if this `Rc` is part of an orphaned cycle. An
    /// orphaned cycle is a cycle in which all members have no owned references
    /// held by `Rc`s outside of the cycle.
    ///
    /// Cycle detection is a zero-cost abstraction. `Rc`s do not pay the cost of
    /// the reachability check unless they use
    /// [`Adoptable::adopt`](crate::Adoptable).
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// struct Foo;
    ///
    /// impl Drop for Foo {
    ///     fn drop(&mut self) {
    ///         println!("dropped!");
    ///     }
    /// }
    ///
    /// let foo  = Rc::new(Foo);
    /// let foo2 = Rc::clone(&foo);
    ///
    /// drop(foo);    // Doesn't print anything
    /// drop(foo2);   // Prints "dropped!"
    /// ```
    ///
    /// ```
    /// use cactusref::{Adoptable, Rc};
    ///
    /// struct Foo(u8);
    ///
    /// impl Drop for Foo {
    ///     fn drop(&mut self) {
    ///         println!("dropped {}!", self.0);
    ///     }
    /// }
    ///
    /// let foo  = Rc::new(Foo(10));
    /// let foo2 = Rc::new(Foo(20));
    ///
    /// Rc::adopt(&foo, &foo2);
    /// Rc::adopt(&foo2, &foo);
    ///
    /// drop(foo);    // Doesn't print anything
    /// drop(foo2);   // Prints "dropped 10!" and "dropped 20!"
    /// ```
    ///
    /// # Cycle Detection and Deallocation Algorithm
    ///
    /// [`Rc::adopt`](crate::Adoptable::adopt) does explicit bookkeeping to
    /// store links to adoptee `Rc`s. Each link increases the strong count on
    /// the adoptee but does not allocate another `Rc`.
    ///
    /// On drop, if a `Rc` has no links, it is dropped like a normal `Rc`. If
    /// the `Rc` has links, it performs a breadth first search using its wrapped
    /// value's Reachable implementation to determine the all `Rc`s that it can
    /// reach.
    ///
    /// After determining all reachable objects, `Rc` reduces the graph to
    /// objects that form a cycle by performing pairwise reachability checks.
    /// During this step, for each object in the cycle, `Rc` counts the number
    /// of refs held by other objects in the cycle.
    ///
    /// Using the cycle-held strong refs, `Rc` computes whether the object graph
    /// is reachable by any non-cycle nodes by comparing strong counts.
    ///
    /// If the cycle is orphaned, `Rc` busts all the link `HashSet`s and
    /// deallocates each object.
    fn drop(&mut self) {
        // If a drop is occuring its because there was an existing `Rc` which
        // is maintaining a strong count. Decrement the strong count on drop,
        // even if this `Rc` is dead.
        self.dec_strong();

        // If `self` is held in a cycle, as we deallocate members of the cycle,
        // they will drop their refs to `self`. To prevent a double free, mark
        // nodes as dead if they have already been deallocated and short
        // circuit.
        if self.is_dead() {
            return;
        }
        unsafe {
            if self.inner().links.borrow().is_empty() {
                // If links is empty, the object is either not in a cycle or
                // part of a cycle that has been link busted for deallocation.
                if self.strong() == 0 {
                    // Remove reverse links so `Drop` does not try to reference
                    // the link we are about to deallocate when doing cycle
                    // detection.
                    for (item, _) in self.inner().back_links.borrow().iter() {
                        let mut links = item.0.as_ref().links.borrow_mut();
                        while links.contains(&Link(self.ptr)) {
                            links.remove(Link(self.ptr));
                        }
                        let mut links = item.0.as_ref().back_links.borrow_mut();
                        while links.contains(&Link(self.ptr)) {
                            links.remove(Link(self.ptr));
                        }
                    }
                    // Kill self because it may have been self-adopted.
                    self.kill();
                    // destroy the contained object
                    ptr::drop_in_place(self.ptr.as_mut());

                    // remove the implicit "strong weak" pointer now that we've
                    // destroyed the contents.
                    self.dec_weak();

                    if self.weak() == 0 {
                        Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                    }
                }
            } else if let Some(cycle) = Self::orphaned_cycle(self) {
                debug!(
                    "cactusref detected orphaned cycle with {} objects",
                    cycle.len()
                );
                // Remove reverse links so `Drop` does not try to reference the
                // link we are about to deallocate when doing cycle detection.
                for ptr in cycle.keys() {
                    let item = ptr.0.as_ref();
                    let mut links = item.links.borrow_mut();
                    links.clear();
                    let mut links = item.back_links.borrow_mut();
                    links.clear();
                }
                for (mut ptr, refcount) in cycle.clone() {
                    trace!(
                        "cactusref dropping member of orphaned cycle with refcount {}",
                        refcount
                    );
                    let item = ptr.0.as_mut();
                    // To be in a cycle, at least one `value` field in an
                    // `RcBox` in the cycle holds a strong reference to `self`.
                    // Mark all nodes in the cycle as dead so when we deallocate
                    // them via the `value` pointer we don't get a double free.
                    item.kill();
                }
                for (mut ptr, _) in cycle {
                    if ptr == Link(self.ptr) {
                        continue;
                    }
                    trace!("cactusref deallocating RcBox.value field of cycle participant");
                    let item = ptr.0.as_mut();
                    // Bust the cycle by deallocating the value that this `Rc`
                    // wraps.  This is safe to do and leave the value field
                    // uninitialized because we are deallocating the entire
                    // linked structure.
                    ptr::drop_in_place(&mut item.value as *mut T);
                }
                // destroy the contained object
                trace!("cactusref deallocating self after dropping all cycle participants");
                ptr::drop_in_place(self.ptr.as_mut());

                // remove the implicit "strong weak" pointer now that we've
                // destroyed the contents.
                self.dec_weak();

                if self.weak() == 0 {
                    Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                }
            } else if self.strong() == 0 {
                let this = Link(self.ptr);
                // We are unreachable but may have been adopted and dropped.
                // Remove reverse links so `Drop` does not try to reference the
                // link we are about to deallocate when doing cycle detection.
                // This removes `self` from the cycle detection loop.
                for (item, _) in self.inner().back_links.borrow().iter() {
                    let mut links = item.0.as_ref().links.borrow_mut();
                    while links.contains(&this) {
                        links.remove(this);
                    }
                    let mut links = item.0.as_ref().back_links.borrow_mut();
                    while links.contains(&this) {
                        links.remove(this);
                    }
                }
                self.inner().back_links.borrow_mut().clear();
                for (item, _) in self.inner().links.borrow().iter() {
                    let mut links = item.0.as_ref().links.borrow_mut();
                    while links.contains(&this) {
                        links.remove(this);
                    }
                    let mut links = item.0.as_ref().back_links.borrow_mut();
                    while links.contains(&this) {
                        links.remove(this);
                    }
                }
                self.inner().links.borrow_mut().clear();

                // To be in a cycle, at least one `value` field in an `RcBox`
                // in the cycle holds a strong reference to `self`. Mark all
                // nodes in the cycle as dead so when we deallocate them via
                // the `value` pointer we don't get a double free.
                self.kill();
                trace!("cactusref deallocating adopted and unreachable member of object graph");
                // destroy the contained object
                ptr::drop_in_place(self.ptr.as_mut());

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
