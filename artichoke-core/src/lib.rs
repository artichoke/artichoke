#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs, warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]
#![forbid(unsafe_code)]

//! # artichoke-core
//!
//! `artichoke-core` crate provides a set of traits that, when implemented,
//! create a complete Ruby interpreter.
//!
//! [`artichoke-backend`](https://artichoke.github.io/artichoke/artichoke_backend/)
//! is one implementation of the `artichoke-core` traits.
//!
//! ## License
//!
//! artichoke-core is licensed with the [MIT License](/LICENSE) (c) Ryan Lopopolo.

use std::borrow::Cow;
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
    /// [`io::Error`] when interacting with virtual filesystem.
    ///
    /// See `artichoke_vfs`.
    Vfs(io::Error),
}

impl Eq for ArtichokeError {}

// TODO: remove this impl. I think this is only a kludge for tests.
impl PartialEq for ArtichokeError {
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        // this is a hack because io::Error does not impl PartialEq
        format!("{}", self) == format!("{}", other)
    }
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
            Self::Exec(backtrace) => write!(f, "{}", backtrace),
            Self::New => write!(f, "Failed to create interpreter"),
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
            Self::Vfs(err) => write!(f, "mrb vfs io error: {}", err),
        }
    }
}

impl error::Error for ArtichokeError {
    #[must_use]
    fn description(&self) -> &str {
        "Artichoke interpreter error"
    }

    #[must_use]
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Self::Vfs(inner) => Some(inner),
            _ => None,
        }
    }
}
