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

//! This crate provides a set of traits that, when implemented, comprise a
//! complete Ruby interpreter.
//!
//! `artichoke-core` is `no_std` + `alloc` with an optional (enabled by default)
//! `std` feature.
//!
//! Interpreters implement the traits in Artichoke Core to indicate which
//! capabilities they offer. Defining interpreters by their capabilities allows
//! for interpreter agnostic implementations of Ruby Core and Standard Library.
//!
//! # Interpreter APIs
//!
//! Artichoke Core defines traits for the following interpreter capabilities:
//!
//! - [`DefineConstant`](constant::DefineConstant): Define global, class, and
//!   module constants to be arbitrary Ruby [`Value`](value::Value)s.
//! - [`Eval`](eval::Eval): Execute Ruby source code on an interpreter from
//!   various sources.
//! - [`Globals`](globals::Globals): Get, set, and unset interpreter-level
//!   global variables.
//! - [`Hash`](hash::Hash): Hashing functions such as building hashers.
//! - [`Intern`](intern::Intern): Intern byte strings to a cheap to copy and
//!   compare symbol type.
//! - [`Io`](io::Io): External I/O APIs, such as writing to the standard output
//!   of the current process.
//! - [`LoadSources`](load::LoadSources): [Require][Kernel#require] source code
//!   from interpreter disk or [`File`](file::File) gems.
//! - [`Parser`](parser::Parser): Manipulate the parser state, e.g. setting the
//!   current filename.
//! - [`Prng`](prng::Prng): An interpreter-level pseudorandom number generator
//!   that is the backend for [`Random::DEFAULT`].
//! - [`Regexp`](regexp::Regexp): Manipulate [`Regexp`] global state.
//! - [`ReleaseMetadata`](release_metadata::ReleaseMetadata): Enable
//!   interpreters to describe themselves.
//! - [`TopSelf`](top_self::TopSelf): Access to the root execution context.
//! - [`Warn`](warn::Warn): Emit warnings.
//!
//! Artichoke Core also describes what capabilities a Ruby
//! [`Value`](value::Value) must have and how to [convert] between Ruby VM and
//! Rust types.
//!
//! # Examples
//!
//! [`artichoke-backend`](https://artichoke.github.io/artichoke/artichoke_backend/)
//! is one implementation of the `artichoke-core` traits.
//!
//! To use all the APIs defined in Artichoke Core, bring the traits into
//! scope by importing the prelude:
//!
//! ```
//! use artichoke_core::prelude::*;
//! ```
//!
//! # Crate features
//!
//! All features are enabled by default:
//!
//! - **std**: By default, `artichoke-core` is `no_std` + `alloc`. Enabling
//!   this feature adds several trait methods that depend on `OsStr` and `Path`
//!   as well as several implementations of `std::error::Error`.
//!
//! [Kernel#require]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
//! [`Random::DEFAULT`]: https://ruby-doc.org/core-2.6.3/Random.html#DEFAULT
//! [`Regexp`]: https://ruby-doc.org/core-2.6.3/Regexp.html#class-Regexp-label-Special+global+variables
//! [convert]: crate::convert

#![no_std]
#![doc(html_root_url = "https://artichoke.github.io/artichoke/artichoke_core")]
#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon-32x32.png")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod class_registry;
pub mod coerce_to_numeric;
pub mod constant;
pub mod convert;
pub mod debug;
pub mod eval;
pub mod file;
pub mod globals;
pub mod hash;
pub mod intern;
pub mod io;
pub mod load;
pub mod module_registry;
pub mod parser;
pub mod prng;
pub mod regexp;
pub mod release_metadata;
pub mod top_self;
pub mod types;
pub mod value;
pub mod warn;

/// A "prelude" for users of the `artichoke-core` crate.
///
/// This prelude is similar to the standard library's prelude in that you'll
/// almost always want to import its entire contents, but unlike the standard
/// library's prelude, you'll have to do so manually:
///
/// ```
/// use artichoke_core::prelude::*;
/// ```
///
/// The prelude may grow over time as additional items see ubiquitous use.
pub mod prelude {
    pub use crate::class_registry::ClassRegistry;
    pub use crate::coerce_to_numeric::CoerceToNumeric;
    pub use crate::constant::DefineConstant;
    pub use crate::convert::{Convert, ConvertMut, TryConvert, TryConvertMut};
    pub use crate::debug::Debug;
    pub use crate::eval::Eval;
    pub use crate::file::File;
    pub use crate::globals::Globals;
    pub use crate::hash::Hash;
    pub use crate::intern::Intern;
    pub use crate::io::Io;
    pub use crate::load::LoadSources;
    pub use crate::module_registry::ModuleRegistry;
    pub use crate::parser::{IncrementLinenoError, Parser};
    pub use crate::prng::Prng;
    pub use crate::regexp::Regexp;
    pub use crate::release_metadata::ReleaseMetadata;
    pub use crate::top_self::TopSelf;
    pub use crate::types::{Ruby, Rust};
    pub use crate::value::Value;
    pub use crate::warn::Warn;
}
