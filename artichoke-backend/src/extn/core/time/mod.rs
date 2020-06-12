use chrono::Local;
use std::fmt;

pub mod backend;
mod boxing;
pub mod mruby;
pub mod trampoline;

use backend::chrono::{Chrono, Factory};
use backend::{MakeTime, TimeType};

pub struct Time(Box<dyn TimeType>);

impl From<Chrono<Local>> for Time {
    fn from(backend: Chrono<Local>) -> Self {
        Self(Box::new(backend))
    }
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Time")
            .field("backend", self.0.as_debug())
            .finish()
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::now()
    }
}

impl Time {
    #[must_use]
    pub fn new() -> Self {
        Self::now()
    }

    #[must_use]
    pub fn now() -> Self {
        Self(Box::new(Factory.now()))
    }

    #[must_use]
    pub fn inner(&self) -> &dyn TimeType {
        self.0.as_ref()
    }

    pub fn inner_mut(&mut self) -> &dyn TimeType {
        self.0.as_mut()
    }
}
