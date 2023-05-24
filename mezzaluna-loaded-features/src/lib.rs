#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::question_mark)] // https://github.com/rust-lang/rust-clippy/issues/8281
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

//! A container for storing loaded features in a Ruby VM.
//!
//! The Artichoke Ruby VM may load code (called "features") with several
//! strategies. Features can be loaded from an in-memory virtual file system
//! (which can also store native extensions) or natively from local disk.
//!
//! The data structures in this crate track which features have been loaded
//! with support for deduplicating features which may reside at multiple paths.
//!
//! # Examples
//!
//! ```
//! use mezzaluna_loaded_features::{Feature, LoadedFeatures};
//!
//! let mut features = LoadedFeatures::new();
//! features.insert(Feature::with_in_memory_path("/src/_lib/test.rb".into()));
//! features.insert(Feature::with_in_memory_path("set.rb".into()));
//! features.insert(Feature::with_in_memory_path("artichoke.rb".into()));
//!
//! for f in features.features() {
//!     println!("Loaded feature at: {}", f.path().display());
//! }
//! ```

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

mod feature;
pub mod loaded_features;

pub use feature::Feature;
#[doc(inline)]
pub use loaded_features::LoadedFeatures;
#[doc(inline)]
#[cfg(feature = "disk")]
#[cfg_attr(docsrs, doc(cfg(feature = "disk")))]
pub use same_file::Handle;
