use crate::core::EncodingRegistry;
use crate::error::Error;
use crate::extn::core::encoding::Encoding;
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;
use artichoke_core::encoding::Encoding as _;

pub type Spec = Encoding;

impl EncodingRegistry for Artichoke {
    type Value = Value;
    type Error = Error;
    type Spec = Spec;

    fn def_encoding(&mut self, spec: Self::Spec) -> Result<(), Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.encodings.insert(spec.flag(), spec);
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
}
