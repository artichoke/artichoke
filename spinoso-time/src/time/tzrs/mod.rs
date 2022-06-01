use tz::datetime::DateTime;

mod build;
mod convert;
mod math;
mod offset;
mod parts;
mod timezone;
mod to_a;

pub use offset::Offset;
pub use to_a::ToA;

use crate::NANOS_IN_SECOND;

/// A wrapper around [`tz::datetime::DateTime`] which contains everything needed for date creation and
/// conversion to match the ruby spec. Seconds and Subseconds are stored independently as i64 and
/// u32 respectively, which gives enough granularity to meet the ruby [`Time`] spec.
///
/// [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
#[must_use]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Time {
    /// A wrapper around [`tz::datetime::DateTime`] to provide date and time formatting
    inner: DateTime,
    /// The offset to used for the provided _time_
    offset: Offset,
}

// constructors
impl Time {
    /// Returns a new Time from the given values in the provided TimeZone.
    ///
    /// Can be used to implment ruby [`Time#new`] (using a [`Timezone`] Object)
    ///
    /// Note: During DST transitions, a specific time can be ambiguous. This method will always pick the earliest date.
    ///
    /// # Examples
    /// ```
    /// use spinoso_time::{Time, Offset};
    /// use tzdb::time_zone::pacific::AUCKLAND;
    /// let offset = Offset::tz(AUCKLAND);
    /// let t = Time::new(2022, 9, 25, 1, 30, 0, 0, offset);
    /// ```
    ///
    /// [`Time#new`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-new
    /// [`Timezone`]: https://ruby-doc.org/core-2.6.3/Time.html#class-Time-label-Timezone+argument
    #[inline]
    #[must_use]
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanoseconds: u32,
        offset: Offset,
    ) -> Self {
        let tz = offset.time_zone_ref();
        let found_date_times = DateTime::find(year, month, day, hour, minute, second, nanoseconds, tz).unwrap();
        let dt = found_date_times
            .unique()
            .expect("Could not find a matching DateTime for this timezone");
        Self { inner: dt, offset }
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
    #[inline]
    #[must_use]
    pub fn now() -> Self {
        let offset = Offset::local();
        let time_zone_ref = offset.time_zone_ref();
        let now = DateTime::now(time_zone_ref).unwrap();
        Self { inner: now, offset }
    }

    /// Returns a Time in the given timezone with the number of seconds and nano_seconds since the Epoch in the specified timezone
    ///
    /// Can be used to implement ruby [`Time#at`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::{Time, Offset};
    /// let offset = Offset::utc();
    /// let t = Time::with_timespec_and_offset(0, 0, offset);
    /// assert_eq!(t.to_int(), 0);
    /// ```
    ///
    /// [`Time#at`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-at
    #[inline]
    #[must_use]
    pub fn with_timespec_and_offset(seconds: i64, nano_seconds: u32, offset: Offset) -> Self {
        let time_zone_ref = offset.time_zone_ref();
        Self {
            inner: DateTime::from_timespec(seconds, nano_seconds, time_zone_ref).unwrap(),
            offset,
        }
    }
}

impl From<ToA> for Time {
    /// Create a new Time object base on a ToA
    ///
    /// Note: This converting from a Time object to a ToA and back again is lossy since ToA does
    /// not store nanoseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let now = Time::local(2022, 7, 8, 12, 34, 56, 1000);
    /// let to_a = now.to_array();
    /// let from_to_a = Time::from(to_a);
    /// assert_eq!(now.second(), from_to_a.second());
    /// assert_ne!(now.nanoseconds(), from_to_a.nanoseconds());
    /// ```
    #[inline]
    #[must_use]
    fn from(to_a: ToA) -> Self {
        Self::new(
            to_a.year, to_a.month, to_a.day, to_a.hour, to_a.min, to_a.sec, 0, to_a.zone,
        )
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
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
    pub fn subsec_fractional(&self) -> (u32, u32) {
        (self.inner.nanoseconds(), NANOS_IN_SECOND)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time_with_fixed_offset(offset: i32) -> Time {
        let offset = Offset::fixed(offset);
        Time::with_timespec_and_offset(0, 0, offset)
    }

    #[test]
    fn time_zone_fixed_offset() {
        assert_eq!("-02:02", time_with_fixed_offset(-7320).time_zone());
        assert_eq!("+00:00", time_with_fixed_offset(0).time_zone());
        assert_eq!("+00:00", time_with_fixed_offset(59).time_zone());
    }
}
