//! Implementations of Ruby [`Time`], a timezone-aware datetime.
//!
//! These modules contain implementations of a timestamp storage struct and
//! associated datetime operations that view that timestamp through the lens of
//! a timezone offset. These timestamps can be used to implement the Ruby `Time`
//! core class.
//!
//! There are two independent backends which can be selected by specifying the
//! `chrono` or `tzrs` feature.
//!
//! `chrono` is based on the [`chrono`] crate.
//! `tzrs` is based on the [`tz-rs`] crate.
//!
//! Both backends store datetimes as a `i64` [Unix timestamp], subsecond
//! nanoseconds as a `u32`, and a timezone offset which can be one of several
//! types.
//!
//! [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
//! [`chrono`]: https://crates.io/crates/chrono
//! [`tzrs`]: https://crates.io/crates/tz-rs
//! [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time

/// A Time struct backed by the `chrono` rust crate
#[cfg(feature = "chrono")]
pub mod chrono;

/// A Time struct backed by the `tz-rs` rust crate
#[cfg(feature = "tzrs")]
pub mod tzrs;
