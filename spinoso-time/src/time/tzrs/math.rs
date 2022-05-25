use core::ops::Add;
use core::time::Duration;

use tz::datetime::DateTime;

use crate::time::tzrs::Time;
use crate::NANOS_IN_SECOND;

impl Add<Duration> for Time {
    type Output = Self;
    fn add(self, _to_add: Duration) -> Self {
        todo!()
    }
}

impl Add<i64> for Time {
    type Output = Self;
    fn add(self, _to_add: i64) -> Self {
        todo!()
    }
}

impl Time {
    // Time#succ (obselete)
    pub fn succ(&self) -> Self {
        todo!()
    }

    /// Rounds sub seconds to a given precision in decimal digits (0 digits by default). It returns
    /// a new Time object. `ndigits` should be zero or a positive integer.
    ///
    /// Can be used to implement [`Time#round`]
    ///
    /// # Examples
    /// ```
    /// use spinoso_time::Time;
    /// let now = Time::local(2010, 3, 30, 5, 43, 25, 123456789);
    /// let rounded = now.round(5);
    /// assert_eq!(now.utc_offset(), rounded.utc_offset());
    /// assert_eq!(123460000, rounded.nanoseconds());
    /// ```
    ///
    /// [`Time#round`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-round
    #[inline]
    #[must_use]
    pub fn round(&self, digits: u32) -> Self {
        match digits {
            9..=u32::MAX => self.clone(),
            digits => {
                let local_time_type = self.inner.local_time_type().clone();
                let mut unix_time = self.to_int();
                let nanos = self.nanoseconds() as f64 / NANOS_IN_SECOND as f64;

                let exponent_shift = 10_u32.pow(digits) as f64;

                let rounded_nanos = (nanos * exponent_shift).round() / exponent_shift;
                let mut new_nanos = (rounded_nanos * NANOS_IN_SECOND as f64) as u32;
                if new_nanos >= NANOS_IN_SECOND {
                    unix_time += 1;
                    new_nanos -= NANOS_IN_SECOND;
                }

                let dt = DateTime::from_timespec_and_local(unix_time, new_nanos, local_time_type);
                Self { inner: dt.unwrap() }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding() {
        let now = Time::local(2010, 3, 30, 5, 43, 25, 123456789);
        assert_eq!(0, now.round(0).nanoseconds());
        assert_eq!(100000000, now.round(1).nanoseconds());
        assert_eq!(120000000, now.round(2).nanoseconds());
        assert_eq!(123000000, now.round(3).nanoseconds());
        assert_eq!(123500000, now.round(4).nanoseconds());
        assert_eq!(123460000, now.round(5).nanoseconds());
        assert_eq!(123457000, now.round(6).nanoseconds());
        assert_eq!(123456800, now.round(7).nanoseconds());
        assert_eq!(123456790, now.round(8).nanoseconds());
        assert_eq!(123456789, now.round(9).nanoseconds());
        assert_eq!(123456789, now.round(10).nanoseconds());
        assert_eq!(123456789, now.round(11).nanoseconds());
    }

    #[test]
    fn rounding_rollup() {
        let now = Time::utc(1999, 12, 31, 23, 59, 59, 900_000_000);
        let rounded = now.round(0);
        let now_unix = now.to_int();
        let rounded_unix = rounded.to_int();
        assert_eq!(0, rounded.nanoseconds());
        assert_eq!(now_unix + 1, rounded_unix);
    }
}
