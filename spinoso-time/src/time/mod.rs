//! Implementations of Ruby [`Time`], a timezone-aware datetime.
//!
//! This module contains implementations of a timestamp storage struct and
//! associated datetime operations that view that timestamp through the lens of
//! a timezone offset. These timestamps can be used to implement the Ruby `Time`
//! core class.
//!
//! [`Time`](self::chrono::Time) is based on the [`chrono`] crate.
//!
//! The chrono backend stores datetimes as a `i64` [Unix timestamp], subsecond
//! nanoseconds as a `u32`, and a [timezone offset] which can be one of several
//! types.
//!
//! [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
//! [`chrono`]: ::chrono
//! [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
//! [timezone offset]: self::chrono::Offset


#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "tzrs")]
pub mod tzrs;
