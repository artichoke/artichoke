//! Define and store module specs on an interpreter.

use core::any::Any;

/// Define and store module specs on an interpreter.
///
/// A module spec is a static set of information the interpreter requires to
/// define a Ruby `Module` object.
pub trait ModuleRegistry {
    /// Concrete value type for boxed Ruby values.
    type Value;

    /// Concrete error type for errors encountered when manipulating the module registry.
    type Error;

    /// Type representing a module specification.
    type Spec: 'static;

    /// Create a module definition bound to a Rust type `T`.
    ///
    /// Module definitions have the same lifetime as the interpreter.
    ///
    /// # Errors
    ///
    /// If the module registry state is inaccessible, an error is returned.
    fn def_module<T>(&mut self, spec: Self::Spec) -> Result<(), Self::Error>
    where
        T: Any;

    /// Retrieve a module definition from the interpreter bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a module spec
    /// registered for it using [`ModuleRegistry::def_module`].
    ///
    /// # Errors
    ///
    /// If the module registry state is inaccessible, an error is returned.
    fn module_spec<T>(&self) -> Result<Option<&Self::Spec>, Self::Error>
    where
        T: Any;

    /// Retrieve whether a module definition exists from the interpreter bound
    /// to Rust type `T`
    ///
    /// If the interpreter cannot find or load a module associated with `T`,
    /// `Ok(None)` is returned.
    ///
    /// # Errors
    ///
    /// If the module registry state is inaccessible, an error is returned.
    fn is_module_defined<T>(&self) -> bool
    where
        T: Any,
    {
        matches!(self.module_spec::<T>(), Ok(Some(_)))
    }

    /// Retrieve a boxed Ruby value containing a `Module` object for the
    /// `Module` bound to Rust type `T`.
    ///
    /// If the interpreter cannot find or load a module associated with `T`,
    /// `Ok(None)` is returned.
    ///
    /// # Errors
    ///
    /// If the module registry state is inaccessible, an error is returned.
    fn module_of<T>(&mut self) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any;
}
