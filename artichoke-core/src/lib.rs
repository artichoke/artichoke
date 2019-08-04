#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

//! # artichoke-core
//!
//! artichoke-core crate provides a set of traits that, when implemented, create
//! a complete Ruby interpreter.
//!
//! artichoke-core is a work in progress. When fully functioning, artichoke-core
//! will provide interpreter agnositc implementations of Ruby Core and Ruby
//! Standard Library that pass ruby/spec if an interpreter implements all of the
//! required traits.

use std::error;
use std::fmt;
use std::io;

pub mod convert;
pub mod def;
pub mod eval;
pub mod exception;
pub mod file;
pub mod load;
pub mod state;
pub mod top_self;
pub mod types;
pub mod value;
pub mod warn;

/// Errors returned by Artichoke interpreters.
#[derive(Debug)]
pub enum ArtichokeError {
    /// Failed to create an argspec.
    ArgSpec,
    /// Failed to convert from a Rust type to a [`Value`](value::Value).
    ConvertToRuby(convert::Error<types::Rust, types::Ruby>),
    /// Failed to convert from a [`Value`](value::Value) to a Rust type.
    ConvertToRust(convert::Error<types::Ruby, types::Rust>),
    /// Exception raised during eval.
    ///
    /// See [`Eval`](eval::Eval).
    Exec(exception::Exception),
    /// Unable to initalize interpreter.
    New,
    /// Class or module with this name is not defined in the artichoke VM.
    NotDefined(String),
    /// Arg count exceeds maximum allowed by the VM.
    TooManyArgs { given: usize, max: usize },
    /// Attempted to use an uninitialized interpreter.
    Uninitialized,
    /// Eval or funcall returned an interpreter-internal value.
    UnreachableValue,
    /// [`io::Error`] when interacting with virtual filesystem.
    ///
    /// See `artichoke_vfs`.
    Vfs(io::Error),
}

impl Eq for ArtichokeError {}

impl PartialEq for ArtichokeError {
    fn eq(&self, other: &Self) -> bool {
        // this is a hack because io::Error does not impl PartialEq
        format!("{}", self) == format!("{}", other)
    }
}

impl fmt::Display for ArtichokeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArtichokeError::ArgSpec => write!(f, "could not generate argspec"),
            ArtichokeError::ConvertToRuby(inner) => write!(f, "conversion error: {}", inner),
            ArtichokeError::ConvertToRust(inner) => write!(f, "conversion error: {}", inner),
            ArtichokeError::Exec(backtrace) => write!(f, "{}", backtrace),
            ArtichokeError::New => write!(f, "failed to create mrb interpreter"),
            ArtichokeError::NotDefined(fqname) => write!(f, "{} not defined", fqname),
            ArtichokeError::TooManyArgs { given, max } => write!(
                f,
                "Too many args for funcall. Gave {}, but max is {}",
                given, max
            ),
            ArtichokeError::Uninitialized => write!(f, "mrb interpreter not initialized"),
            ArtichokeError::UnreachableValue => {
                write!(f, "extracted unreachable type from interpreter")
            }
            ArtichokeError::Vfs(err) => write!(f, "mrb vfs io error: {}", err),
        }
    }
}

impl error::Error for ArtichokeError {
    fn description(&self) -> &str {
        "artichoke interpreter error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            ArtichokeError::ConvertToRuby(inner) => Some(inner),
            ArtichokeError::ConvertToRust(inner) => Some(inner),
            ArtichokeError::Vfs(inner) => Some(inner),
            _ => None,
        }
    }
}
