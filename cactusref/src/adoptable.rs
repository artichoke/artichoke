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

    /// Perform bookkeeping to record that `this` no longer has an owned
    /// reference to `other`. Adoption is a one-way link.
    fn unadopt(this: &Self, other: &Self);
}

/// Implementation of [`Adoptable`] for [`Rc`] which enables `Rc`s to form a
/// cycle of strong references that are reaped by `Rc`'s [`Drop`]
/// implementation.
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
    ///
    /// # Examples
    ///
    /// The following implements a self-referential array.
    ///
    /// ```rust
    /// use cactusref::{Adoptable, Rc};
    /// use std::cell::RefCell;
    ///
    /// #[derive(Default)]
    /// struct Array {
    ///     buffer: Vec<Rc<RefCell<Self>>>,
    /// }
    ///
    /// let array = Rc::new(RefCell::new(Array::default()));
    /// for _ in 0..10 {
    ///     let item = Rc::clone(&array);
    ///     Rc::adopt(&array, &item);
    ///     array.borrow_mut().buffer.push(item);
    /// }
    /// let weak = Rc::downgrade(&array);
    /// // 1 for the array binding, 10 for the `Rc`s in buffer
    /// assert_eq!(Rc::strong_count(&array), 11);
    /// drop(array);
    /// assert!(weak.upgrade().is_none());
    /// assert_eq!(weak.weak_count(), Some(1));
    /// ```
    fn adopt(this: &Self, other: &Self) {
        // Adoption signals the intent to take an owned reference to `other`, so
        // always increment the strong count of other. This allows `this` to be
        // self-referential and allows `this` to own multiple references to
        // `other`. These behaviors allow implementing self-referential
        // collection types.

        // Store a forward reference to `other` in `this`. This bookkeeping logs
        // a strong reference and is used for discovering cycles.
        let mut links = this.inner().links.borrow_mut();
        links.insert(Link(other.ptr));
        // Store a backward reference to `this` in `other`. This bookkeeping is
        // used for discovering cycles.
        let mut links = other.inner().back_links.borrow_mut();
        links.insert(Link(this.ptr));
    }

    /// Perform bookkeeping to record that `this` no longer has an owned
    /// reference to `other`.
    ///
    /// `this` stores a reference to `other`'s `RcBox` so [`Rc`] can detect
    /// cycles with reachability tests in [`Drop`].
    ///
    /// `other` has it's strong count decreased by one. `unadopt` removes
    /// forward and reverse link tracking.
    ///
    /// # Examples
    ///
    /// The following implements a self-referential array.
    ///
    /// ```rust
    /// use cactusref::{Adoptable, Rc};
    /// use std::cell::RefCell;
    ///
    /// #[derive(Default)]
    /// struct Array {
    ///     buffer: Vec<Rc<RefCell<Self>>>,
    /// }
    ///
    /// let array = Rc::new(RefCell::new(Array::default()));
    /// for _ in 0..10 {
    ///     let item = Rc::clone(&array);
    ///     Rc::adopt(&array, &item);
    ///     array.borrow_mut().buffer.push(item);
    /// }
    /// let weak = Rc::downgrade(&array);
    /// // 1 for the array binding, 10 for the `Rc`s in buffer
    /// assert_eq!(Rc::strong_count(&array), 11);
    /// let head = array.borrow_mut().buffer.pop().unwrap();
    /// unsafe {
    ///     Rc::unadopt(&array, &head);
    /// }
    /// drop(head);
    /// assert_eq!(Rc::strong_count(&array), 10);
    /// drop(array);
    /// assert!(weak.upgrade().is_none());
    /// assert_eq!(weak.weak_count(), Some(1));
    /// ```
    fn unadopt(this: &Self, other: &Self) {
        // Remove a forward reference to `other` in `this`. This bookkeeping
        // logs a strong reference and is used for discovering cycles.
        let mut links = this.inner().links.borrow_mut();
        links.remove(Link(other.ptr));
        // Remove a backward reference to `this` in `other`. This bookkeeping is
        // used for discovering cycles.
        let mut links = other.inner().back_links.borrow_mut();
        links.remove(Link(this.ptr));
    }
}
