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
    /// Ruby `Value`s are polymorphic boxes so converting from a Ruby `Value` to
    /// a concrete Rust type may not always succeed.
    ///
    /// Implementations should return [`ArtichokeError::ConvertToRuby`] or
    /// [`ArtichokeError::ConvertToRust`] depending on which direction the
    /// conversion attempted.
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
