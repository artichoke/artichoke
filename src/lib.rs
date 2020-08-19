#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

//! Artichoke Ruby
//!
//! This crate is a Rust and Ruby implementation of the [Ruby programming
//! language][rubylang]. Artichoke is not production-ready, but intends to be a
//! [MRI-compliant][rubyspec] implementation of Ruby 2.6.3.
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
//! # fn example() -> Result<(), Exception> {
//! let mut interp = artichoke::interpreter()?;
//! let result = interp.eval(b"2 + 5")?;
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
//! # fn example() -> Result<(), Exception> {
//! let mut interp = artichoke::interpreter()?;
//! let s = interp.convert_mut("💎");
//! let codepoint = s.funcall(&mut interp, "ord", &[] /* args */, None /* block */)?;
//! let codepoint = codepoint.try_into::<u32>(&interp)?;
//! assert_eq!(128142, codepoint);
//! # Ok(())
//! # }
//! # example().unwrap();
//! ```
//!
//! ## Crate Features
//!
//! All features are enabled by default.
//!
//! - **core-env-system** - Activates the [`std::env`](module@std::env) backend
//!   for the [`ENV` object][core-obj-env]. When this feature is disabled,
//!   access to the environ is emulated with an in-memory store.
//! - **core-math-extra** - Activates additional dependencies to implement some
//!   functions in the [`Math` module][core-mod-math].  When this feature is
//!   disabled, these functions raise `NotImplementedError`.
//! - **core-random** - This includes an implementation of the
//!   [`Random` class][core-class-random]. This feature includes additional
//!   dependencies. When this feature is disabled, Artichoke does not have
//!   support for generating psuedorandom numbers.
//! - **stdlib-securerandom** - An implementation of a CSPRNG for the
//!   [`SecureRandom` module][stdlib-mod-securerandom]. This feature includes
//!   additional dependencies.  When this feature is disabled, the
//!   `SecureRandom` module is not present.
//!
//! [rubylang]: https://www.ruby-lang.org/
//! [rubyspec]: https://github.com/ruby/spec
//! [core-obj-env]: https://ruby-doc.org/core-2.6.3/ENV.html
//! [core-mod-math]: https://ruby-doc.org/core-2.6.3/Math.html
//! [core-class-random]: https://ruby-doc.org/core-2.6.3/Random.html
//! [stdlib-mod-securerandom]: https://ruby-doc.org/stdlib-2.6.3/libdoc/securerandom/rdoc/SecureRandom.html

#![doc(html_root_url = "https://docs.rs/artichoke/0.1.0-pre.0")]
#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon.ico")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

// Ensure code blocks in README.md compile
#[cfg(doctest)]
macro_rules! readme {
    ($x:expr) => {
        #[doc = $x]
        mod readme {}
    };
    () => {
        readme!(include_str!("../README.md"));
    };
}
#[cfg(doctest)]
readme!();

pub use artichoke_backend as backend;

pub mod backtrace;
pub mod parser;
pub mod repl;
pub mod ruby;

/// A "prelude" for users of the `artichoke-backend` crate.
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

    pub use crate::interpreter;
}

pub use artichoke_backend::{Artichoke, Exception};

/// Create a new Artichoke Ruby interpreter.
///
/// # Errors
///
/// If the underlying Artichoke VM backend cannot be initialized, an error is
/// returned.
///
/// If Artichoke Ruby Core or Standard Library cannot be initialized, an error
/// is returned.
pub fn interpreter() -> Result<Artichoke, Exception> {
    let release = prelude::ReleaseMetadata::new()
        .with_ruby_copyright(env!("RUBY_COPYRIGHT"))
        .with_ruby_description(env!("RUBY_DESCRIPTION"))
        .with_ruby_engine_version(env!("CARGO_PKG_VERSION"))
        .with_ruby_patchlevel("0")
        .with_ruby_platform(env!("RUBY_PLATFORM"))
        .with_ruby_release_date(env!("RUBY_RELEASE_DATE"))
        .with_ruby_revision(env!("RUBY_REVISION"))
        .with_ruby_version("2.6.3") // Artichoke targets MRI Ruby 2.6.3
        .with_artichoke_compiler_version(Some(env!("ARTICHOKE_COMPILER_VERSION")));
    artichoke_backend::interpreter_with_config(release)
}
