use bstr::ByteVec;

use crate::core::EncodingRegistry;
use crate::def::NotDefinedError;
use crate::error::Error;
use crate::extn::core::encoding::{Encoding, RUBY_TYPE};
use crate::ffi::InterpreterExtractError;
use crate::prelude::*;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;
use artichoke_core::constant::DefineConstant;
use artichoke_core::encoding::Encoding as _;
use artichoke_core::value::Value as _;

pub type Spec = Encoding;

impl EncodingRegistry for Artichoke {
    type Value = Value;
    type Error = Error;
    type Spec = Spec;

    fn def_encoding(&mut self, spec: Self::Spec) -> Result<(), Error> {
        // Get the Encoding class from the VM. It must be defined in order to
        // attach the encoding constants.
        let _rclass = {
            let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
            let encoding_class = state
                .classes
                .get::<Self::Spec>()
                .ok_or_else(|| NotDefinedError::class(RUBY_TYPE))?;

            let rclass = encoding_class.rclass();
            unsafe { self.with_ffi_boundary(|mrb| rclass.resolve(mrb)) }?
                .ok_or_else(|| NotDefinedError::class(RUBY_TYPE))?
        };

        // Allocate the encoding
        // TODO: Sanitize that the encoding doesn't already exist in the registry
        let id = spec.flag();
        let obj = unsafe { sys::mrb_sys_new_encoding(i64::from(id)) };
        let encoding = Value::from(obj);

        // Generate and assign all the constants for this encoding
        for alias in spec.aliases() {
            // TODO: Better way to unwrap Vec<u8> into &str
            self.define_class_constant::<Self::Spec>(&alias.into_string().unwrap(), encoding)?;
        }

        // We should now be able to register this Encoding as being in use.
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.encodings.insert(id, spec);
        Ok(())
    }

    fn encodings(&self) -> Result<Vec<&Self::Spec>, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;

        let result = state.encodings.values().collect();

        Ok(result)
    }

    fn encoding_of(&self, spec: &Self::Spec) -> Result<Option<Self::Value>, Self::Error> {
        // Check to make sure the encoding was registered
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;

        let id = spec.flag();
        if !state.encodings.contains_key(&id) {
            return Ok(None);
        }

        let obj = unsafe { sys::mrb_sys_new_encoding(i64::from(id)) };
        Ok(Some(Value::from(obj)))
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn encoding_for(&mut self, value: &mut Self::Value) -> Result<&Self::Spec, Self::Error> {
        if value.ruby_type() != Ruby::Encoding {
            let mut message = String::from("uninitialized ");
            message.push_str(RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        let value = unsafe { value.inner().value.i };
        let value: u8 = if let 0..=255 = value {
            // SAFETY: 0..=255 will always be able to be cast into a `u8`
            value as u8
        } else {
            let mut message = String::from("uninitialized ");
            message.push_str(RUBY_TYPE);
            return Err(TypeError::from(message).into());
        };

        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.encodings.get(&value).ok_or_else(|| {
            let mut message = String::from("uninitialized ");
            message.push_str(RUBY_TYPE);
            TypeError::from(message).into()
        })
    }
}
