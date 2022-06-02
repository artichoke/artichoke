use chrono::prelude::*;
use chrono_tz::Tz;

use crate::time::chrono::{Offset, Time};

impl Time {
    /// Returns the year for _time_ (including the century).
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let year = now.year();
    /// ```
    #[inline]
    #[must_use]
    pub fn year(self) -> i32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.year()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.year()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.year()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.year()
            }
        }
    }

    /// Returns the month of the year `1..=12` for _time_.
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
    pub fn month(self) -> u32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.month()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.month()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.month()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.month()
            }
        }
    }

    /// Returns the day of the month `1..=n` for _time_.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let day_of_month = now.day();
    /// ```
    #[inline]
    #[must_use]
    pub fn day(self) -> u32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.day()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.day()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.day()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.day()
            }
        }
    }
}
