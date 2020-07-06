//! Implementations of Ruby [`Array`], a growable vector.
//!
//! This module contains multiple implementations of a backing store for the
//! Ruby `Array` core class. [`Array`](vec::Array) is based on the [`Vec`] type
//! in `std`. [`SmallArray`](smallvec::SmallArray) is based on [`SmallVec`].
//!
//! The smallvec backend uses small vector optimization to store
//! [some elements][inline-capacity] inline without spilling to the heap.
//!
//! The `SmallArray` backend requires the `small-array` Cargo feature to be
//! enabled.
//!
//! [`Array`]: https://ruby-doc.org/core-2.6.3/Array.html
//! [`Vec`]: alloc::vec::Vec
//! [`SmallVec`]: ::smallvec::SmallVec
//! [inline-capacity]: INLINE_CAPACITY

#[cfg(feature = "small-array")]
pub mod smallvec;
pub mod vec;

/// Vectors that implement the small vector optimization can store 8 elements
/// inline without a heap allocation.
///
/// See [`SmallArray`](smallvec::SmallArray).
#[cfg(feature = "small-array")]
pub const INLINE_CAPACITY: usize = 8;
