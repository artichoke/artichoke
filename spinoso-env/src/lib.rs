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
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! ENV is a hash-like accessor for environment variables.
//!
//! This module implements the [`ENV`] singleton object from Ruby Core.
//!
//! In Artichoke, the enviroment variable store is modeled as a hash map of
//! byte vector keys and values, e.g. `HashMap<Vec<u8>, Vec<u8>>`. Backends are
//! expected to convert their internals to this representation in their public
//! APIs. For this reason, all APIs exposed by ENV backends in this crate are
//! fallible.
//!
//! You can use this object in your application by accessing it directly. As a
//! Core API, it is globally available:
//!
//! ```ruby
//! ENV['PATH']
//! ENV['PS1'] = 'artichoke> '
//! ```
//!
//! There are two `ENV` implementations in this crate:
//!
//! - [`Memory`], enabled by default, implements an `ENV` store and accessor on
//!   top of a Rust [`HashMap`]. This backend does not query or modify the host
//!   system.
//! - [`System`], enabled when the **system-env** feature is activated, is a
//!   proxy for the system environment and uses platform-specific APIs defined
//!   in the [Rust Standard Library].
//!
//! # Examples
//!
//! Using the in-memory backend allows safely manipulating an emulated environment:
//!
//! ```
//! # use spinoso_env::Memory;
//! # fn example() -> Result<(), spinoso_env::Error> {
//! let mut env = Memory::new();
//! // This does not alter the behavior of the host Rust process.
//! env.put(b"PATH", None)?;
//! // `Memory` backends start out empty.
//! assert_eq!(env.get(b"HOME")?, None);
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! System backends inherit and mutate the environment from the current Rust
//! process:
//!
//! ```
//! # use spinoso_env::System;
//! const ENV: System = System::new();
//! # fn example() -> Result<(), spinoso_env::Error> {
//! ENV.put(b"RUBY", Some(b"Artichoke"))?;
//! assert!(ENV.get(b"PATH")?.is_some());
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! # Crate features
//!
//! This crate requires [`std`], the Rust Standard Library.
//!
//! All features are enabled by default:
//!
//! - **system-env** - Enable an `ENV` backend that accesses the host system's
//!   environment variables via the [`std::env`](module@std::env) module.
//!
//! [`ENV`]: https://ruby-doc.org/core-2.6.3/ENV.html
//! [`HashMap`]: std::collections::HashMap
//! [Rust Standard Library]: std

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

use core::fmt;
use scolapasta_string_escape::format_debug_escape_into;
use std::borrow::Cow;
use std::error;

mod env;

pub use env::memory::Memory;
#[cfg(feature = "system-env")]
pub use env::system::System;

/// Sum type of all errors possibly returned from [`Memory::put`].
///
/// `put` can return errors under several conditions:
///
/// - The name contains a NUL byte.
/// - The name contains an `=` byte.
/// - The value contains a NUL byte.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// Error that indicates an argument parsing or value logic error occurred.
    ///
    /// See [`ArgumentError`].
    Argument(ArgumentError),
    /// Error that indicates the access to the underlying platform APIs failed.
    ///
    /// See [`InvalidError`].
    Invalid(InvalidError),
}

impl From<ArgumentError> for Error {
    fn from(err: ArgumentError) -> Self {
        Self::Argument(err)
    }
}

impl From<InvalidError> for Error {
    fn from(err: InvalidError) -> Self {
        Self::Invalid(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ENV error")
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Argument(ref err) => Some(err),
            Self::Invalid(ref err) => Some(err),
        }
    }
}

/// Error that indicates an argument parsing or value logic error occurred.
///
/// Argument errors have an associated message.
///
/// This error corresponds to the [Ruby `ArgumentError` Exception class].
///
/// # Examples
///
/// ```
/// # use spinoso_env::ArgumentError;
/// let err = ArgumentError::new();
/// assert_eq!(err.message(), "ArgumentError");
///
/// let err = ArgumentError::with_message("bad environment variable name: contains null byte");
/// assert_eq!(err.message(), "bad environment variable name: contains null byte");
/// ```
///
/// [Ruby `ArgumentError` Exception class]: https://ruby-doc.org/core-2.6.3/ArgumentError.html
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgumentError(&'static str);

impl From<&'static str> for ArgumentError {
    fn from(message: &'static str) -> Self {
        Self::with_message(message)
    }
}

impl Default for ArgumentError {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl error::Error for ArgumentError {}

impl ArgumentError {
    /// Construct a new, default argument error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::ArgumentError;
    /// const ERR: ArgumentError = ArgumentError::new();
    /// assert_eq!(ERR.message(), "ArgumentError");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self("ArgumentError")
    }

    /// Construct a new, argument error with a message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::ArgumentError;
    /// const ERR: ArgumentError = ArgumentError::with_message("bad environment variable name: contains null byte");
    /// assert_eq!(ERR.message(), "bad environment variable name: contains null byte");
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_message(message: &'static str) -> Self {
        Self(message)
    }

    /// Retrieve the exception message associated with this argument error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::ArgumentError;
    /// let err = ArgumentError::new();
    /// assert_eq!(err.message(), "ArgumentError");
    ///
    /// let err = ArgumentError::with_message("bad environment variable name: contains null byte");
    /// assert_eq!(err.message(), "bad environment variable name: contains null byte");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        self.0
    }
}

/// Error that indicates the underlying platform API returned an error.
///
/// This error is typically returned by the operating system and corresponds to
/// `EINVAL`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidError(Cow<'static, [u8]>);

impl fmt::Display for InvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_debug_escape_into(f, self.message())
    }
}

impl error::Error for InvalidError {}

impl From<Vec<u8>> for InvalidError {
    fn from(message: Vec<u8>) -> Self {
        Self(Cow::Owned(message))
    }
}

impl InvalidError {
    /// Construct a new, default invalid error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::InvalidError;
    /// const ERR: InvalidError = InvalidError::new();
    /// assert_eq!(ERR.message(), b"Errno::EINVAL");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        let message: &[u8] = b"Errno::EINVAL";
        Self(Cow::Borrowed(message))
    }

    /// Construct a new, invalid error with a message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::InvalidError;
    /// const ERR: InvalidError = InvalidError::with_message("Invalid argument - setenv()");
    /// assert_eq!(ERR.message(), b"Invalid argument - setenv()");
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_message(message: &'static str) -> Self {
        Self(Cow::Borrowed(message.as_bytes()))
    }

    /// Retrieve the exception message associated with this invalid error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_env::InvalidError;
    /// let err = InvalidError::new();
    /// assert_eq!(err.message(), b"Errno::EINVAL");
    /// ```
    #[inline]
    #[must_use]
    pub fn message(&self) -> &[u8] {
        self.0.as_ref()
    }
}
