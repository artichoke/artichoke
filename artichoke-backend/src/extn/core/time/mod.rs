use chrono::Local;
use std::fmt;

use crate::convert::RustBackedValue;

pub mod backend;
pub mod mruby;
pub mod trampoline;

use backend::chrono::{Chrono, Factory};
use backend::{MakeTime, TimeType};

#[must_use]
pub fn factory() -> impl MakeTime {
    Factory
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

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(backend) = self.0.downcast_ref::<Chrono<Local>>() {
            f.debug_struct("Time").field("backend", backend).finish()
        } else {
            f.debug_struct("Time").field("backend", &"unknown").finish()
        }
    }
}
