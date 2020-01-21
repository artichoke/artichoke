//! Expose the global context, called
//! [_top self_](https://www.sitepoint.com/rubys-top-self-object/), to the
//! interpreter.

use crate::value::Value;

/// Return a [`Value`]-wrapped reference to "top self".
///
/// Top self is the root object that evaled code is executed within. Global
/// methods, classes, and modules are defined in top self.
#[allow(clippy::module_name_repetitions)]
pub trait TopSelf {
    /// Concrete [`Value`] type.
    type Value: Value;

    /// Return a [`Value`]-wrapped reference to "top self".
    ///
    /// Top self is the root object that evaled code is executed within. Global
    /// methods, classes, and modules are defined in top self.
    fn top_self(&mut self) -> Self::Value;
}
