use tz::datetime::DateTime;

use super::{Offset, Result, Time, TimeErr};

// Timezone conversions (returns new Time)
impl Time {
    /// Returns a new Time object representing _time_ based on the provided
    /// offset.
    ///
    /// Can be used to implement [`Time#getlocal`] with a string/number
    /// parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let local_offset = Time::now().unwrap().utc_offset();
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0).unwrap();
    /// let now_local = now_utc.to_local().unwrap();
    /// assert_eq!(now_local.utc_offset(), local_offset);
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeErr`], might come as a result of an offset causing the `unix_time` to
    /// exceed `i64::MAX`
    ///
    /// [`Time#getlocal`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getlocal
    #[inline]
    pub fn to_offset(&self, offset: Offset) -> Result<Self> {
        Self::with_timespec_and_offset(self.inner.unix_time(), self.inner.nanoseconds(), offset)
    }

    /// Returns a new _time_ in UTC.
    ///
    /// Can be used to implement [`Time#getutc`] and [`Time#getgm`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now_local = Time::now().unwrap();
    /// let now_utc = now_local.to_utc().unwrap();
    /// assert_eq!(now_utc.utc_offset(), 0);
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeErr`], might come as a result of an offset causing the `unix_time` to
    /// exceed `i64::MAX`
    ///
    ///
    /// [`Time#getutc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getutc
    /// [`Time#getgm`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getgm
    #[inline]
    pub fn to_utc(&self) -> Result<Self> {
        self.to_offset(Offset::utc())
    }

    /// Returns a new Time object representing _time_ in local time (using the
    /// local time zone in effect for this process).
    ///
    /// Can be used to implement [`Time#getlocal`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let local_offset = Time::now().unwrap().utc_offset();
    /// let now_utc = Time::utc(2022, 7, 8, 12, 34, 56, 0).unwrap();
    /// let now_local = now_utc.to_local().unwrap();
    /// assert_eq!(now_local.utc_offset(), local_offset);
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeErr`], might come as a result of an offset causing the `unix_time` to
    /// exceed `i64::MAX`
    ///
    /// [`Time#getlocal`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-getlocal
    #[inline]
    pub fn to_local(&self) -> Result<Self> {
        self.to_offset(Offset::local())
    }
}

// Timezone mutations
impl Time {
    /// Converts _time_ to the provided time zone, modifying the receiver.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::{Time, Offset};
    /// let mut now = Time::utc(2022, 6, 8, 12, 0, 0, 0).unwrap();
    /// let gmt_plus_one = Offset::from(3600);
    /// now.set_offset(gmt_plus_one);
    /// assert_eq!(13, now.hour());
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeErr`], might come as a result of an offset causing the `unix_time` to
    /// exceed `i64::MAX`
    #[inline]
    pub fn set_offset(&mut self, offset: Offset) -> Result<()> {
        let time_zone_ref = offset.time_zone_ref();

        match self.inner.project(time_zone_ref) {
            Ok(time) => {
                self.inner = time;
                self.offset = offset;
                Ok(())
            }
            Err(error) => Err(TimeErr::from(error)),
        }
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
    /// let mut now = Time::utc(2022, 7, 8, 12, 34, 56, 0).unwrap();
    /// let now_utc_unix = now.to_int();
    /// now.set_local();
    /// let now_local_unix = now.to_int();
    /// assert_eq!(now_utc_unix, now_local_unix);
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeErr`], might come as a result of an offset causing the `unix_time` to
    /// exceed `i64::MAX`
    ///
    /// [`Time#localtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-localtime
    #[inline]
    pub fn set_local(&mut self) -> Result<()> {
        self.set_offset(Offset::local())
    }

    /// Converts _time_ to UTC (GMT), modifying the receiver.
    ///
    /// Can be used to implement [`Time#utc`] and [`Time#gmtime`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let mut now = Time::local(2022, 7, 8, 12, 34, 56, 0).unwrap();
    /// let now_local_unix = now.to_int();
    /// now.set_local();
    /// let now_utc_unix = now.to_int();
    /// assert_eq!(now_local_unix, now_utc_unix);
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeErr`], might come as a result of an offset causing the `unix_time` to
    /// exceed `i64::MAX`
    ///
    /// [`Time#utc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-utc
    /// [`Time#gmtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-gmtime
    #[inline]
    pub fn set_utc(&mut self) -> Result<()> {
        self.set_offset(Offset::utc())
    }

    /// Converts _time_ to the GMT time zone with the provided offset.
    ///
    /// Can be used to implement [`Time#localtime`] with an offset parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::{Time, Offset};
    /// let mut now = Time::utc(2022, 7, 8, 12, 34, 56, 0).unwrap();
    /// assert!(now.is_utc());
    /// let offset = Offset::from(3600);
    /// now.set_offset_from_utc(offset);
    /// assert!(!now.is_utc());
    /// assert_eq!(now.utc_offset(), 3600);
    /// ```
    ///
    /// # Errors
    ///
    /// Can produce a [`TimeErr`], might come as a result of an offset causing the `unix_time` to
    /// exceed `i64::MAX`
    ///
    /// [`Time#localtime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-localtime
    #[inline]
    pub fn set_offset_from_utc(&mut self, offset: Offset) -> Result<()> {
        let time_zone_ref = offset.time_zone_ref();

        self.inner = DateTime::from_timespec(self.to_int(), self.nanoseconds(), time_zone_ref)?;
        self.offset = offset;

        Ok(())
    }
}
