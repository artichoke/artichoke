use core::any::Any;

pub trait ModuleRegistry {
    /// Concrete value type for boxed Ruby values.
    type Value;

    /// Concrete error type for errors encountered when manipulating the module registry.
    type Error;

    /// Type representing a module specification.
    type Spec;

    /// Create a module definition bound to a Rust type `T`.
    ///
    /// Module definitions have the same lifetime as the interpreter because the
    /// module def owns the `mrb_data_type` for the type, which must be
    /// long-lived.
    fn def_module<T>(&mut self, spec: Self::Spec) -> Result<(), Self::Error>
    where
        T: Any;

    /// Retrieve a module definition from the interpreter bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a module spec
    /// registered for it using [`ModuleRegistry::def_module`].
    fn module_spec<T>(&self) -> Result<Option<&Self::Spec>, Self::Error>
    where
        T: Any;

    /// Retrieve whether a module definition exists from the interpreter bound to Rust type `T`
    fn is_module_defined<T>(&self) -> bool
    where
        T: Any,
    {
        matches!(self.module_spec::<T>(), Ok(Some(_)))
    }

    fn module_of<T>(&mut self) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any;
}
