use std::any::Any;
use std::convert::TryFrom;

use crate::class;
use crate::exception::Exception;
use crate::ffi::InterpreterExtractError;
use crate::gc::{MrbGarbageCollection, State as GcState};
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
    /// Class definitions have the same lifetime as the
    /// [`State`](crate::state::State) because the class def owns the
    /// `mrb_data_type` for the type, which must be long-lived.
    fn def_class<T>(&mut self, spec: class::Spec) -> Result<(), Exception>
    where
        T: Any,
    {
        let state = self.state.as_mut().ok_or(InterpreterExtractError)?;
        state.classes.insert::<T>(Box::new(spec));
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
        let state = self.state.as_ref().ok_or(InterpreterExtractError)?;
        let spec = state.classes.get::<T>();
        Ok(spec)
    }

    fn class_of<T>(&mut self) -> Result<Option<Value>, Exception>
    where
        T: Any,
    {
        let state = self.state.as_ref().ok_or(InterpreterExtractError)?;
        let spec = state.classes.get::<T>();
        let spec = if let Some(spec) = spec {
            spec
        } else {
            return Ok(None);
        };
        unsafe {
            let mrb = self.mrb.as_mut();
            if let Some(mut rclass) = spec.rclass(mrb) {
                let class = sys::mrb_sys_class_value(rclass.as_mut());
                Ok(Some(Value::from(class)))
            } else {
                Ok(None)
            }
        }
    }

    fn new_instance<T>(&mut self, args: &[Value]) -> Result<Option<Value>, Exception>
    where
        T: Any,
    {
        // Disable the GC to prevent a collection cycle from re-entering into
        // Rust code while the `State` is moved out of the `mrb`.
        //
        // It is not safe to run with the GC enabled in this method because:
        //
        // 1. This method must hold a borrow on the `State` to grab a handle to
        //    the class spec -> `sys::mrb_data_type`.
        // 2. Because of (1), the `State` must be moved into the `Artichoke`
        //    struct.
        // 3. Because of (2), subsequent mruby FFI calls will have a `NULL` ud
        //    pointer.
        // 4. Because of (3), it is not safe to re-enter into any Artichoke
        //    implemented FFI trampolines that expect to extract an interpreter.
        // 5. Garbage collection mark functions are one such trampoline that are
        //    not safe to re-enter.
        // 6. `Array` is implemented in Rust and implements its GC mark routine
        //    expecting to extract an intialized `Artichoke`.
        // 7. Failing to extract an initialized `Artichoke`, `Array` GC mark is
        //    a no-op.
        // 6. Values in these `Array`s are deallocated as unreachable, creating
        //    dangling references that when accessed result in a use-after-free.
        //
        // The most expedient way to avoid this is turn off the GC when
        // allocating with `mrb_data_object_alloc` below.
        let prior_gc_state = self.disable_gc();

        let state = self.state.as_ref().ok_or(InterpreterExtractError)?;
        let spec = state.classes.get::<T>();
        let spec = if let Some(spec) = spec {
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
        let instance = unsafe {
            let mrb = self.mrb.as_mut();
            if let Some(mut rclass) = spec.rclass(mrb) {
                let value = sys::mrb_obj_new(mrb, rclass.as_mut(), arglen, args.as_ptr());
                Ok(Some(Value::from(value)))
            } else {
                Ok(None)
            }
        };
        if let GcState::Enabled = prior_gc_state {
            self.enable_gc();
        }

        instance
    }
}
