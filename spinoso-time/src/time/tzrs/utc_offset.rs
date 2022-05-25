use tz::timezone::LocalTimeType;

/// Represents the number of seconds offset from UTC
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UtcOffset {
    inner: i32,
}

impl UtcOffset {
    /// Returns a new UtcOffset where offset is the number of seconds from UTC
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::UtcOffset;
    /// let offset = UtcOffset::new(3600);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(offset: i32) -> Self {
        Self { inner: offset }
    }

    /// Returns a tz-rs [`LocalTimeType`] which can be used to generate/project a new Datetime based on
    /// the offset in this struct
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::UtcOffset;
    /// let offset = UtcOffset::new(3600);
    /// let local_time_type = offset.local_time_type();
    /// assert_eq!("GMT", local_time_type.time_zone_designation());
    /// assert_eq!(3600, local_time_type.ut_offset());
    /// assert!(!local_time_type.is_dst());
    /// ```
    ///
    /// [`LocalTimeType`]: https://docs.rs/tz-rs/0.6.9/tz/timezone/struct.LocalTimeType.html
    #[inline]
    #[must_use]
    pub fn local_time_type(&self) -> LocalTimeType {
        LocalTimeType::new(self.inner, false, Some(b"GMT")).unwrap()
    }

    /// Returns the offset in [+/-]HH:MM format
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_time::UtcOffset;
    /// let offset = UtcOffset::new(3600);
    /// assert_eq!("+01:00", offset.to_string());
    /// ```
    #[inline]
    #[must_use]
    pub fn to_string(&self) -> String {
        let flag = if self.inner < 0 { '-' } else { '+' };
        let minutes = self.inner.abs() / 60;

        let offset_hours = minutes / 60;
        let offset_minutes = minutes - (offset_hours * 60);

        format!("{}{:0>2}:{:0>2}", flag, offset_hours, offset_minutes)
    }
}

impl From<&str> for UtcOffset {
    /// Construct a UtcOffset based on the [accepted MRI values]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_offset_formatting() {
        assert_eq!("-02:02", UtcOffset::new(-7320).to_string());
        assert_eq!("+00:00", UtcOffset::new(0).to_string());
        assert_eq!("+00:00", UtcOffset::new(59).to_string());
    }
}
