use core::fmt;
use core::num::TryFromIntError;
use std::error;
use std::str::Utf8Error;

use tz::error::{DateTimeError, ProjectDateTimeError, TzError};

/// A wrapper around some of the errors provided by `tz-rs`.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum TimeError {
    /// Created when trying to create a DateTime, however the projection to a
    /// unix timestamp wasn't achieveable. Generally thrown when exceeding the
    /// range of integers (e.g. `> i64::Max`).
    ///
    /// Note: This is just a wrapper over [`tz::error::ProjectDateTimeError`].
    ProjectionError(ProjectDateTimeError),

    /// Created when one of the parameters of a Datetime falls outside the
    /// allowed ranges (e.g. 13th month, 32 day, 24th hour, etc).
    ///
    /// Note: [`tz::error::DateTimeError`] is only thrown from `tz-rs` when a
    /// provided component value is out of range.
    ///
    /// Note2: This is different from how MRI ruby is implemented. e.g. Second
    /// 60 is valid in MRI, and will just add an additional second instead of
    /// erroring.
    ComponentOutOfRangeError(DateTimeError),

    /// A rescuable error originally from the tz-rs library.
    UnknownTzError(TzError),

    /// Indicates that there was an issue when parsing a string for an offset.
    TzStringError(TzStringError),

    /// The provided tz offset seconds offset is outside of the allowed range.
    TzOutOfRangeError(TzOutOfRangeError),

    /// A rescuable Integer overflow error. Caused when trying to exceed the
    /// bounds of an int.
    IntOverflowError(IntOverflowError),

    /// An rescuable unknown error (instead of panicing)
    Unknown,
}

impl PartialEq for TimeError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::ProjectionError(_) => matches!(other, TimeError::ProjectionError(_)),
            Self::ComponentOutOfRangeError(_) => matches!(other, TimeError::ComponentOutOfRangeError(_)),
            Self::UnknownTzError(_) => matches!(other, TimeError::UnknownTzError(_)),
            Self::TzStringError(_) => matches!(other, TimeError::TzStringError(_)),
            Self::TzOutOfRangeError(_) => matches!(other, TimeError::TzOutOfRangeError(_)),
            Self::IntOverflowError(_) => matches!(other, TimeError::IntOverflowError(_)),
            Self::Unknown => matches!(other, TimeError::Unknown),
        }
    }
}

impl error::Error for TimeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ProjectionError(ref err) => Some(err),
            Self::ComponentOutOfRangeError(ref err) => Some(err),
            Self::UnknownTzError(ref err) => Some(err),
            Self::TzStringError(ref err) => Some(err),
            Self::TzOutOfRangeError(ref err) => Some(err),
            Self::IntOverflowError(ref err) => Some(err),
            Self::Unknown => None,
        }
    }
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProjectionError(error) => error.fmt(f),
            Self::ComponentOutOfRangeError(error) => error.fmt(f),
            Self::UnknownTzError(error) => error.fmt(f),
            Self::TzStringError(ref error) => error.fmt(f),
            Self::TzOutOfRangeError(ref error) => error.fmt(f),
            Self::IntOverflowError(error) => error.fmt(f),
            Self::Unknown => write!(f, "An unknown error occurred"),
        }
    }
}

impl From<ProjectDateTimeError> for TimeError {
    fn from(err: ProjectDateTimeError) -> Self {
        Self::ProjectionError(err)
    }
}

impl From<DateTimeError> for TimeError {
    fn from(err: DateTimeError) -> Self {
        Self::ComponentOutOfRangeError(err)
    }
}

impl From<TzError> for TimeError {
    fn from(error: TzError) -> Self {
        // Allowing matching arms due to documentation
        #[allow(clippy::match_same_arms)]
        match error {
            // These two are generally recoverable within the usable of `spinoso_time`
            // TzError::DateTimeError(error) => Self::from(error),
            TzError::ProjectDateTimeError(error) => Self::from(error),

            // The rest will bleed through, but are included here for reference
            // Occurs when calling system clock
            TzError::SystemTimeError(_) => Self::UnknownTzError(error),
            // Occurs during parsing of TZif files
            TzError::TzFileError(_) => Self::UnknownTzError(error),
            // Occurs during parsing of TZif files (POSIX string parsing)
            TzError::TzStringError(_) => Self::UnknownTzError(error),
            // Occurs during int conversion (e.g. i64 => i32)
            TzError::OutOfRangeError(_) => Self::UnknownTzError(error),
            // Occurs during creation of TimeZoneRef
            TzError::LocalTimeTypeError(_) => Self::UnknownTzError(error),
            // Occurs during creation of TimeZoneRef
            TzError::TransitionRuleError(_) => Self::UnknownTzError(error),
            // Occurs during creation of TimeZoneRef
            TzError::TimeZoneError(_) => Self::UnknownTzError(error),
            // Wrapped by ProjectDateTimeError
            TzError::FindLocalTimeTypeError(_) => Self::UnknownTzError(error),
            // Never explicitly returned
            TzError::IoError(_) => Self::UnknownTzError(error),
            // Never explicity returned
            TzError::Utf8Error(_) => Self::UnknownTzError(error),
            // Never explicitly returned
            TzError::TryFromSliceError(_) => Self::UnknownTzError(error),
            // TzError is non_exhaustive, so the rest are caught as Unknown
            _ => Self::Unknown,
        }
    }
}

impl From<TzStringError> for TimeError {
    fn from(err: TzStringError) -> Self {
        Self::TzStringError(err)
    }
}

impl From<TzOutOfRangeError> for TimeError {
    fn from(err: TzOutOfRangeError) -> Self {
        Self::TzOutOfRangeError(err)
    }
}

impl From<IntOverflowError> for TimeError {
    fn from(error: IntOverflowError) -> Self {
        Self::IntOverflowError(error)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TzStringError;

impl fmt::Display for TzStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "+HH:MM\", \"-HH:MM\", \"UTC\" or \"A\"..\"I\",\"K\"..\"Z\" expected for utc_offset"
        )
    }
}
impl error::Error for TzStringError {}

impl From<Utf8Error> for TzStringError {
    fn from(_: Utf8Error) -> Self {
        TzStringError
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TzOutOfRangeError;

impl fmt::Display for TzOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "utc_offset out of range")
    }
}

impl error::Error for TzOutOfRangeError {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntOverflowError;

impl fmt::Display for IntOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "out of int range")
    }
}

impl error::Error for IntOverflowError {}

impl From<TryFromIntError> for TimeError {
    fn from(_: TryFromIntError) -> Self {
        Self::IntOverflowError(IntOverflowError)
    }
}