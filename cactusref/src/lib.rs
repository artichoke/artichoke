#![feature(
    allocator_api,
    alloc_layout_extra,
    box_into_raw_non_null,
    core_intrinsics,
    doc_spotlight,
    dropck_eyepatch,
    optin_builtin_traits,
    ptr_internals,
    specialization
)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

// does not support Rc::downcast
// Does not support operations on Rc<[T]>

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
