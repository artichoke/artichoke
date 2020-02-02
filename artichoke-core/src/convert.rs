//! Convert between Rust and Ruby objects.

use crate::ArtichokeError;

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
    /// Performs the fallible conversion.
    ///
    /// # Errors
    ///
    /// If boxing or unboxing a value into the specified type fails, an error is
    /// returned.
    fn try_convert(&self, value: T) -> Result<U, ArtichokeError>;
}

/// Provide a fallible converter for types that implement an infallible
/// conversion.
impl<T, U, V> TryConvert<T, U> for V
where
    V: Convert<T, U>,
{
    /// Blanket implementation that always succeeds by delegating to
    /// [`Convert::convert`].
    fn try_convert(&self, value: T) -> Result<U, ArtichokeError> {
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
    /// Performs the fallible conversion.
    ///
    /// # Errors
    ///
    /// If boxing or unboxing a value into the specified type fails, an error is
    /// returned.
    fn try_convert_mut(&mut self, value: T) -> Result<U, ArtichokeError>;
}

/// Provide a mutable fallible converter for types that implement an infallible
/// conversion.
impl<T, U, V> TryConvertMut<T, U> for V
where
    V: ConvertMut<T, U>,
{
    /// Blanket implementation that always succeeds by delegating to
    /// [`Convert::convert`].
    fn try_convert_mut(&mut self, value: T) -> Result<U, ArtichokeError> {
        Ok(ConvertMut::convert_mut(self, value))
    }
}
