use core::fmt::{Display, Formatter, Result};

use super::{Time, ToA};

impl Display for Time {
    /// Returns a conanocial string representation of _time_
    ///
    /// Can be used to implement [`Time#asctime`], [`#Time#ctime`],
    /// [`Time#to_s`] and [`Time#inspect`]
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::tzrs::Time;
    /// let now = Time::utc(2022, 05, 26, 13, 16, 22, 0);
    /// assert_eq!(now.to_string(), "2022-05-26 13:16:22 UTC");
    /// ```
    ///
    /// [`Time#asctime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-asctime
    /// [`Time#ctime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-ctime
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: future
        //self.strftime("%Y-%m-%d %H:%M:%S %z")
        write!(
            f,
            "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2} {}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            self.time_zone()
        )
    }
}

// Conversions
impl Time {
    /// Formats _time_ according to the directives in the given format string.
    ///
    /// Can be used to implement [`Time#strftime`]
    ///
    /// [`Time#stftime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-strftime
    //#[inline]
    //#[must_use]
    //pub fn strftime(&self, _format: &str) -> String {
    //todo!()
    //}

    /// Serialize a `Time` into its components as a [`ToA`].
    ///
    /// `ToA` stores a `Time` as a ten-element struct of time components: [sec,
    /// min, hour, day, month, year, wday, yday, isdst, zone].
    ///
    /// The ordering of the properties is important for the Ruby [`Time#to_a`]
    /// API, and is accessible with the [`ToA::to_tuple`] method.
    ///
    /// Can be used to implement [`Time#to_a`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::Time;
    /// let now = Time::now();
    /// let to_array = now.to_array();
    /// assert_eq!(to_array.sec, now.second());
    /// assert_eq!(to_array.wday, now.day_of_week());
    /// ```
    ///
    /// [`Time#to_a`]: https://ruby-doc.org/core-2.6.3/Time.html#method-i-to_a
    #[inline]
    #[must_use]
    pub fn to_array(self) -> ToA {
        ToA::from(self)
    }
}
