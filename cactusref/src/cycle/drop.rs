use core::ptr;
use std::alloc::{Alloc, Global, Layout};

use crate::cycle::{cycle_refs, DetectCycles};
use crate::ptr::RcBoxPtr;
use crate::{Rc, Reachable};

unsafe impl<#[may_dangle] T: ?Sized + Reachable> Drop for Rc<T> {
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
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Foo;
    ///
    /// impl Drop for Foo {
    ///     fn drop(&mut self) {
    ///         println!("dropped!");
    ///     }
    /// }
    ///
    /// unsafe impl Reachable for Foo {
    ///     fn object_id(&self) -> usize {
    ///         0
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
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
    /// use cactusref::{Adoptable, Rc, Reachable};
    ///
    /// struct Foo(u8);
    ///
    /// impl Drop for Foo {
    ///     fn drop(&mut self) {
    ///         println!("dropped {}!", self.0);
    ///     }
    /// }
    ///
    /// unsafe impl Reachable for Foo {
    ///     fn object_id(&self) -> usize {
    ///         0
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
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
        unsafe {
            self.dec_strong();

            if self.inner().links.borrow().is_empty() {
                // If links is empty, the object is either not in a cycle or
                // part of a cycle that has been link busted for deallocation.
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
            } else if Self::is_orphaned_cycle(self) {
                debug!(
                    "orphaned cycle detected with object ids: {:?}",
                    Self::cycle_objects(self)
                );
                // Break the cycle and remove all links to prevent loops when
                // dropping cycle refs.
                let cycle = cycle_refs(self);
                for item in cycle.keys() {
                    let mut links = item.0.as_ref().links.borrow_mut();
                    links.clear();
                }
                for (mut obj, _) in cycle {
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
