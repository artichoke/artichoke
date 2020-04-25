use std::fmt::Write;

use crate::core::{ConvertMut, Value as _, Warn};
use crate::def::NotDefinedError;
use crate::exception::Exception;
use crate::extn::core::exception::IOError;
use crate::extn::core::warning::Warning;
use crate::module_registry::ModuleRegistry;
use crate::state::output::Output;
use crate::value::Value;
use crate::Artichoke;

impl Warn for Artichoke {
    type Error = Exception;

    fn warn(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        if let Err(err) = self.state.output.write_stderr(b"rb warning: ") {
            let mut message = String::from("Failed to write warning to $stderr: ");
            let _ = write!(&mut message, "{}", err);
            return Err(IOError::new(self, message).into());
        }
        if let Err(err) = self.state.output.write_stderr(message) {
            let mut message = String::from("Failed to write warning to $stderr: ");
            let _ = write!(&mut message, "{}", err);
            return Err(IOError::new(self, message).into());
        }
        if let Err(err) = self.state.output.write_stderr(b"\n") {
            let mut message = String::from("Failed to write warning to $stderr: ");
            let _ = write!(&mut message, "{}", err);
            return Err(IOError::new(self, message).into());
        }
        let spec = self
            .module_spec::<Warning>()?
            .ok_or_else(|| NotDefinedError::module("Warning"))?;
        let warning = spec
            .value(self)
            .ok_or_else(|| NotDefinedError::module("Warning"))?;
        let message = self.convert_mut(message);
        let _ = warning.funcall::<Value>(self, "warn", &[message], None)?;
        Ok(())
    }
}
