//! Convert between Rust and Ruby objects.

use std::error;
use std::fmt;

/// Infallible conversion between two types.
///
/// Implementors may not allocate on the interpreter heap.
///
/// See [`std::convert::From`].
/// See [`ConvertMut`].
pub trait Convert<T, U> {
    /// Performs the infallible conversion.
    fn convert(&self, from: T) -> U;
}

/// Fallible conversions between two types.
///
/// Implementors may not allocate on the interpreter heap.
///
/// See [`std::convert::TryFrom`].
/// See [`TryConvertMut`].
#[allow(clippy::module_name_repetitions)]
pub trait TryConvert<T, U> {
    /// Error type for failed conversions.
    type Error: error::Error;

    /// Performs the fallible conversion.
    ///
    /// # Errors
    ///
    /// If boxing or unboxing a value into the specified type fails, an error is
    /// returned.
    fn try_convert(&self, value: T) -> Result<U, Self::Error>;
}

/// Provide a fallible converter for types that implement an infallible
/// conversion.
impl<T, U, V> TryConvert<T, U> for V
where
    V: Convert<T, U>,
{
    // TODO: this should be the never type.
    // https://github.com/rust-lang/rust/issues/35121
    type Error = Infallible;

    /// Blanket implementation that always succeeds by delegating to
    /// [`Convert::convert`].
    fn try_convert(&self, value: T) -> Result<U, Self::Error> {
        Ok(Convert::convert(self, value))
    }
}

/// Mutable infallible conversion between two types.
///
/// Implementors may allocate on the interpreter heap.
///
/// See [`std::convert::From`].
/// See [`Convert`].
#[allow(clippy::module_name_repetitions)]
pub trait ConvertMut<T, U> {
    /// Performs the infallible conversion.
    fn convert_mut(&mut self, from: T) -> U;
}

/// Mutable fallible conversions between two types.
///
/// Implementors may allocate on the interpreter heap.
///
/// See [`std::convert::TryFrom`].
/// See [`TryConvert`].
pub trait TryConvertMut<T, U> {
    /// Error type for failed conversions.
    type Error: error::Error;

    /// Performs the fallible conversion.
    ///
    /// # Errors
    ///
    /// If boxing or unboxing a value into the specified type fails, an error is
    /// returned.
    fn try_convert_mut(&mut self, value: T) -> Result<U, Self::Error>;
}

/// Provide a mutable fallible converter for types that implement an infallible
/// conversion.
impl<T, U, V> TryConvertMut<T, U> for V
where
    V: ConvertMut<T, U>,
{
    // TODO: this should be the never type.
    // https://github.com/rust-lang/rust/issues/35121
    type Error = Infallible;

    /// Blanket implementation that always succeeds by delegating to
    /// [`Convert::convert`].
    fn try_convert_mut(&mut self, value: T) -> Result<U, Self::Error> {
        Ok(ConvertMut::convert_mut(self, value))
    }
}

/// Uninhabital error type for infallible conversions.
pub enum Infallible {}

impl error::Error for Infallible {}

impl fmt::Debug for Infallible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This error is unreachable")
    }
}

impl fmt::Display for Infallible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This error is unreachable")
    }
}
