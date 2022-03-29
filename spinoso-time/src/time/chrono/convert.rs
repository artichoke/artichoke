//! [`From`] conversions to construct a [`Time`].

use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Tz;

use crate::time::chrono::{Offset, Time};

impl From<DateTime<Utc>> for Time {
    #[inline]
    fn from(time: DateTime<Utc>) -> Self {
        let offset = Offset::Utc;
        let timestamp = time.timestamp();
        let sub_second_nanos = time.timestamp_subsec_nanos();
        Self {
            timestamp,
            sub_second_nanos,
            offset,
        }
    }
}

impl From<DateTime<Local>> for Time {
    #[inline]
    fn from(time: DateTime<Local>) -> Self {
        let offset = Offset::Local;
        let timestamp = time.timestamp();
        let sub_second_nanos = time.timestamp_subsec_nanos();
        Self {
            timestamp,
            sub_second_nanos,
            offset,
        }
    }
}

impl From<DateTime<Tz>> for Time {
    #[inline]
    fn from(time: DateTime<Tz>) -> Self {
        let offset = Tz::from_offset(time.offset()).into();
        let timestamp = time.timestamp();
        let sub_second_nanos = time.timestamp_subsec_nanos();
        Self {
            timestamp,
            sub_second_nanos,
            offset,
        }
    }
}

impl From<DateTime<FixedOffset>> for Time {
    #[inline]
    fn from(time: DateTime<FixedOffset>) -> Self {
        let offset = FixedOffset::from_offset(time.offset()).into();
        let timestamp = time.timestamp();
        let sub_second_nanos = time.timestamp_subsec_nanos();
        Self {
            timestamp,
            sub_second_nanos,
            offset,
        }
    }
}

impl From<Time> for NaiveDateTime {
    #[inline]
    fn from(time: Time) -> Self {
        NaiveDateTime::from_timestamp(time.timestamp, time.sub_second_nanos)
    }
}

impl From<Time> for DateTime<Utc> {
    #[inline]
    fn from(time: Time) -> Self {
        Utc.timestamp(time.timestamp, time.sub_second_nanos)
    }
}

impl From<Time> for DateTime<Local> {
    #[inline]
    fn from(time: Time) -> Self {
        Local.timestamp(time.timestamp, time.sub_second_nanos)
    }
}

impl From<Time> for Duration {
    #[inline]
    fn from(time: Time) -> Self {
        let epoch = Utc.timestamp(0, 0);
        let utc = DateTime::<Utc>::from(time);
        utc.signed_duration_since(epoch)
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use chrono_tz::Tz;

    use crate::time::chrono::{Offset, Time};

    fn date() -> NaiveDateTime {
        // Artichoke's birthday, 2019-04-07T01:30Z + 0.2 seconds
        let time = NaiveDateTime::from_timestamp(1_554_600_621, 0);
        time.with_nanosecond(200_000_000).unwrap()
    }

    #[test]
    fn convert_utc() {
        let utc = Utc.from_utc_datetime(&date());
        let time = Time::from(utc);
        assert_eq!(time.timestamp, 1_554_600_621);
        assert_eq!(time.sub_second_nanos, 200_000_000);
        assert_eq!(time.offset, Offset::Utc);
    }

    #[test]
    fn convert_local() {
        let utc = Local.from_utc_datetime(&date());
        let time = Time::from(utc);
        assert_eq!(time.timestamp, 1_554_600_621);
        assert_eq!(time.sub_second_nanos, 200_000_000);
        assert_eq!(time.offset, Offset::Local);
    }

    #[test]
    fn convert_tz() {
        let utc = Tz::US__Pacific.from_utc_datetime(&date());
        let time = Time::from(utc);
        assert_eq!(time.timestamp, 1_554_600_621);
        assert_eq!(time.sub_second_nanos, 200_000_000);
        assert_eq!(time.offset, Offset::Tz(Tz::US__Pacific));
    }

    #[test]
    fn convert_fixed() {
        let hour = 3600;
        let utc = FixedOffset::west(7 * hour).from_utc_datetime(&date());
        let time = Time::from(utc);
        assert_eq!(time.timestamp, 1_554_600_621);
        assert_eq!(time.sub_second_nanos, 200_000_000);
        assert_eq!(time.offset, Offset::Fixed(FixedOffset::west(7 * hour)));
    }
}
