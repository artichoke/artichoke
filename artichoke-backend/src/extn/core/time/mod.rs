use std::fmt;

use crate::convert::RustBackedValue;

pub mod backend;
pub mod mruby;
pub mod trampoline;

use backend::chrono::Factory;
use backend::{MakeTime, TimeType};

#[must_use]
pub fn factory() -> impl MakeTime {
    Factory
}

pub struct Time(Box<dyn TimeType>);

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Time")
            .field("backend", self.0.as_debug())
            .finish()
    }
}

impl Time {
    fn inner(&self) -> &dyn TimeType {
        self.0.as_ref()
    }
}

impl RustBackedValue for Time {
    fn ruby_type_name() -> &'static str {
        "Time"
    }
}
