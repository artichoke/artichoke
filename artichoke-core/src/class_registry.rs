//! Define and store class specs on an interpreter.

use core::any::Any;

/// Define and store class specs on an interpreter.
///
/// A class spec is a static set of information the interpreter requires to
/// define a Ruby `Class` object.
pub trait ClassRegistry {
    /// Concrete value type for boxed Ruby values.
    type Value;

    /// Concrete error type for errors encountered when manipulating the class registry.
    type Error;

    /// Type representing a class specification.
    type Spec;

    /// Create a class definition bound to a Rust type `T`.
    ///
    /// Class definitions have the same lifetime as the interpreter.
    ///
    /// # Errors
    ///
    /// If the class registry state is inaccessible, an error is returned.
    fn def_class<T>(&mut self, spec: Self::Spec) -> Result<(), Self::Error>
    where
        T: Any;

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`ClassRegistry::def_class`].
    ///
    /// # Errors
    ///
    /// If the class registry state is inaccessible, an error is returned.
    fn class_spec<T>(&self) -> Result<Option<&Self::Spec>, Self::Error>
    where
        T: Any;

    /// Retrieve whether a class definition exists from the state bound to Rust type `T`.
    ///
    /// # Errors
    ///
    /// If the class registry state is inaccessible, an error is returned.
    fn is_class_defined<T>(&self) -> bool
    where
        T: Any,
    {
        matches!(self.class_spec::<T>(), Ok(Some(_)))
    }

    /// Retrieve a boxed Ruby value containing a `Class` object for the `Class`
    /// bound to Rust type `T`.
    ///
    /// If the interpreter cannot find or load a class associated with `T`,
    /// `Ok(None)` is returned.
    ///
    /// # Errors
    ///
    /// If the class registry state is inaccessible, an error is returned.
    fn class_of<T>(&mut self) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any;

    /// Create a new instance of the class bound to the Rust type `T`.
    ///
    /// This method resolves the class referenced by the type `T` and calls
    /// `new` on this Ruby class with the given arguments.
    ///
    /// If the interpreter cannot find or load a class associated with `T`,
    /// `Ok(None)` is returned.
    ///
    /// # Errors
    ///
    /// If the class registry state is inaccessible, an error is returned.
    ///
    /// If an exception is raised on the interpreter, then an error is returned.
    fn new_instance<T>(&mut self, args: &[Self::Value]) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any;
}
