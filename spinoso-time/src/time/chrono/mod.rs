use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

use chrono::prelude::*;

use crate::NANOS_IN_SECOND;

mod build;
mod convert;
mod date;
mod error;
mod math;
mod offset;
mod ops;
mod ordinal;
mod time;
mod timezone;
mod weekday;

pub use error::ComponentOutOfRangeError;
pub use offset::Offset;

/// Implementation of Ruby [`Time`], a timezone-aware datetime, based on
/// [`chrono`].
///
/// `Time` is represented as:
///
/// - a 64-bit signed integer of seconds since January 1, 1970 UTC (a Unix
///   timestamp).
/// - an unsigned 32-bit integer of nanoseconds since the timestamp.
/// - An offset from UTC. See [`Offset`] for the types of supported offsets.
///
/// This data structure allows representing roughly 584 billion years. Unlike
/// MRI, there is no promotion to `Bignum` or `Rational`. The maximum
/// granularity of a `Time` object is nanoseconds.
///
/// `Time` objects are immutable. Date/time value manipulation always returns a
/// new `Time` object.
///
/// # Examples
///
/// ```
/// # use spinoso_time::Time;
/// // Create a Time to the current system clock with local offset
/// let time = Time::now();
/// assert!(!time.is_utc());
/// println!("{}", time.is_sunday());
/// ```
///
/// ```
/// # use spinoso_time::Time;
/// let time = Time::now();
/// let one_hour_ago: Time = time - (60 * 60);
/// assert_eq!(time.to_int() - 3600, one_hour_ago.to_int());
/// assert_eq!(time.nanosecond(), one_hour_ago.nanosecond());
/// ```
///
/// # Implementation notes
///
/// Time stores raw timestamps and only converts to `chrono` [`DateTime`] for
/// computation. [`chrono`] provides an aware datetime view over the raw
/// timestamp.
///
/// [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
#[derive(Debug, Clone, Copy)]
pub struct Time {
    /// The number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka
    /// "Unix timestamp").
    timestamp: i64,
    /// The number of nanoseconds since the last second boundary represented by
    /// `self.timestamp`.
    sub_second_nanos: u32,
    /// Timezone offset.
    offset: Offset,
}

impl Hash for Time {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64(self.timestamp);
        state.write_u32(self.sub_second_nanos);
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.timestamp == other.timestamp && self.sub_second_nanos == other.sub_second_nanos
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.sub_second_nanos.cmp(&other.sub_second_nanos),
        }
    }
}

impl Time {
    /// Returns the value of _time_ as a floating point number of seconds since
    /// the Unix Epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::Time;
    /// let now = Time::now();
    /// let now_f = now.to_float();
    /// let now_i = now.to_int();
    /// assert!(now_i as f64 <= now_f);
    /// ```
    ///
    /// # Implementation notes
    ///
    /// The IEEE 754 double is not accurate enough to represent the exact number
    /// of subsecond nanoseconds in the `Time`.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_float(self) -> f64 {
        // For most practical uses of time, this lossy cast does not lose any
        // precision. The 52-bit mantissa in an `f64` allows storing over 142
        // million years of timestamps.
        let sec = self.timestamp as f64;
        // Both of these lossy casts are guaranteed to not lose precision since
        // both operands are <= 1e10, which is representable losslessly by
        // `f64`.
        let nanos_fractional = self.sub_second_nanos as f64 / (NANOS_IN_SECOND as f64);
        sec + nanos_fractional
    }

    /// Returns the value of _time_ as an integer number of seconds since the
    /// Unix Epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::Time;
    /// let now = Time::now();
    /// let now_f = now.to_float();
    /// let now_i = now.to_int();
    /// assert!(now_i as f64 <= now_f);
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_int(self) -> i64 {
        self.timestamp
    }

    /// Serialize a `Time` into its components as a [`ToA`].
    ///
    /// `ToA` stores a `Time` as a ten-element struct of time components: [sec,
    /// min, hour, day, month, year, wday, yday, isdst, zone].
    ///
    /// The ordering of the properties is important for the Ruby [`Time#to_a`]
    /// API, and is accessible with the [`ToA::to_tuple`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::Time;
    /// let now = Time::now();
    /// let to_a = now.to_a();
    /// assert_eq!(to_a.sec, now.second());
    /// assert_eq!(to_a.wday, now.weekday());
    /// ```
    #[inline]
    #[must_use]
    pub fn to_a(self) -> ToA {
        self.into()
    }
}

/// Serialized representation of a timestamp using a ten-element array of
/// datetime components.
///
/// [sec, min, hour, day, month, year, wday, yday, isdst, zone]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ToA {
    /// The second of the minute `0..=59` for the source _time_.
    pub sec: u32,
    /// The minute of the hour `0..=59` for the source _time_.
    pub min: u32,
    /// The hour of the day `0..=23` for the source _time_.
    pub hour: u32,
    /// The day of the month `1..=n` for the source _time_.
    pub day: u32,
    /// The month of the year `1..=12` for the source _time_.
    pub month: u32,
    /// The year (including the century) for the source _time_.
    pub year: i32,
    /// An integer representing the day of the week, `0..=6`, with Sunday == 0
    /// for the source _time_.
    pub wday: u32,
    /// An integer representing the day of the year, `1..=366` for the source
    /// _time_.
    pub yday: u32,
    /// Whether the source _time_ occurs during Daylight Saving Time in its time
    /// zone.
    pub isdst: bool,
    /// The timezone used for the source _time_.
    // TODO: this should be `&'static str`. Then it can move out of the `chrono`
    // backend.
    pub zone: Offset,
}

impl ToA {
    /// `ToA` represents ten-element array of values for time:
    ///
    /// [sec, min, hour, day, month, year, wday, yday, isdst, zone]
    pub const ELEMENTS: usize = 10;

    /// A ten-element array of values for time:
    ///
    /// [sec, min, hour, day, month, year, wday, yday, isdst, zone]
    #[inline]
    #[must_use]
    pub fn to_tuple(self) -> (u32, u32, u32, u32, u32, i32, u32, u32, bool, Offset) {
        (
            self.sec, self.min, self.hour, self.day, self.month, self.year, self.wday, self.yday, self.isdst,
            self.zone,
        )
    }
}
impl From<Time> for ToA {
    #[inline]
    fn from(time: Time) -> Self {
        Self {
            sec: time.second(),
            min: time.minute(),
            hour: time.hour(),
            day: time.day(),
            month: time.month(),
            year: time.year(),
            wday: time.weekday(),
            yday: time.year_day(),
            isdst: time.is_dst(),
            zone: time.offset,
        }
    }
}

impl TryFrom<ToA> for Time {
    type Error = ComponentOutOfRangeError;

    #[inline]
    fn try_from(time: ToA) -> Result<Self, Self::Error> {
        let ToA {
            sec,
            min,
            hour,
            day,
            month,
            year,
            zone,
            ..
        } = time;

        let date = NaiveDate::from_ymd_opt(year, month, day).ok_or(ComponentOutOfRangeError::Date)?;
        let time = if sec == 60 {
            // Leap second - chrono stores the 60th second in the nanos.
            NaiveTime::from_hms_nano_opt(hour, min, 59, NANOS_IN_SECOND).ok_or(ComponentOutOfRangeError::Time)?
        } else {
            NaiveTime::from_hms_opt(hour, min, sec).ok_or(ComponentOutOfRangeError::Time)?
        };
        let naive = NaiveDateTime::new(date, time);

        let time = Self {
            timestamp: naive.timestamp(),
            sub_second_nanos: naive.nanosecond(),
            offset: zone,
        };
        Ok(time)
    }
}

#[cfg(test)]
mod leap_second {
    use chrono::prelude::*;

    use super::{Time, ToA, NANOS_IN_SECOND};

    #[test]
    fn from_datetime() {
        // halfway through a leap second
        let time = NaiveTime::from_hms_nano(23, 59, 59, 1_500_000_000);
        let date = NaiveDate::from_ymd(2016, 12, 31);
        let datetime = NaiveDateTime::new(date, time);
        let datetime = DateTime::<Utc>::from_utc(datetime, Utc);

        assert!(datetime.nanosecond() > NANOS_IN_SECOND);
        let time = Time::from(datetime);
        assert_eq!(time.second(), 60);
        assert_eq!(time.nanosecond(), 500_000_000);
        let ToA {
            sec,
            min,
            hour,
            day,
            month,
            year,
            ..
        } = time.to_a();
        assert_eq!(sec, 60);
        assert_eq!(min, 59);
        assert_eq!(hour, 23);
        assert_eq!(day, 31);
        assert_eq!(month, 12);
        assert_eq!(year, 2016);
    }
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    use chrono::prelude::*;
    use chrono_tz::Tz;

    use super::Time;

    fn date() -> NaiveDateTime {
        // Artichoke's birthday, 2019-04-07T01:30Z
        NaiveDateTime::from_timestamp(1_554_600_621, 0)
    }

    #[test]
    fn properties_utc() {
        let time = Time::from(DateTime::<Utc>::from_utc(date(), Utc));
        assert_eq!(time.year(), 2019);
        assert_eq!(time.month(), 4);
        assert_eq!(time.day(), 7);
        assert_eq!(time.hour(), 1);
        assert_eq!(time.minute(), 30);
        assert_eq!(time.second(), 21);
        assert_eq!(time.microsecond(), 0);
        assert_eq!(time.nanosecond(), 0);
        assert_eq!(time.weekday(), 0);
        assert_eq!(time.year_day(), 97);
        assert!(!time.is_dst());
        assert_eq!(time.timezone(), Some("UTC"));
    }

    #[test]
    fn properties_named_tz() {
        let time = Time::from(Tz::US__Pacific.from_utc_datetime(&date()));
        assert_eq!(time.year(), 2019);
        assert_eq!(time.month(), 4);
        assert_eq!(time.day(), 6);
        assert_eq!(time.hour(), 18);
        assert_eq!(time.minute(), 30);
        assert_eq!(time.second(), 21);
        assert_eq!(time.microsecond(), 0);
        assert_eq!(time.nanosecond(), 0);
        assert_eq!(time.weekday(), 6);
        assert_eq!(time.year_day(), 96);
        // TODO: Implement DST and timezone detection. This requires a new release of
        // `chrono-tz`.
        //
        // ```
        // assert!(time.is_dst());
        // assert_eq!(time.timezone(), Some("PDT"));
        // ```
    }

    #[test]
    fn properties_fixed() {
        let hour = 3600;
        let time = Time::from(DateTime::<FixedOffset>::from_utc(date(), FixedOffset::west(7 * hour)));
        assert_eq!(time.year(), 2019);
        assert_eq!(time.month(), 4);
        assert_eq!(time.day(), 6);
        assert_eq!(time.hour(), 18);
        assert_eq!(time.minute(), 30);
        assert_eq!(time.second(), 21);
        assert_eq!(time.microsecond(), 0);
        assert_eq!(time.nanosecond(), 0);
        assert_eq!(time.weekday(), 6);
        assert_eq!(time.year_day(), 96);
        // TODO: Implement DST and timezone detection. This requires a new release of
        // `chrono-tz`.
        //
        // ```
        // assert!(time.is_dst());
        // ```
        assert_eq!(time.timezone(), None);
    }

    #[test]
    #[should_panic]
    fn local_has_zone() {
        let time = Time::from(Local.from_local_datetime(&date()).unwrap());
        assert!(time.timezone().is_some());
    }

    #[test]
    fn hash_based_on_timestamp_and_nanos() {
        let utc = Time::from(DateTime::<Utc>::from_utc(date(), Utc));
        let mut hasher = DefaultHasher::new();
        utc.hash(&mut hasher);
        let utc_hash = hasher.finish();

        let zone = Time::from(Tz::US__Pacific.from_utc_datetime(&date()));
        let mut hasher = DefaultHasher::new();
        zone.hash(&mut hasher);
        let zone_hash = hasher.finish();

        assert_eq!(utc_hash, zone_hash);

        let before_utc: Time = utc - 1;
        let mut hasher = DefaultHasher::new();
        before_utc.hash(&mut hasher);
        let before_utc_hash = hasher.finish();

        let before_zone: Time = zone - 1;
        let mut hasher = DefaultHasher::new();
        before_zone.hash(&mut hasher);
        let before_zone_hash = hasher.finish();

        assert_ne!(utc_hash, before_utc_hash);
        assert_ne!(zone_hash, before_zone_hash);
        assert_eq!(before_utc_hash, before_zone_hash);
    }

    #[test]
    fn eq_based_on_timestamp_and_nanos() {
        let utc = Time::from(DateTime::<Utc>::from_utc(date(), Utc));
        let zone = Time::from(Tz::US__Pacific.from_utc_datetime(&date()));
        assert_eq!(utc, zone);
    }

    #[test]
    fn ord_based_on_timestamp_and_nanos() {
        let utc = Time::from(DateTime::<Utc>::from_utc(date(), Utc));
        let zone = Time::from(Tz::US__Pacific.from_utc_datetime(&date()));
        assert_eq!(utc.cmp(&utc), Ordering::Equal);
        assert_eq!(zone.cmp(&zone), Ordering::Equal);
        assert_eq!(utc.cmp(&zone), Ordering::Equal);

        let before_utc: Time = utc - 1;
        let before_zone: Time = zone - 1;
        assert_eq!(utc.cmp(&before_utc), Ordering::Greater);
        assert_eq!(before_utc.cmp(&utc), Ordering::Less);
        assert_eq!(zone.cmp(&before_zone), Ordering::Greater);
        assert_eq!(before_zone.cmp(&zone), Ordering::Less);

        assert_eq!(utc.cmp(&before_zone), Ordering::Greater);
        assert_eq!(before_zone.cmp(&utc), Ordering::Less);
        assert_eq!(zone.cmp(&before_utc), Ordering::Greater);
        assert_eq!(before_utc.cmp(&zone), Ordering::Less);
    }
}
