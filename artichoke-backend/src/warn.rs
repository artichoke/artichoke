use artichoke_core::value::Value as _;
use artichoke_core::warn::Warn;
use std::borrow::Cow;

use crate::convert::Convert;
use crate::extn::core::warning::Warning;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Warn for Artichoke {
    fn warn(&self, message: &[u8]) -> Result<(), ArtichokeError> {
        warn!("rb warning: {}", String::from_utf8_lossy(message));
        let warning = {
            let borrow = self.0.borrow();
            let spec = borrow.module_spec::<Warning>().ok_or_else(|| {
                ArtichokeError::NotDefined(Cow::Borrowed("Warn with uninitialized Warning"))
            })?;
            spec.value(self).ok_or_else(|| {
                ArtichokeError::NotDefined(Cow::Borrowed("Warn with uninitialized Warning"))
            })?
        };
        warning.funcall::<Value>("warn", &[self.convert(message)], None)?;
        Ok(())
    }
}
