//! Timestamp offsets from UTC.

use chrono::{FixedOffset, Local, Utc};
use chrono_tz::Tz;

/// Timestamp offsets from UTC.
///
/// Spinoso time stores integer timestamps with an offset for performing
/// datetime operations.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Offset {
    /// UTC, zero offset, Zulu time.
    Utc,
    /// The local offset of the machine at runtime.
    Local,
    /// A timezone-based offset, defined by [Time Zone Database].
    ///
    /// Timezone offsets are named.
    ///
    /// [Time Zone Database]: https://www.iana.org/time-zones
    Tz(Tz),
    /// A fixed offset from UTC, like `-2:00` or `+10:45`.
    Fixed(FixedOffset),
}

impl From<Utc> for Offset {
    #[inline]
    fn from(_: Utc) -> Self {
        Self::Utc
    }
}

impl From<Local> for Offset {
    #[inline]
    fn from(_: Local) -> Self {
        Self::Local
    }
}

impl From<Tz> for Offset {
    #[inline]
    fn from(timezone: Tz) -> Self {
        Self::Tz(timezone)
    }
}

impl From<FixedOffset> for Offset {
    #[inline]
    fn from(offset: FixedOffset) -> Self {
        Self::Fixed(offset)
    }
}

impl PartialEq<Utc> for Offset {
    fn eq(&self, _: &Utc) -> bool {
        matches!(self, Self::Utc)
    }
}

impl PartialEq<Local> for Offset {
    fn eq(&self, _: &Local) -> bool {
        matches!(self, Self::Local)
    }
}

impl PartialEq<Tz> for Offset {
    fn eq(&self, other: &Tz) -> bool {
        matches!(self, Self::Tz(zone) if zone == other)
    }
}

impl PartialEq<FixedOffset> for Offset {
    fn eq(&self, other: &FixedOffset) -> bool {
        matches!(self, Self::Fixed(offset) if offset == other)
    }
}
