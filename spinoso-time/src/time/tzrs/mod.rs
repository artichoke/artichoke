use tz::datetime::DateTime;
use tz::timezone::{TimeZone, TimeZoneRef};
use tzdb::local_tz;

mod math;
mod to_a;

pub use to_a::ToA;

use crate::NANOS_IN_SECOND;

const UTC: TimeZoneRef<'static> = TimeZoneRef::utc();
/// A wrapper around tz_rs::Datetime which contains everything needed for date creation and
/// conversion to match the ruby spec. Seconds and Subseconds are stored independently as i64 and
/// u32 respectively, which gives enough granularity to meet the ruby [`Time`] spec.
///
/// [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Time {
    inner: DateTime,
}

// the local_tz() fn provided by tzdb will attempt to use iana_time_zone::get_timezone() to get the
// local timezone of the system running this. It's possible that the string returned from
// iana_time_zone is not recognized by tzdb, so this ensures it always returns something (UTC).
#[inline]
#[must_use]
fn local_time_zone() -> TimeZoneRef<'static> {
  match local_tz() {
    Some(tz) => tz,
    None => UTC,
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
impl From<ToA> for Time {
    fn from(_: ToA) -> Self {
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
    /// This can be used to implement [`Time#to_r`] since this gives the two parts needed for a
    /// rational. Note: This function is not enough to implement the full `to_r` since the number
    /// of seconds should be added to the numerator.
    ///
    /// #Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let t = Time::utc(1970, 1, 1, 0, 0, 1, 1000);
    /// assert_eq!(t.subsec_fractional(), (1000, 1000000000));
    /// ```
    ///
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

    // Time#getlocal, Time#[getgm|getutc]
    pub fn in_timezone(&self, tz: TimeZone) -> Self {
        todo!()
    }
}

// Mutators
impl Time {
    // for Time#localtime, Time#[gmtime|utc]
    pub fn set_timezone(&mut self) {
        todo!()
    }

    pub fn round(&mut self, digits: u32) {
        todo!()
    }
}

// Parts
impl Time {
    // Time#[nsec|tv_nsec]
    pub fn nano_second(&self) -> u64 {
        todo!()
    }
    // Time#[usec|tv_usec]
    pub fn micro_second(&self) -> u64 {
        todo!()
    }
    // Time#sec
    pub fn second(&self) -> u32 {
        todo!()
    }
    // Time#min
    pub fn minute(&self) -> u32 {
        todo!()
    }
    // Time#hour
    pub fn hour(&self) -> u32 {
        todo!()
    }
    // Time#[m]day
    pub fn day(&self) -> u32 {
        todo!()
    }
    // Time#mon[th]
    pub fn month(&self) -> u32 {
        todo!()
    }
    // Time#year
    pub fn year(&self) -> i32 {
        todo!()
    }

    // Time#[gmt?|utc?]
    pub fn time_zone<'a>(&self) -> &'a str {
        todo!()
    }

    // Time#[isdst|dst?]
    pub fn is_dst(&self) -> bool {
        todo!()
    }

    // Time#wday
    // Time#[monday?|tuesday?...]
    // 0 indexed to Sunday
    pub fn day_of_week(&self) -> u32 {
        todo!()
    }

    // Time#yday
    pub fn day_of_year(&self) -> u32 {
        todo!()
    }

    // Time#subsec
    // Good luck!
    pub fn sub_sec(&self) -> String {
        todo!()
    }
}
