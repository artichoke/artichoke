use crate::time::chrono::{Offset, Time};

impl Time {
    /// Returns `true` if _time_ occurs during Daylight Saving Time in its time
    /// zone.
    ///
    /// # Implementation notes
    ///
    /// This function is not implemented and always returns `false`.
    #[inline]
    #[must_use]
    pub fn is_dst(self) -> bool {
        // TODO: compute whether a `Time` is daylight time.
        // `chrono` does not expose this.
        let _ = self;
        false
    }

    /// Returns `true` if time represents a time in UTC (GMT).
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let local_time = Time::now();
    /// assert!(!local_time.is_utc());
    /// let utc_time = local_time.to_utc();
    /// assert!(utc_time.is_utc());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_utc(self) -> bool {
        let Self { offset, .. } = self;
        matches!(offset, Offset::Utc)
    }

    /// Returns a new `Time` object representing time in local time (using the
    /// local time zone in effect for this process).
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let local_time = Time::now();
    /// assert!(!local_time.is_utc());
    ///
    /// let local_time2 = local_time.to_local();
    /// assert!(!local_time2.is_utc());
    ///
    /// let utc_time = local_time.to_utc();
    /// assert!(utc_time.is_utc());
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_local(self) -> Self {
        let Self {
            timestamp,
            sub_second_nanos,
            ..
        } = self;
        Self {
            timestamp,
            sub_second_nanos,
            offset: Offset::Local,
        }
    }

    /// Returns a new `Time` object representing time in UTC.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::chrono::Time;
    /// let local_time = Time::now();
    /// assert!(!local_time.is_utc());
    ///
    /// let utc_time = local_time.to_utc();
    /// assert!(utc_time.is_utc());
    ///
    /// let utc_time2 = utc_time.to_utc();
    /// assert!(utc_time2.is_utc());
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_utc(self) -> Self {
        let Self {
            timestamp,
            sub_second_nanos,
            ..
        } = self;
        Self {
            timestamp,
            sub_second_nanos,
            offset: Offset::Utc,
        }
    }

    /// Returns the name of the time zone used for _time_.
    #[inline]
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn timezone(self) -> Option<&'static str> {
        match self.offset {
            Offset::Utc => Some("UTC"),
            // TODO: Should be a zone string.
            Offset::Local => None,
            // TODO: Should be a zone string.
            Offset::Tz(_) => None,
            Offset::Fixed(_) => None,
        }
    }
}
