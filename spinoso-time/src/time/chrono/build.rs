use chrono::prelude::*;

use crate::time::chrono::{Offset, Time};

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
}
