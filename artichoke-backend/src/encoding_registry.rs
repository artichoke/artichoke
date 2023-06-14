use core::ffi::c_void;
use regex::Regex;
use spinoso_exception::TypeError;
use std::sync::OnceLock;

use crate::core::EncodingRegistry;
use crate::def::NotDefinedError;
use crate::encoding;
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;
use artichoke_core::constant::DefineConstant;
use artichoke_core::value::Value as _;
use std::ptr;

impl EncodingRegistry for Artichoke {
    type Value = Value;
    type Error = Error;
    type Spec = encoding::Spec;

    fn def_encoding(&mut self, spec: Self::Spec) -> Result<(), Error> {
        // Setup the class name regex
        static ESCAPABLE_CONSTANT_CHARS: OnceLock<Regex> = OnceLock::new();
        let escapable_const_char_regex = ESCAPABLE_CONSTANT_CHARS.get_or_init(|| {
            // Match any `-` and `.` chars. These will be replaced with `_`
            Regex::new(r"[-\.]+").unwrap()
        });

        // Get the Encoding class from the VM. It must be defined in order to
        // attach the encoding constants.
        let mut rclass = {
            let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
            let encoding_class = state
                .classes
                .get::<Self::Spec>()
                .ok_or_else(|| NotDefinedError::class(encoding::RUBY_TYPE))?;

            let rclass = encoding_class.rclass();
            unsafe { self.with_ffi_boundary(|mrb| rclass.resolve(mrb)) }?
                .ok_or_else(|| NotDefinedError::class(encoding::RUBY_TYPE))?
        };

        // Allocate the encoding
        let data = Box::new(spec);
        let ptr = Box::into_raw(data);
        println!("ptr: {ptr:?}");
        let data_type = Box::new(encoding::DATA_TYPE);

        let obj = unsafe {
            self.with_ffi_boundary(|mrb| {
                let alloc = sys::mrb_data_object_alloc(mrb, rclass.as_mut(), ptr.cast::<c_void>(), data_type.as_ref());
                sys::mrb_sys_obj_value(alloc.cast::<c_void>())
            })?
        };

        unsafe {
            println!("obj.inner.tt: {:?}", obj.tt);
            println!("obj.inner.value.p: {:?}", obj.value.p);
        }

        let allocated_encoding = self.protect(Value::from(obj));

        // Generate and assign all the constants for this encoding
        for alias in spec.names() {
            // Some of the `names` specified contain characters which would
            // require character escaping, however in MRI they are converted to
            // underscores.
            let alias = escapable_const_char_regex.replace_all(alias, "_");
            self.define_class_constant::<Self::Spec>(&alias, allocated_encoding)?;
        }

        // We should now be able to register this Encoding as being in use.
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.encodings.insert(spec);
        Ok(())
    }

    fn encodings(&self) -> Result<Vec<Self::Spec>, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;

        // TODO: Why clone
        let result = state.encodings.clone().into_iter().collect();

        Ok(result)
    }

    fn encoding_of(&self) -> Result<Option<Self::Value>, Self::Error> {
        todo!()
    }

    fn encoding_for(&mut self, value: &Self::Value) -> Result<Self::Spec, Self::Error> {
        // Make sure we have a Data otherwise extraction will fail.
        if value.ruby_type() != Ruby::Data {
            let mut message = String::from("uninitialized ");
            message.push_str(encoding::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        let mut rclass = {
            let state = self.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
            let spec = state
                .classes
                .get::<Self::Spec>()
                .ok_or_else(|| NotDefinedError::class(encoding::RUBY_TYPE))?;
            let rclass = spec.rclass();
            unsafe {
                self.with_ffi_boundary(|mrb| rclass.resolve(mrb))?
                    .ok_or_else(|| NotDefinedError::class(encoding::RUBY_TYPE))?
            }
        };

        // Sanity check that the RClass matches.
        unsafe {
            let value_rclass = self.with_ffi_boundary(|mrb| sys::mrb_sys_class_of_value(mrb, value.inner()))?;
            if !ptr::eq(value_rclass, rclass.as_mut()) {
                let mut message = String::from("Could not extract ");
                message.push_str(encoding::RUBY_TYPE);
                message.push_str(" from receiver");
                return Err(TypeError::from(message).into());
            }
        };

        // Copy data pointer out of the `mrb_value` box.
        let state = self.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let _spec = state
            .classes
            .get::<Self::Spec>()
            .ok_or_else(|| NotDefinedError::class(encoding::RUBY_TYPE))?;
        let data_type = Box::new(encoding::DATA_TYPE);

        println!("v.inner.tt: {:?}", value.inner().tt);
        unsafe {
            println!("v.inner.value.p: {:?}", value.inner().value.p);
        }
        let embedded_data_ptr: *mut c_void = unsafe {
            self.with_ffi_boundary(|mrb| sys::mrb_data_check_get_ptr(mrb, value.inner(), data_type.as_ref()))?
        };
        println!("embedded_data_ptr: {embedded_data_ptr:?}");
        if embedded_data_ptr.is_null() {
            // `Object#allocate` can be used to create `MRB_TT_CDATA` without
            // calling `#initialize`. These objects will return a NULL pointer.
            let mut message = String::from("uninitialized ");
            message.push_str(encoding::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        // Move the data pointer into a `Box`.
        let value: Box<Self::Spec> = unsafe { Box::from_raw(embedded_data_ptr.cast::<Self::Spec>()) };

        // Never called
        println!("fooo: {value:?}");
        // `UnboxedValueGuard` ensures the `Box` wrapper will be forgotten. The
        // mruby GC is responsible for freeing the value.
        //Ok(UnboxedValueGuard::new(HeapAllocated::new(value)))
        Ok(*value)
    }
}
