use core::fmt::{Display, Formatter, Result};

use super::{Time, ToA};

impl Display for Time {
    /// Returns a canonical string representation of _time_.
    ///
    /// Can be used to implement the Ruby method [`Time#asctime`],
    /// [`Time#ctime`], [`Time#to_s`], and [`Time#inspect`].
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
    /// Can be used to implement [`Time#strftime`][ruby-time-strftime].
    ///
    /// # Panics
    ///
    /// Panics on every invocation. Functionality is not implemented.
    ///
    /// [ruby-time-strftime]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-strftime
    #[inline]
    #[must_use]
    pub fn strftime(_: Self, _format: &str) -> String {
        todo!("Not implemented. See https://github.com/artichoke/artichoke/issues/1914")
    }

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
