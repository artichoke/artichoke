use crate::convert::RustBackedValue;

pub mod backend;
pub mod mruby;
pub mod trampoline;

use backend::{chrono, MakeTime, TimeType};

#[must_use]
pub fn factory() -> impl MakeTime {
    chrono::Factory
}

pub struct Time(Box<dyn TimeType>);

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
