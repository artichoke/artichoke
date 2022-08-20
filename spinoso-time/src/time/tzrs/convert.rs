use core::fmt;

use super::{Time, ToA};

impl fmt::Display for Time {
    /// Returns a canonical string representation of _time_.
    ///
    /// `Display` uses the same format as [`Time::to_s`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let now = Time::utc(2022, 05, 26, 13, 16, 22, 0)?;
    /// assert_eq!(now.to_string(), "2022-05-26 13:16:22 UTC");
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// [`Time#asctime`]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-asctime
    /// [`Time#ctime`]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-ctime
    /// [`Time#to_s`]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-to_s
    /// [`Time#inspect`]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-inspect
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://github.com/ruby/ruby/blob/v3_1_2/time.c#L4007-L4017
        const UTC_FORMAT: &str = "%Y-%m-%d %H:%M:%S UTC";
        const FORMAT: &str = "%Y-%m-%d %H:%M:%S %z";

        if self.is_utc() {
            strftime::fmt::strftime(self, UTC_FORMAT, f).map_err(|_| fmt::Error)
        } else {
            strftime::fmt::strftime(self, FORMAT, f).map_err(|_| fmt::Error)
        }
    }
}

// Conversions
impl Time {
    /// Serialize a `Time` into its components as a [`ToA`].
    ///
    /// `ToA` stores a `Time` as a ten-element struct of time components: [sec,
    /// min, hour, day, month, year, wday, yday, isdst, zone].
    ///
    /// The ordering of the properties is important for the Ruby [`Time#to_a`]
    /// API.
    ///
    /// Can be used to implement [`Time#to_a`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{Time, TimeError};
    /// # fn example() -> Result<(), TimeError> {
    /// let now = Time::now()?;
    /// let to_array = now.to_array();
    /// assert_eq!(to_array.sec, now.second());
    /// assert_eq!(to_array.wday, now.day_of_week());
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// [`Time#to_a`]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-to_a
    #[inline]
    #[must_use]
    pub fn to_array(self) -> ToA {
        ToA::from(self)
    }
}
