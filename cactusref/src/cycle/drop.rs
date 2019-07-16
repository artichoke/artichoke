use core::ptr;
use hashbrown::HashMap;
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
    /// store links to adoptee `Rc`s. These links form a graph of reachable
    /// objects which are used to detect cycles.
    ///
    /// On drop, if an `Rc` has no links, it is dropped like a normal `Rc`. If
    /// the `Rc` has links, `Drop` performs a breadth first search by traversing
    /// the forward and backward links stored in each `Rc`. Deallocating cycles
    /// requires correct use of [`Adoptable::adopt`](crate::Adoptable::adopt)
    /// and [`Adoptable::unadopt`](crate::Adoptable::unadopt) to perform the
    /// reachability bookkeeping.
    ///
    /// After determining all reachable objects, `Rc` reduces the graph to
    /// objects that form a cycle by performing pairwise reachability checks.
    /// During this step, for each object in the cycle, `Rc` counts the number
    /// of refs held by other objects in the cycle.
    ///
    /// Using the cycle-held references, `Rc` computes whether the object graph
    /// is reachable by any non-cycle nodes by comparing strong counts.
    ///
    /// If the cycle is orphaned, `Rc` busts all the link structures and
    /// deallocates each object.
    ///
    /// ## Performance
    ///
    /// Cycle detection uses breadth first search to trace the object graph.
    /// The runtime complexity of detecting a cycle is `O(links + nodes)` where
    /// links is the number of adoptions that are alive and nodes is the number
    /// of objects in the cycle.
    ///
    /// Determining whether the cycle is orphaned builds on cycle detection and
    /// iterates over all nodes in the graph to see if their strong count is
    /// greater than the number of references in the cycle. The runtime
    /// complexity of finding an orphaned cycle is `O(links + nodes)` where
    /// links is the number of adoptions that are alive and nodes is the number
    /// objects in the cycle.
    fn drop(&mut self) {
        // If `self` is held in a cycle, as we deallocate members of the cycle,
        // they will drop their refs to `self`. To prevent a double free, mark
        // nodes as dead if they have already been deallocated and short
        // circuit.
        if self.is_dead() {
            return;
        }

        // If a drop is occuring it is because there was an existing `Rc` which
        // is maintaining a strong count. Decrement the strong count on drop,
        // even if this `Rc` is dead. This ensures `Weak::upgrade` behaves
        // correctly for deallocated cycles and does not cause a use-after-free.
        self.dec_strong();

        unsafe {
            if self.inner().links.borrow().is_empty() {
                // If links is empty, the object is either not in a cycle or
                // part of a cycle that has been link busted for deallocation.
                if self.strong() == 0 {
                    drop_unreachable(self);
                }
            } else if let Some(cycle) = Self::orphaned_cycle(self) {
                drop_cycle(self, cycle);
            } else if self.strong() == 0 {
                drop_unreachable_with_adoptions(self);
            }
        }
    }
}

unsafe fn drop_unreachable<T: ?Sized>(this: &mut Rc<T>) {
    // Remove reverse links so `this` is not included in cycle detection for
    // objects that had adopted `this`. This prevents a use-after-free in
    // `DetectCycles::orphaned_cycle`.
    for (item, _) in this.inner().back_links.borrow().iter() {
        let link = Link(this.ptr);
        let mut links = item.0.as_ref().links.borrow_mut();
        while links.contains(&link) {
            links.remove(link);
        }
        let mut links = item.0.as_ref().back_links.borrow_mut();
        while links.contains(&link) {
            links.remove(link);
        }
    }
    // Mark `this` as pending deallocation. This is not strictly necessary since
    // `this` is unreachable, but `kill`ing `this ensures we don't double-free.
    this.kill();
    // destroy the contained object
    ptr::drop_in_place(this.ptr.as_mut());

    // remove the implicit "strong weak" pointer now that we've destroyed the
    // contents.
    this.dec_weak();

    if this.weak() == 0 {
        Global.dealloc(this.ptr.cast(), Layout::for_value(this.ptr.as_ref()));
    }
}

unsafe fn drop_cycle<T: ?Sized>(this: &mut Rc<T>, cycle: HashMap<Link<T>, usize>) {
    debug!(
        "cactusref detected orphaned cycle with {} objects",
        cycle.len()
    );
    // Remove reverse links so `this` is not included in cycle detection for
    // objects that had adopted `this`. This prevents a use-after-free in
    // `DetectCycles::orphaned_cycle`.
    //
    // Because the entire cycle is unreachable, the only forward and backward
    // links are to objects in the cycle that we are about to deallocate. This
    // allows us to bust the cycle detection by clearing all links.
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
        // To be in a cycle, at least one `value` field in an `RcBox` in the
        // cycle holds a strong reference to `this`. Mark all nodes in the cycle
        // as dead so when we deallocate them via the `value` pointer we don't
        // get a double-free.
        item.kill();
    }
    for (mut ptr, _) in cycle {
        if ptr.0 == this.ptr {
            // Do not drop `this` until the rest of the cycle is deallocated.
            continue;
        }
        trace!("cactusref deallocating wrapped value of cycle member");
        let item = ptr.0.as_mut();
        // Bust the cycle by deallocating the value that this `Rc` wraps. This
        // is safe to do and leave the value field uninitialized because we are
        // deallocating the entire linked structure.
        ptr::drop_in_place(&mut item.value as *mut T);
    }
    // destroy the contained object
    trace!("cactusref deallocating self after dropping all cycle members");
    ptr::drop_in_place(this.ptr.as_mut());

    // remove the implicit "strong weak" pointer now that we've
    // destroyed the contents.
    this.dec_weak();

    if this.weak() == 0 {
        Global.dealloc(this.ptr.cast(), Layout::for_value(this.ptr.as_ref()));
    }
}

unsafe fn drop_unreachable_with_adoptions<T: ?Sized>(this: &mut Rc<T>) {
    let link = Link(this.ptr);
    // `this` is unreachable but may have been adopted and dropped. Remove
    // reverse links so `Drop` does not try to reference the link we are about
    // to deallocate when doing cycle detection. This removes `self` from the
    // cycle detection loop. This prevents a use-after-free in
    // `DetectCycles::orphaned_cycle`.
    for (item, _) in this.inner().back_links.borrow().iter() {
        let mut links = item.0.as_ref().links.borrow_mut();
        while links.contains(&link) {
            links.remove(link);
        }
        let mut links = item.0.as_ref().back_links.borrow_mut();
        while links.contains(&link) {
            links.remove(link);
        }
    }
    // Clear links in `this`. This is not strictly necessary since `this` is
    // unreachable, but `clear`ing `this ensures we don't double-free.
    this.inner().back_links.borrow_mut().clear();
    for (item, _) in this.inner().links.borrow().iter() {
        let mut links = item.0.as_ref().links.borrow_mut();
        while links.contains(&link) {
            links.remove(link);
        }
        let mut links = item.0.as_ref().back_links.borrow_mut();
        while links.contains(&link) {
            links.remove(link);
        }
    }
    this.inner().links.borrow_mut().clear();

    // Mark `this` as pending deallocation. This is not strictly necessary since
    // `this` is unreachable, but `kill`ing `this ensures we don't double-free.
    this.kill();
    trace!("cactusref deallocating adopted and unreachable member of object graph");
    // destroy the contained object
    ptr::drop_in_place(this.ptr.as_mut());

    // remove the implicit "strong weak" pointer now that we've
    // destroyed the contents.
    this.dec_weak();

    if this.weak() == 0 {
        Global.dealloc(this.ptr.cast(), Layout::for_value(this.ptr.as_ref()));
    }
}
