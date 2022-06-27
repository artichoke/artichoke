use chrono::prelude::*;
use chrono_tz::Tz;

use crate::time::chrono::{Offset, Time};
use crate::{MICROS_IN_NANO, NANOS_IN_SECOND};

impl Time {
    /// Returns the hour of the day `0..=23` for _time_.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let hour_of_day = now.hour();
    /// ```
    #[inline]
    #[must_use]
    pub fn hour(self) -> u32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.hour()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.hour()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.hour()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.hour()
            }
        }
    }

    /// Returns the minute of the hour `0..=59` for _time_.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let minute_of_hour = now.minute();
    /// ```
    #[inline]
    #[must_use]
    pub fn minute(self) -> u32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.minute()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.minute()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.minute()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.minute()
            }
        }
    }

    /// Returns the second of the minute `0..=60` for _time_.
    ///
    /// Seconds range from zero to 60 to allow the system to inject [leap
    /// seconds].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let second_of_minute = now.second();
    /// if second_of_minute >= 60 {
    ///     // `now` is during a leap second
    /// }
    /// ```
    ///
    /// [leap seconds]: https://en.wikipedia.org/wiki/Leap_second
    #[inline]
    #[must_use]
    pub fn second(self) -> u32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let second = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.second()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.second()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.second()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.second()
            }
        };
        // `chrono` stores leap seconds in the `sub_second_nanos` field.
        // Normalize so nanos is `0..1_000_000_000`.
        if sub_second_nanos >= NANOS_IN_SECOND {
            second + 1
        } else {
            second
        }
    }

    /// Returns the number of microseconds for _time_.
    ///
    /// This method returns microseconds since the last second (including [leap
    /// seconds]. The range of this value is always from 0 to 999,999.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let usec_since_last_second = now.microseconds();
    /// if usec_since_last_second >= 1_000_000 {
    ///     // `now` is during a leap second
    /// }
    /// ```
    ///
    /// [leap seconds]: https://en.wikipedia.org/wiki/Leap_second
    #[inline]
    #[must_use]
    pub const fn microseconds(self) -> u32 {
        self.sub_second_nanos / MICROS_IN_NANO
    }

    /// Returns the number of nanoseconds for _time_.
    ///
    /// This method returns nanoseconds since the last second (including [leap
    /// seconds]. The range of this value is always from 0 to 999,999,999.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let nsec_since_last_second = now.nanoseconds();
    /// ```
    ///
    /// # Implementation notes
    ///
    /// The IEEE 754 double is not accurate enough to represent the exact number
    /// of nanoseconds since the Unix Epoch. [`nanoseconds`](Self::nanoseconds)
    /// is more accurate than [`to_float`](Self::to_float).
    ///
    /// [leap seconds]: https://en.wikipedia.org/wiki/Leap_second
    #[inline]
    #[must_use]
    pub fn nanoseconds(self) -> u32 {
        let Self { sub_second_nanos, .. } = self;
        // `chrono` stores leap seconds in the `sub_second_nanos` field.
        // Normalize so nanos is `0..1_000_000_000`.
        sub_second_nanos
            .checked_sub(NANOS_IN_SECOND)
            .unwrap_or(sub_second_nanos)
    }

    /// Returns the fraction for time.
    ///
    /// The return value can be a rational number. This result is more accurate
    /// than [`to_float`](Self::to_float) because IEEE 754 double precision is
    /// not sufficient to losslessly encode nanosecond fractions of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let time = Time::now();
    /// let (sub_second_units, units_per_second) = time.subsec();
    /// ```
    ///
    /// # Implementation notes
    ///
    /// Time has a maximum granularity of nanoseconds, so in practice this
    /// method always returns nanoseconds, but the returned tuple accommodates
    /// returning any fractional part, such as millis or micros.
    #[inline]
    #[must_use]
    pub const fn subsec(self) -> (u32, u32) {
        (self.sub_second_nanos, NANOS_IN_SECOND)
    }
}
