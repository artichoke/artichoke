use std::any::Any;

use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::module;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

pub trait ModuleRegistry {
    fn def_module<T>(&mut self, spec: module::Spec) -> Result<(), Error>
    where
        T: Any;

    fn module_spec<T>(&self) -> Result<Option<&module::Spec>, Error>
    where
        T: Any;

    fn is_module_defined<T>(&self) -> bool
    where
        T: Any,
    {
        matches!(self.module_spec::<T>(), Ok(Some(_)))
    }

    fn module_of<T>(&mut self) -> Result<Option<Value>, Error>
    where
        T: Any;
}

impl ModuleRegistry for Artichoke {
    /// Create a module definition bound to a Rust type `T`.
    ///
    /// Module definitions have the same lifetime as the interpreter because the
    /// module def owns the `mrb_data_type` for the type, which must be
    /// long-lived.
    fn def_module<T>(&mut self, spec: module::Spec) -> Result<(), Error>
    where
        T: Any,
    {
        let state = self.state.as_mut().ok_or_else(InterpreterExtractError::new)?;
        state.modules.insert::<T>(Box::new(spec));
        Ok(())
    }

    /// Retrieve a module definition from the interpreter bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a module spec
    /// registered for it using [`ModuleRegistry::def_module`].
    fn module_spec<T>(&self) -> Result<Option<&module::Spec>, Error>
    where
        T: Any,
    {
        let state = self.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state.modules.get::<T>();
        Ok(spec)
    }

    fn module_of<T>(&mut self) -> Result<Option<Value>, Error>
    where
        T: Any,
    {
        let state = self.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state.modules.get::<T>();
        let spec = if let Some(spec) = spec {
            spec
        } else {
            return Ok(None);
        };
        let rclass = spec.rclass();
        let rclass = unsafe { self.with_ffi_boundary(|mrb| rclass.resolve(mrb))? };
        if let Some(mut rclass) = rclass {
            let module = unsafe { sys::mrb_sys_module_value(rclass.as_mut()) };
            let module = Value::from(module);
            Ok(Some(module))
        } else {
            Ok(None)
        }
    }
}
