use std::any::Any;

use crate::core::ModuleRegistry;
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::module;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

impl ModuleRegistry for Artichoke {
    type Value = Value;
    type Error = Error;
    type Spec = module::Spec;

    fn def_module<T>(&mut self, spec: Self::Spec) -> Result<(), Self::Error>
    where
        T: Any,
    {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.modules.insert::<T>(Box::new(spec));
        Ok(())
    }

    fn module_spec<T>(&self) -> Result<Option<&Self::Spec>, Self::Error>
    where
        T: Any,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state.modules.get::<T>();
        Ok(spec)
    }

    fn module_of<T>(&mut self) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
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
