use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Weekday};
use std::fmt;

use crate::extn::core::time::backend::{MakeTime, TimeType};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chrono<T: TimeZone>(DateTime<T>);

impl<T> Copy for Chrono<T>
where
    T: TimeZone,
    T::Offset: Copy,
{
}

impl<T> Chrono<T>
where
    T: TimeZone,
{
    fn new(time: DateTime<T>) -> Self {
        Self(time)
    }
}

impl<T> TimeType for Chrono<T>
where
    T: TimeZone + fmt::Debug,
{
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn day(&self) -> u32 {
        self.0.day()
    }

    fn hour(&self) -> u32 {
        self.0.hour()
    }

    fn minute(&self) -> u32 {
        self.0.minute()
    }

    fn month(&self) -> u32 {
        self.0.month()
    }

    fn nanosecond(&self) -> u32 {
        self.0.nanosecond()
    }

    fn second(&self) -> u32 {
        self.0.second()
    }

    fn microsecond(&self) -> u32 {
        self.0.nanosecond() / 1_000
    }

    fn weekday(&self) -> u32 {
        self.0.weekday().num_days_from_sunday()
    }

    fn year_day(&self) -> u32 {
        self.0.ordinal()
    }

    fn year(&self) -> i32 {
        self.0.year()
    }

    fn to_float(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let sec = self.0.timestamp() as f64;
        let nanos_fractional = f64::from(self.0.timestamp_subsec_nanos()) / 1_000_000_000_f64;
        sec + nanos_fractional
    }

    fn to_int(&self) -> i64 {
        self.0.timestamp()
    }

    fn is_monday(&self) -> bool {
        self.0.weekday() == Weekday::Mon
    }

    fn is_tuesday(&self) -> bool {
        self.0.weekday() == Weekday::Tue
    }

    fn is_wednesday(&self) -> bool {
        self.0.weekday() == Weekday::Wed
    }

    fn is_thursday(&self) -> bool {
        self.0.weekday() == Weekday::Thu
    }

    fn is_friday(&self) -> bool {
        self.0.weekday() == Weekday::Fri
    }

    fn is_saturday(&self) -> bool {
        self.0.weekday() == Weekday::Sat
    }

    fn is_sunday(&self) -> bool {
        self.0.weekday() == Weekday::Sun
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Factory {
    _private: (),
}

impl Factory {
    /// Constructs a new, default `Factory` for the chrono Time backend.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl MakeTime for Factory {
    type Time = Chrono<Local>;

    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn now(&self) -> Self::Time {
        Chrono::new(Local::now())
    }
}
