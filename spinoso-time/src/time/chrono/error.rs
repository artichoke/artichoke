use core::fmt;
use std::error::Error;

/// Error returned when constructing a [`Time`] from a [`ToA`].
///
/// This error is returned when a time component in the `ToA` exceeds the maximum
/// permissible value for a datetime. For example, invalid values include a
/// datetime 5000 days or 301 seconds.
///
/// # Examples
///
/// Invalid date component:
///
/// ```
/// # use spinoso_time::{Offset, Time, ToA, ComponentOutOfRangeError};
/// let to_a = ToA {
///     sec: 21,
///     min: 3,
///     hour: 23,
///     day: 5000,
///     month: 4,
///     year: 2020,
///     wday: 0,
///     yday: 96,
///     isdst: true,
///     zone: Offset::Local,
/// };
/// let time = Time::try_from(to_a);
/// assert_eq!(time, Err(ComponentOutOfRangeError::Date));
/// ```
///
/// Invalid time component:
///
/// ```
/// # use spinoso_time::{Offset, Time, ToA, ComponentOutOfRangeError};
/// let to_a = ToA {
///     sec: 301,
///     min: 3,
///     hour: 23,
///     day: 5,
///     month: 4,
///     year: 2020,
///     wday: 0,
///     yday: 96,
///     isdst: true,
///     zone: Offset::Local,
/// };
/// let time = Time::try_from(to_a);
/// assert_eq!(time, Err(ComponentOutOfRangeError::Time));
/// ```
///
/// [`Time`]: [`time::chrono::Time`]
/// [`ToA`]: [`time::chrono::ToA`]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComponentOutOfRangeError {
    /// Date component (year, month, day) out of range.
    Date,
    /// Time component (hour, minute, second) out of range.
    Time,
}

impl fmt::Display for ComponentOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date => f.write_str("Date component (year, month, day) out of range"),
            Self::Time => f.write_str("Time component (hour, minute, second) out of range"),
        }
    }
}

impl Error for ComponentOutOfRangeError {}
