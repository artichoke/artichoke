use core::ops::{Add, Sub};
use core::time::Duration;

use tz::datetime::DateTime;

use crate::time::tzrs::Time;
use crate::NANOS_IN_SECOND;

impl Time {
    /// Rounds sub seconds to a given precision in decimal digits (0 digits by
    /// default). It returns a new Time object. `ndigits` should be zero or a
    /// positive integer.
    ///
    /// Can be used to implement [`Time#round`]
    ///
    /// # Examples
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::local(2010, 3, 30, 5, 43, 25, 123456789);
    /// let rounded = now.round(5);
    /// assert_eq!(now.utc_offset(), rounded.utc_offset());
    /// assert_eq!(123460000, rounded.nanoseconds());
    /// ```
    ///
    /// [`Time#round`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-round
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    pub fn round(&self, digits: u32) -> Self {
        match digits {
            9..=u32::MAX => *self,
            // Does integer truncation with round up at 5.
            //
            // ``console
            // [3.1.2] > t = Time.at(Time.new(2010, 3, 30, 5, 43, 25).to_i, 123_456_789, :nsec)
            // => 2010-03-30 05:43:25.123456789 -0700
            // [3.1.2] > (0..9).each {|d| u = t.round(d); puts "#{d}: #{u.nsec}" }
            // 0: 0
            // 1: 100000000
            // 2: 120000000
            // 3: 123000000
            // 4: 123500000
            // 5: 123460000
            // 6: 123457000
            // 7: 123456800
            // 8: 123456790
            // 9: 123456789
            // ```
            digits => {
                let local_time_type = *self.inner.local_time_type();
                let mut unix_time = self.to_int();
                let nanos = self.nanoseconds();

                // `digits` is guaranteed to be at most `8` so these subtractions
                // can never underflow.
                let truncating_divisor = 10_u64.pow(9 - digits - 1);
                let rounding_multiple = 10_u64.pow(9 - digits);

                let truncated = u64::from(nanos) / truncating_divisor;
                let mut new_nanos = if truncated % 10 >= 5 {
                    (truncated / 10) + 1
                } else {
                    truncated / 10
                }
                .checked_mul(rounding_multiple)
                .and_then(|nanos| nanos.try_into().ok())
                .expect("new nanos are a truncated version of input which is in bounds for u32");

                if new_nanos >= NANOS_IN_SECOND {
                    unix_time += 1;
                    new_nanos -= NANOS_IN_SECOND;
                }

                let dt = DateTime::from_timespec_and_local(unix_time, new_nanos, local_time_type)
                    .expect("Could not round the datetime");
                Self {
                    inner: dt,
                    offset: self.offset,
                }
            }
        }
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        let unix_time = self.inner.unix_time();
        let nanoseconds = self.inner.nanoseconds();
        let offset = self.offset;

        let duration_seconds = i64::try_from(duration.as_secs()).expect("Duration too large");
        let duration_subsecs = duration.subsec_nanos();

        let mut seconds = unix_time.checked_add(duration_seconds).expect("Duration too large");
        let mut nanoseconds = nanoseconds.checked_add(duration_subsecs).expect("Duration too large");

        if nanoseconds > NANOS_IN_SECOND {
            seconds += 1;
            nanoseconds -= NANOS_IN_SECOND;
        }

        Self::Output::with_timespec_and_offset(seconds, nanoseconds, offset)
    }
}

impl Add<i8> for Time {
    type Output = Self;

    fn add(self, seconds: i8) -> Self::Output {
        self + i64::from(seconds)
    }
}

impl Add<u8> for Time {
    type Output = Self;

    fn add(self, seconds: u8) -> Self::Output {
        self + u64::from(seconds)
    }
}

impl Add<i16> for Time {
    type Output = Self;

    fn add(self, seconds: i16) -> Self::Output {
        self + i64::from(seconds)
    }
}

impl Add<u16> for Time {
    type Output = Self;

    fn add(self, seconds: u16) -> Self::Output {
        self + u64::from(seconds)
    }
}

impl Add<i32> for Time {
    type Output = Self;

    fn add(self, seconds: i32) -> Self::Output {
        self + i64::from(seconds)
    }
}

impl Add<u32> for Time {
    type Output = Self;

    fn add(self, seconds: u32) -> Self::Output {
        self + u64::from(seconds)
    }
}

impl Add<i64> for Time {
    type Output = Self;

    fn add(self, seconds: i64) -> Self::Output {
        if seconds.is_negative() {
            let seconds = seconds
                .checked_neg()
                .and_then(|secs| u64::try_from(secs).ok())
                .expect("Duration too large");
            self - Duration::from_secs(seconds)
        } else {
            let seconds = u64::try_from(seconds).expect("Duration too large");
            self + Duration::from_secs(seconds)
        }
    }
}

impl Add<u64> for Time {
    type Output = Self;

    fn add(self, seconds: u64) -> Self::Output {
        let duration = Duration::from_secs(seconds);
        self + duration
    }
}

impl Add<f32> for Time {
    type Output = Self;

    fn add(self, seconds: f32) -> Self::Output {
        self + f64::from(seconds)
    }
}

impl Add<f64> for Time {
    type Output = Self;

    fn add(self, seconds: f64) -> Self::Output {
        if seconds.is_sign_positive() {
            self + Duration::from_secs_f64(seconds)
        } else {
            let seconds = -seconds;
            self - Duration::from_secs_f64(seconds)
        }
    }
}

impl Sub<Duration> for Time {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        let unix_time = self.inner.unix_time();
        let nanoseconds = self.inner.nanoseconds();
        let offset = self.offset;

        let duration_seconds = i64::try_from(duration.as_secs()).expect("Duration too large");
        let duration_subsecs = duration.subsec_nanos();

        let mut seconds = unix_time.checked_sub(duration_seconds).expect("Duration too large");
        let nanoseconds = if let Some(nanos) = nanoseconds.checked_sub(duration_subsecs) {
            nanos
        } else {
            seconds -= 1;
            nanoseconds + NANOS_IN_SECOND - duration_subsecs
        };

        Self::Output::with_timespec_and_offset(seconds, nanoseconds, offset)
    }
}

impl Sub<i8> for Time {
    type Output = Self;

    fn sub(self, seconds: i8) -> Self::Output {
        self - i64::from(seconds)
    }
}

impl Sub<u8> for Time {
    type Output = Self;

    fn sub(self, seconds: u8) -> Self::Output {
        self - u64::from(seconds)
    }
}

impl Sub<i16> for Time {
    type Output = Self;

    fn sub(self, seconds: i16) -> Self::Output {
        self - i64::from(seconds)
    }
}

impl Sub<u16> for Time {
    type Output = Self;

    fn sub(self, seconds: u16) -> Self::Output {
        self - u64::from(seconds)
    }
}

impl Sub<i32> for Time {
    type Output = Self;

    fn sub(self, seconds: i32) -> Self::Output {
        self - i64::from(seconds)
    }
}

impl Sub<u32> for Time {
    type Output = Self;

    fn sub(self, seconds: u32) -> Self::Output {
        self - u64::from(seconds)
    }
}

impl Sub<i64> for Time {
    type Output = Self;

    fn sub(self, seconds: i64) -> Self::Output {
        self + -seconds
    }
}

impl Sub<u64> for Time {
    type Output = Self;

    fn sub(self, seconds: u64) -> Self::Output {
        let duration = Duration::from_secs(seconds);
        self - duration
    }
}

impl Sub<f32> for Time {
    type Output = Self;

    fn sub(self, seconds: f32) -> Self::Output {
        self - f64::from(seconds)
    }
}

impl Sub<f64> for Time {
    type Output = Self;

    fn sub(self, seconds: f64) -> Self::Output {
        self + -seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datetime() -> Time {
        // halfway through a second
        Time::utc(2019, 4, 7, 23, 59, 59, 500_000_000)
    }

    #[test]
    fn rounding() {
        let dt = Time::utc(2010, 3, 30, 5, 43, 25, 123_456_789);
        assert_eq!(0, dt.round(0).nanoseconds());
        assert_eq!(100_000_000, dt.round(1).nanoseconds());
        assert_eq!(120_000_000, dt.round(2).nanoseconds());
        assert_eq!(123_000_000, dt.round(3).nanoseconds());
        assert_eq!(123_500_000, dt.round(4).nanoseconds());
        assert_eq!(123_460_000, dt.round(5).nanoseconds());
        assert_eq!(123_457_000, dt.round(6).nanoseconds());
        assert_eq!(123_456_800, dt.round(7).nanoseconds());
        assert_eq!(123_456_790, dt.round(8).nanoseconds());
        assert_eq!(123_456_789, dt.round(9).nanoseconds());
        assert_eq!(123_456_789, dt.round(10).nanoseconds());
        assert_eq!(123_456_789, dt.round(11).nanoseconds());
    }

    #[test]
    fn rounding_rollup() {
        let dt = Time::utc(1999, 12, 31, 23, 59, 59, 900_000_000);
        let rounded = dt.round(0);
        let dt_unix = dt.to_int();
        let rounded_unix = rounded.to_int();
        assert_eq!(0, rounded.nanoseconds());
        assert_eq!(dt_unix + 1, rounded_unix);
    }

    #[test]
    fn add_int_to_time() {
        let dt = datetime();
        let succ: Time = dt + 1;
        assert_eq!(dt.to_int() + 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 0);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 500_000_000 {
            assert!(succ.nanoseconds() - 500_000_000 < 50);
        } else {
            assert!(500_000_000 - succ.nanoseconds() < 50);
        }
    }

    #[test]
    fn add_subsec_float_to_time() {
        let dt = datetime();
        let succ: Time = dt + 0.2;
        assert_eq!(dt.to_int(), succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 59);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 700_000_000 {
            assert!(succ.nanoseconds() - 700_000_000 < 50);
        } else {
            assert!(700_000_000 - succ.nanoseconds() < 50);
        }

        let dt = datetime();
        let succ: Time = dt + 0.7;
        assert_eq!(dt.to_int() + 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 0);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 200_000_000 {
            assert!(succ.nanoseconds() - 200_000_000 < 50);
        } else {
            assert!(200_000_000 - succ.nanoseconds() < 50);
        }
    }

    #[test]
    fn add_float_to_time() {
        let dt = datetime();
        let succ: Time = dt + 1.2;
        assert_eq!(dt.to_int() + 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 0);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 700_000_000 {
            assert!(succ.nanoseconds() - 700_000_000 < 50);
        } else {
            assert!(700_000_000 - succ.nanoseconds() < 50);
        }

        let dt = datetime();
        let succ: Time = dt + 1.7;
        assert_eq!(dt.to_int() + 2, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 1);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 200_000_000 {
            assert!(succ.nanoseconds() - 200_000_000 < 50);
        } else {
            assert!(200_000_000 - succ.nanoseconds() < 50);
        }
    }

    #[test]
    fn sub_int_to_time() {
        let dt = datetime();
        let succ: Time = dt - 1;
        assert_eq!(dt.to_int() - 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 58);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 500_000_000 {
            assert!(succ.nanoseconds() - 500_000_000 < 50);
        } else {
            assert!(500_000_000 - succ.nanoseconds() < 50);
        }
    }

    #[test]
    fn sub_subsec_float_to_time() {
        let dt = datetime();
        let succ: Time = dt - 0.2;
        assert_eq!(dt.to_int(), succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 59);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 300_000_000 {
            assert!(succ.nanoseconds() - 300_000_000 < 50);
        } else {
            assert!(300_000_000 - succ.nanoseconds() < 50);
        }

        let dt = datetime();
        let succ: Time = dt - 0.7;
        assert_eq!(dt.to_int() - 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 58);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 800_000_000 {
            assert!(succ.nanoseconds() - 800_000_000 < 50);
        } else {
            assert!(800_000_000 - succ.nanoseconds() < 50);
        }
    }

    #[test]
    fn sub_float_to_time() {
        let dt = datetime();
        let succ: Time = dt - 1.2;
        assert_eq!(dt.to_int() - 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 58);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 300_000_000 {
            assert!(succ.nanoseconds() - 300_000_000 < 50);
        } else {
            assert!(300_000_000 - succ.nanoseconds() < 50);
        }

        let dt = datetime();
        let succ: Time = dt - 1.7;
        assert_eq!(dt.to_int() - 2, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 57);
        // handle in-exactitude of float arithmetic
        if succ.nanoseconds() > 800_000_000 {
            assert!(succ.nanoseconds() - 800_000_000 < 50);
        } else {
            assert!(800_000_000 - succ.nanoseconds() < 50);
        }
    }
}
