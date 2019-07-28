use crate::sys;
use crate::value::Value;
use crate::Mrb;

/// Return a [`Value`]-wrapped reference to "top self".
///
/// Top self is the root object that evaled code is executed within. Global
/// methods, classes, and modules are defined in top self.
#[allow(clippy::module_name_repetitions)]
pub trait TopSelf {
    /// Return a [`Value`]-wrapped reference to "top self".
    ///
    /// Top self is the root object that evaled code is executed within. Global
    /// methods, classes, and modules are defined in top self.
    fn top_self(&self) -> Value;
}

impl TopSelf for Mrb {
    fn top_self(&self) -> Value {
        Value::new(self, unsafe { sys::mrb_top_self(self.borrow().mrb) })
    }
}
