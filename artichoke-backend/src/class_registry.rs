use std::any::Any;
use std::convert::TryFrom;

use crate::class;
use crate::exception::Exception;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
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

    fn class_of<T>(&mut self) -> Result<Option<Value>, Exception>
    where
        T: Any;

    fn new_instance<T>(&mut self, args: &[Value]) -> Result<Option<Value>, Exception>
    where
        T: Any;
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

    fn class_of<T>(&mut self) -> Result<Option<Value>, Exception>
    where
        T: Any,
    {
        let spec = if let Some(spec) = self.state.classes.get::<T>() {
            spec
        } else {
            return Ok(None);
        };
        let class = unsafe {
            let mrb = self.mrb.as_mut();
            if let Some(mut rclass) = spec.rclass(mrb) {
                sys::mrb_sys_class_value(rclass.as_mut())
            } else {
                return Ok(None);
            }
        };
        Ok(Some(Value::new(self, class)))
    }

    fn new_instance<T>(&mut self, args: &[Value]) -> Result<Option<Value>, Exception>
    where
        T: Any,
    {
        let spec = if let Some(spec) = self.state.classes.get::<T>() {
            spec
        } else {
            return Ok(None);
        };
        let args = args.iter().map(Value::inner).collect::<Vec<_>>();
        let arglen = if let Ok(len) = Int::try_from(args.len()) {
            len
        } else {
            return Ok(None);
        };
        let value = unsafe {
            let mrb = self.mrb.as_mut();
            if let Some(mut rclass) = spec.rclass(mrb) {
                sys::mrb_obj_new(mrb, rclass.as_mut(), arglen, args.as_ptr())
            } else {
                return Ok(None);
            }
        };
        Ok(Some(Value::new(self, value)))
    }
}
