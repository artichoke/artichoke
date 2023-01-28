#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::manual_let_else)]
#![cfg_attr(test, allow(clippy::non_ascii_literal))]
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

//! Routines for debug escaping Ruby Strings.
//!
//! Ruby Strings are conventionally UTF-8 byte sequences. When calling
//! [`String#inspect`] or [`Symbol#inspect`], these maybe UTF-8 byte strings are
//! escaped to have a valid and printable UTF-8 representation.
//!
//! This crate exposes functions and iterators for encoding arbitrary byte
//! slices as valid, printable UTF-8.
//!
//! # Ruby debug escapes
//!
//! Ruby produces debug escapes that look like:
//!
//! ```console
//! [2.6.3] > "Artichoke Ruby is made with Rust.
//!
//! Invalid UTF-8: \xFF.
//!
//! Slash \\ and quote \" are escaped."
//! => "Artichoke Ruby is made with Rust.\n\nInvalid UTF-8: \xFF.\n\nSlash \\ and quote \" are escaped."
//! ```
//!
//! Ruby escape sequences differ than Rust escape sequences for some characters.
//! For example `0x0C`:
//!
//! ```
//! # use scolapasta_string_escape::Literal;
//! // Rust
//! assert_eq!('\x0C'.escape_debug().collect::<String>(), r"\u{c}");
//! // Ruby
//! assert_eq!(Literal::from(0x0C).as_str(), r"\f");
//! ```
//!
//! # Examples
//!
//! ```
//! # use scolapasta_string_escape::format_debug_escape_into;
//! const EXAMPLE: &[u8] = b"Artichoke Ruby is made with Rust.
//!
//! Invalid UTF-8: \xFF.
//!
//! Slash \\ and quote \" are escaped.";
//!
//! # fn example() -> Result<(), core::fmt::Error> {
//! let mut escaped = String::new();
//! format_debug_escape_into(&mut escaped, EXAMPLE)?;
//! assert_eq!(
//!     escaped,
//!     r#"Artichoke Ruby is made with Rust.\n\nInvalid UTF-8: \xFF.\n\nSlash \\ and quote \" are escaped."#,
//! );
//! # Ok(())
//! # }
//! # example().unwrap();
//! ```
//!
//! # `no_std`
//!
//! This crate is `no_std` compatible. This crate does not depend on [`alloc`].
//!
//! [`String#inspect`]: https://ruby-doc.org/core-3.1.2/String.html#method-i-inspect
//! [`Symbol#inspect`]: https://ruby-doc.org/core-3.1.2/Symbol.html#method-i-inspect
//! [`alloc`]: https://doc.rust-lang.org/alloc/

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

// Having access to `String` in tests is convenient to collect `Inspect`
// iterators for whole content comparisons.
#[cfg(any(test, doctest))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod literal;
mod string;

pub use literal::*;
pub use string::*;
