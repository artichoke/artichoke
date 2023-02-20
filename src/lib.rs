#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

//! Artichoke Ruby
//!
//! This crate is a Rust and Ruby implementation of the [Ruby programming
//! language][rubylang]. Artichoke is not production-ready, but intends to be a
//! [MRI-compliant][rubyspec] implementation of [recent MRI Ruby][mri-target].
//!
//! [mri-target]: https://github.com/artichoke/artichoke/blob/trunk/RUBYSPEC.md#mri-target
//!
//! This crate provides:
//!
//! - An embeddable Ruby interpreter, which can be created with the
//!   [`interpreter`] function.
//! - A Rust and Ruby implementation of Ruby Core and Standard Library using
//!   high-quality Rust dependencies and modern Ruby.
//! - Support for injecting Rust code and types into the interpreter with a
//!   rubygems-style [`File`](prelude::File) API.
//! - The ability to disable parts of the interpreter VM or library functions at
//!   compile time. For example, deny access to the system environ by disabling
//!   the `core-env-system` feature.
//!
//! ## Usage
//!
//! You can create an interpreter and begin executing code on it:
//!
//! ```
//! use artichoke::prelude::*;
//!
//! # fn example() -> Result<(), Error> {
//! let mut interp = artichoke::interpreter()?;
//! let result = interp.eval(b"2 + 5")?;
//! # interp.close();
//! # Ok(())
//! # }
//! # example().unwrap();
//! ```
//!
//! Artichoke supports calling Ruby functions from Rust and converting between
//! Ruby boxed values and Rust native types:
//!
//! ```
//! use artichoke::prelude::*;
//!
//! # fn example() -> Result<(), Error> {
//! let mut interp = artichoke::interpreter()?;
//! let s = interp.try_convert_mut("💎")?;
//! let codepoint = s.funcall(
//!     &mut interp,
//!     "ord",
//!     &[],  /* args */
//!     None, /* block */
//! )?;
//! let codepoint = codepoint.try_convert_into::<u32>(&interp)?;
//! assert_eq!(128142, codepoint);
//! # interp.close();
//! # Ok(())
//! # }
//! # example().unwrap();
//! ```
//!
//! ## Crate Features
//!
//! All features are enabled by default.
//!
//! The features exposed by this crate are unstable. Please refer to the
//! documentation in the [source `Cargo.toml`].
//!
//! [rubylang]: https://www.ruby-lang.org/
//! [rubyspec]: https://github.com/ruby/spec
//! [source `Cargo.toml`]: https://github.com/artichoke/artichoke/blob/trunk/Cargo.toml

#![doc(html_root_url = "https://docs.rs/artichoke/0.1.0-pre.0")]
#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon-32x32.png")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

pub use artichoke_backend as backend;

#[cfg(feature = "backtrace")]
pub mod backtrace;
#[cfg(feature = "cli")]
mod filename;
pub mod parser;
#[cfg(feature = "cli")]
mod readline_bind_mode;
#[cfg(feature = "cli")]
pub mod repl;
#[cfg(feature = "cli")]
pub mod ruby;

/// A "prelude" for users of the `artichoke` crate.
///
/// This prelude is similar to the standard library's prelude in that you'll
/// almost always want to import its entire contents, but unlike the standard
/// library's prelude, you'll have to do so manually:
///
/// ```
/// use artichoke::prelude::*;
/// ```
///
/// The prelude may grow over time as additional items see ubiquitous use.
pub mod prelude {
    pub use artichoke_backend::prelude::*;
}

use artichoke_backend::release_metadata::ReleaseMetadata;
pub use artichoke_backend::{Artichoke, Error};

/// Create a new Artichoke Ruby interpreter.
///
/// # Errors
///
/// If the underlying Artichoke VM backend cannot be initialized, an error is
/// returned.
///
/// If Artichoke Ruby Core or Standard Library cannot be initialized, an error
/// is returned.
pub fn interpreter() -> Result<Artichoke, Error> {
    let release = ReleaseMetadata::new()
        .with_ruby_copyright(env!("RUBY_COPYRIGHT"))
        .with_ruby_description(env!("RUBY_DESCRIPTION"))
        .with_ruby_engine_version(env!("CARGO_PKG_VERSION"))
        .with_ruby_patchlevel("0")
        .with_ruby_platform(env!("RUBY_PLATFORM"))
        .with_ruby_release_date(env!("RUBY_RELEASE_DATE"))
        .with_ruby_revision(env!("RUBY_REVISION"))
        .with_ruby_version("3.1.2") // Artichoke targets MRI Ruby 3.1.2
        .with_artichoke_compiler_version(Some(env!("ARTICHOKE_COMPILER_VERSION")));
    artichoke_backend::interpreter_with_config(release)
}
