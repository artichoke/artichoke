#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::option_if_let_else)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Virtual filesystem.
//!
//! Artichoke proxies all filesystem access through a virtual filesystem. The
//! filesystem can store Ruby sources and extension hooks in memory and will
//! support proxying to the host filesystem for reads and writes.
//!
//! Artichoke uses the virtual filesystem to track metadata about loaded
//! features.
//!
//! Artichoke has several virtual filesystem implementations. Only some of them
//! support reading from the system fs.

#[cfg(feature = "rubylib-native-filesystem-loader")]
pub mod rubylib;

#[doc(inline)]
#[cfg(feature = "rubylib-native-filesystem-loader")]
pub use rubylib::Rubylib;
