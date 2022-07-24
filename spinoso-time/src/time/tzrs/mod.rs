use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

use tz::datetime::DateTime;

mod build;
mod convert;
mod error;
mod math;
mod offset;
mod parts;
mod timezone;
mod to_a;

pub use error::TimeError;
pub use offset::{Offset, MAX_OFFSET_SECONDS, MIN_OFFSET_SECONDS};
pub use to_a::ToA;

/// Alias for [`std::result::Result`] with the unified `TimeError`
pub type Result<T> = std::result::Result<T, TimeError>;

use crate::NANOS_IN_SECOND;

/// Implementation of Ruby [`Time`], a timezone-aware datetime, based on
/// [`tz-rs`] and [`tzdb`].
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
/// # Examples
///
/// ```
/// # use spinoso_time::tzrs::{Time, TimeError};
/// # fn example() -> Result<(), TimeError> {
/// // Create a Time to the current system clock with local offset
/// let time = Time::now()?;
/// assert!(!time.is_utc());
/// println!("{}", time.is_sunday());
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// ```
/// # use spinoso_time::tzrs::{Time, TimeError};
/// # fn example() -> Result<(), TimeError> {
/// let time = Time::now()?;
/// let one_hour_ago: Time = time.checked_sub_u64(60 * 60)?;
/// assert_eq!(time.to_int() - 3600, one_hour_ago.to_int());
/// assert_eq!(time.nanoseconds(), one_hour_ago.nanoseconds());
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// [`tz-rs`]: tz
/// [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Time {
    /// A wrapper around [`tz::datetime::DateTime`] to provide date and time
    /// formatting.
    inner: DateTime,
    /// The offset to used for the provided _time_.
    offset: Offset,
}

impl Hash for Time {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash is only based on the nanos since epoch:
        //
        // ```console
        // [3.1.2] > t = Time.now
        // => 2022-06-26 14:41:03.192545 -0700
        // [3.1.2] > t.zone
        // => "PDT"
        // [3.1.2] > t.hash
        // => 3894887943343456722
        // [3.1.2] > u = t.utc
        // => 2022-06-26 21:41:03.192545 UTC
        // [3.1.2] > u.zone
        // => "UTC"
        // [3.1.2] > u.hash
        // => 3894887943343456722
        // ```
        state.write_i128(self.inner.total_nanoseconds());
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.inner.total_nanoseconds() == other.inner.total_nanoseconds()
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
        self.inner.total_nanoseconds().cmp(&other.inner.total_nanoseconds())
    }
}

// constructors
impl Time {
    /// Returns a new Time from the given values in the provided `offset`.
    ///
    /// Can be used to implement the Ruby method [`Time#new`] (using a
    /// [`Timezone`] Object).
    ///
    /// **Note**: During DST transitions, a specific time can be ambiguous. This
    /// method will always pick the latest date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, Offset, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let offset = Offset::try_from("+1200")?;
    /// let t = Time::new(2022, 9, 25, 1, 30, 0, 0, offset);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeError`], generally when provided values are out of range.
    ///
    /// [`Time#new`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-new
    /// [`Timezone`]: https://ruby-doc.org/core-2.6.3/Time.html#class-Time-label-Timezone+argument
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanoseconds: u32,
        offset: Offset,
    ) -> Result<Self> {
        let tz = offset.time_zone_ref();
        let found_date_times = DateTime::find(year, month, day, hour, minute, second, nanoseconds, tz)?;

        // .latest() will always return `Some(DateTime)`
        // FIXME: this assertion is not consistent with the docs in `tz-rs`.
        let dt = found_date_times.latest().expect("No datetime found with this offset");
        Ok(Self { inner: dt, offset })
    }

    /// Returns a Time with the current time in the System Timezone.
    ///
    /// Can be used to implement the Ruby method [`Time#now`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, Offset, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let now = Time::now()?;
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeError`], however these should never been seen in regular usage.
    ///
    /// [`Time#now`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-now
    #[inline]
    pub fn now() -> Result<Self> {
        let offset = Offset::local();
        let time_zone_ref = offset.time_zone_ref();
        let now = DateTime::now(time_zone_ref)?;
        Ok(Self { inner: now, offset })
    }

    /// Returns a Time in the given timezone with the number of `seconds` and
    /// `nanoseconds` since the Epoch in the specified timezone.
    ///
    /// Can be used to implement the Ruby method [`Time#at`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, Offset, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let offset = Offset::utc();
    /// let t = Time::with_timespec_and_offset(0, 0, offset)?;
    /// assert_eq!(t.to_int(), 0);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeError`], however these should not be seen during regular usage.
    ///
    /// [`Time#at`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-at
    #[inline]
    pub fn with_timespec_and_offset(seconds: i64, nanoseconds: u32, offset: Offset) -> Result<Self> {
        let time_zone_ref = offset.time_zone_ref();
        let dt = DateTime::from_timespec(seconds, nanoseconds, time_zone_ref)?;
        Ok(Self { inner: dt, offset })
    }
}

impl TryFrom<ToA> for Time {
    type Error = TimeError;

    /// Create a new Time object base on a `ToA`
    ///
    /// **Note**: This converting from a Time object to a `ToA` and back again
    /// is lossy since `ToA` does not store nanoseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, Offset, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let now = Time::local(2022, 7, 8, 12, 34, 56, 1000)?;
    /// let to_a = now.to_array();
    /// let from_to_a = Time::try_from(to_a)?;
    /// assert_eq!(now.second(), from_to_a.second());
    /// assert_ne!(now.nanoseconds(), from_to_a.nanoseconds());
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeError`], generally when provided values are out of range.
    #[inline]
    fn try_from(to_a: ToA) -> Result<Self> {
        let offset = Offset::try_from(to_a.zone).unwrap_or_else(|_| Offset::utc());

        Self::new(
            to_a.year, to_a.month, to_a.day, to_a.hour, to_a.min, to_a.sec, 0, offset,
        )
    }
}

// Core
impl Time {
    /// Returns the number of seconds as a signed integer since the Epoch.
    ///
    /// This function can be used to implement the Ruby methods [`Time#to_i`]
    /// and [`Time#tv_sec`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, Offset, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let t = Time::utc(1970, 1, 1, 0, 1, 0, 0)?;
    /// assert_eq!(t.to_int(), 60);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// [`Time#to_i`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-to_i
    /// [`Time#tv_sec`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-tv_sec
    #[inline]
    #[must_use]
    pub fn to_int(&self) -> i64 {
        self.inner.unix_time()
    }

    /// Returns the number of seconds since the Epoch with fractional nanos
    /// included at IEEE 754-2008 accuracy.
    ///
    /// This function can be used to implement the Ruby method [`Time#to_f`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, Offset, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let now = Time::utc(1970, 1, 1, 0, 1, 0, 1000)?;
    /// assert_eq!(now.to_float(), 60.000001);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// [`Time#to_f`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-to_f
    #[inline]
    #[must_use]
    pub fn to_float(&self) -> f64 {
        // A `f64` mantissa is only 52 bits wide, so putting 64 bits in there
        // will result in a rounding issues, however this is expected in the
        // Ruby spec.
        #[allow(clippy::cast_precision_loss)]
        let sec = self.to_int() as f64;
        let nanos_fractional = f64::from(self.inner.nanoseconds()) / f64::from(NANOS_IN_SECOND);
        sec + nanos_fractional
    }

    /// Returns the numerator and denominator for the number of nanoseconds of
    /// the Time struct unsimplified.
    ///
    /// This can be used directly to implement [`Time#subsec`].
    ///
    /// This function can be used in combination with [`to_int`] to implement
    /// [`Time#to_r`].
    ///
    /// #Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, Offset, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let t = Time::utc(1970, 1, 1, 0, 0, 1, 1000)?;
    /// assert_eq!(t.subsec_fractional(), (1000, 1000000000));
    /// # Ok(())
    /// # }
    /// # example().unwrap();
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
        let offset = Offset::fixed(offset).unwrap();
        Time::with_timespec_and_offset(0, 0, offset).unwrap()
    }

    #[test]
    fn time_zone_fixed_offset() {
        assert_eq!("-0202", time_with_fixed_offset(-7320).time_zone());
        assert_eq!("+0000", time_with_fixed_offset(0).time_zone());
        assert_eq!("+0000", time_with_fixed_offset(59).time_zone());
    }
}
