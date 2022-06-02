use crate::time::chrono::Time;
use crate::NANOS_IN_SECOND;

impl Time {
    /// Returns a new Time object, one second later than time.
    ///
    /// This method should log a deprecation warning if [`Time::nanosecond`] is
    /// non-zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let now = Time::now();
    /// let next = now.succ();
    /// assert_eq!(now.to_int() + 1, next.to_int());
    /// ```
    #[inline]
    #[must_use]
    pub fn succ(self) -> Self {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        Self {
            timestamp: timestamp + 1,
            sub_second_nanos,
            offset,
        }
    }

    /// Returns the difference between two `Time` objects as an `f64` of seconds.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn difference(self, other: Self) -> f64 {
        if let Some(sub_second_nanos) = self.sub_second_nanos.checked_sub(other.sub_second_nanos) {
            (self.timestamp - other.timestamp) as f64 + f64::from(sub_second_nanos) / f64::from(NANOS_IN_SECOND)
        } else {
            let sub_second_nanos = NANOS_IN_SECOND - (other.sub_second_nanos - self.sub_second_nanos);
            (self.timestamp - other.timestamp - 1) as f64 + f64::from(sub_second_nanos) / f64::from(NANOS_IN_SECOND)
        }
    }
}
