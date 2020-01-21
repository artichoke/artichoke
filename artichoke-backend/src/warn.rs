use artichoke_core::value::Value as _;
use artichoke_core::warn::Warn;
use std::borrow::Cow;

use crate::convert::Convert;
use crate::exception::Exception;
use crate::extn::core::exception::RuntimeError;
use crate::extn::core::warning::Warning;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Warn for Artichoke {
    type Error = Exception;

    fn warn(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        warn!("rb warning: {}", String::from_utf8_lossy(message));
        let warning = {
            let spec = self
                .state()
                .module_spec::<Warning>()
                .ok_or_else(|| {
                    ArtichokeError::NotDefined(Cow::Borrowed("Warn with uninitialized Warning"))
                })
                .map_err(|err| RuntimeError::new(self, format!("{}", err)))?;
            spec.value(self)
                .ok_or_else(|| {
                    ArtichokeError::NotDefined(Cow::Borrowed("Warn with uninitialized Warning"))
                })
                .map_err(|err| RuntimeError::new(self, format!("{}", err)))?
        };
        let _ = warning.funcall::<Value>(self, "warn", &[self.convert(message)], None)?;
        Ok(())
    }
}
