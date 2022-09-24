use core::fmt;

use super::{Time, ToA};

impl fmt::Display for Time {
    /// Returns a canonical string representation of _time_.
    ///
    /// `Display` uses the same format as [`Time#to_s`].
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
    /// [`Time#to_s`]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-to_s
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
    /// Formats _time_ according to the directives in the given format string.
    ///
    /// Can be used to implement [`Time#strftime`]. The resulting byte string
    /// will have the same encoding as the format byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_time::tzrs::{TimeError, Time};
    /// # #[derive(Debug)]
    /// # enum Error { Time(TimeError), Strftime(strftime::Error) };
    /// # impl From<TimeError> for Error { fn from(err: TimeError) -> Self { Self::Time(err) } }
    /// # impl From<strftime::Error> for Error { fn from(err: strftime::Error) -> Self { Self::Strftime(err) } }
    /// # fn example() -> Result<(), Error> {
    /// let now = Time::utc(2022, 05, 26, 13, 16, 22, 276)?;
    /// assert_eq!(
    ///     now.strftime("Today is %c 🎉".as_bytes())?,
    ///     "Today is Thu May 26 13:16:22 2022 🎉".as_bytes(),
    /// );
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// # Errors
    ///
    /// Can return [`strftime::Error`] if formatting fails. See
    /// [`strftime::bytes::strftime`] for more details.
    ///
    /// [`Time#strftime`]: https://ruby-doc.org/core-3.1.2/Time.html#method-i-strftime
    #[inline]
    pub fn strftime(&self, format: &[u8]) -> Result<Vec<u8>, strftime::Error> {
        // Requires ASCII-compatible encoding (which rules out things like
        // UTF-16). ASCII, Binary, and UTF-8 are considered ASCII-compatible.
        //
        // ```
        // [3.1.2] * Time.now.strftime("abc %c")
        // => "abc Sat Aug 20 12:18:56 2022"
        // [3.1.2] > Time.now.strftime("abc %c 📦")
        // => "abc Sat Aug 20 12:19:04 2022 📦"
        // [3.1.2] > Time.now.strftime("abc %c 📦 \xFF")
        // => "abc Sat Aug 20 12:19:12 2022 📦 \xFF"
        // [3.1.2] > Time.now.strftime("abc %c 📦 \xFF".encode(Encoding::UTF_16))
        // (irb):5:in `encode': "\xFF" on UTF-8 (Encoding::InvalidByteSequenceError)
        //         from (irb):5:in `<main>'
        //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
        // [3.1.2] > Time.now.strftime("abc %c 📦 \xFF".encode(Encoding::UTF_8))
        // => "abc Sat Aug 20 12:20:10 2022 📦 \xFF"
        // ```
        strftime::bytes::strftime(self, format)
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
