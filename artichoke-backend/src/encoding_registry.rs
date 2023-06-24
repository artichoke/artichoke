use bstr::ByteVec;

use crate::core::EncodingRegistry;
use crate::def::NotDefinedError;
use crate::error::Error;
use crate::extn::core::encoding::Encoding;
use crate::ffi::InterpreterExtractError;
use crate::prelude::*;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;
use artichoke_core::constant::DefineConstant;
use artichoke_core::value::Value as _;

pub type Spec = Encoding;

/// Define an encoding which can be stored by `EncodingRegistry`.
pub trait EncodingSpec {
    const RUBY_TYPE: &'static str;

    fn id(&self) -> i64;
    fn aliases(&self) -> Vec<Vec<u8>>;
    fn is_ascii_compatible(&self) -> bool;
    fn is_dummy(&self) -> bool;
    fn inspect(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn names(&self) -> &'static [&'static str];
}

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
                .ok_or_else(|| NotDefinedError::class(Spec::RUBY_TYPE))?;

            let rclass = encoding_class.rclass();
            unsafe { self.with_ffi_boundary(|mrb| rclass.resolve(mrb)) }?
                .ok_or_else(|| NotDefinedError::class(Spec::RUBY_TYPE))?
        };

        // Allocate the encoding
        // TODO: Sanitize that the encoding doesn't already exist in the registry
        let id = spec.id();
        let obj = unsafe { sys::mrb_sys_new_encoding(id) };
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

    fn encoding_of(&self, _spec: &Self::Spec) -> Result<Option<Self::Value>, Self::Error> {
        todo!()
    }

    fn encoding_for(&mut self, value: &mut Self::Value) -> Result<&Self::Spec, Self::Error> {
        if value.ruby_type() != Ruby::Encoding {
            let mut message = String::from("uninitialized ");
            message.push_str(Spec::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        let value = unsafe { value.inner().value.i };

        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.encodings.get(&value).ok_or_else(|| {
            let mut message = String::from("uninitialized ");
            message.push_str(Spec::RUBY_TYPE);
            TypeError::from(message).into()
        })
    }
}
