use chrono::prelude::*;
use chrono_tz::Tz;

use crate::time::chrono::{Offset, Time};

impl Time {
    /// Returns an integer representing the day of the year, `1..=366`.
    ///
    /// This method returns the date's ordinal day.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let ordinal = now.year_day();
    /// assert!(ordinal > 0);
    /// assert!(ordinal <= 366);
    /// ```
    #[inline]
    #[must_use]
    pub fn year_day(self) -> u32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.ordinal()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.ordinal()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.ordinal()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.ordinal()
            }
        }
    }
}
