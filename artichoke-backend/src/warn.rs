use log::warn;

use crate::convert::Convert;
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::{Mrb, MrbError};

/// Interpreters that implement [`MrbWarn`] expose methods for emitting warnings
/// during execution.
///
/// Some functionality required to be compliant with ruby-spec is deprecated or
/// invalid behavior and ruby-spec expects a warning to be emitted to `$stderr`
/// using the
/// [`Warning`](https://ruby-doc.org/core-2.6.3/Warning.html#method-i-warn)
/// module from the standard library.
#[allow(clippy::module_name_repetitions)]
pub trait MrbWarn {
    /// Emit a warning message using `Kernel#warn`.
    ///
    /// This method appends newlines to message if necessary.
    fn warn(&self, message: &str) -> Result<(), MrbError>;
}

impl MrbWarn for Mrb {
    fn warn(&self, message: &str) -> Result<(), MrbError> {
        warn!("rb warning: {}", message);
        let mrb = self.borrow().mrb;
        let kernel = unsafe {
            let stderr = sys::mrb_gv_get(mrb, self.borrow_mut().sym_intern("$stderr"));
            if sys::mrb_sys_value_is_nil(stderr) {
                return Ok(());
            }
            let kernel = (*mrb).kernel_module;
            Value::new(self, sys::mrb_sys_module_value(kernel))
        };
        kernel.funcall::<(), _, _>("warn", &[Value::from_mrb(self, message)])
    }
}
