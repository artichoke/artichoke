use core::fmt;
use std::error;

use tz::error::{DateTimeError, ProjectDateTimeError, TzError};

use super::offset::OffsetError;

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

    /// The provided time zone string cannot be used.
    OffsetError(OffsetError),

    /// A rescuable error originally from the tz-rs library.
    UnknownTzError(TzError),

    /// An rescuable unknown error (instead of panicing)
    Unknown,
}

impl error::Error for TimeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ProjectionError(ref err) => Some(err),
            Self::ComponentOutOfRangeError(ref err) => Some(err),
            Self::OffsetError(ref err) => Some(err),
            Self::UnknownTzError(ref err) => Some(err),
            Self::Unknown => None,
        }
    }
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProjectionError(error) => error.fmt(f),
            Self::ComponentOutOfRangeError(error) => error.fmt(f),
            Self::OffsetError(error) => error.fmt(f),
            Self::UnknownTzError(error) => error.fmt(f),
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

impl From<OffsetError> for TimeError {
    fn from(err: OffsetError) -> Self {
        Self::OffsetError(err)
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
