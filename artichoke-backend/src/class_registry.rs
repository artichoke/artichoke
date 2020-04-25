use std::any::Any;

use crate::class;
use crate::ffi::Exception;
use crate::Artichoke;

pub trait ClassRegistry {
    fn def_class<T>(&mut self, spec: class::Spec) -> Result<(), Exception>
    where
        T: Any;

    fn class_spec<T>(&self) -> Result<Option<&class::Spec>, Exception>
    where
        T: Any;

    fn is_class_defined<T>(&self) -> bool
    where
        T: Any,
    {
        if let Ok(Some(_)) = self.class_spec::<T>() {
            true
        } else {
            false
        }
    }
}

impl ClassRegistry for Artichoke {
    /// Create a class definition bound to a Rust type `T`.
    ///
    /// Class definitions have the same lifetime as the [`State`] because the
    /// class def owns the `mrb_data_type` for the type, which must be
    /// long-lived.
    fn def_class<T>(&mut self, spec: class::Spec) -> Result<(), Exception>
    where
        T: Any,
    {
        self.state.classes.insert::<T>(Box::new(spec));
        Ok(())
    }

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`ClassRegistry::def_class`].
    fn class_spec<T>(&self) -> Result<Option<&class::Spec>, Exception>
    where
        T: Any,
    {
        Ok(self.state.classes.get::<T>())
    }
}
