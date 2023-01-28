#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![allow(clippy::manual_let_else)]
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

//! Functions for working with file system paths and loading Ruby source code.
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
mod platform_string;

pub use paths::{
    absolutize_relative_to, is_explicit_relative, is_explicit_relative_bytes, memory_loader_ruby_load_path,
    normalize_slashes,
};
pub use platform_string::{
    bytes_to_os_str, bytes_to_os_string, os_str_to_bytes, os_string_to_bytes, ConvertBytesError,
};
