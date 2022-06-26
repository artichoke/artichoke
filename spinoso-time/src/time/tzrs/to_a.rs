use super::Time;

/// Serialized representation of a timestamp using a ten-element array of
/// datetime components.
///
/// [sec, min, hour, day, month, year, wday, yday, isdst, zone]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToA {
    /// The second of the minute `0..=59` for the source _time_.
    pub sec: u8,
    /// The minute of the hour `0..=59` for the source _time_.
    pub min: u8,
    /// The hour of the day `0..=23` for the source _time_.
    pub hour: u8,
    /// The day of the month `1..=n` for the source _time_.
    pub day: u8,
    /// The month of the year `1..=12` for the source _time_.
    pub month: u8,
    /// The year (including the century) for the source _time_.
    pub year: i32,
    /// An integer representing the day of the week, `0..=6`, with Sunday == 0
    /// for the source _time_.
    pub wday: u8,
    /// An integer representing the day of the year, `1..=366` for the source
    /// _time_.
    pub yday: u16,
    /// Whether the source _time_ occurs during Daylight Saving Time in its time
    /// zone.
    pub isdst: bool,
    /// The timezone used for the source _time_.
    pub zone: String,
}

impl From<Time> for ToA {
    #[inline]
    #[must_use]
    fn from(time: Time) -> Self {
        Self {
            sec: time.second(),
            min: time.minute(),
            hour: time.hour(),
            day: time.day(),
            month: time.month(),
            year: time.year(),
            wday: time.day_of_week(),
            yday: time.day_of_year(),
            isdst: time.is_dst(),
            zone: String::from(time.time_zone()),
        }
    }
}
