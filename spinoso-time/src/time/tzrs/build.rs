use super::Offset;
use super::Time;

impl Time {
    /// Returns a Time based on the provided values in the local timezone
    ///
    /// Can be used to implement ruby [`Time#local`], [`Time#mktime`]
    ///
    /// [`Time#local`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-local
    /// [`Time#mktime`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-mktime
    #[inline]
    #[must_use]
    pub fn local(year: i32, month: u8, month_day: u8, hour: u8, minute: u8, second: u8, nanoseconds: u32) -> Self {
        Time::new(
            year,
            month,
            month_day,
            hour,
            minute,
            second,
            nanoseconds,
            Offset::local(),
        )
    }

    /// Returns a Time based on the provided values in UTC
    ///
    /// Can be used to implement ruby [`Time#utc`], [`Time#gm`]
    ///
    /// [`Time#utc`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-utc
    /// [`Time#gm`]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-gm
    #[inline]
    #[must_use]
    pub fn utc(year: i32, month: u8, month_day: u8, hour: u8, minute: u8, second: u8, nanoseconds: u32) -> Self {
        Time::new(year, month, month_day, hour, minute, second, nanoseconds, Offset::utc())
    }
}
