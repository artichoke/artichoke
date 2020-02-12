#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs, warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]
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
//!
//! ## License
//!
//! artichoke-core is licensed with the MIT License (c) Ryan Lopopolo.

pub mod convert;
pub mod eval;
pub mod file;
pub mod intern;
pub mod load;
pub mod parser;
pub mod top_self;
pub mod types;
pub mod value;
pub mod warn;
