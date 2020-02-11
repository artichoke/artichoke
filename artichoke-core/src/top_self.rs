//! Expose the global context, called [_top self_][topself], to the interpreter.
//!
//! [topself]: https://www.sitepoint.com/rubys-top-self-object/

use crate::value::Value;

/// Return a `Value`-wrapped reference to [_top self_][topself].
///
/// Top self is the root object that evaled code is executed within. Global
/// methods, classes, and modules are defined in top self.
///
/// [topself]: https://www.sitepoint.com/rubys-top-self-object/
pub trait TopSelf {
    /// Concrete [`Value`] type.
    type Value: Value;

    /// Return a [`Value`]-wrapped reference to "top self".
    ///
    /// Top self is the root object that evaled code is executed within. Global
    /// methods, classes, and modules are defined in top self.
    fn top_self(&self) -> Self::Value;
}
