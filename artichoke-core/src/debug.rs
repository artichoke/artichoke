//! Routines for debugging and printing exception messages.

use crate::value::Value;

/// Debugging and `Exception` message support.
pub trait Debug {
    /// Concrete type for return values from eval.
    type Value: Value;

    /// Return a name for the given value's type that is suitable for using in
    /// an `Exception` message.
    ///
    /// Some immediate types like `true`, `false`, and `nil` are shown by value
    /// rather than by class.
    ///
    /// This function suppresses all errors and returns an empty string on error.
    fn inspect_type_name_for_value(&mut self, value: Self::Value) -> &str;

    /// Return the class name for the given value's type.
    ///
    /// Even immediate types will have their class name spelled out. For
    /// example, calling this function with `nil` will return `"NilClass"`.
    ///
    /// This function suppresses all errors and returns an empty string on error.
    fn class_name_for_value(&mut self, value: Self::Value) -> &str;
}
