use regex::Regex;
use tz::timezone::{LocalTimeType, TimeZoneRef};
use tzdb::local_tz;
use tzdb::time_zone::etc::GMT;

const SECONDS_IN_MINUTE: i32 = 60;
const SECONDS_IN_HOUR: i32 = SECONDS_IN_MINUTE * 60;

/// tzdb provides [`local_tz`] to get the local system timezone. If this ever fails, we can
/// assume `GMT`. `GMT` is used instead of `UTC` since it has a [`time_zone_designation`] - which
/// if it is an empty string, then it is considered to be a UTC time.
///
/// Note: this matches MRI Ruby implmentation. Where `TZ="" ruby -e "puts Time::now"` will return a
/// new _time_ with 0 offset from UTC, but still still report as a non utc time.
///
/// [`local_tz`]: https://docs.rs/tzdb/latest/tzdb/fn.local_tz.html
/// [`time_zone_designation`]: https://docs.rs/tz-rs/0.6.9/tz/timezone/struct.LocalTimeType.html#method.time_zone_designation
#[inline]
#[must_use]
fn local_time_zone() -> TimeZoneRef<'static> {
    match local_tz() {
        Some(tz) => tz,
        None => GMT,
    }
}

/// Represents the number of seconds offset from UTC
#[allow(variant_size_differences)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Offset {
    /// UTC offset, zero offset, Zulu time
    Utc,
    /// Fixed offset from UTC
    ///
    /// Note: A fixed offset of 0 is different from UTC time
    Fixed([LocalTimeType; 1]),
    /// A time zone based offset
    Tz(TimeZoneRef<'static>),
}

impl<'a> Offset {
    /// Generate a UTC based offset
    #[inline]
    #[must_use]
    pub fn utc() -> Self {
        Self::Utc
    }

    /// Generate an offset based on the detected local time zone of the system
    ///
    /// Detection is done by [`tzdb::local_tz`], and if it fails will return a GMT timezone
    ///
    /// [`tzdb::local_tz`]: https://docs.rs/tzdb/latest/tzdb/fn.local_tz.html
    #[inline]
    #[must_use]
    pub fn local() -> Self {
        Self::Tz(local_time_zone())
    }

    /// Generate an offset with a number of seconds from UTC.
    #[inline]
    #[must_use]
    pub fn fixed(offset: i32) -> Self {
        let sign = if offset.is_negative() { '-' } else { '+' };
        let hours = offset / 3600;
        let minutes = (offset / 60) % 60;
        let offset_name = format!("GMT {}{}{}", sign, hours, minutes);
        let local_time_type =
            LocalTimeType::new(offset, false, Some(offset_name.as_bytes())).expect("Couldn't create fixed offset");
        Self::Fixed([local_time_type])
    }

    /// Generate an offset based on a provided [`tz::timezone::TimeZoneRef`]
    ///
    /// This can be combined with [`tzdb`] to generate offsets based on predefined iana time zones
    ///
    /// ```
    /// use spinoso_time::tzrs::Offset;
    /// use tzdb::time_zone::pacific::AUCKLAND;
    /// let offset = Offset::tz(AUCKLAND);
    /// ```
    ///
    /// [`tz:timezone::TimeZoneRef`]: https://docs.rs/tz-rs/0.6.9/tz/timezone/struct.TimeZoneRef.html
    /// [`tzdb`]: https://docs.rs/tzdb/latest/tzdb/index.html
    #[inline]
    #[must_use]
    pub fn tz(tz: TimeZoneRef<'static>) -> Self {
        Self::Tz(tz)
    }

    /// Returns a `TimeZoneRef` which can be used to generate and project _time_.
    #[inline]
    #[must_use]
    pub fn time_zone_ref(&'a self) -> TimeZoneRef<'a> {
        match self {
            Self::Utc => TimeZoneRef::utc(),
            Self::Fixed(local_time_types) => match TimeZoneRef::new(&[], local_time_types, &[], &None) {
                Ok(tz) => tz,
                Err(_) => GMT,
            },

            Self::Tz(zone) => *zone,
        }
    }
}

impl From<&str> for Offset {
    /// Construct a Offset based on the [accepted MRI values]
    ///
    /// Accepts:
    ///
    /// - `[+/-]HH[:]MM`
    /// - A-I representing +01:00 to +09:00
    /// - K-M representing +10:00 to +12:00
    /// - N-Y representing -01:00 to -12:00
    /// - Z representing 0 offset
    ///
    /// [accepted MRI values]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-new
    #[inline]
    #[must_use]
    fn from(input: &str) -> Self {
        match input {
            "A" => Self::fixed(1),
            "B" => Self::fixed(2),
            "C" => Self::fixed(3),
            "D" => Self::fixed(4),
            "E" => Self::fixed(5),
            "F" => Self::fixed(6),
            "G" => Self::fixed(7),
            "H" => Self::fixed(8),
            "I" => Self::fixed(9),
            "K" => Self::fixed(10),
            "L" => Self::fixed(11),
            "M" => Self::fixed(12),
            "N" => Self::fixed(-1),
            "O" => Self::fixed(-2),
            "P" => Self::fixed(-3),
            "Q" => Self::fixed(-4),
            "R" => Self::fixed(-5),
            "S" => Self::fixed(-6),
            "T" => Self::fixed(-7),
            "U" => Self::fixed(-8),
            "V" => Self::fixed(-9),
            "W" => Self::fixed(-10),
            "X" => Self::fixed(-11),
            "Y" => Self::fixed(-12),
            "Z" => Self::utc(),
            _ => {
                lazy_static! {
                    static ref HH_MM_MATCHER: Regex = Regex::new(r"^([\-\+]{1})(\d{2}):(\d{2})$").unwrap();
                }
                if HH_MM_MATCHER.is_match(input) {
                    let caps = HH_MM_MATCHER.captures(input).unwrap();

                    let sign = if caps.get(1).unwrap().as_str() == "+" { 1 } else { -1 };
                    let hours = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let minutes = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();

                    let offset_seconds: i32 = sign * ((hours * SECONDS_IN_HOUR) + (minutes * SECONDS_IN_MINUTE));
                    Self::fixed(offset_seconds)
                } else {
                    // TODO: ArgumentError
                    Self::utc()
                }
            }
        }
    }
}

impl From<TimeZoneRef<'static>> for Offset {
    #[inline]
    #[must_use]
    fn from(tz: TimeZoneRef<'static>) -> Self {
        Self::tz(tz)
    }
}

impl From<i32> for Offset {
    /// Construct a Offset with the offset in seconds from UTC
    #[inline]
    #[must_use]
    fn from(seconds: i32) -> Self {
        Self::fixed(seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn offset_seconds_from_fixed_offset(input: &str) -> i32 {
        let offset = Offset::from(input);
        let local_time_type = offset.time_zone_ref().local_time_types()[0];
        local_time_type.ut_offset()
    }

    #[test]
    fn fixed_zero_is_not_utc() {
        let offset = Offset::from(0);
        assert!(matches!(offset, Offset::Fixed(_)));
    }

    #[test]
    fn z_is_utc() {
        let offset = Offset::from("Z");
        assert!(matches!(offset, Offset::Utc));
    }

    #[test]
    fn from_str_hh_mm() {
        assert_eq!(0, offset_seconds_from_fixed_offset("+00:00"));
        assert_eq!(0, offset_seconds_from_fixed_offset("-00:00"));
        assert_eq!(60, offset_seconds_from_fixed_offset("+00:01"));
        assert_eq!(-60, offset_seconds_from_fixed_offset("-00:01"));
        assert_eq!(3600, offset_seconds_from_fixed_offset("+01:00"));
        assert_eq!(-3600, offset_seconds_from_fixed_offset("-01:00"));
        assert_eq!(7320, offset_seconds_from_fixed_offset("+02:02"));
        assert_eq!(-7320, offset_seconds_from_fixed_offset("-02:02"));
        assert_eq!(362_340, offset_seconds_from_fixed_offset("+99:99"));
        assert_eq!(-362_340, offset_seconds_from_fixed_offset("-99:99"));
    }
}
