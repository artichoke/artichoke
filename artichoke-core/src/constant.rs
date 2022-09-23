//! Define constants on an interpreter.
//!
//! Constants can be an arbitrary Ruby value. Constants can be defined globally,
//! on a class, or on a module.

use crate::value::Value;

/// Define constants on an interpreter.
///
/// Constants can be an arbitrary Ruby value. Constants can be defined globally,
/// on a class, or on a module.
pub trait DefineConstant {
    /// Concrete type for Ruby values.
    type Value: Value;

    /// Concrete error type for fallible operations.
    type Error;

    /// Define a global constant.
    ///
    /// # Errors
    ///
    /// If the given constant name is not valid, an error is returned.
    ///
    /// If the interpreter cannot define the constant, an error is returned.
    fn define_global_constant(&mut self, constant: &str, value: Self::Value) -> Result<(), Self::Error>;

    /// Define a class constant.
    ///
    /// The class is specified by the type parameter `T`.
    ///
    /// # Errors
    ///
    /// If the class named by type `T` is not defined, an error is returned.
    ///
    /// If the given constant name is not valid, an error is returned.
    ///
    /// If the interpreter cannot define the constant, an error is returned.
    fn define_class_constant<T>(&mut self, constant: &str, value: Self::Value) -> Result<(), Self::Error>
    where
        T: 'static;

    /// Define a module constant.
    ///
    /// The class is specified by the type parameter `T`.
    ///
    /// # Errors
    ///
    /// If the module named by type `T` is not defined, an error is returned.
    ///
    /// If the given constant name is not valid, an error is returned.
    ///
    /// If the interpreter cannot define the constant, an error is returned.
    fn define_module_constant<T>(&mut self, constant: &str, value: Self::Value) -> Result<(), Self::Error>
    where
        T: 'static;
}
