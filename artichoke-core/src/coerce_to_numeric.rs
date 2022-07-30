//! Coerce Ruby values to native numerics.

use crate::value::Value;

/// Coerce Ruby values to native numerics (floats and integers).
pub trait CoerceToNumeric {
    /// Concrete type of boxed Ruby value as inputs to coerce functions.
    type Value: Value;

    /// Concrete float type to coerce values into, e.g. [`f64`].
    type Float;

    /// Concrete error type for errors encountered when coercing values.
    type Error;

    /// Coerce the given Ruby value to a `Float`.
    ///
    /// This coercion mechanism is used by Ruby to handle mixed-type numeric
    /// operations: it is intended to find a compatible common type between the two
    /// operands of the operator.
    ///
    /// See [`Numeric#coerce`].
    ///
    /// # Errors
    ///
    /// If a Ruby `nil` is given, an error is returned.
    ///
    /// If the given value does not subclass [`Numeric`], an error is returned.
    ///
    /// If the [`Numeric` class] is not defined, an error is returned.
    ///
    /// If the underlying interpreter returns an error when calling `#to_f` or
    /// [`Numeric#coerce`], the error is returned.
    ///
    /// [`Numeric`]: https://ruby-doc.org/core-3.1.2/Numeric.html
    /// [`Numeric` class]: https://ruby-doc.org/core-3.1.2/Numeric.html
    /// [`Numeric#coerce`]: https://ruby-doc.org/core-3.1.2/Numeric.html#method-i-coerce
    fn coerce_to_float(&mut self, value: Self::Value) -> Result<Self::Float, Self::Error>;
}
