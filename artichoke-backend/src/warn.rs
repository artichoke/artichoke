use std::fmt::Write;

use crate::core::{ModuleRegistry, TryConvertMut, Value as _, Warn};
use crate::def::NotDefinedError;
use crate::error::Error;
use crate::extn::core::exception::IOError;
use crate::extn::core::warning::Warning;
use crate::ffi::InterpreterExtractError;
use crate::state::output::Output;
use crate::string::WriteError;
use crate::Artichoke;

impl Warn for Artichoke {
    type Error = Error;

    fn warn(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        if let Err(err) = state.output.write_stderr(b"rb warning: ") {
            let mut message = String::from("Failed to write warning to $stderr: ");
            write!(&mut message, "{}", err).map_err(WriteError::from)?;
            return Err(IOError::from(message).into());
        }
        if let Err(err) = state.output.write_stderr(message) {
            let mut message = String::from("Failed to write warning to $stderr: ");
            write!(&mut message, "{}", err).map_err(WriteError::from)?;
            return Err(IOError::from(message).into());
        }
        if let Err(err) = state.output.write_stderr(b"\n") {
            let mut message = String::from("Failed to write warning to $stderr: ");
            write!(&mut message, "{}", err).map_err(WriteError::from)?;
            return Err(IOError::from(message).into());
        }
        let warning = self
            .module_of::<Warning>()?
            .ok_or_else(|| NotDefinedError::module("Warning"))?;
        let message = self.try_convert_mut(message)?;
        warning.funcall(self, "warn", &[message], None)?;
        Ok(())
    }
}
