#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs, warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

//! # artichoke-core
//!
//! `artichoke-core` crate provides a set of traits that, when implemented,
//! create a complete Ruby interpreter.
//!
//! [`artichoke-backend`](https://artichoke.github.io/artichoke/artichoke_backend/)
//! is one implementation of the `artichoke-core` traits.

use std::error;
use std::fmt;
use std::io;

pub mod convert;
pub mod eval;
pub mod file;
pub mod load;
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
    /// Exception raised during eval.
    ///
    /// See [`Eval`](eval::Eval).
    // TODO: disabled for migration Exec(exception::Exception),
    Exec(String),
    /// Unable to initalize interpreter.
    New,
    /// Class or module with this name is not defined in the artichoke VM.
    NotDefined(String),
    /// Arg count exceeds maximum allowed by the VM.
    TooManyArgs {
        /// Number of arguments supplied.
        given: usize,
        /// Maximum number of arguments supported.
        max: usize,
    },
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

// TODO: remove this impl. I think this is only a kludge for tests.
impl PartialEq for ArtichokeError {
    fn eq(&self, other: &Self) -> bool {
        // this is a hack because io::Error does not impl PartialEq
        format!("{}", self) == format!("{}", other)
    }
}

impl fmt::Display for ArtichokeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ArgSpec => write!(f, "could not generate argspec"),
            Self::ConvertToRuby { from, to } => {
                write!(f, "failed to convert from {} to {}", from, to)
            }
            Self::ConvertToRust { from, to } => {
                write!(f, "failed to convert from {} to {}", from, to)
            }
            Self::Exec(backtrace) => write!(f, "{}", backtrace),
            Self::New => write!(f, "failed to create mrb interpreter"),
            Self::NotDefined(fqname) => write!(f, "{} not defined", fqname),
            Self::TooManyArgs { given, max } => write!(
                f,
                "Too many args for funcall. Gave {}, but max is {}",
                given, max
            ),
            Self::Uninitialized => write!(f, "mrb interpreter not initialized"),
            Self::UnreachableValue => write!(f, "extracted unreachable type from interpreter"),
            Self::Vfs(err) => write!(f, "mrb vfs io error: {}", err),
        }
    }
}

impl error::Error for ArtichokeError {
    fn description(&self) -> &str {
        "artichoke interpreter error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Self::Vfs(inner) => Some(inner),
            _ => None,
        }
    }
}
