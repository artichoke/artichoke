use core::any::Any;

pub trait ClassRegistry {
    /// Concrete value type for boxed Ruby values.
    type Value;

    /// Concrete error type for errors encountered when manipulating the class registry.
    type Error;

    /// Type representing a class specification.
    type Spec;

    /// Create a class definition bound to a Rust type `T`.
    ///
    /// Class definitions have the same lifetime as the
    /// [`State`](crate::state::State) because the class def owns the
    /// `mrb_data_type` for the type, which must be long-lived.
    fn def_class<T>(&mut self, spec: Self::Spec) -> Result<(), Self::Error>
    where
        T: Any;

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`ClassRegistry::def_class`].
    fn class_spec<T>(&self) -> Result<Option<&Self::Spec>, Self::Error>
    where
        T: Any;

    /// Retrieve whether a class definition exists from the state bound to Rust type `T`.
    fn is_class_defined<T>(&self) -> bool
    where
        T: Any,
    {
        matches!(self.class_spec::<T>(), Ok(Some(_)))
    }

    fn class_of<T>(&mut self) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any;

    fn new_instance<T>(&mut self, args: &[Self::Value]) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any;
}
