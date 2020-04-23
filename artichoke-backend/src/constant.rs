use std::ffi::CString;

use crate::core::DefineConstant;
use crate::def::{ConstantNameError, NotDefinedError};
use crate::exception::Exception;
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
        let name =
            CString::new(constant).map_err(|_| ConstantNameError::new(String::from(constant)))?;
        unsafe {
            let mrb = self.mrb.as_mut();
            sys::mrb_define_global_const(mrb, name.as_ptr() as *const i8, value.inner());
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
        let name =
            CString::new(constant).map_err(|_| ConstantNameError::new(String::from(constant)))?;
        let mrb = self.mrb.as_mut();
        let mut rclass = borrow
            .class_spec::<T>()
            .and_then(|spec| spec.rclass(mrb))
            .ok_or_else(|| NotDefinedError::class_constant(String::from(constant)))?;
        unsafe {
            sys::mrb_define_const(
                mrb,
                rclass.as_mut(),
                name.as_ptr() as *const i8,
                value.inner(),
            );
        }
        Ok(())
    }

    fn define_module_constant<T>(
        &mut self,
        constant: &str,
        value: Self::Value,
    ) -> Result<(), Self::Error>
    where
        T: 'static,
    {
        let name =
            CString::new(constant).map_err(|_| ConstantNameError::new(String::from(constant)))?;
        let mrb = self.mrb.as_mut();
        let mut rclass = borrow
            .module_spec::<T>()
            .and_then(|spec| spec.rclass(mrb))
            .ok_or_else(|| NotDefinedError::module_constant(String::from(constant)))?;
        unsafe {
            sys::mrb_define_const(
                mrb,
                rclass.as_mut(),
                name.as_ptr() as *const i8,
                value.inner(),
            );
        }
        Ok(())
    }
}
