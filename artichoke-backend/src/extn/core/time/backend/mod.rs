use std::any::Any;

use crate::Artichoke;

pub mod chrono;

/// Common API for [`Time`](crate::extn::core::time::Time) backends.
pub trait TimeType: Any {
    /// Returns the day of the month (1..n) for time.
    fn day(&self) -> u32;

    /// Returns the hour of the day (0..23) for time.
    fn hour(&self) -> u32;

    /// Returns the minute of the hour (0..59) for time.
    fn minute(&self) -> u32;

    /// Returns the month of the year (1..12) for time.
    fn month(&self) -> u32;

    /// Returns the number of nanoseconds for time.
    fn nanosecond(&self) -> u32;

    /// Returns the second of the minute (0..60) for time.
    ///
    /// *Note*: Seconds range from zero to 60 to allow the system to inject leap
    /// seconds. See <https://en.wikipedia.org/wiki/Leap_second> for further
    /// details.
    fn second(&self) -> u32;

    /// Returns the number of microseconds for time.
    fn microsecond(&self) -> u32;

    /// Returns an integer representing the day of the week, 0..6, with Sunday
    /// == 0.
    fn weekday(&self) -> u32;

    /// Returns an integer representing the day of the year, 1..366.
    fn year_day(&self) -> u32;

    /// Returns the year for time (including the century).
    fn year(&self) -> i32;

    /// Returns the value of time as a floating point number of seconds since
    /// the Epoch.
    fn to_float(&self) -> f64;

    /// Returns the value of time as an integer number of seconds since the
    /// Epoch.
    fn to_int(&self) -> i64;

    /// Returns `true` if time represents Monday.
    fn is_monday(&self) -> bool;

    /// Returns `true` if time represents Tuesday.
    fn is_tuesday(&self) -> bool;

    /// Returns `true` if time represents Wednesday.
    fn is_wednesday(&self) -> bool;

    /// Returns `true` if time represents Thursday.
    fn is_thursday(&self) -> bool;

    /// Returns `true` if time represents Friday.
    fn is_friday(&self) -> bool;

    /// Returns `true` if time represents Saturday.
    fn is_saturday(&self) -> bool;

    /// Returns `true` if time represents Sunday.
    fn is_sunday(&self) -> bool;
}

pub trait MakeTime: Any {
    fn now(&self, interp: &Artichoke) -> Box<dyn TimeType>;
}

#[allow(clippy::missing_safety_doc)]
mod internal {
    downcast!(dyn super::TimeType);
    downcast!(dyn super::MakeTime);
}
