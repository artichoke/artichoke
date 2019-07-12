//! Unsafe trait for implementing reachability checks for cycle detection and
//! reaping used by [`CactusRef`](crate::CactusRef).
//!
//! This module provides automatic implementations for `&T`, `&mut T`,
//! `*const T`, `*mut T`, and `Box<T>` for any `T` that implements
//! [`Reachable`].

/// Reachability tests for objects wrapped by [`CactusRef`](crate::CactusRef).
/// This trait is unsafe.
///
/// **Warning**: If this trait is implemented incorrectly,
/// [`CactusRef`](crate::CactusRef) has undefined behavior. Memory may leak or
/// be double freed.
pub unsafe trait Reachable {
    /// An identifier for this object that is unique within an arena.
    ///
    /// An arena consists of any objects that _may_ reference each other.
    fn object_id(&self) -> usize;

    /// Reachability test used by [`CactusRef`](crate::CactusRef) to perform
    /// breadth first search for cycle detection in [`Drop`] implementation.
    ///
    /// **Warning**: If this function is implemented incorrectly,
    /// [`CactusRef`](crate::CactusRef) has undefined behavior. Memory may leak
    /// or be double freed.
    fn can_reach(&self, object_id: usize) -> bool;
}

unsafe impl Reachable for () {
    fn object_id(&self) -> usize {
        0
    }

    fn can_reach(&self, _object_id: usize) -> bool {
        false
    }
}

unsafe impl<T: Reachable> Reachable for &T {
    fn object_id(&self) -> usize {
        Reachable::object_id(*self)
    }

    fn can_reach(&self, object_id: usize) -> bool {
        Reachable::can_reach(*self, object_id)
    }
}

unsafe impl<T: Reachable> Reachable for &mut T {
    fn object_id(&self) -> usize {
        Reachable::object_id(*self)
    }

    fn can_reach(&self, object_id: usize) -> bool {
        Reachable::can_reach(*self, object_id)
    }
}

unsafe impl<T: Reachable> Reachable for *const T {
    fn object_id(&self) -> usize {
        unsafe { Reachable::object_id(&**self) }
    }

    fn can_reach(&self, object_id: usize) -> bool {
        unsafe { Reachable::can_reach(&**self, object_id) }
    }
}

unsafe impl<T: Reachable> Reachable for *mut T {
    fn object_id(&self) -> usize {
        unsafe { Reachable::object_id(&**self) }
    }

    fn can_reach(&self, object_id: usize) -> bool {
        unsafe { Reachable::can_reach(&**self, object_id) }
    }
}

unsafe impl<T: Reachable> Reachable for Box<T> {
    fn object_id(&self) -> usize {
        self.as_ref().object_id()
    }

    fn can_reach(&self, object_id: usize) -> bool {
        self.as_ref().can_reach(object_id)
    }
}
