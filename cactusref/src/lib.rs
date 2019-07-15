#![feature(
    allocator_api,
    alloc_layout_extra,
    box_into_raw_non_null,
    core_intrinsics,
    dropck_eyepatch,
    optin_builtin_traits,
    ptr_internals,
    specialization
)]
#![deny(warnings, intra_doc_link_resolution_failure, missing_docs)]
#![deny(clippy::all, clippy::pedantic)]

//! # 🌵 `CactusRef`
//!
//! Single-threaded, cycle-aware, reference-counting pointers. 'Rc' stands
//! for 'Reference Counted'.
//!
//! The type [`Rc<T>`](`Rc`) provides shared ownership of a value of type `T`,
//! allocated in the heap. Invoking [`clone`](Clone::clone) on [`Rc`] produces a
//! new pointer to the same value in the heap. When the last externally
//! reachable [`Rc`] pointer to a given value is destroyed, the pointed-to value
//! is also destroyed.
//!
//! `Rc` can **detect and deallocate cycles** of `Rc`s through the use of
//! [`Adoptable`]. Cycle detection is a zero-cost abstraction.
//!
//! 🌌 `CactusRef` depends on _several_ unstable Rust features and can only be
//! built on nightly. `CactusRef` implements `std::rc`'s pinning APIs which
//! requires at least Rust 1.33.0.
//!
//! ## `CactusRef` vs. `std::rc`
//!
//! The `Rc` in `CactusRef` is derived from [`std::rc::Rc`] and `CactusRef`
//! implements most of the API from `std`.
//!
//! `cactusref::Rc` does not implement the following APIs that are present on
//! [`rc::Rc`](std::rc::Rc):
//!
//! - [`std::rc::Rc::downcast`](std::rc::Rc::downcast)
//! - [`CoerceUnsized`](core::ops::CoerceUnsized)
//! - [`DispatchFromDyn`](core::ops::DispatchFromDyn)
//!
//! If you do not depend on these APIs, `cactusref` is a drop-in replacement for
//! [`std::rc`].
//!
//! ## Cycle Detection
//!
//! `Rc` implements [`Adoptable`] to log bookkeeping entries for strong
//! ownership links to other `Rc`s that may form a cycle. The ownership links
//! tracked by these bookkeeping entries form an object graph of reachable
//! `Rc`s. On `drop`, `Rc` uses these entries to conduct a reachability trace
//! of the object graph to determine if it is part of an _orphaned cycle_. An
//! orphaned cycle is a cycle where the only strong references to all nodes in
//! the cycle come from other nodes in the cycle.
//!
//! Cycle detection is a zero-cost abstraction. If you never
//! `use cactusref::Adoptable;`, `Drop` uses the same implementation as
//! `std::rc::Rc` (and leaks in the same way as `std::rc::Rc` if you form a
//! cycle of strong references). The only costs you pay are the memory costs of
//! one [`Cell<usize>`](std::cell::Cell) for preventing double frees, two empty
//! [`RefCell`](std::cell::RefCell)`<`[`HashMap`](std::collections::HashMap)`<NonNull<T>, usize>>`
//! for tracking adoptions, and an if statement to check if these structures are
//! empty on `drop`.
//!
//! Cycle detection uses breadth-first search for traversing the object graph.
//! The algorithm supports arbitrarily large object graphs and will not overflow
//! the stack during the reachability trace.
//!
//! ## Self-Referential Structures
//!
//! `CactusRef` can be used to implement collections that own strong references
//! to themselves. The following implements a doubly-linked list that is fully
//! deallocated once the `list` binding is dropped.
//!
//! ```rust
//! use cactusref::{Adoptable, Rc};
//! use std::cell::RefCell;
//! use std::iter;
//!
//! struct Node<T> {
//!     pub prev: Option<Rc<RefCell<Self>>>,
//!     pub next: Option<Rc<RefCell<Self>>>,
//!     pub data: T,
//! }
//!
//! struct List<T> {
//!     pub head: Option<Rc<RefCell<Node<T>>>>,
//! }
//!
//! impl<T> List<T> {
//!     fn pop(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
//!         let head = self.head.take()?;
//!         let tail = head.borrow_mut().prev.take();
//!         let next = head.borrow_mut().next.take();
//!         if let Some(ref tail) = tail {
//!             Rc::unadopt(&head, &tail);
//!             Rc::unadopt(&tail, &head);
//!             tail.borrow_mut().next = next.as_ref().map(Rc::clone);
//!             if let Some(ref next) = next {
//!                 Rc::adopt(tail, next);
//!             }
//!         }
//!         if let Some(ref next) = next {
//!             Rc::unadopt(&head, &next);
//!             Rc::unadopt(&next, &head);
//!             next.borrow_mut().prev = tail.as_ref().map(Rc::clone);
//!             if let Some(ref tail) = tail {
//!                 Rc::adopt(next, tail);
//!             }
//!         }
//!         self.head = next;
//!         Some(head)
//!     }
//! }
//!
//! impl<T> From<Vec<T>> for List<T> {
//!     fn from(list: Vec<T>) -> Self {
//!         let nodes = list
//!             .into_iter()
//!             .map(|data| {
//!                 Rc::new(RefCell::new(Node {
//!                     prev: None,
//!                     next: None,
//!                     data,
//!                 }))
//!             })
//!             .collect::<Vec<_>>();
//!         for i in 0..nodes.len() - 1 {
//!             let curr = &nodes[i];
//!             let next = &nodes[i + 1];
//!             curr.borrow_mut().next = Some(Rc::clone(next));
//!             next.borrow_mut().prev = Some(Rc::clone(curr));
//!             Rc::adopt(curr, next);
//!             Rc::adopt(next, curr);
//!         }
//!         let tail = &nodes[nodes.len() - 1];
//!         let head = &nodes[0];
//!         tail.borrow_mut().next = Some(Rc::clone(head));
//!         head.borrow_mut().prev = Some(Rc::clone(tail));
//!         Rc::adopt(tail, head);
//!         Rc::adopt(head, tail);
//!
//!         let head = Rc::clone(head);
//!         Self { head: Some(head) }
//!     }
//! }
//!
//! let list = iter::repeat(())
//!     .map(|_| "a".repeat(1024 * 1024))
//!     .take(10)
//!     .collect::<Vec<_>>();
//! let mut list = List::from(list);
//! let head = list.pop().unwrap();
//! assert_eq!(Rc::strong_count(&head), 1);
//! assert_eq!(list.head.as_ref().map(Rc::strong_count), Some(3));
//! let weak = Rc::downgrade(&head);
//! drop(head);
//! assert!(weak.upgrade().is_none());
//! drop(list);
//! // all memory consumed by the list nodes is reclaimed.
//! ```

#[macro_use]
extern crate log;

mod adoptable;
mod cycle;
mod link;
mod ptr;
mod rc;
#[cfg(test)]
mod tests;
mod weak;

pub use adoptable::Adoptable;
pub use rc::Rc;
pub use weak::Weak;

/// Cactus alias for [`Rc`].
pub type CactusRef<T> = Rc<T>;

/// Cactus alias for [`Weak`].
pub type CactusWeakRef<T> = Weak<T>;
