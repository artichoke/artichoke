#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Time is an abstraction of dates and times.
//!
//! This module implements the [`Time`] class from Ruby Core.
//!
//! In Artichoke, Time is represented as a 64-bit signed integer of seconds
//! since January 1, 1970 UTC (the Unix Epoch) and an unsigned 32-bit integer of
//! subsecond nanoseconds. This allows representing roughly 584 billion years.
//!
//! You can use this class in your application by accessing it directly. As a
//! Core class, it is globally available:
//!
//! ```ruby
//! Time.now
//! ```
//!
//! This implementation of `Time` is dependant on the selected feature. The `chrono` feature uses the [`chrono`] crate, and the `tzrs` feature uses the [`tzdb`] for getting the local timezone information, and combines with the [`tz-rs`] crate to generate the time.
//!
//! # Crate features
//!
//! This crate supports two backends which are mutually exclusive to each other. These backends can
//! be selected using the following features:
//!
//! - `chrono` which is backed by the [`chrono`] crate
//! - `tzrs` which is backed by the [`tz-rs`] crate
//!
//! This crate requires [`std`], the Rust Standard Library.
//!
//! [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
//! [`chrono`]: https://crates.io/crates/chrono
//! [`tz-rs`]: https://crates.io/crates/tz-rs
//! [`tzdb`]: https://crates.io/crates/tzdb

// Ensure code blocks in `README.md` compile
#[cfg(all(doctest, feature = "chrono"))]
#[doc = include_str!("../README.md")]
mod readme {}

use core::time::Duration;

mod time;

#[cfg(feature = "chrono")]
pub use time::chrono;
#[cfg(feature = "tzrs")]
pub use time::tzrs;

/// Number of nanoseconds in one second.
#[allow(clippy::cast_possible_truncation)] // 1e9 < u32::MAX
pub const NANOS_IN_SECOND: u32 = Duration::from_secs(1).as_nanos() as u32;

/// Number of microseconds in one nanosecond.
#[allow(clippy::cast_possible_truncation)] // 1000 < u32::MAX
pub const MICROS_IN_NANO: u32 = Duration::from_micros(1).as_nanos() as u32;
