use std::ffi::CString;

use crate::core::DefineConstant;
use crate::def::{ConstantNameError, NotDefinedError};
use crate::exception::Exception;
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

impl DefineConstant for Artichoke {
    type Value = Value;

    type Error = Exception;

    fn define_global_constant(
        &mut self,
        constant: &str,
        value: Self::Value,
    ) -> Result<(), Self::Error> {
        let name = if let Ok(name) = CString::new(constant) {
            name
        } else {
            return Err(ConstantNameError::from(String::from(constant)).into());
        };
        unsafe {
            self.with_ffi_boundary(|mrb| {
                sys::mrb_define_global_const(mrb, name.as_ptr(), value.inner())
            })?;
        }
        Ok(())
    }

    fn define_class_constant<T>(
        &mut self,
        constant: &str,
        value: Self::Value,
    ) -> Result<(), Self::Error>
    where
        T: 'static,
    {
        let name = if let Ok(name) = CString::new(constant) {
            name
        } else {
            return Err(ConstantNameError::from(String::from(constant)).into());
        };
        let state = self.state.as_mut().ok_or(InterpreterExtractError::new())?;
        let spec = state
            .classes
            .get::<T>()
            .ok_or_else(|| NotDefinedError::class_constant(String::from(constant)))?;
        let rclass = spec.rclass();

        let rclass = unsafe { self.with_ffi_boundary(|mrb| rclass.resolve(mrb))? };
        if let Some(mut rclass) = rclass {
            unsafe {
                self.with_ffi_boundary(|mrb| {
                    sys::mrb_define_const(mrb, rclass.as_mut(), name.as_ptr(), value.inner());
                })?;
            }
            Ok(())
        } else {
            Err(NotDefinedError::class_constant(String::from(constant)).into())
        }
    }

    fn define_module_constant<T>(
        &mut self,
        constant: &str,
        value: Self::Value,
    ) -> Result<(), Self::Error>
    where
        T: 'static,
    {
        let name = if let Ok(name) = CString::new(constant) {
            name
        } else {
            return Err(ConstantNameError::from(String::from(constant)).into());
        };
        let state = self.state.as_mut().ok_or(InterpreterExtractError::new())?;
        let spec = state
            .modules
            .get::<T>()
            .ok_or_else(|| NotDefinedError::module_constant(String::from(constant)))?;
        let rclass = spec.rclass();

        let rclass = unsafe { self.with_ffi_boundary(|mrb| rclass.resolve(mrb))? };
        if let Some(mut rclass) = rclass {
            unsafe {
                self.with_ffi_boundary(|mrb| {
                    sys::mrb_define_const(mrb, rclass.as_mut(), name.as_ptr(), value.inner());
                })?;
            }
            Ok(())
        } else {
            Err(NotDefinedError::module_constant(String::from(constant)).into())
        }
    }
}
