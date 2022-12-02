#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
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

//! An implementation of [Ruby's pseudo-random number generator][ruby-random],
//! or PRNG.
//!
//! The PRNG produces a deterministic sequence of bits which approximate true
//! randomness. The sequence may be represented by integers, floats, or binary
//! strings.
//!
//! The generator may be initialized with either a system-generated or
//! user-supplied seed value.
//!
//! PRNGs are currently implemented as a modified Mersenne Twister with a period
//! of 2**19937-1.
//!
//! # Implementation notes
//!
//! This RNG reproduces the same random bytes and floats as MRI. It may differ
//! when returning elements confined to a distribution.
//!
//! # Examples
//!
//! Generate integers:
//!
//! ```
//! # use spinoso_random::Random;
//! let seed = [627457_u32, 697550, 16438, 41926];
//! let mut random = Random::with_array_seed(seed);
//! let rand = random.next_int32();
//! ```
//!
//! Generate random numbers in a range:
//!
//! ```
//! # #[cfg(feature = "random-rand")]
//! # use spinoso_random::{rand, Error, Max, Rand, Random};
//! # #[cfg(feature = "random-rand")]
//! # fn example() -> Result<(), Error> {
//! let mut random = Random::new()?;
//! let max = Max::Integer(10);
//! let mut rand = rand(&mut random, max)?;
//! assert!(matches!(rand, Rand::Integer(x) if x < 10));
//! # Ok(())
//! # }
//! # #[cfg(feature = "random-rand")]
//! # example().unwrap();
//! ```
//!
//! # `no_std`
//!
//! This crate is `no_std` compatible when built without the `std` feature. This
//! crate does not depend on [`alloc`].
//!
//! # Crate features
//!
//! All features are enabled by default.
//!
//! - **random-rand** - Enables range sampling methods for the [`rand()`]
//!   function.  Activating this feature also activates the **rand-traits**
//!   feature. Dropping this feature removes the [`rand`] dependency.
//! - **rand-traits** - Enables implementations of [`RngCore`] on the [`Random`]
//!   type. Dropping this feature removes the [`rand_core`] dependency.
//! - **std** - Enables a dependency on the Rust Standard Library. Activating
//!   this feature enables [`std::error::Error`] impls on error types in this
//!   crate.
//!
#![cfg_attr(
    not(feature = "std"),
    doc = "[`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html"
)]
#![cfg_attr(feature = "rand-traits", doc = "[`RngCore`]: rand_core::RngCore")]
#![cfg_attr(
    not(feature = "rand-traits"),
    doc = "[`RngCore`]: https://docs.rs/rand_core/latest/rand_core/trait.RngCore.html"
)]
#![cfg_attr(feature = "rand-traits", doc = "[`rand_core`]: ::rand_core")]
#![cfg_attr(
    not(feature = "rand-traits"),
    doc = "[`rand_core`]: https://docs.rs/rand_core/latest/rand_core/"
)]
#![cfg_attr(feature = "random-rand", doc = "[`rand`]: ::rand")]
#![cfg_attr(not(feature = "random-rand"), doc = "[`rand`]: https://docs.rs/rand/latest/rand/")]
#![cfg_attr(
    not(feature = "random-rand"),
    doc = "[`rand()`]: https://artichoke.github.io/artichoke/spinoso_random/fn.rand.html"
)]
//! [ruby-random]: https://ruby-doc.org/core-3.1.2/Random.html
//! [`alloc`]: https://doc.rust-lang.org/alloc/

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

extern crate alloc;

#[cfg(any(feature = "std", test, doctest))]
extern crate std;

use core::fmt;
#[cfg(feature = "std")]
use std::error;

#[cfg(feature = "random-rand")]
mod rand;
mod random;
mod urandom;

pub use random::{new_seed, seed_to_key, Random};
pub use urandom::urandom;

#[cfg(feature = "random-rand")]
pub use self::rand::{rand, Max, Rand};

/// Sum type of all errors possibly returned from `Random` functions.
///
/// Random functions in `spinoso-random` return errors in the following
/// conditions:
///
/// - The platform source of cryptographic randomness is unavailable.
/// - The platform source of cryptographic randomness does not have sufficient
///   entropy to return the requested bytes.
/// - Constraints for bounding random numbers are invalid.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Error {
    #[cfg_attr(
        feature = "random-rand",
        doc = "Error that indicates [`rand()`] was passed an invalid constraint."
    )]
    ///
    /// See [`ArgumentError`].
    Argument(ArgumentError),
    /// Error that indicates that [`Random::new`] failed to generate a random
    /// seed.
    ///
    /// See [`InitializeError`].
    Initialize(InitializeError),
    /// Error that indicates that [`new_seed`] failed to generate a random seed.
    ///
    /// See [`NewSeedError`].
    NewSeed(NewSeedError),
    /// Error that indicates that [`urandom()`] failed to generate the requested
    /// random bytes from the platform source of randomness.
    ///
    /// See [`UrandomError`].
    Urandom(UrandomError),
}

impl From<ArgumentError> for Error {
    #[inline]
    fn from(err: ArgumentError) -> Self {
        Self::Argument(err)
    }
}

impl From<InitializeError> for Error {
    #[inline]
    fn from(err: InitializeError) -> Self {
        Self::Initialize(err)
    }
}

impl From<NewSeedError> for Error {
    #[inline]
    fn from(err: NewSeedError) -> Self {
        Self::NewSeed(err)
    }
}

impl From<UrandomError> for Error {
    #[inline]
    fn from(err: UrandomError) -> Self {
        Self::Urandom(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Random error")
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Argument(ref err) => Some(err),
            Self::Initialize(ref err) => Some(err),
            Self::NewSeed(ref err) => Some(err),
            Self::Urandom(ref err) => Some(err),
        }
    }
}

/// Error that indicates a `Random` random number generator failed to
/// initialize.
///
/// When initializing an [`Random`] with a random seed, gathering entropy from
/// the host system can fail.
///
/// This error corresponds to the [Ruby `RuntimeError` Exception class].
///
/// # Examples
///
/// ```
/// # use spinoso_random::InitializeError;
/// let err = InitializeError::new();
/// assert_eq!(err.message(), "failed to get urandom");
/// ```
///
/// [Ruby `RuntimeError` Exception class]: https://ruby-doc.org/core-3.1.2/RuntimeError.html
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InitializeError {
    _private: (),
}

impl InitializeError {
    /// Construct a new, default initialize error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::InitializeError;
    /// const ERR: InitializeError = InitializeError::new();
    /// assert_eq!(ERR.message(), "failed to get urandom");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the exception message associated with this initialization
    /// error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::InitializeError;
    /// let err = InitializeError::new();
    /// assert_eq!(err.message(), "failed to get urandom");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        "failed to get urandom"
    }
}

impl fmt::Display for InitializeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl error::Error for InitializeError {}

/// Error that indicates the system source of cryptographically secure
/// randomness failed to read the requested bytes.
///
/// This can occur if the source is unknown or lacks sufficient entropy.
///
/// This error is returned by [`urandom()`]. See its documentation for more
/// details.
///
/// This error corresponds to the [Ruby `RuntimeError` Exception class].
///
/// # Examples
///
/// ```
/// # use spinoso_random::UrandomError;
/// let err = UrandomError::new();
/// assert_eq!(err.message(), "failed to get urandom");
/// ```
///
/// [Ruby `RuntimeError` Exception class]: https://ruby-doc.org/core-3.1.2/RuntimeError.html
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UrandomError {
    _private: (),
}

impl UrandomError {
    /// Construct a new, default urandom error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::UrandomError;
    /// const ERR: UrandomError = UrandomError::new();
    /// assert_eq!(ERR.message(), "failed to get urandom");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the exception message associated with this urandom error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::UrandomError;
    /// let err = UrandomError::new();
    /// assert_eq!(err.message(), "failed to get urandom");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        "failed to get urandom"
    }
}

impl fmt::Display for UrandomError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl error::Error for UrandomError {}

/// Error that indicates the system source of cryptographically secure
/// randomness failed to read sufficient bytes to create a new seed.
///
/// This can occur if the source is unknown or lacks sufficient entropy.
///
/// This error is returned by [`new_seed`]. See its documentation for more
/// details.
///
/// This error corresponds to the [Ruby `RuntimeError` Exception class].
///
/// # Examples
///
/// ```
/// # use spinoso_random::NewSeedError;
/// let err = NewSeedError::new();
/// assert_eq!(err.message(), "failed to get urandom");
/// ```
///
/// [Ruby `RuntimeError` Exception class]: https://ruby-doc.org/core-3.1.2/RuntimeError.html
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewSeedError {
    _private: (),
}

impl NewSeedError {
    /// Construct a new, default new seed error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::NewSeedError;
    /// const ERR: NewSeedError = NewSeedError::new();
    /// assert_eq!(ERR.message(), "failed to get urandom");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the exception message associated with this new seed error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::NewSeedError;
    /// let err = NewSeedError::new();
    /// assert_eq!(err.message(), "failed to get urandom");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        "failed to get urandom"
    }
}

impl fmt::Display for NewSeedError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl error::Error for NewSeedError {}

/// Error that indicates a random number could not be generated with the given
/// bounds.
///
#[cfg_attr(
    feature = "random-rand",
    doc = "This error is returned by [`rand()`]. See its documentation for more details."
)]
///
/// This error corresponds to the [Ruby `ArgumentError` Exception class].
///
/// # Examples
///
/// ```
/// # use spinoso_random::ArgumentError;
/// let err = ArgumentError::new();
/// assert_eq!(err.message(), "ArgumentError");
/// ```
///
/// [Ruby `ArgumentError` Exception class]: https://ruby-doc.org/core-3.1.2/ArgumentError.html
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ArgumentError(ArgumentErrorInner);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum ArgumentErrorInner {
    Default,
    DomainError,
    #[cfg(feature = "random-rand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "random-rand")))]
    Rand(Max),
}

impl ArgumentError {
    /// Construct a new, default argument error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::ArgumentError;
    /// const ERR: ArgumentError = ArgumentError::new();
    /// assert_eq!(ERR.message(), "ArgumentError");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(ArgumentErrorInner::Default)
    }

    /// Construct a new domain error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::ArgumentError;
    /// const ERR: ArgumentError = ArgumentError::domain_error();
    /// assert_eq!(ERR.message(), "Numerical argument out of domain");
    /// ```
    #[inline]
    #[must_use]
    pub const fn domain_error() -> Self {
        Self(ArgumentErrorInner::DomainError)
    }

    /// Construct a new argument error from an invalid [`Max`] constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::{ArgumentError, Max};
    /// const ERR: ArgumentError = ArgumentError::with_rand_max(Max::Integer(-1));
    /// assert_eq!(ERR.message(), "invalid argument");
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "random-rand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "random-rand")))]
    pub const fn with_rand_max(max: Max) -> Self {
        Self(ArgumentErrorInner::Rand(max))
    }

    /// Retrieve the exception message associated with this new seed error.
    ///
    #[cfg_attr(feature = "random-rand", doc = "# Implementation notes")]
    #[cfg_attr(
        feature = "random-rand",
        doc = "Argument errors constructed with [`ArgumentError::with_rand_max`] return"
    )]
    #[cfg_attr(
        feature = "random-rand",
        doc = "an incomplete error message. Prefer to use the [`Display`] impl to"
    )]
    #[cfg_attr(feature = "random-rand", doc = "retrieve error messages from [`ArgumentError`].")]
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::ArgumentError;
    /// let err = ArgumentError::new();
    /// assert_eq!(err.message(), "ArgumentError");
    /// let err = ArgumentError::domain_error();
    /// assert_eq!(err.message(), "Numerical argument out of domain");
    /// ```
    ///
    /// [`Display`]: fmt::Display
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self.0 {
            ArgumentErrorInner::Default => "ArgumentError",
            ArgumentErrorInner::DomainError => "Numerical argument out of domain",
            #[cfg(feature = "random-rand")]
            ArgumentErrorInner::Rand(_) => "invalid argument",
        }
    }

    /// Return whether this argument error is a domain error.
    ///
    /// Domain errors are typically reported as `Errno::EDOM` in MRI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::ArgumentError;
    /// let err = ArgumentError::domain_error();
    /// assert!(err.is_domain_error());
    /// let err = ArgumentError::new();
    /// assert!(!err.is_domain_error());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_domain_error(self) -> bool {
        matches!(self.0, ArgumentErrorInner::DomainError)
    }
}

impl fmt::Display for ArgumentError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ArgumentErrorInner::Default | ArgumentErrorInner::DomainError => f.write_str(self.message()),
            #[cfg(feature = "random-rand")]
            ArgumentErrorInner::Rand(max) => write!(f, "invalid argument - {max}"),
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for ArgumentError {}
