#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::let_underscore_untyped)] // https://github.com/rust-lang/rust-clippy/pull/10442#issuecomment-1516570154
#![allow(clippy::manual_let_else)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::question_mark)] // https://github.com/rust-lang/rust-clippy/issues/8281
#![allow(clippy::unnecessary_lazy_evaluations)] // https://github.com/rust-lang/rust-clippy/issues/8109
#![cfg_attr(test, allow(clippy::non_ascii_literal))]
#![allow(unknown_lints)]
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

//! # artichoke-backend
//!
//! `artichoke-backend` crate provides a Ruby interpreter. It is currently
//! implemented with [mruby](https://github.com/mruby/mruby) bindings exported
//! by the [`sys`] module.
//!
//! ## Execute Ruby Code
//!
//! `artichoke-backend` crate exposes several mechanisms for executing Ruby code
//! on the interpreter.
//!
//! ### Evaling Source Code
//!
//! The `artichoke-backend` interpreter implements
//! [`Eval` from `artichoke-core`](crate::core::Eval).
//!
//! ```rust
//! use artichoke_backend::prelude::*;
//!
//! # fn example() -> Result<(), Error> {
//! let mut interp = artichoke_backend::interpreter()?;
//! let result = interp.eval(b"10 * 10")?;
//! let result = result.try_convert_into::<i64>(&interp)?;
//! assert_eq!(result, 100);
//! # interp.close();
//! # Ok(())
//! # }
//! # example().unwrap();
//! ```
//!
//! ### Calling Functions on Ruby Objects
//!
//! [`Value`](value::Value)s returned by the `artichoke-backend` interpreter
//! implement [`Value` from `artichoke-core`](crate::core::Value), which enables
//! calling Ruby functions from Rust.
//!
//! ```rust
//! use artichoke_backend::prelude::*;
//!
//! # fn example() -> Result<(), Error> {
//! let mut interp = artichoke_backend::interpreter()?;
//! let result = interp.eval(b"'ruby funcall'")?;
//! let result = result.funcall(&mut interp, "length", &[], None)?;
//! let result = result.try_convert_into::<i64>(&mut interp)?;
//! assert_eq!(result, 12);
//! # interp.close();
//! # Ok(())
//! # }
//! # example().unwrap();
//! ```
//!
//! ## Virtual File System and `Kernel#require`
//!
//! The `artichoke-backend` interpreter includes an in-memory virtual
//! file system.  The file system stores Ruby sources and Rust extension functions
//! that are similar to MRI C extensions.
//!
//! The virtual file system enables applications built with `artichoke-backend`
//! to `require` sources that are embedded in the binary without host file system
//! access.
//!
//! ## Embed Rust Types in Ruby `Value`s
//!
//! `artichoke-backend` exposes a concept similar to `data`-typed values in MRI
//! and mruby.
//!
//! When Rust types implement a special trait, they can be embedded in a Ruby
//! [`Value`](value::Value) and passed through the Ruby VM as a Ruby object.
//! Classes defined in this way can define methods in Rust or Ruby.
//!
//! Examples of these types include:
//!
//! - `Regexp` and `MatchData`, which are backed by regular expressions from the
//!   `onig` and `regex` crates.
//! - `ENV`, which glues Ruby to an environ backend.
//!
//! ## Converters Between Ruby and Rust Types
//!
//! The [`convert` module](convert) provides implementations for conversions
//! between boxed Ruby values and native Rust types like `i64` and
//! `HashMap<String, Option<Vec<u8>>>` using an `artichoke-backend` interpreter.

#![doc(html_root_url = "https://artichoke.github.io/artichoke/artichoke_backend")]
#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon-32x32.png")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

#[macro_use]
#[doc(hidden)]
pub mod macros;

mod artichoke;
pub mod block;
pub mod class;
mod class_registry;
mod coerce_to_numeric;
mod constant;
pub mod convert;
mod debug;
pub mod def;
pub mod error;
mod eval;
pub mod exception_handler;
pub mod extn;
pub mod ffi;
pub mod fmt;
pub mod gc;
mod globals;
mod hash;
mod intern;
mod interpreter;
mod io;
mod load;
pub mod load_path;
pub mod method;
pub mod module;
mod module_registry;
mod parser;
mod platform_string;
#[cfg(feature = "core-random")]
mod prng;
#[cfg(feature = "core-regexp")]
mod regexp;
pub mod release_metadata;
pub mod state;
pub mod sys;
mod top_self;
pub mod types;
pub mod value;
mod warn;

#[cfg(test)]
mod test;

pub use artichoke_core::prelude as core;

pub use crate::artichoke::{Artichoke, Guard};
pub use crate::error::{Error, RubyException};
pub use crate::interpreter::{interpreter, interpreter_with_config};

/// A "prelude" for users of the `artichoke-backend` crate.
///
/// This prelude is similar to the standard library's prelude in that you'll
/// almost always want to import its entire contents, but unlike the standard
/// library's prelude, you'll have to do so manually:
///
/// ```
/// use artichoke_backend::prelude::*;
/// ```
///
/// The prelude may grow over time as additional items see ubiquitous use.
pub mod prelude {
    pub use artichoke_core::prelude::*;
    pub use spinoso_exception::core::*;

    pub use crate::error::{self, Error, RubyException};
    pub use crate::gc::MrbGarbageCollection;
    pub use crate::release_metadata::ReleaseMetadata;
    pub use crate::{Artichoke, Guard};
}
