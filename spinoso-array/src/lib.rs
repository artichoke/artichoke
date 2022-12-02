#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Contiguous growable vector types that implement the [Ruby `Array`] API.
//!
//! `Array` types are growable vectors with potentially heap-allocated contents.
//! The types in this crate can be passed by pointer over FFI.
//!
//! `Array` types implement the mutating APIs in the Ruby `Array` core class as
//! well as indexing and slicing APIs.
//!
//! `spinoso-array` is part of a [collection of crates] that implement the data
//! structures that comprise the Ruby Core and Standard Library implementation
//! for [Artichoke Ruby].
//!
//! # Array types
//!
//! - [`Array`] is based on [`Vec`] from the Rust `alloc` crate and standard
//!   library. This Spinoso array type is enabled by default.
#![cfg_attr(
    feature = "small-array",
    doc = "- [`SmallArray`] is based on [`SmallVec`] and implements the small vector"
)]
#![cfg_attr(
    feature = "small-array",
    doc = "  optimization – small arrays are stored inline without a heap allocation."
)]
#![cfg_attr(
    feature = "small-array",
    doc = "  This Spinoso array type requires the **small-array** Cargo feature."
)]
#![cfg_attr(
    feature = "tiny-array",
    doc = "- [`TinyArray`] is based on [`TinyVec`] and implements the small vector"
)]
#![cfg_attr(
    feature = "tiny-array",
    doc = "  optimization – small arrays are stored inline without a heap allocation."
)]
#![cfg_attr(
    feature = "tiny-array",
    doc = "  This Spinoso array type requires the **tiny-array** Cargo feature."
)]
//!
//!
//! # `no_std`
//!
//! This crate is `no_std` with a required dependency on the [`alloc`] crate.
//!
//! # Examples
//!
//! You can create an [`Array<T>`](Array) with [`new`](Array::new):
//!
//! ```
//! # use spinoso_array::Array;
//! let ary: Array<i32> = Array::new();
//! ```
//!
//! Or with one of the many [`From`] and [`FromIterator`] implementations:
//!
//! ```
//! # use core::iter;
//! # use spinoso_array::Array;
//! let ary: Array<i32> = Array::from(vec![1, 2, 3, 4]);
//! let ary2: Array<i32> = iter::repeat(1).take(10).collect();
//! ```
//!
//! You can [`push`](Array::push) values onto the end of an array (which will
//! grow the array as needed):
//!
//! ```
//! # use spinoso_array::Array;
//! let mut ary = Array::from(&[1, 2]);
//! ary.push(3);
//! ```
//!
//! Popping values behaves similarly:
//!
//! ```
//! # use spinoso_array::Array;
//! let mut ary = Array::from(&[1, 2]);
//! assert_eq!(ary.pop(), Some(2));
//! ```
//!
//! Arrays also support indexing (through the [`Index`] and [`IndexMut`]
//! traits):
//!
//! ```
//! # use spinoso_array::Array;
//! let mut a = Array::from(&[1, 2, 3]);
//! let three = a[2];
//! a[1] = a[1] + 5;
//! ```
//!
//! The `Array` vector types in this crate differ from [`Vec`] in the Rust `std`
//! by offering many specialized slicing and mutation APIs. For example, rather
//! than offering APIs like [`Vec::drain`] and [`Vec::splice`], array types in
//! this crate offer specialized methods like [`shift`], [`shift_n`],
//! [`unshift`], and [`unshift_n`] and `splice`-like methods with
//! [`insert_slice`], and [`set_slice`].
//!
//! ```
//! # use spinoso_array::Array;
//! let mut a = Array::from(&[1, 2, 3]);
//! a.unshift(0);
//! assert_eq!(a, [0, 1, 2, 3]);
//! let b = a.shift_n(10);
//! assert_eq!(a, []);
//! assert_eq!(b, [0, 1, 2, 3]);
//! ```
//!
//! # Panics
//!
//! `Array`s in this crate do not expose panicking slicing operations (except for
//! their [`Index`] and [`IndexMut`] implementations). Instead of panicking,
//! slicing APIs operate until the end of the vector or return `&[]`. Mutating
//! APIs extend `Array`s on out of bounds access.
//!
//! [Ruby `Array`]: https://ruby-doc.org/core-3.1.2/Array.html
//! [collection of crates]: https://crates.io/keywords/spinoso
//! [Artichoke Ruby]: https://www.artichokeruby.org/
//! [`Vec`]: alloc::vec::Vec
#![cfg_attr(feature = "small-array", doc = "[`SmallVec`]: smallvec::SmallVec")]
#![cfg_attr(feature = "tiny-array", doc = "[`TinyVec`]: tinyvec::TinyVec")]
//! [`From`]: core::convert::From
//! [`FromIterator`]: core::iter::FromIterator
//! [`Index`]: core::ops::Index
//! [`IndexMut`]: core::ops::IndexMut
//! [`Vec::drain`]: alloc::vec::Vec::drain
//! [`Vec::splice`]: alloc::vec::Vec::splice
//! [`shift`]: Array::shift
//! [`shift_n`]: Array::shift_n
//! [`unshift`]: Array::unshift
//! [`unshift_n`]: Array::unshift_n
//! [`insert_slice`]: Array::insert_slice
//! [`set_slice`]: Array::set_slice

// This crate is `no_std` + `alloc`
#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

extern crate alloc;

mod array;

#[cfg(feature = "small-array")]
pub use array::smallvec::SmallArray;
#[cfg(feature = "tiny-array")]
pub use array::tinyvec::TinyArray;
pub use array::vec::{Array, RawParts};
#[cfg(any(feature = "small-array", feature = "tiny-array"))]
pub use array::INLINE_CAPACITY;
