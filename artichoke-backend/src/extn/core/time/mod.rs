//! Time is an abstraction of dates and times.
//!
//! This module implements the [`Time`] class from Ruby Core.
//!
//! In Artichoke, Time is represented as a 64-bit signed integer of seconds
//! since January 1, 1970 UTC and an unsigned 32-bit integer of subsecond
//! nanoseconds. This allows representing roughly 584 billion years.
//!
//! You can use this class in your application by accessing it directly. As a
//! Core class, it is globally available:
//!
//! ```ruby
//! Time.now
//! ```
//!
//! This implementation of `Time` supports the system clock via the
//! [`chrono`] and [`chrono-tz`] crates.
//!
//! [`Time`]: https://ruby-doc.org/core-2.6.3/Time.html
//! [`chrono`]: https://crates.io/crates/chrono
//! [`chrono-tz`]: https://crates.io/crates/chrono-tz

use crate::convert::HeapAllocatedData;

pub mod mruby;
pub mod trampoline;

#[doc(inline)]
pub use spinoso_time::chrono::Time;

impl HeapAllocatedData for Time {
    const RUBY_TYPE: &'static str = "Time";
}
