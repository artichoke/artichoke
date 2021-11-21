#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::option_if_let_else)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
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

//! The Ruby Math module.
//!
//! The Math module contains module functions for basic trigonometric and
//! transcendental functions. See class [`Float`] for a list of constants that
//! define Ruby's floating point accuracy.
//!
//! This crate defines math operations as free functions. These functions differ
//! from those defined in Rust [`core`] by returning a [`DomainError`] when an
//! input is outside the domain of the function and results in [`NaN`].
//!
//! `spinoso-math` assumes the Ruby VM uses double precision [`f64`] floats.
//!
//! # Examples
//!
//! Compute the hypotenuse:
//!
//! ```
//! use spinoso_math as math;
//! assert_eq!(math::hypot(3.0, 4.0), 5.0);
//! ```
//!
//! Compute log with respect to the base 10 and handle domain errors:
//!
//! ```
//! use spinoso_math as math;
//! assert_eq!(math::log10(1.0), Ok(0.0));
//! assert_eq!(math::log10(10.0), Ok(1.0));
//! assert_eq!(math::log10(1e100), Ok(100.0));
//!
//! assert_eq!(math::log10(0.0), Ok(f64::NEG_INFINITY));
//! assert!(math::log10(-0.1).is_err());
//!
//! // A NaN return value is distinct from a `DomainError`.
//! assert!(matches!(math::log10(f64::NAN), Ok(result) if result.is_nan()));
//! ```
//!
//! # Crate features
//!
//! All features are enabled by default.
//!
//! - **full** - Enables implementations of math functions that do not have
//!   implementations in Rust [`core`]. Dropping this feature removes the
//!   [`libm`] dependency.
//!
//! [`Float`]: https://ruby-doc.org/core-2.6.3/Float.html
//! [`NaN`]: f64::NAN
//! [`alloc`]: https://doc.rust-lang.org/alloc/

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

#[doc(inline)]
pub use core::f64::consts::E;
#[doc(inline)]
pub use core::f64::consts::PI;
use core::fmt;
use std::error;

mod math;

pub use math::*;

/// A handle to the `Math` module.
///
/// This is a copy zero-sized type with no associated methods. This type exists
/// so a Ruby VM can attempt to unbox this type and statically dispatch to
/// functions defined in this crate.
///
/// # Examples
///
/// ```
/// # use spinoso_math::Math;
/// const MATH: Math = Math::new();
/// ```
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Math {
    _private: (),
}

impl Math {
    /// Constructs a new, default `Math`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::Math;
    /// const MATH: Math = Math::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

/// Sum type of all errors possibly returned from `Math` functions.
///
/// Math functions in `spinoso-math` return errors in the following conditions:
///
/// - The parameters evaluate to a result that is out of range.
/// - The function is not implemented due to missing compile-time flags.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// Error that indicates a math function returned a value that was out of
    /// range.
    ///
    /// This error can be used to differentiate between [`NaN`](f64::NAN) inputs
    /// and what would be `NaN` outputs.
    ///
    /// See [`DomainError`].
    Domain(DomainError),
    /// Error that indicates a `Math` module function is not implemented.
    ///
    /// See [`NotImplementedError`].
    NotImplemented(NotImplementedError),
}

impl Error {
    /// Retrieve the exception message associated with this error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::{DomainError, Error, NotImplementedError};
    /// let err = Error::from(DomainError::new());
    /// assert_eq!(err.message(), "Math::DomainError");
    ///
    /// let err = Error::from(NotImplementedError::with_message(
    ///     "Artichoke was not built with Math::erf support"
    /// ));
    /// assert_eq!(err.message(), "Artichoke was not built with Math::erf support");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::Domain(err) => err.message(),
            Self::NotImplemented(err) => err.message(),
        }
    }
}

impl From<DomainError> for Error {
    #[inline]
    fn from(err: DomainError) -> Self {
        Self::Domain(err)
    }
}

impl From<NotImplementedError> for Error {
    #[inline]
    fn from(err: NotImplementedError) -> Self {
        Self::NotImplemented(err)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Math error")
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Domain(ref err) => Some(err),
            Self::NotImplemented(ref err) => Some(err),
        }
    }
}

/// Error that indicates a math function evaluated to an out of range value.
///
/// Domain errors have an associated message.
///
/// This error corresponds to the [Ruby `Math::DomainError` Exception class]. It
/// can be used to differentiate between [`NaN`](f64::NAN) inputs and what would
/// be `NaN` outputs.
///
/// # Examples
///
/// ```
/// # use spinoso_math::DomainError;
/// let err = DomainError::new();
/// assert_eq!(err.message(), "Math::DomainError");
///
/// let err = DomainError::with_message(
///     r#"Numerical argument is out of domain - "acos""#,
/// );
/// assert_eq!(err.message(), r#"Numerical argument is out of domain - "acos""#);
/// ```
///
/// [Ruby `Math::DomainError` Exception class]: https://ruby-doc.org/core-2.6.3/Math/DomainError.html
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DomainError(&'static str);

impl From<&'static str> for DomainError {
    #[inline]
    fn from(message: &'static str) -> Self {
        Self(message)
    }
}

impl DomainError {
    /// Construct a new, default domain error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::DomainError;
    /// const ERR: DomainError = DomainError::new();
    /// assert_eq!(ERR.message(), "Math::DomainError");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        // ```
        // [2.6.3] > Math::DomainError.new.message
        // => "Math::DomainError"
        // ```
        Self("Math::DomainError")
    }

    /// Construct a new, domain error with a message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::DomainError;
    /// const ERR: DomainError = DomainError::with_message(
    ///     r#"Numerical argument is out of domain - "acos""#,
    /// );
    /// assert_eq!(ERR.message(), r#"Numerical argument is out of domain - "acos""#);
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_message(message: &'static str) -> Self {
        Self(message)
    }

    /// Retrieve the exception message associated with this error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::DomainError;
    /// let err = DomainError::new();
    /// assert_eq!(err.message(), "Math::DomainError");
    ///
    /// let err = DomainError::with_message(
    ///     r#"Numerical argument is out of domain - "acos""#,
    /// );
    /// assert_eq!(err.message(), r#"Numerical argument is out of domain - "acos""#);
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for DomainError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl error::Error for DomainError {}

/// Error that indicates a `Math` module function is not implemented.
///
/// Some math functions are not available in the [Rust core library] and require
/// this crate to be built with extra compile-time features to enable [additional
/// dependencies].
///
/// Not implemented errors have an associated message.
///
/// This error corresponds to the [Ruby `NotImplementedError` Exception class].
///
/// # Examples
///
/// ```
/// # use spinoso_math::NotImplementedError;
/// let err = NotImplementedError::new();
/// assert_eq!(err.message(), "NotImplementedError");
///
/// let err = NotImplementedError::with_message(
///     "Artichoke was not built with Math::erf support"
/// );
/// assert_eq!(err.message(), "Artichoke was not built with Math::erf support");
/// ```
///
/// [Rust core library]: https://doc.rust-lang.org/std/primitive.f64.html
/// [additional dependencies]: https://crates.io/crates/libm
/// [Ruby `NotImplementedError` Exception class]: https://ruby-doc.org/core-2.6.3/NotImplementedError.html
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotImplementedError(&'static str);

impl NotImplementedError {
    /// Construct a new, default not implemented error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::NotImplementedError;
    /// const ERR: NotImplementedError = NotImplementedError::new();
    /// assert_eq!(ERR.message(), "NotImplementedError");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self("NotImplementedError")
    }

    /// Construct a new, not implemented error with a message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::NotImplementedError;
    /// const ERR: NotImplementedError = NotImplementedError::with_message(
    ///     "Artichoke was not built with Math::erf support"
    /// );
    /// assert_eq!(ERR.message(), "Artichoke was not built with Math::erf support");
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_message(message: &'static str) -> Self {
        Self(message)
    }

    /// Retrieve the exception message associated with this not implemented
    /// error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_math::NotImplementedError;
    /// let err = NotImplementedError::new();
    /// assert_eq!(err.message(), "NotImplementedError");
    ///
    /// let err = NotImplementedError::with_message(
    ///     "Artichoke was not built with Math::erf support"
    /// );
    /// assert_eq!(err.message(), "Artichoke was not built with Math::erf support");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        self.0
    }
}

impl From<&'static str> for NotImplementedError {
    #[inline]
    fn from(message: &'static str) -> Self {
        Self(message)
    }
}

impl fmt::Display for NotImplementedError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl error::Error for NotImplementedError {}
