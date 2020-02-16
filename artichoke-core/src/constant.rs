//! Define constants on an interpreter.
//!
//! Constants can be an arbitrary Ruby value. Constants can be defined globally,
//! on a class, or on a module.

use std::error;

use crate::value::Value;

/// Deifne constants on an interprter.
///
/// Constants can be an arbitrary Ruby value. Constants can be defined globally,
/// on a class, or on a module.
#[allow(clippy::module_name_repetitions)]
pub trait DefineConstant {
    /// Concrete type for Ruby values.
    type Value: Value;

    /// Concrete error type for fallible operations.
    type Error: error::Error;

    /// Define a global constant.
    fn define_global_constant(
        &mut self,
        constant: &str,
        value: Self::Value,
    ) -> Result<(), Self::Error>;

    /// Define a class constant.
    ///
    /// The class is specified by the type parameter `T`.
    fn define_class_constant<T: 'static>(
        &mut self,
        constant: &str,
        value: Self::Value,
    ) -> Result<(), Self::Error>;

    /// Define a module constant.
    ///
    /// The class is specified by the type parameter `T`.
    fn define_module_constant<T: 'static>(
        &mut self,
        constant: &str,
        value: Self::Value,
    ) -> Result<(), Self::Error>;
}
