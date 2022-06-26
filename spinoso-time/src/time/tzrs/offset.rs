use std::str;

use once_cell::sync::Lazy;
use regex::Regex;
use tz::timezone::{LocalTimeType, TimeZoneRef};
#[cfg(feature = "tzrs-local")]
use tzdb::local_tz;
use tzdb::time_zone::etc::GMT;

const SECONDS_IN_MINUTE: i32 = 60;
const SECONDS_IN_HOUR: i32 = SECONDS_IN_MINUTE * 60;

/// tzdb provides [`local_tz`] to get the local system timezone. If this ever
/// fails, we can assume `GMT`. `GMT` is used instead of `UTC` since it has a
/// [`time_zone_designation`] - which if it is an empty string, then it is
/// considered to be a UTC time.
///
/// Note: this matches MRI Ruby implmentation. Where `TZ="" ruby -e "puts
/// Time::now"` will return a new _time_ with 0 offset from UTC, but still still
/// report as a non UTC time:
///
/// ```console
/// $ TZ="" ruby -e 'puts RUBY_VERSION' -e 't = Time.now' -e 'puts t' -e 'puts t.utc?'
/// 3.1.2
/// 2022-06-26 22:22:25 +0000
/// false
/// ```
///
/// [`local_tz`]: tzdb::local_tz
/// [`time_zone_designation`]: tz::timezone::LocalTimeType::time_zone_designation
#[inline]
#[must_use]
#[cfg(feature = "tzrs-local")]
fn local_time_zone() -> TimeZoneRef<'static> {
    match local_tz() {
        Some(tz) => tz,
        None => GMT,
    }
}

#[inline]
#[must_use]
#[cfg(not(feature = "tzrs-local"))]
fn local_time_zone() -> TimeZoneRef<'static> {
    GMT
}

/// Generates a [+/-]HHMM timezone format from a given number of seconds
/// Note: the actual seconds element is effectively ignored here
#[inline]
#[must_use]
fn offset_hhmm_from_seconds(seconds: i32) -> String {
    let flag = if seconds < 0 { '-' } else { '+' };
    let minutes = seconds.abs() / 60;

    let offset_hours = minutes / 60;
    let offset_minutes = minutes - (offset_hours * 60);

    format!("{}{:0>2}{:0>2}", flag, offset_hours, offset_minutes)
}

/// Represents the number of seconds offset from UTC.
#[allow(variant_size_differences)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Offset {
    /// UTC offset, zero offset, Zulu time.
    Utc,
    /// Fixed offset from UTC.
    ///
    /// **Note**: A fixed offset of 0 is different from UTC time.
    Fixed([LocalTimeType; 1]),
    /// A time zone based offset.
    Tz(TimeZoneRef<'static>),
}

impl<'a> Offset {
    /// Generate a UTC based offset.
    #[inline]
    #[must_use]
    pub fn utc() -> Self {
        Self::Utc
    }

    /// Generate an offset based on the detected local time zone of the system.
    ///
    /// Detection is done by [`tzdb::local_tz`], and if it fails will return a
    /// GMT timezone.
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
        let offset_name = offset_hhmm_from_seconds(offset);
        let local_time_type =
            LocalTimeType::new(offset, false, Some(offset_name.as_bytes())).expect("Couldn't create fixed offset");
        Self::Fixed([local_time_type])
    }

    /// Generate an offset based on a provided [`tz::timezone::TimeZoneRef`].
    ///
    /// This can be combined with [`tzdb`] to generate offsets based on
    /// predefined IANA time zones.
    ///
    /// ```
    /// use spinoso_time::tzrs::Offset;
    /// use tzdb::time_zone::pacific::AUCKLAND;
    /// let offset = Offset::tz(AUCKLAND);
    /// ```
    #[inline]
    #[must_use]
    pub fn tz(tz: TimeZoneRef<'static>) -> Self {
        Self::Tz(tz)
    }

    /// Returns a `TimeZoneRef` which can be used to generate and project
    /// _time_.
    #[inline]
    #[must_use]
    pub fn time_zone_ref(&self) -> TimeZoneRef<'_> {
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
    /// Construct a Offset based on the [accepted MRI values].
    ///
    /// Accepts:
    ///
    /// - `[+/-]HH[:]MM`
    /// - A-I representing +01:00 to +09:00.
    /// - K-M representing +10:00 to +12:00.
    /// - N-Y representing -01:00 to -12:00.
    /// - Z representing UTC/Zulu time (0 offset).
    ///
    /// [accepted MRI values]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-new
    #[inline]
    #[must_use]
    fn from(input: &str) -> Self {
        static HH_MM_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^([\-\+]{1})(\d{2}):?(\d{2})$").expect("Regex should be valid"));

        match input {
            "A" => Self::fixed(SECONDS_IN_HOUR),
            "B" => Self::fixed(2 * SECONDS_IN_HOUR),
            "C" => Self::fixed(3 * SECONDS_IN_HOUR),
            "D" => Self::fixed(4 * SECONDS_IN_HOUR),
            "E" => Self::fixed(5 * SECONDS_IN_HOUR),
            "F" => Self::fixed(6 * SECONDS_IN_HOUR),
            "G" => Self::fixed(7 * SECONDS_IN_HOUR),
            "H" => Self::fixed(8 * SECONDS_IN_HOUR),
            "I" => Self::fixed(9 * SECONDS_IN_HOUR),
            "K" => Self::fixed(10 * SECONDS_IN_HOUR),
            "L" => Self::fixed(11 * SECONDS_IN_HOUR),
            "M" => Self::fixed(12 * SECONDS_IN_HOUR),
            "N" => Self::fixed(-SECONDS_IN_HOUR),
            "O" => Self::fixed(-2 * SECONDS_IN_HOUR),
            "P" => Self::fixed(-3 * SECONDS_IN_HOUR),
            "Q" => Self::fixed(-4 * SECONDS_IN_HOUR),
            "R" => Self::fixed(-5 * SECONDS_IN_HOUR),
            "S" => Self::fixed(-6 * SECONDS_IN_HOUR),
            "T" => Self::fixed(-7 * SECONDS_IN_HOUR),
            "U" => Self::fixed(-8 * SECONDS_IN_HOUR),
            "V" => Self::fixed(-9 * SECONDS_IN_HOUR),
            "W" => Self::fixed(-10 * SECONDS_IN_HOUR),
            "X" => Self::fixed(-11 * SECONDS_IN_HOUR),
            "Y" => Self::fixed(-12 * SECONDS_IN_HOUR),
            // ```console
            // [3.1.2] > Time.new(2022, 6, 26, 13, 57, 6, 'Z')
            // => 2022-06-26 13:57:06 UTC
            // [3.1.2] > Time.new(2022, 6, 26, 13, 57, 6, 'Z').utc?
            // => true
            // [3.1.2] > Time.new(2022, 6, 26, 13, 57, 6, 'UTC')
            // => 2022-06-26 13:57:06 UTC
            // [3.1.2] > Time.new(2022, 6, 26, 13, 57, 6, 'UTC').utc?
            // => true
            // ```
            "Z" | "UTC" => Self::utc(),
            _ => {
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

impl From<&[u8]> for Offset {
    fn from(input: &[u8]) -> Self {
        let input = str::from_utf8(input).expect("Invalid UTF8");
        Offset::from(input)
    }
}

impl From<String> for Offset {
    fn from(input: String) -> Self {
        Offset::from(input.as_str())
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

    fn offset_name(offset: &Offset) -> &str {
        match offset {
            Offset::Utc => "UTC",
            Offset::Fixed(ltt) => ltt[0].time_zone_designation(),
            Offset::Tz(_) => "Ambiguous timezone name",
        }
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
    fn from_binary_string() {
        let tz: &[u8] = b"Z";
        let offset = Offset::from(tz);
        assert!(matches!(offset, Offset::Utc));
    }

    #[test]
    fn from_str_hh_mm() {
        assert_eq!(0, offset_seconds_from_fixed_offset("+0000"));
        assert_eq!(0, offset_seconds_from_fixed_offset("-0000"));
        assert_eq!(60, offset_seconds_from_fixed_offset("+0001"));
        assert_eq!(-60, offset_seconds_from_fixed_offset("-0001"));
        assert_eq!(3600, offset_seconds_from_fixed_offset("+0100"));
        assert_eq!(-3600, offset_seconds_from_fixed_offset("-0100"));
        assert_eq!(7320, offset_seconds_from_fixed_offset("+0202"));
        assert_eq!(-7320, offset_seconds_from_fixed_offset("-0202"));
        assert_eq!(362_340, offset_seconds_from_fixed_offset("+9999"));
        assert_eq!(-362_340, offset_seconds_from_fixed_offset("-9999"));
        assert_eq!(3660, offset_seconds_from_fixed_offset("+0061"));
    }

    #[test]
    fn from_str_hh_colon_mm() {
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
        assert_eq!(3660, offset_seconds_from_fixed_offset("+00:61"));
    }

    #[test]
    fn from_str_hh_mm_strange() {
        assert_eq!(3660, offset_seconds_from_fixed_offset("+0061"));
    }

    #[test]
    fn fixed_time_zone_designation() {
        assert_eq!("+0000", offset_name(&Offset::from(0)));
        assert_eq!("+0000", offset_name(&Offset::from(59)));
        assert_eq!("+0001", offset_name(&Offset::from(60)));
        assert_eq!("-0001", offset_name(&Offset::from(-60)));
        assert_eq!("+0100", offset_name(&Offset::from(3600)));
        assert_eq!("-0100", offset_name(&Offset::from(-3600)));
        assert_eq!("+0202", offset_name(&Offset::from(7320)));
        assert_eq!("-0202", offset_name(&Offset::from(-7320)));
        assert_eq!("+9959", offset_name(&Offset::from(359_940)));
        assert_eq!("-9959", offset_name(&Offset::from(-359_940)));

        // Unexpected cases
        assert_eq!("-0000", offset_name(&Offset::from(-59)));

        // FIXME: Should error instead
        assert_eq!("+10000", offset_name(&Offset::from(360_000)));
    }
}
