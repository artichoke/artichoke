//! Convert between Rust and Ruby objects.

use crate::ArtichokeError;

/// Infallible conversion between two types.
///
/// See [`std::convert::From`].
pub trait Convert<T, U> {
    /// Performs the infallible conversion.
    fn convert(&self, from: T) -> U;
}

/// Fallible conversions between two types.
///
/// See [`std::convert::TryFrom`].
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
