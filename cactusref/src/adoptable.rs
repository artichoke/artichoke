use crate::link::Link;
use crate::ptr::RcBoxPtr;
use crate::Rc;

/// Perform bookkeeping to link two objects with an owned reference.
///
/// Calling [`Adoptable::adopt`] builds an object graph which can be used by
/// implementors to detect cycles.
///
/// **Warning**: this trait is unsafe because if it is implemented incorrectly,
/// memory may leak or be double freed.
pub unsafe trait Adoptable {
    /// Perform bookkeeping to record that `this` has an owned reference to
    /// `other`. Adoption is a one-way link.
    fn adopt(this: &Self, other: &Self);
}

/// Implementation of [`Adoptable`] for [`Rc`] which enables `Rc`s to form a
/// cycle of strong references that are reaped by `Rc`'s [`Drop`]
/// implementation.
#[doc(spotlight)]
unsafe impl<T: ?Sized> Adoptable for Rc<T> {
    /// Perform bookkeeping to record that `this` has an owned reference to
    /// `other`.
    ///
    /// `this` stores a reference to `other`'s `RcBox` so [`Rc`] can detect
    /// cycles with reachability tests in [`Drop`].
    ///
    /// `other` has it's strong count increased by one without having a
    /// droppable `Rc` created. During cycle detection, this increased strong
    /// count is used to determine whether the cycle is reachable by any objects
    /// outside of the cycle.
    fn adopt(this: &Self, other: &Self) {
        let mut links = this.inner().links.borrow_mut();
        // Do not adopt self, do not adopt other multiple times
        if !Self::ptr_eq(this, other) && !links.contains(&Link(other.ptr)) {
            other.inc_strong();
            links.insert(Link(other.ptr));
        }
    }
}
