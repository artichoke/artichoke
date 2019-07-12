#![feature(
    allocator_api,
    box_into_raw_non_null,
    core_intrinsics,
    dropck_eyepatch,
    optin_builtin_traits
)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;

mod link;
mod ptr;
mod rc;
mod reachable;
#[cfg(test)]
mod tests;
mod weak;

pub use reachable::Reachable;
pub use rc::{Rc, Rc as CactusRef};
pub use weak::{Weak, Weak as CactusWeakRef};
