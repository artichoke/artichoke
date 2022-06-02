use chrono::prelude::*;
use chrono_tz::Tz;

use crate::time::chrono::{Offset, Time};

impl Time {
    /// Returns an integer representing the day of the week, `0..=6`, with
    /// Sunday == 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// match now {
    ///     time if time.is_sunday() => assert_eq!(time.weekday(), 0),
    ///     time if time.is_monday() => assert_eq!(time.weekday(), 1),
    ///     time if time.is_tuesday() => assert_eq!(time.weekday(), 2),
    ///     time if time.is_wednesday() => assert_eq!(time.weekday(), 3),
    ///     time if time.is_thursday() => assert_eq!(time.weekday(), 4),
    ///     time if time.is_friday() => assert_eq!(time.weekday(), 5),
    ///     time if time.is_saturday() => assert_eq!(time.weekday(), 6),
    ///     _ => {}
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn weekday(self) -> u32 {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        weekday.num_days_from_sunday()
    }

    /// Returns true if _time_ represents Sunday.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// if now.is_sunday() {
    ///     // go grocery shopping
    ///     assert_eq!(now.weekday(), 0);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_sunday(self) -> bool {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        matches!(weekday, Weekday::Sun)
    }

    /// Returns true if _time_ represents Monday.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// if now.is_monday() {
    ///     // go to work
    ///     assert_eq!(now.weekday(), 1);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_monday(self) -> bool {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        matches!(weekday, Weekday::Mon)
    }

    /// Returns true if _time_ represents Tuesday.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// if now.is_tuesday() {
    ///     // go to the gym
    ///     assert_eq!(now.weekday(), 2);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_tuesday(self) -> bool {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        matches!(weekday, Weekday::Tue)
    }

    /// Returns true if _time_ represents Wednesday.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// if now.is_wednesday() {
    ///     // hump day!
    ///     assert_eq!(now.weekday(), 3);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_wednesday(self) -> bool {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        matches!(weekday, Weekday::Wed)
    }

    /// Returns true if _time_ represents Thursday.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// if now.is_thursday() {
    ///     // Chinese food delivery for dinner
    ///     assert_eq!(now.weekday(), 4);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_thursday(self) -> bool {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        matches!(weekday, Weekday::Thu)
    }

    /// Returns true if _time_ represents Friday.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// if now.is_friday() {
    ///     // TGIF
    ///     // Friday, Friday, gotta get down
    ///     assert_eq!(now.weekday(), 5);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_friday(self) -> bool {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        matches!(weekday, Weekday::Fri)
    }

    /// Returns true if _time_ represents Saturday.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// if now.is_saturday() {
    ///     // hike at the lake
    ///     assert_eq!(now.weekday(), 6);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_saturday(self) -> bool {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        let weekday = match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                aware.weekday()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                aware.weekday()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                aware.weekday()
            }
        };
        matches!(weekday, Weekday::Sat)
    }
}
