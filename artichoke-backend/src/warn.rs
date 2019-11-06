use artichoke_core::value::Value as _;

use crate::convert::Convert;
use crate::extn::core::warning::Warning;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

/// Interpreters that implement [`Warn`] expose methods for emitting warnings
/// during execution.
///
/// Some functionality required to be compliant with ruby-spec is deprecated or
/// invalid behavior and ruby-spec expects a warning to be emitted to `$stderr`
/// using the
/// [`Warning`](https://ruby-doc.org/core-2.6.3/Warning.html#method-i-warn)
/// module from the standard library.
pub trait Warn {
    /// Emit a warning message using `Warning#warn`.
    ///
    /// This method appends newlines to message if necessary.
    fn warn(&self, message: &str) -> Result<(), ArtichokeError>;
}

impl Warn for Artichoke {
    fn warn(&self, message: &str) -> Result<(), ArtichokeError> {
        warn!("rb warning: {}", message);
        let warning = self.0.borrow().module_spec::<Warning>().ok_or_else(|| {
            ArtichokeError::NotDefined("Warn with uninitialized Warning".to_owned())
        })?;
        let warning = warning.borrow().value(self).ok_or_else(|| {
            ArtichokeError::NotDefined("Warn with uninitialized Warning".to_owned())
        })?;
        warning.funcall::<Value>(self, "warn", &[self.convert(message)], None)?;
        Ok(())
    }
}
