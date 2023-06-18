use regex::Regex;
use std::sync::OnceLock;

use crate::convert::BoxUnboxVmValue as _;
use crate::core::EncodingRegistry;
use crate::def::NotDefinedError;
use crate::encoding;
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::value::Value;
use crate::Artichoke;
use artichoke_core::constant::DefineConstant;

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
        let _rclass = {
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
        let encoding = encoding::Encoding::alloc_value(spec, self)?;

        // Generate and assign all the constants for this encoding
        for alias in spec.names() {
            // Some of the `names` specified contain characters which would
            // require character escaping, however in MRI they are converted to
            // underscores.
            let alias = escapable_const_char_regex.replace_all(alias, "_");
            self.define_class_constant::<Self::Spec>(&alias, encoding)?;
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

    fn encoding_for(&mut self, value: &mut Self::Value) -> Result<Self::Spec, Self::Error> {
        let encoding = unsafe { encoding::Encoding::unbox_from_value(value, self)? };
        Ok(*encoding)
    }
}
