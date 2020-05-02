use std::borrow::Cow;

use crate::core::{Convert, Globals, Intern};
use crate::exception::Exception;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

// TODO: Handle invalid variable names. For now this is delegated to mruby.

impl Globals for Artichoke {
    type Value = Value;

    type Error = Exception;

    fn set_global_variable<T>(&mut self, name: T, value: &Self::Value) -> Result<(), Self::Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let sym = self.intern_symbol(name);
        let mrb = self.0.borrow().mrb;
        unsafe {
            sys::mrb_gv_set(mrb, sym, value.inner());
        }
        Ok(())
    }

    /// Unset global variable pointed to by `name`.
    ///
    /// Unsetting a global variable removes the name from the global storage
    /// table. Unset globals resolve to `nil` in the Ruby VM.
    ///
    /// Unsetting a global that is currently unset is a no-op.
    ///
    /// # Errors
    ///
    /// If the name is not a valid global name, an error is returned.
    fn unset_global_variable<T>(&mut self, name: T) -> Result<(), Self::Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let sym = self.intern_symbol(name);
        let mrb = self.0.borrow().mrb;
        let nil = self.convert(None::<Value>);
        unsafe {
            sys::mrb_gv_set(mrb, sym, nil.inner());
        }
        Ok(())
    }

    fn get_global_variable<T>(&mut self, name: T) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let sym = self.intern_symbol(name);
        let mrb = self.0.borrow().mrb;
        let value = unsafe { sys::mrb_gv_get(mrb, sym) };
        // NOTE: This implementation is not compliant with the spec laid out in
        // the trait documentation. This implementation always returns `Some(_)`
        // even if the global is unset.
        Ok(Some(Value::new(self, value)))
    }
}
