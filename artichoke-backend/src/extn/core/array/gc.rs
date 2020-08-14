//! Garbage collection routines for `Array` types.

use super::Array;
use crate::gc::MrbGarbageCollection;
use crate::Artichoke;

/// Mark all elements in the `Array` as reachable to the garbage collector.
///
/// This method ensures that the contents of the conained
/// [`sys::mrb_value`]s do not get deallocated while the given `Array` is alive
/// in the mruby VM.
pub fn mark(ary: &Array, interp: &mut Artichoke) {
    for elem in ary {
        interp.mark_value(&elem);
    }
}

/// The count of [`sys::mrb_value`]s in the given `Array`.
///
/// This method allows for `Array`s with holes or other virtualized
/// elements. `Array` does not store virtual elements so this method always
/// returns the array's length.
#[must_use]
pub fn children(ary: &Array) -> usize {
    ary.len()
}
