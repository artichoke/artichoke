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

//! Secure random number generator interface.
//!
//! This module implements the [`SecureRandom`] package from the Ruby Standard
//! Library. It is an interface to secure random number generators which are
//! suitable for generating session keys in HTTP cookies, etc.
//!
//! This implementation of `SecureRandom` supports the system RNG via the
//! [`getrandom`] crate. This implementation does not depend on OpenSSL.
//!
//! # Examples
//!
//! Generate cryptographically secure random bytes:
//!
//! ```rust
//! # fn example() -> Result<(), spinoso_securerandom::Error> {
//! let bytes = spinoso_securerandom::random_bytes(Some(1024))?;
//! assert_eq!(bytes.len(), 1024);
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! Generate base64-encoded random data:
//!
//! ```rust
//! # fn example() -> Result<(), spinoso_securerandom::Error> {
//! let bytes = spinoso_securerandom::base64(Some(1024))?;
//! assert_eq!(bytes.len(), 1368);
//! assert!(bytes.is_ascii());
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! Generate random floats and integers in a range bounded from zero to a
//! maximum:
//!
//! ```rust
//! # use spinoso_securerandom::{DomainError, Max, Rand};
//! # fn example() -> Result<(), DomainError> {
//! let rand = spinoso_securerandom::random_number(Max::None)?;
//! assert!(matches!(rand, Rand::Float(_)));
//!
//! let rand = spinoso_securerandom::random_number(Max::Integer(57))?;
//! assert!(matches!(rand, Rand::Integer(_)));
//!
//! let rand = spinoso_securerandom::random_number(Max::Float(57.0))?;
//! assert!(matches!(rand, Rand::Float(_)));
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! Generate version 4 random UUIDs:
//!
//! ```rust
//! # fn example() -> Result<(), spinoso_securerandom::Error> {
//! let uuid = spinoso_securerandom::uuid()?;
//! assert_eq!(uuid.len(), 36);
//! assert!(uuid.chars().all(|ch| ch == '-' || ch.is_ascii_hexdigit()));
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! [`SecureRandom`]: https://ruby-doc.org/stdlib-2.6.3/libdoc/securerandom/rdoc/SecureRandom.html
//! [`getrandom`]: https://crates.io/crates/getrandom

// Ensure code blocks in README.md compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

use core::fmt;
use rand::distributions::Alphanumeric;
use rand::{self, Rng, RngCore};
use std::error;

use scolapasta_hex as hex;

mod uuid;

const DEFAULT_REQUESTED_BYTES: usize = 16;

/// Sum type of all errors possibly returned from [`random_bytes`].
///
/// `random_bytes` can return errors under several conditions:
///
/// - The given byte length is not a valid [`usize`].
/// - The underlying source of randomness returns an error when generating the
///   requested random bytes.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// Error that indicates an argument parsing or value logic error occurred.
    ///
    /// See [`ArgumentError`].
    Argument(ArgumentError),
    /// Error that indicates the underlying source of randomness failed to
    /// generate the requested random bytes.
    ///
    /// See [`RandomBytesError`].
    RandomBytes(RandomBytesError),
}

impl From<ArgumentError> for Error {
    #[inline]
    fn from(err: ArgumentError) -> Self {
        Self::Argument(err)
    }
}

impl From<RandomBytesError> for Error {
    #[inline]
    fn from(err: RandomBytesError) -> Self {
        Self::RandomBytes(err)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecureRandom error")
    }
}

impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Argument(ref err) => Some(err),
            Self::RandomBytes(ref err) => Some(err),
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
/// # use spinoso_securerandom::ArgumentError;
/// let err = ArgumentError::new();
/// assert_eq!(err.message(), "ArgumentError");
///
/// let err = ArgumentError::with_message("negative string size (or size too big)");
/// assert_eq!(err.message(), "negative string size (or size too big)");
/// ```
///
/// [Ruby `ArgumentError` Exception class]: https://ruby-doc.org/core-2.6.3/ArgumentError.html
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgumentError(&'static str);

impl From<&'static str> for ArgumentError {
    #[inline]
    fn from(message: &'static str) -> Self {
        Self::with_message(message)
    }
}

impl Default for ArgumentError {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ArgumentError {
    #[inline]
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
    /// # use spinoso_securerandom::ArgumentError;
    /// const ERR: ArgumentError = ArgumentError::new();
    /// assert_eq!(ERR.message(), "ArgumentError");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self("ArgumentError")
    }

    /// Construct a new argument error with a message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::ArgumentError;
    /// const ERR: ArgumentError = ArgumentError::with_message("negative string size (or size too big)");
    /// assert_eq!(ERR.message(), "negative string size (or size too big)");
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
    /// # use spinoso_securerandom::ArgumentError;
    /// let err = ArgumentError::new();
    /// assert_eq!(err.message(), "ArgumentError");
    ///
    /// let err = ArgumentError::with_message("negative string size (or size too big)");
    /// assert_eq!(err.message(), "negative string size (or size too big)");
    /// ```
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        self.0
    }
}

/// Error that indicates the underlying source of randomness failed to generate
/// the requested random bytes.
///
/// This error is typically returned by the operating system.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RandomBytesError {
    _private: (),
}

impl fmt::Display for RandomBytesError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl error::Error for RandomBytesError {}

impl RandomBytesError {
    /// Construct a new, default random bytes error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::RandomBytesError;
    /// const ERR: RandomBytesError = RandomBytesError::new();
    /// assert_eq!(ERR.message(), "OS Error: Failed to generate random bytes");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the exception message associated with this random bytes error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::RandomBytesError;
    /// let err = RandomBytesError::new();
    /// assert_eq!(err.message(), "OS Error: Failed to generate random bytes");
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn message(self) -> &'static str {
        "OS Error: Failed to generate random bytes"
    }
}

/// Error that indicates the given maximum value is not finite and cannot be
/// used to bound a domain for generating random numbers.
///
/// This error is returned by [`random_number`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DomainError {
    _private: (),
}

impl fmt::Display for DomainError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl error::Error for DomainError {}

impl DomainError {
    /// Construct a new, default domain error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::DomainError;
    /// const ERR: DomainError = DomainError::new();
    /// assert_eq!(ERR.message(), "Numerical argument out of domain");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Retrieve the exception message associated with this domain error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::DomainError;
    /// let err = DomainError::new();
    /// assert_eq!(err.message(), "Numerical argument out of domain");
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn message(self) -> &'static str {
        "Numerical argument out of domain"
    }
}

/// A handle to the underlying secure random number generator.
///
/// This is a copy zero-sized type with no associated methods. This type exists
/// so a Ruby VM can attempt to unbox this type and statically dispatch to
/// functions defined in this crate.
///
/// # Examples
///
/// ```
/// # use spinoso_securerandom::SecureRandom;
/// const RANDOM: SecureRandom = SecureRandom::new();
/// ```
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecureRandom {
    _private: (),
}

impl SecureRandom {
    /// Constructs a new, default `SecureRandom`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_securerandom::SecureRandom;
    /// const RANDOM: SecureRandom = SecureRandom::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

/// Generate a vector of random bytes.
///
/// If `len` is [`Some`] and non-negative, generate a vector of `len` random
/// bytes. If `len` is [`None`], generate 16 random bytes.
///
/// # Examples
///
/// ```rust
/// # fn example() -> Result<(), spinoso_securerandom::Error> {
/// let bytes = spinoso_securerandom::random_bytes(Some(1024))?;
/// assert_eq!(bytes.len(), 1024);
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// # Errors
///
/// If the given length is negative, return an [`ArgumentError`].
///
/// If the underlying source of randomness returns an error, return a
/// [`RandomBytesError`].
#[inline]
pub fn random_bytes(len: Option<i64>) -> Result<Vec<u8>, Error> {
    let len = match len.map(usize::try_from) {
        Some(Ok(0)) => return Ok(Vec::new()),
        Some(Ok(len)) => len,
        Some(Err(_)) => {
            let err = ArgumentError::with_message("negative string size (or size too big)");
            return Err(Error::Argument(err));
        }
        None => DEFAULT_REQUESTED_BYTES,
    };
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0; len];
    if rng.try_fill_bytes(&mut bytes).is_err() {
        return Err(Error::RandomBytes(RandomBytesError::new()));
    }
    Ok(bytes)
}

/// Max value when generating a random number from a range.
///
/// In Ruby, the `rand` family of functions generate random numbers form within
/// a range. This range is always anchored on the left by zero. The `Max` enum
/// allows callers to specify the upper bound of the range. If the `None`
/// variant is given, the default is set to generate floats in the range of
/// `[0.0, 1.0)`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Max {
    /// Generate floats in the range `[0, max)`.
    ///
    /// If `max` is less than or equal to zero, the range defaults to floats
    /// in `[0.0, 1.0]`.
    ///
    /// If `max` is [`NaN`](f64::NAN), an error is returned.
    Float(f64),
    /// Generate signed integers in the range `[0, max)`.
    ///
    /// If `max` is less than or equal to zero, the range defaults to floats
    /// in `[0.0, 1.0]`.
    Integer(i64),
    /// Generate floats in the range `[0.0, 1.0]`.
    None,
}

/// Random numeric value generated from the secure random number generator.
///
/// In Ruby, the `rand` family of functions generate random numbers that are
/// either floats or signed integers.
///
/// The numeric contents of this enum will never be negative and will always be
/// finite.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rand {
    /// A random float that is greater than or equal to zero.
    Float(f64),
    /// A random signed integer that is greater than or equal to zero.
    Integer(i64),
}

/// Generate a single random number, either a float or an integer.
///
/// In Ruby, the `rand` family of functions generate random numbers that are
/// either floats or signed integers.
///
/// The random numbers returned by this function will never be negative and will
/// always be finite.
///
/// In Ruby, the `rand` family of functions generate random numbers form within
/// a range. This range is always anchored on the left by zero. See the [`Max`]
/// enum documentation for how to bound the random numbers returned by this
/// function.
///
/// # Examples
///
/// ```rust
/// # use spinoso_securerandom::{Max, Rand};
/// # fn example() -> Result<(), spinoso_securerandom::DomainError> {
/// let rand = spinoso_securerandom::random_number(Max::None)?;
/// assert!(matches!(rand, Rand::Float(_)));
///
/// let rand = spinoso_securerandom::random_number(Max::Integer(57))?;
/// assert!(matches!(rand, Rand::Integer(_)));
///
/// let rand = spinoso_securerandom::random_number(Max::Integer(-20))?;
/// assert!(matches!(rand, Rand::Float(_)));
///
/// let rand = spinoso_securerandom::random_number(Max::Integer(0))?;
/// assert!(matches!(rand, Rand::Float(_)));
///
/// let rand = spinoso_securerandom::random_number(Max::Float(57.0))?;
/// assert!(matches!(rand, Rand::Float(_)));
///
/// let rand = spinoso_securerandom::random_number(Max::Float(-20.0))?;
/// assert!(matches!(rand, Rand::Float(_)));
///
/// let rand = spinoso_securerandom::random_number(Max::Float(0.0))?;
/// assert!(matches!(rand, Rand::Float(_)));
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// # Errors
///
/// If the float given in a [`Max::Float`] variant is [`NaN`](f64::NAN) or
/// infinite, a [`DomainError`] is returned.
#[inline]
pub fn random_number(max: Max) -> Result<Rand, DomainError> {
    let mut rng = rand::thread_rng();
    match max {
        Max::Float(max) if !max.is_finite() => {
            // NOTE: MRI returns `Errno::EDOM` exception class.
            Err(DomainError::new())
        }
        Max::Float(max) if max <= 0.0 => {
            let number = rng.gen_range(0.0..1.0);
            Ok(Rand::Float(number))
        }
        Max::Float(max) => {
            let number = rng.gen_range(0.0..max);
            Ok(Rand::Float(number))
        }
        Max::Integer(max) if !max.is_positive() => {
            let number = rng.gen_range(0.0..1.0);
            Ok(Rand::Float(number))
        }
        Max::Integer(max) => {
            let number = rng.gen_range(0..max);
            Ok(Rand::Integer(number))
        }
        Max::None => {
            let number = rng.gen_range(0.0..1.0);
            Ok(Rand::Float(number))
        }
    }
}

/// Generate a hex-encoded [`String`] of random bytes.
///
/// If `len` is [`Some`] and non-negative, generate a vector of `len` random
/// bytes. If `len` is [`None`], generate 16 random bytes. Take the resulting
/// bytes and hexadecimal encode them.
///
/// # Examples
///
/// ```rust
/// # fn example() -> Result<(), spinoso_securerandom::Error> {
/// let bytes = spinoso_securerandom::hex(Some(1024))?;
/// assert_eq!(bytes.len(), 2048);
/// assert!(bytes.is_ascii());
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// # Errors
///
/// If the given length is negative, return an [`ArgumentError`].
///
/// If the underlying source of randomness returns an error, return a
/// [`RandomBytesError`].
#[inline]
pub fn hex(len: Option<i64>) -> Result<String, Error> {
    let bytes = random_bytes(len)?;
    Ok(hex::encode(bytes))
}

/// Generate a base64-encoded [`String`] of random bytes.
///
/// If `len` is [`Some`] and non-negative, generate a vector of `len` random
/// bytes. If `len` is [`None`], generate 16 random bytes. Take the resulting
/// bytes and base64 encode them.
///
/// # Examples
///
/// ```rust
/// # fn example() -> Result<(), spinoso_securerandom::Error> {
/// let bytes = spinoso_securerandom::base64(Some(1024))?;
/// assert_eq!(bytes.len(), 1368);
/// assert!(bytes.is_ascii());
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// # Errors
///
/// If the given length is negative, return an [`ArgumentError`].
///
/// If the underlying source of randomness returns an error, return a
/// [`RandomBytesError`].
#[inline]
pub fn base64(len: Option<i64>) -> Result<String, Error> {
    let bytes = random_bytes(len)?;
    Ok(base64::encode(bytes))
}

/// Generate a URL-safe base64-encoded [`String`] of random bytes.
///
/// If `len` is [`Some`] and non-negative, generate a vector of `len` random
/// bytes. If `len` is [`None`], generate 16 random bytes. Take the resulting
/// bytes and base64 encode them.
///
/// # Examples
///
/// ```rust
/// # fn example() -> Result<(), spinoso_securerandom::Error> {
/// let bytes = spinoso_securerandom::urlsafe_base64(Some(1024), false)?;
/// assert_eq!(bytes.len(), 1366);
/// assert!(bytes.is_ascii());
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// # Errors
///
/// If the given length is negative, return an [`ArgumentError`].
///
/// If the underlying source of randomness returns an error, return a
/// [`RandomBytesError`].
#[inline]
pub fn urlsafe_base64(len: Option<i64>, padding: bool) -> Result<String, Error> {
    let bytes = random_bytes(len)?;
    if padding {
        Ok(base64::encode_config(bytes, base64::URL_SAFE))
    } else {
        Ok(base64::encode_config(bytes, base64::URL_SAFE_NO_PAD))
    }
}

/// Generate a random sequence of ASCII alphanumeric bytes.
///
/// If `len` is [`Some`] and non-negative, generate a [`String`] of `len`
/// random ASCII alphanumeric bytes. If `len` is [`None`], generate 16 random
/// alphanumeric bytes.
///
/// The returned [`Vec<u8>`](Vec) is guaranteed to contain only ASCII bytes.
///
/// # Examples
///
/// ```rust
/// # use std::error::Error;
/// # fn example() -> Result<(), Box<dyn Error>> {
/// let bytes = spinoso_securerandom::alphanumeric(Some(1024))?;
/// let bytes = String::from_utf8(bytes)?;
/// assert_eq!(bytes.len(), 1024);
/// assert!(bytes.is_ascii());
/// assert!(bytes.find(|ch: char| !ch.is_ascii_alphanumeric()).is_none());
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// # Errors
///
/// If the given length is negative, return an [`ArgumentError`].
///
/// If the underlying source of randomness returns an error, return a
/// [`RandomBytesError`].
#[inline]
pub fn alphanumeric(len: Option<i64>) -> Result<Vec<u8>, ArgumentError> {
    let len = match len.map(usize::try_from) {
        Some(Ok(0)) => return Ok(Vec::new()),
        Some(Ok(len)) => len,
        Some(Err(_)) => {
            let err = ArgumentError::with_message("negative string size (or size too big)");
            return Err(err);
        }
        None => DEFAULT_REQUESTED_BYTES,
    };
    let rng = rand::thread_rng();
    let string = rng.sample_iter(Alphanumeric).take(len).collect();
    Ok(string)
}

/// Generate a version 4 UUID and return a [`String`].
///
/// A version 4 UUID is randomly generated. See [RFC 4122] for details.
///
/// # Examples
///
/// ```rust
/// # fn example() -> Result<(), spinoso_securerandom::Error> {
/// let uuid = spinoso_securerandom::uuid()?;
/// assert_eq!(uuid.len(), 36);
/// assert!(uuid.chars().all(|ch| ch == '-' || ch.is_ascii_hexdigit()));
/// # Ok(())
/// # }
/// # example().unwrap()
/// ```
///
/// # Errors
///
/// If the underlying source of randomness returns an error, return a
/// [`RandomBytesError`].
///
/// [RFC 4122]: https://tools.ietf.org/html/rfc4122#section-4.4
#[inline]
pub fn uuid() -> Result<String, RandomBytesError> {
    uuid::v4()
}

#[cfg(test)]
mod tests {
    use core::ops::Not;
    use rand::CryptoRng;

    use super::{alphanumeric, base64, hex, random_bytes, random_number, uuid, DomainError, Error, Max, Rand};

    fn rng_must_be_cryptographically_secure<T: CryptoRng>(rng: T) {
        drop(rng);
    }

    #[test]
    fn rand_thread_rng_must_be_cryptographically_secure() {
        rng_must_be_cryptographically_secure(rand::thread_rng());
    }

    #[test]
    fn random_bytes_default_bytes() {
        // https://github.com/ruby/ruby/blob/v2_6_3/lib/securerandom.rb#L135
        assert_eq!(super::DEFAULT_REQUESTED_BYTES, 16);
        let default_requested_bytes = random_bytes(None).unwrap();
        assert_eq!(default_requested_bytes.len(), 16);
    }

    #[test]
    fn random_bytes_len_must_be_positive() {
        assert!(matches!(random_bytes(Some(-1)), Err(Error::Argument(_))));
        assert!(matches!(base64(Some(-1)), Err(Error::Argument(_))));
        assert!(matches!(hex(Some(-1)), Err(Error::Argument(_))));
        assert!(alphanumeric(Some(-1)).is_err());
    }

    #[test]
    fn random_bytes_zero_len_gives_empty_result() {
        assert!(random_bytes(Some(0)).unwrap().is_empty());
        assert!(base64(Some(0)).unwrap().is_empty());
        assert!(hex(Some(0)).unwrap().is_empty());
        assert!(alphanumeric(Some(0)).unwrap().is_empty());
    }

    #[test]
    fn random_bytes_nonzero_len_gives_len_result() {
        assert_eq!(random_bytes(Some(32)).unwrap().len(), 32);
        assert_eq!(base64(Some(32)).unwrap().len(), 44);
        assert_eq!(hex(Some(32)).unwrap().len(), 64);
        assert_eq!(alphanumeric(Some(32)).unwrap().len(), 32);

        // for a len that is not a power of two
        assert_eq!(random_bytes(Some(57)).unwrap().len(), 57);
        assert_eq!(base64(Some(57)).unwrap().len(), 76);
        assert_eq!(hex(Some(57)).unwrap().len(), 114);
        assert_eq!(alphanumeric(Some(57)).unwrap().len(), 57);
    }

    #[test]
    fn random_bytes_none_len_gives_len_16_result() {
        assert_eq!(random_bytes(None).unwrap().len(), 16);
        assert_eq!(base64(None).unwrap().len(), 24);
        assert_eq!(hex(None).unwrap().len(), 32);
        assert_eq!(alphanumeric(None).unwrap().len(), 16);
    }

    #[test]
    fn random_number_domain_error() {
        assert_eq!(random_number(Max::Float(f64::NAN)), Err(DomainError::new()));
        assert_eq!(random_number(Max::Float(f64::INFINITY)), Err(DomainError::new()));
        assert_eq!(random_number(Max::Float(f64::NEG_INFINITY)), Err(DomainError::new()));
    }

    #[test]
    fn random_number_in_float_out_float() {
        assert!(matches!(random_number(Max::None), Ok(Rand::Float(_))));
        assert!(matches!(random_number(Max::Float(0.5)), Ok(Rand::Float(_))));
        assert!(matches!(random_number(Max::Float(1.0)), Ok(Rand::Float(_))));
        assert!(matches!(random_number(Max::Float(9000.63)), Ok(Rand::Float(_))));
        assert!(matches!(random_number(Max::Float(0.0)), Ok(Rand::Float(_))));
        assert!(matches!(random_number(Max::Float(-0.0)), Ok(Rand::Float(_))));
        assert!(matches!(random_number(Max::Float(-1.0)), Ok(Rand::Float(_))));
    }

    #[test]
    fn random_number_in_neg_integer_out_float() {
        assert!(matches!(random_number(Max::Integer(-1)), Ok(Rand::Float(_))));
    }

    #[test]
    fn random_number_in_zero_integer_out_float() {
        assert!(matches!(random_number(Max::Integer(0)), Ok(Rand::Float(_))));
    }

    #[test]
    fn random_number_in_pos_integer_out_integer() {
        assert!(matches!(random_number(Max::Integer(1)), Ok(Rand::Integer(_))));
        assert!(matches!(random_number(Max::Integer(9000)), Ok(Rand::Integer(_))));
        assert!(matches!(random_number(Max::Integer(i64::MAX)), Ok(Rand::Integer(_))));
    }

    #[test]
    fn uuid_format() {
        let id = uuid().unwrap();
        assert_eq!(id.len(), 36);
        assert!(id.chars().all(|ch| ch == '-' || ch.is_ascii_hexdigit()));
        assert!(id.chars().any(char::is_uppercase).not());
        assert_eq!(&id[14..15], "4");
    }

    #[test]
    fn alphanumeric_format() {
        let random = alphanumeric(Some(1024)).unwrap();
        assert!(random.iter().all(|&byte| byte.is_ascii_alphanumeric()));
    }
}
