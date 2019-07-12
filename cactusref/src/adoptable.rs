//! Trait for implementing cycles with strong references.
//!
//! This module provides an implementation of [`Adoptable`] for [`Rc`] which
//! enables `Rc`s to form a cycle of strong references that `Rc`'s [`Drop`]
//! implementation can reap.

use crate::link::Link;
use crate::ptr::RcBoxPtr;
use crate::{Rc, Reachable};

/// Take strong ownership of an object without forming a reference cycle.
///
/// To correctly use this trait, do not store strong references to form a cycle.
#[doc(spotlight)]
pub trait Adoptable {
    /// `this` takes ownership of `other` without having a strong reference.
    fn adopt(this: &Self, other: &Self);
}

impl<T: ?Sized + Reachable> Adoptable for Rc<T> {
    /// `this` takes ownership of `other` without having an `Rc`.
    ///
    /// `this` stores a reference to `other`'s `RcBox` that is a manual
    /// bookkeeping entry used by the reachability tests in `Rc`'s [`Drop`]
    /// implementation.
    ///
    /// `other` has it's strong count increased by one without having a
    /// droppable `Rc` created. During cycle detection, this increased strong
    /// count is used to determine whether the cycle is reachable by any objects
    /// outside of the cycle.
    fn adopt(this: &Self, other: &Self) {
        let other_id = other.inner().value.object_id();
        let mut links = this.inner().links.borrow_mut();
        if this.inner().value.object_id() != other_id && !links.contains(&Link(other.ptr)) {
            other.inc_strong();
            links.insert(Link(other.ptr));
        }
    }
}
