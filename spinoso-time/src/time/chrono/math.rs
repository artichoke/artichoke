use crate::time::chrono::Time;

impl Time {
    /// Returns a new Time object, one second later than time.
    ///
    /// This method should log a deprecation warning if [`Time::nanosecond`] is
    /// non-zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::Time;
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
}
