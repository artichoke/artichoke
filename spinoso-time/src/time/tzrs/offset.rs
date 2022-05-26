use tz::timezone::{LocalTimeType, TimeZoneRef};
use tzdb::local_tz;
use tzdb::time_zone::etc::GMT;

/// tzdb provides [`local_tz`] to get the local system timezone. If this ever fails, we can
/// assume `GMT`. `GMT` is used instead of `UTC` since it has a [`time_zone_designation`] - which
/// if it is an empty string, then it is considered to be a UTC time.
///
/// Note: this matches MRI Ruby implmentation. Where `TZ="" ruby -e "puts Time::now"` will return a
/// new _time_ with 0 offset from UTC, but still still report as a non utc time.
///
/// [`local_tz`]: https://docs.rs/tzdb/latest/tzdb/fn.local_tz.html
/// [`time_zone_designation`]: https://docs.rs/tz-rs/0.6.9/tz/timezone/struct.LocalTimeType.html#method.time_zone_designation
#[inline]
#[must_use]
fn local_time_zone() -> TimeZoneRef<'static> {
    match local_tz() {
        Some(tz) => tz,
        None => GMT,
    }
}

/// Represents the number of seconds offset from UTC
#[allow(variant_size_differences)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Offset {
    /// UTC offset, zero offset, Zulu time
    Utc,
    /// Fixed offset from UTC
    ///
    /// Note: A fixed offset of 0 is different from UTC time
    Fixed([LocalTimeType; 1]),
    /// A time zone based offset
    Tz(TimeZoneRef<'static>),
}

impl Offset {
    /// Generate a UTC based offset
    pub fn utc() -> Self {
        Self::Utc
    }

    /// Generate an offset based on the detected local time zone of the system
    ///
    /// Detection is done by [`tzdb::local_tz`], and if it fails will return a GMT timezone
    ///
    /// [`tzdb::local_tz`]: https://docs.rs/tzdb/latest/tzdb/fn.local_tz.html
    pub fn local() -> Self {
        Self::Tz(local_time_zone())
    }

    /// Generate an offset with a number of seconds from UTC.
    pub fn fixed(offset: i32) -> Self {
        let local_time_type = LocalTimeType::new(offset, false, Some(b"GMT")).unwrap();
        Self::Fixed([local_time_type])
    }

    /// Generate an offset based on a provided [`tz::timezone::TimeZoneRef`]
    ///
    /// This can be combined with [`tzdb`] to generate offsets based on predefined iana time zones
    ///
    /// ```
    /// use spinoso_time::Offset;
    /// use tzdb::time_zone::pacific::AUCKLAND;
    /// let offset = Offset::tz(AUCKLAND);
    /// ```
    ///
    /// [`tz:timezone::TimeZoneRef`]: https://docs.rs/tz-rs/0.6.9/tz/timezone/struct.TimeZoneRef.html
    /// [`tzdb`]: https://docs.rs/tzdb/latest/tzdb/index.html
    pub fn tz(tz: TimeZoneRef<'static>) -> Self {
        Self::Tz(tz)
    }

    /// Returns a `TimeZoneRef` which can be used to generate and project _time_.
    pub fn time_zone_ref<'a>(&'a self) -> TimeZoneRef<'a> {
        match self {
            Self::Utc => TimeZoneRef::utc(),
            Self::Fixed(local_time_types) => match TimeZoneRef::new(&[], local_time_types, &[], &None) {
                Ok(tz) => tz,
                Err(_) => GMT,
            },

            Self::Tz(zone) => *zone,
        }
    }
}

impl From<&str> for Offset {
    /// Construct a Offset based on the [accepted MRI values]
    ///
    /// Accepts:
    ///
    /// - [+/-]HH[:]MM
    /// - A-I representing +01:00 to +09:00
    /// - K-M representing +10:00 to +12:00
    /// - N-Y representing -01:00 to -12:00
    /// - Z representing 0 offset
    ///
    /// [accepted MRI values]: https://ruby-doc.org/core-2.6.3/Time.html#method-c-new
    #[inline]
    #[must_use]
    fn from(_: &str) -> Self {
        todo!()
    }
}

impl From<TimeZoneRef<'static>> for Offset {
    /// Construct a Offset with the offset in second from UTC
    #[inline]
    #[must_use]
    fn from(tz: TimeZoneRef<'static>) -> Self {
        Self::tz(tz)
    }
}

impl From<i32> for Offset {
    /// Construct a Offset with the offset in second from UTC
    #[inline]
    #[must_use]
    fn from(seconds: i32) -> Self {
        Self::fixed(seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_not_utc() {
        let offset = Offset::from(0);
        assert!(matches!(offset, Offset::Fixed(_)));
    }

    //#[test]
    //fn z_is_utc() {
    // TODO: Z is a special case
    //let offset = Offset::from("Z");
    //assert_eq!("UTC", offset.to_string());
    //}
}
