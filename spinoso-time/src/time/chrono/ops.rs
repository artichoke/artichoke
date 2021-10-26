use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Tz;
use core::ops::{Add, Sub};
use std::time;

use crate::time::chrono::{Offset, Time};

impl Add<Duration> for Time {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                (aware + duration).into()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                (aware + duration).into()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                (aware + duration).into()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                (aware + duration).into()
            }
        }
    }
}

impl Add<time::Duration> for Time {
    type Output = Self;

    fn add(self, duration: time::Duration) -> Self::Output {
        let duration = Duration::from_std(duration).expect("Duration too large");
        self + duration
    }
}

impl Add<i8> for Time {
    type Output = Self;

    fn add(self, seconds: i8) -> Self::Output {
        self + i64::from(seconds)
    }
}

impl Add<u8> for Time {
    type Output = Self;

    fn add(self, seconds: u8) -> Self::Output {
        self + u64::from(seconds)
    }
}

impl Add<i16> for Time {
    type Output = Self;

    fn add(self, seconds: i16) -> Self::Output {
        self + i64::from(seconds)
    }
}

impl Add<u16> for Time {
    type Output = Self;

    fn add(self, seconds: u16) -> Self::Output {
        self + u64::from(seconds)
    }
}

impl Add<i32> for Time {
    type Output = Self;

    fn add(self, seconds: i32) -> Self::Output {
        self + i64::from(seconds)
    }
}

impl Add<u32> for Time {
    type Output = Self;

    fn add(self, seconds: u32) -> Self::Output {
        self + u64::from(seconds)
    }
}

impl Add<i64> for Time {
    type Output = Self;

    fn add(self, seconds: i64) -> Self::Output {
        let duration = if let Ok(seconds) = u64::try_from(seconds) {
            let duration = time::Duration::from_secs(seconds);
            Duration::from_std(duration).expect("Duration too large")
        } else {
            let seconds = seconds
                .checked_neg()
                .and_then(|secs| u64::try_from(secs).ok())
                .expect("Duration too large");
            let duration = time::Duration::from_secs(seconds);
            let duration = Duration::from_std(duration).expect("Duration too large");
            -duration
        };
        self + duration
    }
}

impl Add<u64> for Time {
    type Output = Self;

    fn add(self, seconds: u64) -> Self::Output {
        let duration = time::Duration::from_secs(seconds);
        let duration = Duration::from_std(duration).expect("Duration too large");
        self + duration
    }
}

impl Add<f32> for Time {
    type Output = Self;

    fn add(self, seconds: f32) -> Self::Output {
        let duration = if seconds > 0.0 {
            let duration = time::Duration::from_secs_f32(seconds);
            Duration::from_std(duration).expect("Duration too large")
        } else {
            let seconds = -seconds;
            let duration = time::Duration::from_secs_f32(seconds);
            Duration::from_std(duration).expect("Duration too large")
        };
        self + duration
    }
}

impl Add<f64> for Time {
    type Output = Self;

    fn add(self, seconds: f64) -> Self::Output {
        let duration = if seconds > 0.0 {
            let duration = time::Duration::from_secs_f64(seconds);
            Duration::from_std(duration).expect("Duration too large")
        } else {
            let seconds = -seconds;
            let duration = time::Duration::from_secs_f64(seconds);
            Duration::from_std(duration).expect("Duration too large")
        };
        self + duration
    }
}

impl Sub<Time> for Time {
    type Output = Self;

    fn sub(self, other: Time) -> Self::Output {
        let duration = Duration::from(other);
        self - duration
    }
}

impl Sub<Duration> for Time {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        let Self {
            timestamp,
            sub_second_nanos,
            offset,
        } = self;
        let naive = NaiveDateTime::from_timestamp(timestamp, sub_second_nanos);
        match offset {
            Offset::Utc => {
                let aware = DateTime::<Utc>::from_utc(naive, Utc);
                (aware - duration).into()
            }
            Offset::Local => {
                let offset = Local.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Local>::from_utc(naive, offset);
                (aware - duration).into()
            }
            Offset::Tz(timezone) => {
                let offset = timezone.offset_from_utc_datetime(&naive);
                let aware = DateTime::<Tz>::from_utc(naive, offset);
                (aware - duration).into()
            }
            Offset::Fixed(offset) => {
                let aware = DateTime::<FixedOffset>::from_utc(naive, offset);
                (aware - duration).into()
            }
        }
    }
}

impl Sub<time::Duration> for Time {
    type Output = Self;

    fn sub(self, duration: time::Duration) -> Self::Output {
        let duration = Duration::from_std(duration).expect("Duration too large");
        self - duration
    }
}

impl Sub<i8> for Time {
    type Output = Self;

    fn sub(self, seconds: i8) -> Self::Output {
        self - i64::from(seconds)
    }
}

impl Sub<u8> for Time {
    type Output = Self;

    fn sub(self, seconds: u8) -> Self::Output {
        self - u64::from(seconds)
    }
}

impl Sub<i16> for Time {
    type Output = Self;

    fn sub(self, seconds: i16) -> Self::Output {
        self - i64::from(seconds)
    }
}

impl Sub<u16> for Time {
    type Output = Self;

    fn sub(self, seconds: u16) -> Self::Output {
        self - u64::from(seconds)
    }
}

impl Sub<i32> for Time {
    type Output = Self;

    fn sub(self, seconds: i32) -> Self::Output {
        self - i64::from(seconds)
    }
}

impl Sub<u32> for Time {
    type Output = Self;

    fn sub(self, seconds: u32) -> Self::Output {
        self - u64::from(seconds)
    }
}

impl Sub<i64> for Time {
    type Output = Self;

    fn sub(self, seconds: i64) -> Self::Output {
        let duration = if let Ok(seconds) = u64::try_from(seconds) {
            let duration = time::Duration::from_secs(seconds);
            Duration::from_std(duration).expect("Duration too large")
        } else {
            let seconds = seconds
                .checked_neg()
                .and_then(|secs| u64::try_from(secs).ok())
                .expect("Duration too large");
            let duration = time::Duration::from_secs(seconds);
            let duration = Duration::from_std(duration).expect("Duration too large");
            -duration
        };
        self - duration
    }
}

impl Sub<u64> for Time {
    type Output = Self;

    fn sub(self, seconds: u64) -> Self::Output {
        let duration = time::Duration::from_secs(seconds);
        let duration = Duration::from_std(duration).expect("Duration too large");
        self - duration
    }
}

impl Sub<f32> for Time {
    type Output = Self;

    fn sub(self, seconds: f32) -> Self::Output {
        let duration = if seconds > 0.0 {
            let duration = time::Duration::from_secs_f32(seconds);
            Duration::from_std(duration).expect("Duration too large")
        } else {
            let seconds = -seconds;
            let duration = time::Duration::from_secs_f32(seconds);
            Duration::from_std(duration).expect("Duration too large")
        };
        self - duration
    }
}

impl Sub<f64> for Time {
    type Output = Self;

    fn sub(self, seconds: f64) -> Self::Output {
        let duration = if seconds > 0.0 {
            let duration = time::Duration::from_secs_f64(seconds);
            Duration::from_std(duration).expect("Duration too large")
        } else {
            let seconds = -seconds;
            let duration = time::Duration::from_secs_f64(seconds);
            Duration::from_std(duration).expect("Duration too large")
        };
        self - duration
    }
}

#[cfg(test)]
mod tests {
    use super::Time;
    use chrono::prelude::*;

    fn datetime() -> DateTime<Utc> {
        // halfway through a second
        let time = NaiveTime::from_hms_nano(23, 59, 59, 500_000_000);
        let date = NaiveDate::from_ymd(2019, 4, 7);
        let datetime = NaiveDateTime::new(date, time);
        DateTime::<Utc>::from_utc(datetime, Utc)
    }

    #[test]
    fn add_int_to_time() {
        let dt = Time::from(datetime());
        let succ: Time = dt + 1;
        assert_eq!(dt.timestamp + 1, succ.timestamp);
        assert_eq!(dt.to_int() + 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 0);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 500_000_000 {
            assert!(succ.nanosecond() - 500_000_000 < 50);
        } else {
            assert!(500_000_000 - succ.nanosecond() < 50);
        }
    }

    #[test]
    fn add_subsec_float_to_time() {
        let dt = Time::from(datetime());
        let succ: Time = dt + 0.2;
        assert_eq!(dt.timestamp, succ.timestamp);
        assert_eq!(dt.to_int(), succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 59);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 700_000_000 {
            assert!(succ.nanosecond() - 700_000_000 < 50);
        } else {
            assert!(700_000_000 - succ.nanosecond() < 50);
        }

        let dt = Time::from(datetime());
        let succ: Time = dt + 0.7;
        assert_eq!(dt.timestamp + 1, succ.timestamp);
        assert_eq!(dt.to_int() + 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 0);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 200_000_000 {
            assert!(succ.nanosecond() - 200_000_000 < 50);
        } else {
            assert!(200_000_000 - succ.nanosecond() < 50);
        }
    }

    #[test]
    fn add_float_to_time() {
        let dt = Time::from(datetime());
        let succ: Time = dt + 1.2;
        assert_eq!(dt.timestamp + 1, succ.timestamp);
        assert_eq!(dt.to_int() + 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 0);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 700_000_000 {
            assert!(succ.nanosecond() - 700_000_000 < 50);
        } else {
            assert!(700_000_000 - succ.nanosecond() < 50);
        }

        let dt = Time::from(datetime());
        let succ: Time = dt + 1.7;
        assert_eq!(dt.timestamp + 2, succ.timestamp);
        assert_eq!(dt.to_int() + 2, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_ne!(dt.day(), succ.day());
        assert_ne!(dt.hour(), succ.hour());
        assert_ne!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 1);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 200_000_000 {
            assert!(succ.nanosecond() - 200_000_000 < 50);
        } else {
            assert!(200_000_000 - succ.nanosecond() < 50);
        }
    }

    #[test]
    fn sub_int_to_time() {
        let dt = Time::from(datetime());
        let succ: Time = dt - 1;
        assert_eq!(dt.timestamp - 1, succ.timestamp);
        assert_eq!(dt.to_int() - 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 58);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 500_000_000 {
            assert!(succ.nanosecond() - 500_000_000 < 50);
        } else {
            assert!(500_000_000 - succ.nanosecond() < 50);
        }
    }

    #[test]
    fn sub_subsec_float_to_time() {
        let dt = Time::from(datetime());
        let succ: Time = dt - 0.2;
        assert_eq!(dt.timestamp, succ.timestamp);
        assert_eq!(dt.to_int(), succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 59);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 300_000_000 {
            assert!(succ.nanosecond() - 300_000_000 < 50);
        } else {
            assert!(300_000_000 - succ.nanosecond() < 50);
        }

        let dt = Time::from(datetime());
        let succ: Time = dt - 0.7;
        assert_eq!(dt.timestamp - 1, succ.timestamp);
        assert_eq!(dt.to_int() - 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 58);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 800_000_000 {
            assert!(succ.nanosecond() - 800_000_000 < 50);
        } else {
            assert!(800_000_000 - succ.nanosecond() < 50);
        }
    }

    #[test]
    fn sub_float_to_time() {
        let dt = Time::from(datetime());
        let succ: Time = dt - 1.2;
        assert_eq!(dt.timestamp - 1, succ.timestamp);
        assert_eq!(dt.to_int() - 1, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 58);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 300_000_000 {
            assert!(succ.nanosecond() - 300_000_000 < 50);
        } else {
            assert!(300_000_000 - succ.nanosecond() < 50);
        }

        let dt = Time::from(datetime());
        let succ: Time = dt - 1.7;
        assert_eq!(dt.timestamp - 2, succ.timestamp);
        assert_eq!(dt.to_int() - 2, succ.to_int());
        assert_eq!(dt.year(), succ.year());
        assert_eq!(dt.month(), succ.month());
        assert_eq!(dt.day(), succ.day());
        assert_eq!(dt.hour(), succ.hour());
        assert_eq!(dt.minute(), succ.minute());
        assert_eq!(succ.second(), 57);
        // handle inexactitude of float arithmetic
        if succ.nanosecond() > 800_000_000 {
            assert!(succ.nanosecond() - 800_000_000 < 50);
        } else {
            assert!(800_000_000 - succ.nanosecond() < 50);
        }
    }
}
