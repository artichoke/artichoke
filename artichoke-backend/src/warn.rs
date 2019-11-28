use artichoke_core::value::Value as _;
use artichoke_core::warn::Warn;

use crate::convert::Convert;
use crate::extn::core::warning::Warning;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Warn for Artichoke {
    fn warn(&self, message: &[u8]) -> Result<(), ArtichokeError> {
        warn!("rb warning: {}", String::from_utf8_lossy(message));
        let borrow = self.0.borrow();
        let warning = borrow.module_spec::<Warning>().ok_or_else(|| {
            ArtichokeError::NotDefined("Warn with uninitialized Warning".to_owned())
        })?;
        let warning = warning.value(self).ok_or_else(|| {
            ArtichokeError::NotDefined("Warn with uninitialized Warning".to_owned())
        })?;
        warning.funcall::<Value>("warn", &[self.convert(message)], None)?;
        Ok(())
    }
}
