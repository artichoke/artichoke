//! Convert between Rust and Ruby objects.

/// Infallible conversion between two types.
///
/// Implementors may not allocate on the interpreter heap.
///
/// See [`core::convert::From`].
/// See [`ConvertMut`].
pub trait Convert<T, U> {
    /// Performs the infallible conversion.
    fn convert(&self, from: T) -> U;
}

/// Fallible conversions between two types.
///
/// Implementors may not allocate on the interpreter heap.
///
/// See [`core::convert::TryFrom`].
/// See [`TryConvertMut`].
pub trait TryConvert<T, U> {
    /// Error type for failed conversions.
    type Error;

    /// Performs the fallible conversion.
    ///
    /// # Errors
    ///
    /// If boxing or unboxing a value into the specified type fails, an error is
    /// returned.
    fn try_convert(&self, value: T) -> Result<U, Self::Error>;
}

/// Mutable infallible conversion between two types.
///
/// Implementors may allocate on the interpreter heap.
///
/// See [`core::convert::From`].
/// See [`Convert`].
pub trait ConvertMut<T, U> {
    /// Performs the infallible conversion.
    fn convert_mut(&mut self, from: T) -> U;
}

/// Mutable fallible conversions between two types.
///
/// Implementors may allocate on the interpreter heap.
///
/// See [`core::convert::TryFrom`].
/// See [`TryConvert`].
pub trait TryConvertMut<T, U> {
    /// Error type for failed conversions.
    type Error;

    /// Performs the fallible conversion.
    ///
    /// # Errors
    ///
    /// If boxing or unboxing a value into the specified type fails, an error is
    /// returned.
    fn try_convert_mut(&mut self, value: T) -> Result<U, Self::Error>;
}
