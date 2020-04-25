use std::any::Any;

use crate::exception::Exception;
use crate::module;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

pub trait ModuleRegistry {
    fn def_module<T>(&mut self, spec: module::Spec) -> Result<(), Exception>
    where
        T: Any;

    fn module_spec<T>(&self) -> Result<Option<&module::Spec>, Exception>
    where
        T: Any;

    fn is_module_defined<T>(&self) -> bool
    where
        T: Any,
    {
        if let Ok(Some(_)) = self.module_spec::<T>() {
            true
        } else {
            false
        }
    }

    fn module_of<T>(&mut self) -> Result<Option<Value>, Exception>
    where
        T: Any;
}

impl ModuleRegistry for Artichoke {
    /// Create a module definition bound to a Rust type `T`.
    ///
    /// Module definitions have the same lifetime as the interpreter because the
    /// module def owns the `mrb_data_type` for the type, which must be
    /// long-lived.
    fn def_module<T>(&mut self, spec: module::Spec) -> Result<(), Exception>
    where
        T: Any,
    {
        self.state.modules.insert::<T>(Box::new(spec));
        Ok(())
    }

    /// Retrieve a module definition from the interpreter bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a module spec
    /// registered for it using [`ModuleRegistry::def_module`].
    fn module_spec<T>(&self) -> Result<Option<&module::Spec>, Exception>
    where
        T: Any,
    {
        Ok(self.state.modules.get::<T>())
    }

    fn module_of<T>(&mut self) -> Result<Option<Value>, Exception>
    where
        T: Any,
    {
        let spec = if let Some(spec) = self.state.modules.get::<T>() {
            spec
        } else {
            return Ok(None);
        };
        let module = unsafe {
            let mrb = self.mrb.as_mut();
            if let Some(mut rclass) = spec.rclass(mrb) {
                sys::mrb_sys_module_value(rclass.as_mut())
            } else {
                return Ok(None);
            }
        };
        Ok(Some(Value::new(self, module)))
    }
}
