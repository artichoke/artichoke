use tz::datetime::DateTime;
use tz::timezone::{TimeZoneRef, LocalTimeType};
use tzdb::local_tz;
use tzdb::time_zone::etc::{GMT};

mod math;
mod to_a;

pub use to_a::ToA;

use crate::{NANOS_IN_SECOND, MICROS_IN_NANO};

const UTC: TimeZoneRef<'static> = TimeZoneRef::utc();
/// A wrapper around [`tz::datetime::Datetime`] which contains everything needed for date creation and
/// conversion to match the ruby spec. Seconds and Subseconds are stored independently as i64 and
/// u32 respectively, which gives enough granularity to meet the ruby [`Time`] spec.
///
/// [`tz::datetime::Datetime`]: https://docs.rs/tz-rs/0.6.9/tz/datetime/struct.DateTime.html
/// [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Time {
    inner: DateTime,
}

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

// constructors
impl Time {
    /// Returns a new Time from the given values in the provided TimeZone.
    ///
    /// Can be used to implment ruby [`Time#new`]
    ///
    /// Note: During DST transitions, a specific time can be ambiguous. This method will always pick the earliest date.
    ///
    /// # Examples
    /// ```
    /// use spinoso_time::Time;
    /// use tzdb::time_zone::pacific::AUCKLAND;
    /// let t = Time::new(2022, 9, 25, 1, 30, 0, 0, AUCKLAND);
    /// ```
    ///
    /// [`Time#new`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-new
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanoseconds: u32,
        tz: TimeZoneRef<'static>,
    ) -> Self {
        let found_date_times = DateTime::find(year, month, day, hour, minute, second, nanoseconds, tz).unwrap();
        let dt = found_date_times
            .unique()
            .expect("Could not find a matching DateTime for this timezone");
        Self { inner: dt }
    }

    /// Returns a Time based on the provided values in the local timezone
    ///
    /// Can be used to implement ruby [`Time#local`], [`Time#mktime`]
    ///
    /// [`Time#local`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-local
    /// [`Time#mktime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-mktime
    pub fn local(year: i32, month: u8, month_day: u8, hour: u8, minute: u8, second: u8, nanoseconds: u32) -> Self {
        let tz = local_time_zone();
        Time::new(year, month, month_day, hour, minute, second, nanoseconds, tz)
    }

    /// Returns a Time based on the provided values in UTC
    ///
    /// Can be used to implement ruby [`Time#utc`], [`Time#gm`]
    ///
    /// [`Time#utc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-utc
    /// [`Time#gm`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-gm
    pub fn utc(year: i32, month: u8, month_day: u8, hour: u8, minute: u8, second: u8, nanoseconds: u32) -> Self {
        Time::new(
            year,
            month,
            month_day,
            hour,
            minute,
            second,
            nanoseconds,
            TimeZoneRef::utc(),
        )
    }

    /// Returns a Time with the current time in the System Timezone
    ///
    /// Can be used to implement ruby [`Time#now`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let now = Time::now();
    /// ```
    ///
    /// [`Time#now`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-now
    pub fn now() -> Self {
        let tz = local_time_zone();
        let now = DateTime::now(tz).unwrap();
        Self { inner: now }
    }

    /// Returns a Time in the given timezone with the number of seconds and nano_seconds since the Epoch in the specified timezone
    ///
    /// Can be used to implement ruby [`Time#at`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// use tzdb::time_zone::UTC;
    /// let t = Time::with_timezone(0, 0, UTC);
    /// assert_eq!(t.to_int(), 0);
    /// ```
    ///
    /// [`Time#at`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-at
    pub fn with_timezone(seconds: i64, nano_seconds: u32, tz: TimeZoneRef<'static>) -> Self {
        Self {
            inner: DateTime::from_timespec(seconds, nano_seconds, tz).unwrap(),
        }
    }
}

// Time#[gm|local|mktime|utc]
impl From<ToA<'_>> for Time {
    fn from(_: ToA<'_>) -> Self {
        todo!()
    }
}

// Core
impl Time {
    /// Returns the number of seconds as a signed integer since the Epoch.
    ///
    /// This function can be used to implement the ruby methods [`Time#to_i`] and [`Time#tv_sec`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let t = Time::utc(1970, 1, 1, 0, 1, 0, 0);
    /// assert_eq!(t.to_int(), 60)
    /// ```
    ///
    /// [`Time#to_i`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-to_i
    /// [`Time#tv_sec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-tv_sec
    pub fn to_int(&self) -> i64 {
        self.inner.unix_time()
    }

    /// Returns the number of seconds since the Epoch with fractional nanos included at IEEE
    /// 754-2008 accuracy.
    ///
    /// This function can be used to implement the ruby method [`Time#to_f``]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let now = Time::utc(1970, 1, 1, 0, 1, 0, 1000);
    /// assert_eq!(now.to_float(), 60.000001)
    /// ```
    ///
    /// [`Time#to_f`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-to_f
    pub fn to_float(&self) -> f64 {
        let sec = self.to_int() as f64;
        let nanos_fractional = (self.inner.nanoseconds() as f64) / (NANOS_IN_SECOND as f64);
        sec + nanos_fractional
    }

    /// Returns the numerator and denominator for the number of nano seconds of the Time struct
    /// unsimplified.
    ///
    /// This can be used directly to implement [`Time#subsec`].
    ///
    /// This function can be used in combination with [`to_int`] to implement [`Time#to_r`].
    ///
    /// #Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let t = Time::utc(1970, 1, 1, 0, 0, 1, 1000);
    /// assert_eq!(t.subsec_fractional(), (1000, 1000000000));
    /// ```
    ///
    /// [`Time#subsec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-subsec
    /// [`to_int`]: struct.Time.html#method.to_int
    /// [`Time#to_r`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-to_r
    pub fn subsec_fractional(&self) -> (u32, u32) {
        (self.inner.nanoseconds(), NANOS_IN_SECOND)
    }
}

// Conversions
impl Time {
    // Time#[asctime|ctime]
    pub fn to_string(&self) -> String {
        todo!()
    }

    // Time#strftime
    // Time#[to_s|inspect] uses "%Y-%m-%d %H:%M:%S UTC
    pub fn strftime(&self, format: String) -> String {
        todo!()
    }

    // Time#to_a
    pub fn to_array(&self) -> ToA {
        todo!()
    }

    /// Returns a new Time object representing _time_ in the provided timezone
    ///
    /// Can be used to implement [`Time#getlocal`] with a provided timezone
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// use tz::timezone::TimeZoneRef;
    /// let utc_time_zone = TimeZoneRef::utc();
    /// let now = Time::now();
    /// let now_utc = now.to_timezone(utc_time_zone);
    /// assert_eq!(now_utc.utc_offset(), 0);
    /// ```
    pub fn to_timezone(&self, tz: TimeZoneRef<'static>) -> Self {
        Self::with_timezone(
            self.inner.unix_time(),
            self.inner.nanoseconds(),
            tz
        )
    }

    /// Returns a new _time_ in UTC
    ///
    /// Can be used to implement [`Time#getutc`] and [`Time#getgm`]
    ///
    /// #Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let now_local = Time::now();
    /// let now_utc = now_local.to_utc();
    /// assert_eq!(now_utc.utc_offset(), 0);
    /// ```
    ///
    /// [`Time#getutc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getutc
    /// [`Time#getgm`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getgm
    pub fn to_utc(&self) -> Self {
        self.to_timezone(UTC)
    }

    /// Returns a new Time object representing _time_ in local time (using the local time zone in
    /// effect for this process)
    ///
    /// Can be used to implement [`Time#getlocal`]
    ///
    /// #Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let local_offset = Time::now().utc_offset();
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let now_local = now_utc.to_local();
    /// assert_eq!(now_local.utc_offset(), local_offset);
    /// ```
    ///
    /// [`Time#getlocal`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getlocal
    pub fn to_local(&self) -> Self {
        let tz = local_time_zone();
        self.to_timezone(tz)
    }

    // TODO: Implement based on an Offset struct that takes both strings and i32s
    // Time#getlocal(offset)
    //pub fn to_offset(&self, offset: Offset) -> Self {
        //todo!()
        //
    //}
}

// Mutators
impl Time {
    /// Converts _time_ to the provided time zone, modifying the receiver
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(false)
    /// ```
    pub fn set_timezone(&mut self, tz: TimeZoneRef<'static>) {
        // TODO: ProjectionErrors from project() are propogated from `Time::from_timespec` which
        // generally come from an error on checked_add overflowing the seconds component of the
        // unix time. Need to decide how to handle these kinds of errors (e.g. panic?)
        match self.inner.project(tz) {
            Ok(time) => self.inner = time,
            Err(_) => (),
        }
    }

    /// Converts _time_ to local time (using the local time zone in effective at the creation time
    /// of _time_) modifying the receiver
    ///
    /// Can be used to implement [`Time#localtime`] without a parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let mut now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let now_utc_unix = now.to_int();
    /// now.set_local();
    /// let now_local_unix = now.to_int();
    /// assert_eq!(now_utc_unix, now_local_unix);
    /// ```
    ///
    /// [`Time#localtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-localtime
    pub fn set_local(&mut self) {
        let tz = local_time_zone();
        self.set_timezone(tz)
    }

    /// Converts _time_ to UTC (GMT), modifying the receiver
    ///
    /// Can be used to implement [`Time#utc`] and [`Time#gmtime`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let mut now = Time::local(2022, 7, 8, 12, 34, 56, 0);
    /// let now_local_unix = now.to_int();
    /// now.set_local();
    /// let now_utc_unix = now.to_int();
    /// assert_eq!(now_local_unix, now_utc_unix);
    /// ```
    ///
    /// [`Time#utc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-utc
    /// [`Time#gmtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-gmtime
    pub fn set_utc(&mut self) {
        self.set_timezone(UTC)
    }

    /// Converts _time_ to the GMT time zone with the provided offset
    ///
    /// Can be used to implement [`Time#localtime`] with an offset parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let mut now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert!(now.is_utc());
    /// now.set_offset_from_utc(3600);
    /// assert!(!now.is_utc());
    /// assert_eq!(now.utc_offset(), 3600);
    /// ```
    ///
    /// [`Time#localtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-localtime
    pub fn set_offset_from_utc(&mut self, offset: i32) {
        let local_time_type = LocalTimeType::new(offset, false, Some(b"GMT")).unwrap();
        self.inner = DateTime::from_timespec_and_local(
            self.to_int(),
            self.nanoseconds(),
            local_time_type
        ).unwrap()
    }

    pub fn round(&mut self, digits: u32) {
        todo!()
    }
}

// Parts
impl Time {
    /// Returns the number of nanoseconds for _time_
    ///
    /// The lowest digits of to_f and nsec are different because IEEE 754 double is not accurate
    /// enough to represent the exact number of nanoseconds since the Epoch.
    ///
    /// Can be used to implement [`Time#nsec`] and [`Time#tv_nsec`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::{Time, NANOS_IN_SECOND};
    /// let t = Time::utc(2022, 1, 1, 12, 0, 0, 1);
    /// let t_float = t.to_float();
    /// let float_nanos = (t_float - t_float.round()) * NANOS_IN_SECOND as f64;
    /// assert_ne!(float_nanos, 1f64);
    /// assert_eq!(t.nanoseconds(), 1);
    /// ```
    ///
    /// [`Time#nsec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-nsec
    /// [`Time#tv_nsec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-tv_nsec
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
    /// use spinoso_time::{Time, MICROS_IN_NANO};
    /// let t = Time::utc(2022, 1, 1, 12, 0, 0, 1 * MICROS_IN_NANO);
    /// assert_eq!(t.microseconds(), 1);
    /// ```
    ///
    /// [`Time#usec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-usec
    /// [`Time#tv_usec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-tv_usec
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let second_of_minute = now.second();
    /// assert_eq!(second_of_minute, 56);
    /// ```
    ///
    /// [`Time#sec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-sec
    /// [leap seconds]: https://en.wikipedia.org/wiki/Leap_second
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let minute_of_hour = now.minute();
    /// assert_eq!(minute_of_hour, 34);
    /// ```
    ///
    /// [`Time#minute`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-min
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let hour_of_day = now.hour();
    /// assert_eq!(hour_of_day, 12);
    /// ```
    ///
    /// [`Time#hour`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-hour
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let day_of_month = now.day();
    /// assert_eq!(day_of_month, 8);
    /// ```
    ///
    /// [`Time#day`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-day
    /// [`Time#mday`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-mday
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let month_of_year = now.month();
    /// assert_eq!(month_of_year, 7);
    /// ```
    ///
    /// [`Time#mon`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-mon
    /// [`Time#month`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-mon
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.year(), 2022);
    /// ```
    ///
    /// [`Time#year`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-year
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
    /// use spinoso_time::Time;
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now_utc.time_zone(), "");
    /// ```
    ///
    /// [`UTC LocalTimeType`]: https://docs.rs/tz-rs/0.6.9/src/tz/timezone/mod.rs.html#180
    /// [`empty string`]: https://docs.rs/tz-rs/0.6.9/src/tz/timezone/mod.rs.html#210
    pub fn time_zone(&self) -> &str {
        self.inner.local_time_type().time_zone_designation()
    }

    /// Returns true if the time zone is UTC
    ///
    /// Can be used to implement [`Time#utc?`] and [`Time#gmt?`]
    ///
    //// # Examples
    /// ```
    /// use spinoso_time::Time;
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert!(now_utc.is_utc());
    /// ```
    ///
    /// [`Time#utc?`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-utc-3F
    /// [`Time#gmt?`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-gmt-3F
    pub fn is_utc(&self) -> bool {
        self.time_zone() == ""
    }

    /// Returns the offset in seconds between the timezone of _time_ and UTC
    ///
    /// Can be used to implement [`Time#utc_offset`] and [`Time#gmt_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.utc_offset(), 0);
    /// ```
    ///
    /// [`Time#utc_offset`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-utc_offset
    /// [`Time#gmt_offset`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-gmt_offset
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
    /// use spinoso_time::Time;
    /// use tzdb::time_zone::{europe::AMSTERDAM, pacific::AUCKLAND};
    /// let now_ams = Time::new(2022, 5, 18, 16, 0, 0, 0, AMSTERDAM);
    /// assert!(now_ams.is_dst());
    /// let now_auckland = Time::new(2022, 5, 18, 16, 0, 0, 0, AUCKLAND);
    /// assert!(!now_auckland.is_dst());
    /// ```
    ///
    /// [`Time#dst?`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-dst-3F
    /// [`Time#isdst`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-isdst
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.day_of_week(), 5);
    /// ```
    ///
    /// [`Time#wday`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-wday
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
    /// use spinoso_time::Time;
    /// let now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert_eq!(now.day_of_year(), 188);
    /// ```
    ///
    /// [`Time#yday`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-yday
    pub fn day_of_year(&self) -> u16 {
        self.inner.year_day()
    }
}
