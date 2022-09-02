//! Implementations of Ruby [`Time`], a timezone-aware datetime.
//!
//! This module contains implementations of a timestamp storage struct and
//! associated datetime operations that view that timestamp through the lens of
//! a timezone offset. These timestamps can be used to implement the Ruby `Time`
//! core class.
//!
//! There are several independent backends which can be selected by specifying
//! the appropriate feature:
//!
//! - `tzrs` is based on the [`tz-rs`] crate.
//!
//! Backends store datetimes as a `i64` [Unix timestamp], subsecond nanoseconds
//! as a `u32`, and a timezone offset which can be one of several types.
//!
//! [`Time`]: https://ruby-doc.org/core-3.1.2/Time.html
//! [`tzrs`]: https://crates.io/crates/tz-rs
//! [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time

/// A Time struct backed by the [`tz-rs`](tz) crate.
#[cfg(feature = "tzrs")]
pub mod tzrs;
