use std::fmt::Write;

use crate::def::NotDefinedError;
use crate::exception::Exception;
use crate::extn::core::exception::IOError;
use crate::extn::core::warning::Warning;
use crate::state::output::Output;
use crate::value::Value;
use crate::{Artichoke, ConvertMut, ValueLike, Warn};

impl Warn for Artichoke {
    type Error = Exception;

    fn warn(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        {
            let mut borrow = self.0.borrow_mut();
            borrow
                .output
                .write_stderr(b"rb warning: ")
                .and_then(|_| borrow.output.write_stderr(message))
                .and_then(|_| borrow.output.write_stderr(b"\n"))
                .map_err(|err| {
                    let mut message = String::from("Failed to write warning to $stderr: ");
                    let _ = write!(&mut message, "{}", err);
                    IOError::new(self, message)
                })?
        }
        let warning = {
            let borrow = self.0.borrow();
            let spec = borrow
                .module_spec::<Warning>()
                .ok_or_else(|| NotDefinedError::module("Warning"))?;
            spec.value(self)
                .ok_or_else(|| NotDefinedError::module("Warning"))?
        };
        let message = self.convert_mut(message);
        let _ = warning.funcall::<Value>("warn", &[message], None)?;
        Ok(())
    }
}
