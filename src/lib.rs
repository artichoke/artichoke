#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

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
//!   rubygems-style [`File`](backend::File) API.
//! - The ability to disable parts of the interpreter VM or library functions at
//!   compile time. For example, deny access to the system environ by disabling
//!   the `core-env-system` feature.
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
//!
//! ## Crate Features
//!
//! - `core-env-system` - **Enabled** by default. This activates the `std::env`
//!   backend for the [`ENV` object][core-obj-env]. When this feature is
//!   disabled, access to the environ is emulated with an in-memory store.
//! - `core-math-extra` - **Enabled** by default. This enables additional
//!   dependencies to implement some functions in the
//!   [`Math` module][core-mod-math].  When this feature is disabled, these
//!   functions raise `NotImplementedError`.
//! - `core-random` - **Enabled** by default. This includes an implementation of
//!   the [`Random` class][core-class-random]. This feature includes additional
//!   dependencies. When this feature is disabled, Artichoke does not have
//!   support for generating psuedorandom numbers.
//! - `stdlib-securerandom` - **Enabled** by default. This feature includes an
//!   implementation of a CSPRNG for the
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

pub mod backtrace;
pub mod parser;
pub mod repl;
pub mod ruby;
