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

use std::borrow::Cow;
use std::error;
use std::fmt;

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

/// Errors returned by Artichoke interpreters.
#[derive(Debug, PartialEq, Eq)]
pub enum ArtichokeError {
    /// Failed to convert from a Rust type to a [`Value`](value::Value).
    ConvertToRuby {
        /// Source type of conversion.
        from: types::Rust,
        /// Destination type of conversion.
        to: types::Ruby,
    },
    /// Failed to convert from a [`Value`](value::Value) to a Rust type.
    ConvertToRust {
        /// Source type of conversion.
        from: types::Ruby,
        /// Destination type of conversion.
        to: types::Rust,
    },
    /// Constant name is invalid for the VM backend.
    ///
    /// For example, if the name contains a NUL byte, or is invalid UTF-8.
    InvalidConstantName,
    /// Class or module with this name is not defined in the artichoke VM.
    NotDefined(Cow<'static, str>),
    /// Arg count exceeds maximum allowed by the VM.
    TooManyArgs {
        /// Number of arguments supplied.
        given: usize,
        /// Maximum number of arguments supported.
        max: usize,
    },
    /// Attempted to use an uninitialized interpreter.
    Uninitialized,
    /// Attempted to extract Rust object from uninitialized `Value`.
    UninitializedValue(&'static str),
    /// Eval or funcall returned an interpreter-internal value.
    UnreachableValue,
}

impl fmt::Display for ArtichokeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConvertToRuby { from, to } => {
                write!(f, "Failed to convert from {} to {}", from, to)
            }
            Self::ConvertToRust { from, to } => {
                write!(f, "Failed to convert from {} to {}", from, to)
            }
            Self::InvalidConstantName => write!(f, "Invalid constant name"),
            Self::NotDefined(fqname) => write!(f, "{} not defined", fqname),
            Self::TooManyArgs { given, max } => write!(
                f,
                "Too many args for funcall. Gave {}, but max is {}",
                given, max
            ),
            Self::Uninitialized => write!(f, "Interpreter not initialized"),
            Self::UninitializedValue(class) => write!(
                f,
                "Attempted to extract pointer from uninitialized Value with class {}",
                class
            ),
            Self::UnreachableValue => write!(f, "Extracted unreachable type from interpreter"),
        }
    }
}

impl error::Error for ArtichokeError {}
