use std::io::{self, Write};

use crate::convert::Convert;
use crate::exception::Exception;
use crate::extn::core::exception::RuntimeError;
use crate::extn::core::warning::Warning;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError, ValueLike, Warn};

impl Warn for Artichoke {
    type Error = Exception;

    fn warn(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        let _ = io::stderr().write_all(b"rb warning: ");
        let _ = io::stderr().write_all(message);
        let _ = io::stderr().write_all(b"\n");
        let warning = {
            let borrow = self.0.borrow();
            let spec = borrow
                .module_spec::<Warning>()
                .ok_or_else(|| ArtichokeError::NotDefined("Warn with uninitialized Warning".into()))
                .map_err(|err| RuntimeError::new(self, err.to_string()))?;
            spec.value(self)
                .ok_or_else(|| ArtichokeError::NotDefined("Warn with uninitialized Warning".into()))
                .map_err(|err| RuntimeError::new(self, err.to_string()))?
        };
        let message = self.convert(message);
        let _ = warning.funcall::<Value>("warn", &[message], None)?;
        Ok(())
    }
}
