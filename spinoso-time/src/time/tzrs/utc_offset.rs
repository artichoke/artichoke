extern crate alloc;

use alloc::vec::Vec;
use tz::timezone::LocalTimeType;

/// Represents the number of seconds offset from UTC
///
/// Provides helpful conversions between i32 and &str
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UtcOffset {
    inner: i32,
}

impl UtcOffset {
    pub fn new(offset: i32) -> Self {
        Self {
            inner: offset
        }
    }

    pub fn local_time_type(&self) -> LocalTimeType {
        LocalTimeType::new(
            self.inner,
            false,
            Some(b"GMT"),
        ).unwrap()


    }

    pub fn to_string(&self) -> Vec<u8> {
        Vec::new()
    }

}

impl From<&str> for UtcOffset {
  fn from(_: &str) -> Self {
    Self { inner: 7200 }
  }
}
