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
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Functions for working with filesystem paths and loading Ruby source code.
//!
//! # Examples
//!
//! ```
//! # use scolapasta_path::is_explicit_relative;
//! assert!(is_explicit_relative("./test/loader"));
//! assert!(is_explicit_relative("../rake/test_task"));
//!
//! assert!(!is_explicit_relative("json/pure"));
//! assert!(!is_explicit_relative("/artichoke/src/json/pure"));
//! ```

mod paths;

pub use paths::{is_explicit_relative, is_explicit_relative_bytes, memory_loader_ruby_load_path, normalize_slashes};
