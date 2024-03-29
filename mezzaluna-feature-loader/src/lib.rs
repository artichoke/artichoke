#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::needless_borrow)]
// https://github.com/rust-lang/rust-clippy/pull/5998#issuecomment-731855891
#![allow(clippy::map_err_ignore)]
#![allow(clippy::option_if_let_else)]
#![allow(unknown_lints)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
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

//! Ruby feature loader.
//!
//! An Artichoke Ruby VM may load code (called "features") with several
//! strategies. Features can be loaded from an in-memory virtual filesystem
//! which can also store native extensions, natively from local disk, or via a
//! set of search paths given by the `RUBYLIB` environment variable on
//! interpreter boot.

pub mod loaded_features;
mod loader;

#[doc(inline)]
pub use loaded_features::LoadedFeatures;
pub use loader::Loader;
#[cfg(feature = "rubylib")]
pub use loader::Rubylib;

#[doc(inline)]
#[cfg(feature = "disk")]
pub use same_file::Handle;
