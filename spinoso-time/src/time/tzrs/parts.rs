use super::Offset;
use super::Time;
use crate::MICROS_IN_NANO;

// Parts
impl Time {
    /// Returns the number of nanoseconds for _time_
    ///
    /// The lowest digits of `to_f` and nsec are different because IEEE 754 double is not accurate
    /// enough to represent the exact number of nanoseconds since the Epoch.
    ///
    /// Can be used to implement [`Time#nsec`] and [`Time#tv_nsec`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::{tzrs::Time, NANOS_IN_SECOND};
    /// let t = Time::utc(2022, 1, 1, 12, 0, 0, 1);
    /// let t_float = t.to_float();
    /// let float_nanos = (t_float - t_float.round()) * NANOS_IN_SECOND as f64;
    /// assert_ne!(float_nanos, 1f64);
    /// assert_eq!(t.nanoseconds(), 1);
    /// ```
    ///
    /// [`Time#nsec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-nsec
    /// [`Time#tv_nsec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-tv_nsec
    #[inline]
    #[must_use]
    pub fn nanoseconds(&self) -> u32 {
        self.inner.nanoseconds()
    }

    /// Returns the number of microseconds for _time_
    ///
    /// Can be used to implement [`Time#usec`] and [`Time#tv_usec`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::{tzrs::Time, MICROS_IN_NANO};
    /// let t = Time::utc(2022, 1, 1, 12, 0, 0, 1 * MICROS_IN_NANO);
    /// assert_eq!(t.microseconds(), 1);
    /// ```
    ///
    /// [`Time#usec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-usec
    /// [`Time#tv_usec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-tv_usec
    #[inline]
    #[must_use]
    pub fn microseconds(&self) -> u32 {
        self.inner.nanoseconds() / MICROS_IN_NANO
    }

    /// Returns the second of the minute (0..60) for _time_
    ///
    /// Seconds range from zero to 60 to allow the system to inject [leap seconds].
    ///
    /// Can be used to implement [`Time#sec`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let second_of_minute = now.second();
    /// assert_eq!(second_of_minute, 56);
    /// ```
    ///
    /// [`Time#sec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-sec
    /// [leap seconds]: https://en.wikipedia.org/wiki/Leap_second
    #[inline]
    #[must_use]
    pub fn second(&self) -> u8 {
        self.inner.second()
    }

    /// Returns the minute of the hour (0..59) for _time_
    ///
    /// Can be used to implement [`Time#min`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let minute_of_hour = now.minute();
    /// assert_eq!(minute_of_hour, 34);
    /// ```
    ///
    /// [`Time#minute`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-min
    #[inline]
    #[must_use]
    pub fn minute(&self) -> u8 {
        self.inner.minute()
    }

    /// Returns the hour of the day (0..23) for _time_
    ///
    /// Can be used to implement [`Time#min`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let hour_of_day = now.hour();
    /// assert_eq!(hour_of_day, 12);
    /// ```
    ///
    /// [`Time#hour`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-hour
    #[inline]
    #[must_use]
    pub fn hour(&self) -> u8 {
        self.inner.hour()
    }

    /// Returns the day of the month (1..n) for _time_
    ///
    /// Can be used to implement [`Time#day`] and [`Time#mday`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let day_of_month = now.day();
    /// assert_eq!(day_of_month, 8);
    /// ```
    ///
    /// [`Time#day`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-day
    /// [`Time#mday`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-mday
    #[inline]
    #[must_use]
    pub fn day(&self) -> u8 {
        self.inner.month_day()
    }

    /// Returns the month of the year (1..12) for _time_
    ///
    /// Can be used to implement [`Time#mon`] and [`Time#month`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let month_of_year = now.month();
    /// assert_eq!(month_of_year, 7);
    /// ```
    ///
    /// [`Time#mon`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-mon
    /// [`Time#month`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-mon
    #[inline]
    #[must_use]
    pub fn month(&self) -> u8 {
        self.inner.month()
    }

    /// Returns the year for _time_ (including the century)
    ///
    /// Can be used to implement [`Time#year`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.year(), 2022);
    /// ```
    ///
    /// [`Time#year`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-year
    #[inline]
    #[must_use]
    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    /// Returns the name of the time zone as a string
    ///
    /// Note: UTC is an empty string due to the [`UTC LocaleTimeType`] being constructed with None,
    /// which is later coerced into an [`empty string`]
    ///
    /// # Examples
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!("UTC", now_utc.time_zone());
    /// ```
    ///
    /// [`UTC LocalTimeType`]: https://docs.rs/tz-rs/0.6.9/src/tz/timezone/mod.rs.html#180
    /// [`empty string`]: https://docs.rs/tz-rs/0.6.9/src/tz/timezone/mod.rs.html#210
    #[inline]
    #[must_use]
    pub fn time_zone(&self) -> String {
        match self.offset {
            Offset::Utc => "UTC".to_string(),
            Offset::Fixed(_) => {
                let ut_offset = self.inner.local_time_type().ut_offset();
                let flag = if ut_offset < 0 { '-' } else { '+' };
                let minutes = ut_offset.abs() / 60;

                let offset_hours = minutes / 60;
                let offset_minutes = minutes - (offset_hours * 60);

                format!("{}{:0>2}:{:0>2}", flag, offset_hours, offset_minutes)
            }
            Offset::Tz(_) => self.inner.local_time_type().time_zone_designation().to_string(),
        }
        //self.inner.local_time_type().time_zone_designation().to_string()
    }

    /// Returns true if the time zone is UTC
    ///
    /// Can be used to implement [`Time#utc?`] and [`Time#gmt?`]
    ///
    //// # Examples
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert!(now_utc.is_utc());
    /// ```
    ///
    /// [`Time#utc?`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-utc-3F
    /// [`Time#gmt?`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-gmt-3F
    #[inline]
    #[must_use]
    pub fn is_utc(&self) -> bool {
        Offset::Utc == self.offset
    }

    /// Returns the offset in seconds between the timezone of _time_ and UTC
    ///
    /// Can be used to implement [`Time#utc_offset`] and [`Time#gmt_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.utc_offset(), 0);
    /// ```
    ///
    /// [`Time#utc_offset`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-utc_offset
    /// [`Time#gmt_offset`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-gmt_offset
    #[inline]
    #[must_use]
    pub fn utc_offset(&self) -> i32 {
        self.inner.local_time_type().ut_offset()
    }

    /// Returns `true` if _time_ occurs during Daylight Saving Time in its time zone.
    ///
    /// Can be used to implement [`Time#dst?`] and [`Time#isdst`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::{Time, Offset};
    /// use tzdb::time_zone::{europe::AMSTERDAM, pacific::AUCKLAND};
    /// let now_ams = Time::new(2022, 5, 18, 16, 0, 0, 0, Offset::from(AMSTERDAM));
    /// assert!(now_ams.is_dst());
    /// let now_auckland = Time::new(2022, 5, 18, 16, 0, 0, 0, Offset::from(AUCKLAND));
    /// assert!(!now_auckland.is_dst());
    /// ```
    ///
    /// [`Time#dst?`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-dst-3F
    /// [`Time#isdst`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-isdst
    #[inline]
    #[must_use]
    pub fn is_dst(&self) -> bool {
        self.inner.local_time_type().is_dst()
    }

    /// Returns an integer representing the day of the week, 0..6, with Sunday == 0
    ///
    /// Can be used to implement [`Time#wday`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.day_of_week(), 5);
    /// ```
    ///
    /// [`Time#wday`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-wday
    #[inline]
    #[must_use]
    pub fn day_of_week(&self) -> u8 {
        self.inner.week_day()
    }

    /// Returns an integer representing the day of the year, 1..366.
    ///
    /// Can be used to implement [`Time#yday`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.day_of_year(), 188);
    /// ```
    ///
    /// [`Time#yday`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-yday
    #[inline]
    #[must_use]
    pub fn day_of_year(&self) -> u16 {
        self.inner.year_day()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_parts() {
        let dt = Time::utc(2022, 7, 8, 12, 34, 56, 1910);
        assert_eq!(2022, dt.year());
        assert_eq!(7, dt.month());
        assert_eq!(8, dt.day());
        assert_eq!(12, dt.hour());
        assert_eq!(34, dt.minute());
        assert_eq!(56, dt.second());
        assert_eq!(1910, dt.nanoseconds());
        assert_eq!(1, dt.microseconds());
        assert!(dt.is_utc());
    }
}
