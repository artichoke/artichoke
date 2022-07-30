use super::{Offset, Result, Time};

impl Time {
    /// Returns a Time based on the provided values in the local timezone.
    ///
    /// Can be used to implement Ruby [`Time#local`], [`Time#mktime`].
    ///
    /// # Errors
    ///
    /// Can produce a [`super::TimeError`], generally when provided values are out of range
    ///
    /// [`Time#local`]: https://ruby-doc.org/core-3.1.2/Time.html#method-c-local
    /// [`Time#mktime`]: https://ruby-doc.org/core-3.1.2/Time.html#method-c-mktime
    #[inline]
    pub fn local(
        year: i32,
        month: u8,
        month_day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanoseconds: u32,
    ) -> Result<Self> {
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

    /// Returns a Time based on the provided values in UTC.
    ///
    /// Can be used to implement Ruby [`Time#utc`], [`Time#gm`].
    ///
    /// # Errors
    ///
    /// Can produce a [`super::TimeError`], generally when provided values are out of range
    ///
    /// [`Time#utc`]: https://ruby-doc.org/core-3.1.2/Time.html#method-c-utc
    /// [`Time#gm`]: https://ruby-doc.org/core-3.1.2/Time.html#method-c-gm
    #[inline]
    pub fn utc(
        year: i32,
        month: u8,
        month_day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanoseconds: u32,
    ) -> Result<Self> {
        Time::new(year, month, month_day, hour, minute, second, nanoseconds, Offset::utc())
    }
}
