use tz::datetime::DateTime;

use super::{Offset, Time};

// Timezone conversions (returns new Time)
impl Time {
    /// Returns a new Time object representing _time_ based on the provided
    /// offset.
    ///
    /// Can be used to implement [`Time#getlocal`] with a string/number
    /// parameter.
    ///
    /// #Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let local_offset = Time::now().utc_offset();
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let now_local = now_utc.to_local();
    /// assert_eq!(now_local.utc_offset(), local_offset);
    /// ```
    ///
    /// [`Time#getlocal`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getlocal
    #[inline]
    pub fn to_offset(&self, offset: Offset) -> Self {
        Self::with_timespec_and_offset(self.inner.unix_time(), self.inner.nanoseconds(), offset)
    }

    /// Returns a new _time_ in UTC.
    ///
    /// Can be used to implement [`Time#getutc`] and [`Time#getgm`].
    ///
    /// #Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now_local = Time::now();
    /// let now_utc = now_local.to_utc();
    /// assert_eq!(now_utc.utc_offset(), 0);
    /// ```
    ///
    /// [`Time#getutc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getutc
    /// [`Time#getgm`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getgm
    #[inline]
    pub fn to_utc(&self) -> Self {
        self.to_offset(Offset::utc())
    }

    /// Returns a new Time object representing _time_ in local time (using the
    /// local time zone in effect for this process).
    ///
    /// Can be used to implement [`Time#getlocal`].
    ///
    /// #Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let local_offset = Time::now().utc_offset();
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let now_local = now_utc.to_local();
    /// assert_eq!(now_local.utc_offset(), local_offset);
    /// ```
    ///
    /// [`Time#getlocal`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getlocal
    #[inline]
    pub fn to_local(&self) -> Self {
        self.to_offset(Offset::local())
    }
}

// Timezone mutations
impl Time {
    /// Converts _time_ to the provided time zone, modifying the receiver.
    ///
    /// # Examples
    /// TODO
    #[inline]
    pub fn set_offset(&mut self, offset: Offset) {
        // TODO: ProjectionErrors from project() are propogated from `Time::from_timespec` which
        // generally come from an error on checked_add overflowing the seconds component of the
        // unix time. Need to decide how to handle these kinds of errors (e.g. panic?)
        let time_zone_ref = offset.time_zone_ref();
        if let Ok(time) = self.inner.project(time_zone_ref) {
            self.inner = time;
        }
        self.offset = offset;
    }

    /// Converts _time_ to local time (using the local time zone in effective at the creation time
    /// of _time_) modifying the receiver.
    ///
    /// Can be used to implement [`Time#localtime`] without a parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let mut now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// let now_utc_unix = now.to_int();
    /// now.set_local();
    /// let now_local_unix = now.to_int();
    /// assert_eq!(now_utc_unix, now_local_unix);
    /// ```
    ///
    /// [`Time#localtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-localtime
    #[inline]
    pub fn set_local(&mut self) {
        self.set_offset(Offset::local());
    }

    /// Converts _time_ to UTC (GMT), modifying the receiver.
    ///
    /// Can be used to implement [`Time#utc`] and [`Time#gmtime`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let mut now = Time::local(2022, 7, 8, 12, 34, 56, 0);
    /// let now_local_unix = now.to_int();
    /// now.set_local();
    /// let now_utc_unix = now.to_int();
    /// assert_eq!(now_local_unix, now_utc_unix);
    /// ```
    ///
    /// [`Time#utc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-utc
    /// [`Time#gmtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-gmtime
    #[inline]
    pub fn set_utc(&mut self) {
        self.set_offset(Offset::utc());
    }

    /// Converts _time_ to the GMT time zone with the provided offset.
    ///
    /// Can be used to implement [`Time#localtime`] with an offset parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::{Time, Offset};
    /// let mut now = Time::utc(2022, 7, 8, 12, 34, 56, 0);
    /// assert!(now.is_utc());
    /// let offset = Offset::from(3600);
    /// now.set_offset_from_utc(offset);
    /// assert!(!now.is_utc());
    /// assert_eq!(now.utc_offset(), 3600);
    /// ```
    ///
    /// [`Time#localtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-localtime
    #[inline]
    pub fn set_offset_from_utc(&mut self, offset: Offset) {
        let time_zone_ref = offset.time_zone_ref();
        self.inner = DateTime::from_timespec(self.to_int(), self.nanoseconds(), time_zone_ref)
            .expect("Could not find a matching DateTime");
        self.offset = offset;
    }
}
