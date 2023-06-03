#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::module_name_repetitions)]
// TODO: warn on missing docs once crate is API-complete.
// #![warn(missing_docs)]
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

//! A String object holds and manipulates an arbitrary sequence of bytes,
//! typically representing characters.

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod nul_terminated_vec;
mod vec;

pub use imp::Buf;
#[cfg(feature = "nul-terminated")]
use nul_terminated_vec as imp;
#[cfg(not(feature = "nul-terminated"))]
use vec as imp;
