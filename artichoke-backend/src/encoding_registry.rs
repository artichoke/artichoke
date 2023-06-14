//use std::any::Any;

use crate::convert::boxing::BoxUnboxVmValue;
use crate::core::EncodingRegistry;
use crate::def::NotDefinedError;
use crate::encoding;
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::value::Value;
use crate::Artichoke;
use artichoke_core::constant::DefineConstant;
//use core::ffi::c_void;
use regex::Regex;
use std::sync::OnceLock;

impl EncodingRegistry for Artichoke {
    type Value = Value;
    type Error = Error;
    type Spec = encoding::Spec;

    fn def_encoding(&mut self, spec: Self::Spec) -> Result<(), Error> {
        // Setup the class name regex
        static ESCAPABLE_CONSTANT_CHARS: OnceLock<Regex> = OnceLock::new();
        let enscapable_const_char_regex = ESCAPABLE_CONSTANT_CHARS.get_or_init(|| {
            // Match any `-` and `.` chars. These will be replaced with `_`
            Regex::new(r"[-\.]+").unwrap()
        });

        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;

        // Get the Encoding class from the VM. It must be defined in order to
        // attach the encoding constants.
        let encoding_class = state
            .classes
            .get::<Self::Spec>()
            .ok_or_else(|| NotDefinedError::class(encoding::RUBY_TYPE))?;

        let rclass = encoding_class.rclass();
        unsafe { self.with_ffi_boundary(|mrb| rclass.resolve(mrb)) }?;

        // Allocate our encoding and register its variants as constants against
        // the Encoding class.
        let obj = Self::Spec::alloc_value(spec, self)?;
        let allocated_encoding = self.protect(obj);

        for alias in spec.names() {
            // Some of the `names` specified contain characters which would
            // require character escaping, however in MRI they are converted to
            // underscores.
            let alias = enscapable_const_char_regex.replace_all(alias, "_");
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
}
