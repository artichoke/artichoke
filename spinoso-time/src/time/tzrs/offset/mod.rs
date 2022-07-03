use std::slice;
use std::str;

use once_cell::sync::Lazy;
use regex::Regex;
use tz::timezone::{LocalTimeType, TimeZoneRef};
#[cfg(feature = "tzrs-local")]
use tzdb::local_tz;
use tzdb::time_zone::etc::GMT;

mod error;

pub use error::{OffsetError, OutOfRangeError, TzStringError};

const SECONDS_IN_MINUTE: i32 = 60;
const SECONDS_IN_HOUR: i32 = SECONDS_IN_MINUTE * 60;
/// The maximum allowed offset in seconds from UTC in the future for a fixed
/// offset. This is equal to the number of seconds in 1 day, minus 1
pub const MAX_OFFSET_SECONDS: i32 = 24 * 60 * 60 - 1;
/// The maximum allowed offset in seconds from UTC in the past for a fixed
/// offset. This is equal to the number of seconds in 1 day, minus 1
pub const MIN_OFFSET_SECONDS: i32 = -MAX_OFFSET_SECONDS;

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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    inner: OffsetType,
}

/// Represents the type of offset from UTC.
#[allow(variant_size_differences)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OffsetType {
    /// UTC offset, zero offset, Zulu time
    Utc,
    /// Fixed offset from UTC.
    ///
    /// **Note**: A fixed offset of 0 is different from UTC time.
    Fixed(LocalTimeType),
    /// A time zone based offset.
    Tz(TimeZoneRef<'static>),
}

impl Offset {
    /// Generate a UTC based offset.
    #[inline]
    #[must_use]
    pub fn utc() -> Self {
        Self { inner: OffsetType::Utc }
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
        Self {
            inner: OffsetType::Tz(local_time_zone()),
        }
    }

    /// Generate an offset with a number of seconds from UTC.
    ///
    /// # Errors
    ///
    /// Return a [`OffsetError::OutOfRangeError`] when outside of range of
    /// acceptable offset of [`MIN_OFFSET_SECONDS`] to [`MAX_OFFSET_SECONDS`]
    #[inline]
    pub fn fixed(offset: i32) -> Result<Self, OffsetError> {
        if !(MIN_OFFSET_SECONDS..=MAX_OFFSET_SECONDS).contains(&offset) {
            return Err(OutOfRangeError.into());
        }

        let offset_name = offset_hhmm_from_seconds(offset);
        // creation of the LocalTimeType is never expected to fail, since the
        // bounds we are more restrictive of the values than the struct itself.
        let local_time_type = LocalTimeType::new(offset, false, Some(offset_name.as_bytes()))
            .expect("Failed to LocalTimeType for fixed offset");

        Ok(Self {
            inner: OffsetType::Fixed(local_time_type),
        })
    }

    /// Generate an offset based on a provided [`tz::timezone::TimeZoneRef`].
    ///
    /// This can be combined with [`tzdb`] to generate offsets based on
    /// predefined IANA time zones.
    #[inline]
    #[must_use]
    fn tz(tz: TimeZoneRef<'static>) -> Self {
        Self {
            inner: OffsetType::Tz(tz),
        }
    }

    /// Returns whether this offset is UTC.
    #[inline]
    #[must_use]
    pub fn is_utc(&self) -> bool {
        matches!(self.inner, OffsetType::Utc)
    }

    /// Returns a `TimeZoneRef` which can be used to generate and project
    /// _time_.
    #[inline]
    #[must_use]
    pub(crate) fn time_zone_ref(&self) -> TimeZoneRef<'_> {
        match self.inner {
            OffsetType::Utc => TimeZoneRef::utc(),
            OffsetType::Fixed(ref local_time_type) => {
                match TimeZoneRef::new(&[], slice::from_ref(local_time_type), &[], &None) {
                    Ok(tz) => tz,
                    Err(_) => GMT,
                }
            }

            OffsetType::Tz(zone) => zone,
        }
    }
}

impl TryFrom<&str> for Offset {
    type Error = OffsetError;

    /// Construct a Offset based on the [accepted MRI values]
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
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "A" => Ok(Self::fixed(SECONDS_IN_HOUR)?),
            "B" => Ok(Self::fixed(2 * SECONDS_IN_HOUR)?),
            "C" => Ok(Self::fixed(3 * SECONDS_IN_HOUR)?),
            "D" => Ok(Self::fixed(4 * SECONDS_IN_HOUR)?),
            "E" => Ok(Self::fixed(5 * SECONDS_IN_HOUR)?),
            "F" => Ok(Self::fixed(6 * SECONDS_IN_HOUR)?),
            "G" => Ok(Self::fixed(7 * SECONDS_IN_HOUR)?),
            "H" => Ok(Self::fixed(8 * SECONDS_IN_HOUR)?),
            "I" => Ok(Self::fixed(9 * SECONDS_IN_HOUR)?),
            "K" => Ok(Self::fixed(10 * SECONDS_IN_HOUR)?),
            "L" => Ok(Self::fixed(11 * SECONDS_IN_HOUR)?),
            "M" => Ok(Self::fixed(12 * SECONDS_IN_HOUR)?),
            "N" => Ok(Self::fixed(-SECONDS_IN_HOUR)?),
            "O" => Ok(Self::fixed(-2 * SECONDS_IN_HOUR)?),
            "P" => Ok(Self::fixed(-3 * SECONDS_IN_HOUR)?),
            "Q" => Ok(Self::fixed(-4 * SECONDS_IN_HOUR)?),
            "R" => Ok(Self::fixed(-5 * SECONDS_IN_HOUR)?),
            "S" => Ok(Self::fixed(-6 * SECONDS_IN_HOUR)?),
            "T" => Ok(Self::fixed(-7 * SECONDS_IN_HOUR)?),
            "U" => Ok(Self::fixed(-8 * SECONDS_IN_HOUR)?),
            "V" => Ok(Self::fixed(-9 * SECONDS_IN_HOUR)?),
            "W" => Ok(Self::fixed(-10 * SECONDS_IN_HOUR)?),
            "X" => Ok(Self::fixed(-11 * SECONDS_IN_HOUR)?),
            "Y" => Ok(Self::fixed(-12 * SECONDS_IN_HOUR)?),
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
            "Z" | "UTC" => Ok(Self::utc()),
            _ => {
                static HH_MM_MATCHER: Lazy<Regex> =
                    Lazy::new(|| Regex::new(r"^([\-\+]{1})(\d{2}):?(\d{2})$").unwrap());
                if HH_MM_MATCHER.is_match(input) {
                    let caps = HH_MM_MATCHER.captures(input).unwrap();

                    let sign = if caps.get(1).unwrap().as_str() == "+" { 1 } else { -1 };
                    let hours = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let minutes = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();

                    if hours > 23 || minutes > 59 {
                        return Err(OutOfRangeError.into());
                    }

                    let offset_seconds: i32 = sign * ((hours * SECONDS_IN_HOUR) + (minutes * SECONDS_IN_MINUTE));
                    Ok(Self::fixed(offset_seconds)?)
                } else {
                    Err(TzStringError.into())
                }
            }
        }
    }
}

impl TryFrom<&[u8]> for Offset {
    type Error = OffsetError;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let input = str::from_utf8(input)?;
        Offset::try_from(input)
    }
}

impl TryFrom<String> for Offset {
    type Error = OffsetError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Offset::try_from(input.as_str())
    }
}

impl From<TimeZoneRef<'static>> for Offset {
    #[inline]
    #[must_use]
    fn from(tz: TimeZoneRef<'static>) -> Self {
        Self::tz(tz)
    }
}

impl TryFrom<i32> for Offset {
    type Error = OffsetError;
    /// Construct a Offset with the offset in seconds from UTC
    #[inline]
    fn try_from(seconds: i32) -> Result<Self, Self::Error> {
        Self::fixed(seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::error::{OffsetError, OutOfRangeError, TzStringError};
    use super::*;

    fn offset_seconds_from_fixed_offset(input: &str) -> Result<i32, OffsetError> {
        let offset = Offset::try_from(input)?;
        let local_time_type = offset.time_zone_ref().local_time_types()[0];
        Ok(local_time_type.ut_offset())
    }

    fn fixed_offset_name(offset_seconds: i32) -> Result<String, OffsetError> {
        let offset = Offset::fixed(offset_seconds)?;

        match offset.inner {
            OffsetType::Fixed(ref local_time_type) => Ok(local_time_type.time_zone_designation().to_string()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn fixed_zero_is_not_utc() {
        let offset = Offset::try_from(0).unwrap();
        assert!(!offset.is_utc());
    }

    #[test]
    fn utc_is_utc() {
        let offset = Offset::utc();
        assert!(offset.is_utc());
    }

    #[test]
    fn z_is_utc() {
        let offset = Offset::try_from("Z").unwrap();
        assert!(offset.is_utc());
    }

    #[test]
    fn from_binary_string() {
        let tz: &[u8] = b"Z";
        let offset = Offset::try_from(tz).unwrap();
        assert!(offset.is_utc());
    }

    #[test]
    fn from_str_hh_mm() {
        assert_eq!(Some(0), offset_seconds_from_fixed_offset("+0000").ok());
        assert_eq!(Some(0), offset_seconds_from_fixed_offset("-0000").ok());
        assert_eq!(Some(60), offset_seconds_from_fixed_offset("+0001").ok());
        assert_eq!(Some(-60), offset_seconds_from_fixed_offset("-0001").ok());
        assert_eq!(Some(3600), offset_seconds_from_fixed_offset("+0100").ok());
        assert_eq!(Some(-3600), offset_seconds_from_fixed_offset("-0100").ok());
        assert_eq!(Some(7320), offset_seconds_from_fixed_offset("+0202").ok());
        assert_eq!(Some(-7320), offset_seconds_from_fixed_offset("-0202").ok());

        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("+2400")
        );
        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("-2400")
        );
        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("+0060")
        );
        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("-0060")
        );
    }

    #[test]
    fn from_str_hh_colon_mm() {
        assert_eq!(Some(0), offset_seconds_from_fixed_offset("+00:00").ok());
        assert_eq!(Some(0), offset_seconds_from_fixed_offset("-00:00").ok());
        assert_eq!(Some(60), offset_seconds_from_fixed_offset("+00:01").ok());
        assert_eq!(Some(-60), offset_seconds_from_fixed_offset("-00:01").ok());
        assert_eq!(Some(3600), offset_seconds_from_fixed_offset("+01:00").ok());
        assert_eq!(Some(-3600), offset_seconds_from_fixed_offset("-01:00").ok());
        assert_eq!(Some(7320), offset_seconds_from_fixed_offset("+02:02").ok());
        assert_eq!(Some(-7320), offset_seconds_from_fixed_offset("-02:02").ok());

        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("+2400")
        );
        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("-2400")
        );
        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("+0060")
        );
        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            offset_seconds_from_fixed_offset("-0060")
        );
    }

    #[test]
    fn from_str_invalid_fixed_strings() {
        let invalid_fixed_strings = [
            "+01:010", "+010:10", "+010:010", "0110", "01:10", "01-10", "+01-10", "+01::10",
        ];

        for invalid_string in invalid_fixed_strings {
            assert_eq!(
                Err(OffsetError::TzStringError(TzStringError)),
                Offset::try_from(invalid_string),
                "Expected Offset::TzStringError for {}",
                invalid_string,
            );
        }
    }

    #[test]
    fn fixed_time_zone_designation() {
        assert_eq!("+0000", fixed_offset_name(0).unwrap());
        assert_eq!("+0000", fixed_offset_name(59).unwrap());
        assert_eq!("+0001", fixed_offset_name(60).unwrap());
        assert_eq!("-0001", fixed_offset_name(-60).unwrap());
        assert_eq!("+0100", fixed_offset_name(3600).unwrap());
        assert_eq!("-0100", fixed_offset_name(-3600).unwrap());
        assert_eq!("+0202", fixed_offset_name(7320).unwrap());
        assert_eq!("-0202", fixed_offset_name(-7320).unwrap());

        assert_eq!("+2359", fixed_offset_name(MAX_OFFSET_SECONDS).unwrap());
        assert_eq!("-2359", fixed_offset_name(MIN_OFFSET_SECONDS).unwrap());

        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            fixed_offset_name(MAX_OFFSET_SECONDS + 1)
        );

        assert_eq!(
            Err(OffsetError::OutOfRangeError(OutOfRangeError)),
            fixed_offset_name(MIN_OFFSET_SECONDS - 1)
        );
    }
}
