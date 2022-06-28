use core::fmt;
use std::error;

use tz::error::{DateTimeError, ProjectDateTimeError, TzError};

use super::offset::OffsetError;

/// A wrapper around some of the errors provided by `tz-rs`.
#[derive(Debug)]
pub enum TimeErr {
    /// Created when trying to create a DateTime, however the projection to a unix timestamp wasn't
    /// achieveable. Generally thrown when exceeding the range of integers (e.g. `> i64::Max`).
    ///
    /// Note: This is just a wrapper over [`tz::error::ProjectDateTimeError`].
    ProjectionError(ProjectDateTimeError),

    /// Created when one of the parameters of a Datetime falls outside the allowed ranges (e.g.
    /// 13th month, 32 day, 24th hour, etc)
    ///
    /// Note: [`tz::error::DateTimeError`] is only thrown from `tz-rs` when a provided component value is out of range.
    ///
    /// Note2: This is different from how MRI ruby is implemented. e.g. Second 60 is valid in MRI, and
    /// will just add an additional second instead of erroring.
    ComponentOutOfRangeError(DateTimeError),

    /// The provided time zone string cannot be used
    OffsetError(OffsetError),
}

impl error::Error for TimeErr {}

impl fmt::Display for TimeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeErr::ProjectionError(error) => error.fmt(f),
            TimeErr::ComponentOutOfRangeError(error) => error.fmt(f),
            TimeErr::OffsetError(error) => error.fmt(f),
        }
    }
}

impl From<ProjectDateTimeError> for TimeErr {
    fn from(err: ProjectDateTimeError) -> Self {
        Self::ProjectionError(err)
    }
}

impl From<DateTimeError> for TimeErr {
    fn from(err: DateTimeError) -> Self {
        Self::ComponentOutOfRangeError(err)
    }
}

impl From<OffsetError> for TimeErr {
    fn from(err: OffsetError) -> Self {
        Self::OffsetError(err)
    }
}

impl From<TzError> for TimeErr {
    fn from(error: TzError) -> Self {
        match error {
            // These two are generally recoverable within the usable of `spinoso_time`
            // TzError::DateTimeError(error) => Self::from(error),
            TzError::ProjectDateTimeError(error) => Self::from(error),

            // The rest will bleed through, but are included here for reference
            TzError::SystemTimeError(error) => panic!("{}", error), //Occurs when calling system clock
            TzError::TzFileError(error) => panic!("{}", error),     //Occurs during parsing of TZif files
            TzError::TzStringError(error) => panic!("{}", error), //Occurs during parsing of TZif files (POSIX string parsing)
            TzError::OutOfRangeError(error) => panic!("{}", error), //Occurs during int conversion (e.g. i64 => i32)
            TzError::LocalTimeTypeError(error) => panic!("{}", error), //Occurs during creation of TimeZoneRef
            TzError::TransitionRuleError(error) => panic!("{}", error), //Occurs during creation of TimeZoneRef
            TzError::TimeZoneError(error) => panic!("{}", error), //Occurs during creation of TimeZoneRef
            TzError::FindLocalTimeTypeError(error) => panic!("{}", error), //Wrapped by ProjectDateTimeError
            TzError::IoError(error) => panic!("{}", error),       //Never explicitly returned
            TzError::Utf8Error(error) => panic!("{}", error),     //Never explicity returned
            TzError::TryFromSliceError(error) => panic!("{}", error), //Never explicitly returned
            _ => panic!("Unhandled error"),
        }
    }
}
