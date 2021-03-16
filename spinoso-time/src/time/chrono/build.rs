use chrono::prelude::*;

use crate::time::chrono::{Offset, Time};
use crate::NANOS_IN_SECOND;

impl Default for Time {
    /// The zero-argument [`Time#new`] constructor creates a local time set to
    /// the current system time.
    ///
    /// [`Time#new`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-new
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    /// Creates a new `Time` object for the current time with a local offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let now = Time::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::now()
    }

    /// Creates a new `Time` object for the current time with a local offset.
    ///
    /// This is same as [`new`](Self::new).
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let now = Time::now();
    /// ```
    #[inline]
    #[must_use]
    pub fn now() -> Self {
        let now = Utc::now();
        let offset = Offset::Local;
        let timestamp = now.timestamp();
        let sub_second_nanos = now.timestamp_subsec_nanos();
        Self {
            timestamp,
            sub_second_nanos,
            offset,
        }
    }

    /// Creates a new `Time` object from the `seconds` and `sub_seconds_nanos`
    /// since the Unix EPOCH with a local offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::Time;
    /// let epoch = Time::at(0, 0);
    /// let epoch_plus_1_nano = Time::at(0, 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn at(seconds: i64, sub_second_nanos: i64) -> Self {
        let offset = Offset::Local;

        let timestamp = seconds + (sub_second_nanos / i64::from(NANOS_IN_SECOND));
        let sub_second_nanos = sub_second_nanos % i64::from(NANOS_IN_SECOND);

        // Handle negative sub_seconds_nanos
        let (timestamp, sub_second_nanos) = if sub_second_nanos > 0 {
            (timestamp, sub_second_nanos)
        } else {
            (timestamp - 1, (i64::from(NANOS_IN_SECOND) - sub_second_nanos.abs()))
        };

        Self {
            timestamp,
            sub_second_nanos: sub_second_nanos as u32,
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time::chrono::{Offset, Time};

    #[test]
    fn time_new_is_local_offset() {
        let time = Time::new();
        assert_eq!(time.offset, Offset::Local);
    }

    #[test]
    fn time_now_is_local_offset() {
        let time = Time::now();
        assert_eq!(time.offset, Offset::Local);
    }

    #[test]
    fn time_default_is_local_offset() {
        let time = Time::default();
        assert_eq!(time.offset, Offset::Local);
    }

    #[test]
    fn time_at_with_seconds_and_sub_second_nanos() {
        let time = Time::at(100, 100);
        assert_eq!(time.timestamp, 100);
        assert_eq!(time.sub_second_nanos, 100);
    }

    #[test]
    fn time_at_with_overflowing_sub_second_nanos() {
        let time = Time::at(100, 1_000_000_001);
        assert_eq!(time.timestamp, 101);
        assert_eq!(time.sub_second_nanos, 1);
    }

    #[test]
    fn time_at_with_negative_sub_second_nanos() {
        let time = Time::at(100, -1);
        assert_eq!(time.timestamp, 99);
        assert_eq!(time.sub_second_nanos, 999_999_999);
    }
}
