use core::fmt;
use std::error;
use std::str::Utf8Error;

/// A Unified Error type from construction and parsing for [`super::Offset`]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetError {
    /// Indicates that there was an issue when parsing a string for an offset.
    TzStringError(TzStringError),
    /// Indicates that a provided values was out of range (e.g. number of
    /// seconds for a fixed offset).
    OutOfRangeError(OutOfRangeError),
}

impl error::Error for OffsetError {}

impl fmt::Display for OffsetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffsetError::TzStringError(error) => error.fmt(f),
            OffsetError::OutOfRangeError(error) => error.fmt(f),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TzStringError;

impl fmt::Display for TzStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "expected [+/-]HH[:]MM, A..K, J..Z for utc_offset")
    }
}
impl error::Error for TzStringError {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "utc_offset out of range")
    }
}

impl error::Error for OutOfRangeError {}

impl From<TzStringError> for OffsetError {
    fn from(error: TzStringError) -> Self {
        Self::TzStringError(error)
    }
}

impl From<OutOfRangeError> for OffsetError {
    fn from(error: OutOfRangeError) -> Self {
        Self::OutOfRangeError(error)
    }
}

impl From<Utf8Error> for OffsetError {
    fn from(_: Utf8Error) -> Self {
        Self::TzStringError(TzStringError)
    }
}
