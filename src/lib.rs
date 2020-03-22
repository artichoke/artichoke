#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

//! Artichoke Ruby
//!
//! This crate is a Rust, and Ruby implementation of the Ruby programming
//! language. Artichoke is not production-ready, but intends to be a
//! [MRI-compliant](https://github.com/ruby/spec) implementation of Ruby 2.6.3.
//!
//! This crate provides:
//!
//! - An embeddable Ruby interpreter, which can be crated with the
//!   [`interpreter`] function.
//! - A Rust and Ruby implementation of Ruby Core and Standard Library using
//!   high-quality Rust dependencies and modern Ruby.
//! - Support for injecting Rust code and types into the interpreter with a
//!   rubygems-style [`File`](backend::File) API.
//! - The ability to disable parts of the interpreter VM or library functions at
//!   compile time. For example, access to the system environ can be denied by
//!   disabling the `core-env-system` feature.
//!
//! ## Usage
//!
//! You can create an interpreter and begin executing code on it:
//!
//! ```
//! # use artichoke::backend::{Eval, ValueLike};
//! # fn main() -> Result<(), artichoke::backend::exception::Exception> {
//! let mut interp = artichoke::interpreter()?;
//! let result = interp.eval(b"2 + 5")?;
//! # Ok(())
//! # }
//! ```
//!
//! Artichoke supports calling Ruby functions from Rust and converting between
//! Ruby boxed values and Rust native types:
//!
//! ```
//! # use artichoke::backend::{ConvertMut, ValueLike};
//! # fn main() -> Result<(), artichoke::backend::exception::Exception> {
//! let mut interp = artichoke::interpreter()?;
//! let s = interp.convert_mut("💎");
//! let codepoint = s.funcall::<u32>("ord", &[] /* args */, None /* block parameter */)?;
//! assert_eq!(128142, codepoint);
//! # Ok(())
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/artichoke/0.1.0-pre.0")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
#[cfg(doctest)]
doc_comment::doctest!("../artichoke-backend/README.md");
#[cfg(doctest)]
doc_comment::doctest!("../artichoke-core/README.md");
#[cfg(doctest)]
doc_comment::doctest!("../spec-runner/README.md");

pub use artichoke_backend as backend;
pub use backend::interpreter;

pub mod parser;
pub mod repl;
pub mod ruby;
