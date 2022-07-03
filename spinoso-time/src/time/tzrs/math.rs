use core::fmt;
use core::num::TryFromIntError;
use core::time::Duration;
use std::error;

use tz::datetime::DateTime;

use super::{Time, TimeError};
use crate::NANOS_IN_SECOND;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    TryFromIntError(TryFromIntError),
    RangeError,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::TryFromIntError(error) => write!(f, "Range too large: {}", error),
            Self::RangeError => write!(f, "Range too large"),
        }
    }
}
impl error::Error for MathError {}

impl From<TryFromIntError> for MathError {
    fn from(u: TryFromIntError) -> Self {
        Self::TryFromIntError(u)
    }
}

impl Time {
    /// Rounds sub seconds to a given precision in decimal digits (0 digits by
    /// default). It returns a new Time object. `ndigits` should be zero or a
    /// positive integer.
    ///
    /// Can be used to implement [`Time#round`]
    ///
    /// # Examples
    /// ```
    /// # use spinoso_time::tzrs::{Time, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let now = Time::local(2010, 3, 30, 5, 43, 25, 123456789)?;
    /// let rounded = now.round(5);
    /// assert_eq!(now.utc_offset(), rounded.utc_offset());
    /// assert_eq!(123460000, rounded.nanoseconds());
    /// # Ok(())
    /// # }
    /// # example().unwrap()
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
                    ((truncated / 10) + 1) * rounding_multiple
                } else {
                    (truncated / 10) * rounding_multiple
                }
                .try_into()
                .expect("new nanos are a truncated version of input which is in bounds for u32");

                if new_nanos >= NANOS_IN_SECOND {
                    unix_time += 1;
                    new_nanos -= NANOS_IN_SECOND;
                }

                // Rounding should never cause an error generating a new time since it's always a truncation
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

// Addition
impl Time {
    /// Addition — Adds some duration to _time_ and returns that value as a new
    /// Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add(self, duration: Duration) -> Result<Self, TimeError> {
        let unix_time = self.inner.unix_time();
        let nanoseconds = self.inner.nanoseconds();
        let offset = self.offset;

        let duration_seconds = i64::try_from(duration.as_secs()).map_err(MathError::TryFromIntError)?;
        let duration_subsecs = duration.subsec_nanos();

        let mut seconds = unix_time.checked_add(duration_seconds).ok_or(MathError::RangeError)?;
        let mut nanoseconds = nanoseconds.checked_add(duration_subsecs).ok_or(MathError::RangeError)?;

        if nanoseconds > NANOS_IN_SECOND {
            seconds += 1;
            nanoseconds -= NANOS_IN_SECOND;
        }

        Self::with_timespec_and_offset(seconds, nanoseconds, offset)
    }

    /// Addition — Adds some i8 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_i8(&self, seconds: i8) -> Result<Self, TimeError> {
        self.checked_add_i64(i64::from(seconds))
    }

    /// Addition — Adds some u8 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_u8(&self, seconds: u8) -> Result<Self, TimeError> {
        self.checked_add_u64(u64::from(seconds))
    }

    /// Addition — Adds some i16 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_i16(&self, seconds: i16) -> Result<Self, TimeError> {
        self.checked_add_i64(i64::from(seconds))
    }

    /// Addition — Adds some u16 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_u16(&self, seconds: u16) -> Result<Self, TimeError> {
        self.checked_add_u64(u64::from(seconds))
    }

    /// Addition — Adds some i32 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_i32(&self, seconds: i32) -> Result<Self, TimeError> {
        self.checked_add_i64(i64::from(seconds))
    }

    /// Addition — Adds some u32 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_u32(&self, seconds: u32) -> Result<Self, TimeError> {
        self.checked_add_u64(u64::from(seconds))
    }

    /// Addition — Adds some i64 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_i64(&self, seconds: i64) -> Result<Self, TimeError> {
        if seconds.is_negative() {
            let seconds = seconds
                .checked_neg()
                .and_then(|secs| u64::try_from(secs).ok())
                .expect("Duration too large");
            self.checked_sub_u64(seconds)
        } else {
            let seconds = u64::try_from(seconds).expect("Duration too large");
            self.checked_add_u64(seconds)
        }
    }

    /// Addition — Adds some u64 to _time_ and returns that value as a new Time
    /// object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_u64(&self, seconds: u64) -> Result<Self, TimeError> {
        let duration = Duration::from_secs(seconds);
        self.checked_add(duration)
    }

    /// Addition — Adds some f32 fraction seconds to _time_ and returns that
    /// value as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_f32(&self, seconds: f32) -> Result<Self, TimeError> {
        self.checked_add_f64(f64::from(seconds))
    }

    /// Addition — Adds some f64 fraction seconds to _time_ and returns that
    /// value as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_add_f64(&self, seconds: f64) -> Result<Self, TimeError> {
        if seconds.is_sign_positive() {
            self.checked_add(Duration::from_secs_f64(seconds))
        } else {
            self.checked_sub(Duration::from_secs_f64(-seconds))
        }
    }
}

// Subtraction
impl Time {
    /// Subtration — Subtracts the given duration from _time_ and returns that
    /// value as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub(self, duration: Duration) -> Result<Self, TimeError> {
        let unix_time = self.inner.unix_time();
        let nanoseconds = self.inner.nanoseconds();
        let offset = self.offset;

        let duration_seconds = i64::try_from(duration.as_secs()).map_err(MathError::TryFromIntError)?;
        let duration_subsecs = duration.subsec_nanos();

        let mut seconds = unix_time.checked_sub(duration_seconds).ok_or(MathError::RangeError)?;
        let nanoseconds = if let Some(nanos) = nanoseconds.checked_sub(duration_subsecs) {
            nanos
        } else {
            seconds -= 1;
            nanoseconds + NANOS_IN_SECOND - duration_subsecs
        };

        Self::with_timespec_and_offset(seconds, nanoseconds, offset)
    }

    /// Subtration — Subtracts the given i8 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_i8(self, seconds: i8) -> Result<Self, TimeError> {
        self.checked_sub_i64(i64::from(seconds))
    }

    /// Subtration — Subtracts the given u8 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_u8(self, seconds: u8) -> Result<Self, TimeError> {
        self.checked_sub_u64(u64::from(seconds))
    }

    /// Subtration — Subtracts the given i16 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_i16(self, seconds: i16) -> Result<Self, TimeError> {
        self.checked_sub_i64(i64::from(seconds))
    }

    /// Subtration — Subtracts the given u16 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_u16(self, seconds: u16) -> Result<Self, TimeError> {
        self.checked_sub_u64(u64::from(seconds))
    }

    /// Subtration — Subtracts the given i32 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_i32(self, seconds: i32) -> Result<Self, TimeError> {
        self.checked_sub_i64(i64::from(seconds))
    }

    /// Subtration — Subtracts the given u32 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_u32(self, seconds: u32) -> Result<Self, TimeError> {
        self.checked_sub_u64(u64::from(seconds))
    }

    /// Subtration — Subtracts the given i64 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_i64(self, seconds: i64) -> Result<Self, TimeError> {
        if seconds.is_negative() {
            let seconds = seconds
                .checked_neg()
                .and_then(|secs| u64::try_from(secs).ok())
                .expect("Duration too large");
            self.checked_add_u64(seconds)
        } else {
            let seconds = u64::try_from(seconds).expect("Duration too large");
            self.checked_sub_u64(seconds)
        }
    }

    /// Subtration — Subtracts the given u64 from _time_ and returns that value
    /// as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_u64(self, seconds: u64) -> Result<Self, TimeError> {
        let duration = Duration::from_secs(seconds);
        self.checked_sub(duration)
    }

    /// Subtration — Subtracts the given f32 as fraction seconds from _time_
    /// and returns that value as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_f32(self, seconds: f32) -> Result<Self, TimeError> {
        self.checked_sub_f64(f64::from(seconds))
    }

    /// Subtration — Subtracts the given f64 as fraction seconds from _time_
    /// and returns that value as a new Time object.
    ///
    /// # Errors
    ///
    /// If this function attempts to overflow the the number of seconds as an
    /// i64 then a [`TimeError`] will be returned.
    pub fn checked_sub_f64(self, seconds: f64) -> Result<Self, TimeError> {
        if seconds.is_sign_positive() {
            self.checked_sub(Duration::from_secs_f64(seconds))
        } else {
            self.checked_add(Duration::from_secs_f64(-seconds))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datetime() -> Time {
        // halfway through a second
        Time::utc(2019, 4, 7, 23, 59, 59, 500_000_000).unwrap()
    }

    #[test]
    fn rounding() {
        let dt = Time::utc(2010, 3, 30, 5, 43, 25, 123_456_789).unwrap();
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
        let dt = Time::utc(1999, 12, 31, 23, 59, 59, 900_000_000).unwrap();
        let rounded = dt.round(0);
        let dt_unix = dt.to_int();
        let rounded_unix = rounded.to_int();
        assert_eq!(0, rounded.nanoseconds());
        assert_eq!(dt_unix + 1, rounded_unix);
    }

    #[test]
    fn add_int_to_time() {
        let dt = datetime();
        let succ: Time = dt.checked_add_u64(1).unwrap();
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
        let succ: Time = dt.checked_add_f64(0.2).unwrap();
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
        let succ: Time = dt.checked_add_f64(0.7).unwrap();
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
        let succ: Time = dt.checked_add_f64(1.2).unwrap();
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
        let succ: Time = dt.checked_add_f64(1.7).unwrap();
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
        let succ: Time = dt.checked_sub_u64(1).unwrap();
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
        let succ: Time = dt.checked_sub_f64(0.2).unwrap();
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
        let succ: Time = dt.checked_sub_f64(0.7).unwrap();
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
        let succ: Time = dt.checked_sub_f64(1.2).unwrap();
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
        let succ: Time = dt.checked_sub_f64(1.7).unwrap();
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
