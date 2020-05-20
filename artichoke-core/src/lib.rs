#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

//! # artichoke-core
//!
//! `artichoke-core` crate provides a set of traits that, when implemented,
//! provide a complete Ruby interpreter.
//!
//! [`artichoke-backend`](https://artichoke.github.io/artichoke/artichoke_backend/)
//! is one implementation of the `artichoke-core` traits.
//!
//! ## Core APIs
//!
//! `artichoke-core` contains traits for the core set of APIs an interpreter
//! must implement. The traits in `artichoke-core` define:
//!
//! - APIs a concrete VM must implement to support the Artichoke runtime and
//!   frontends.
//! - How to box polymorphic core types into [Ruby `Value`](value::Value).
//! - [Interoperability](convert) between the VM backend and the
//!   Rust-implemented core.
//!
//! Some of the core APIs a Ruby implementation must provide are
//! [evaluating code](eval::Eval),
//! [converting Rust data structures to boxed `Value`s on the interpreter heap](convert::ConvertMut),
//! and [interning `Symbol`s](intern::Intern).

#![doc(html_root_url = "https://artichoke.github.io/artichoke/artichoke_core")]
#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon.ico")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

pub mod constant;
pub mod convert;
pub mod eval;
pub mod file;
pub mod globals;
pub mod intern;
pub mod io;
pub mod load;
pub mod parser;
pub mod prng;
pub mod regexp;
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
    pub use crate::constant::DefineConstant;
    pub use crate::convert::{Convert, ConvertMut, TryConvert, TryConvertMut};
    pub use crate::eval::Eval;
    pub use crate::file::File;
    pub use crate::globals::Globals;
    pub use crate::intern::Intern;
    pub use crate::io::Io;
    pub use crate::load::LoadSources;
    pub use crate::parser::{IncrementLinenoError, Parser};
    pub use crate::prng::Prng;
    pub use crate::regexp::Regexp;
    pub use crate::top_self::TopSelf;
    pub use crate::types::{Ruby, Rust};
    pub use crate::value::Value;
    pub use crate::warn::Warn;
}
