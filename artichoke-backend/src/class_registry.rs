use std::any::Any;

use crate::class;
use crate::core::ClassRegistry;
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

impl ClassRegistry for Artichoke {
    type Value = Value;
    type Error = Error;
    type Spec = class::Spec;

    fn def_class<T>(&mut self, spec: Self::Spec) -> Result<(), Self::Error>
    where
        T: Any,
    {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.classes.insert::<T>(Box::new(spec));
        Ok(())
    }

    fn class_spec<T>(&self) -> Result<Option<&Self::Spec>, Self::Error>
    where
        T: Any,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state.classes.get::<T>();
        Ok(spec)
    }

    fn class_of<T>(&mut self) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state.classes.get::<T>();
        let rclass = if let Some(spec) = spec {
            spec.rclass()
        } else {
            return Ok(None);
        };
        let value_class = unsafe {
            self.with_ffi_boundary(|mrb| {
                if let Some(mut rclass) = rclass.resolve(mrb) {
                    let value_class = sys::mrb_sys_class_value(rclass.as_mut());
                    Some(Value::from(value_class))
                } else {
                    None
                }
            })?
        };
        Ok(value_class)
    }

    fn new_instance<T>(&mut self, args: &[Self::Value]) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Any,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state.classes.get::<T>();
        let rclass = if let Some(spec) = spec {
            spec.rclass()
        } else {
            return Ok(None);
        };
        let args = args.iter().map(Value::inner).collect::<Vec<_>>();
        let arglen = if let Ok(len) = sys::mrb_int::try_from(args.len()) {
            len
        } else {
            return Ok(None);
        };
        let instance = unsafe {
            self.with_ffi_boundary(|mrb| {
                if let Some(mut rclass) = rclass.resolve(mrb) {
                    let value = sys::mrb_obj_new(mrb, rclass.as_mut(), arglen, args.as_ptr());
                    Some(Value::from(value))
                } else {
                    None
                }
            })?
        };

        Ok(instance)
    }
}
