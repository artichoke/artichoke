//! Convert between Rust and Ruby objects.

use std::error;
use std::fmt;

/// Infallible conversion between two types.
///
/// See [`std::convert::From`].
pub trait Convert {
    /// Concrete type for the interpreter.
    type Artichoke;

    /// Concrete type for source of the conversion.
    type From;

    /// Concrete type for an error this conversion may produce.
    ///
    /// `Error` is required for infallible conversions so they can be wrapped
    /// by a [fallible conversion](TryConvert) blanked implementation.
    type Error;

    /// Performs the infallible conversion.
    fn convert(interp: Self::Artichoke, value: Self::From) -> Self;
}

/// Fallible conversions between two types.
///
/// See [`std::convert::TryFrom`].
#[allow(clippy::module_name_repetitions)]
pub trait TryConvert
where
    Self: Sized,
{
    /// Concrete type for the interpreter.
    type Artichoke;

    /// Concrete type for source of the conversion.
    type From;

    /// Concrete type for an error this conversion may produce.
    ///
    /// `Error` is required for infallible conversions so they can be wrapped
    /// by a [fallible conversion](TryConvert) blanked implementation.
    type Error;

    /// Performs the fallible conversion.
    unsafe fn try_convert(interp: Self::Artichoke, value: Self::From) -> Result<Self, Self::Error>;
}

/// Provide a fallible converter for types that implement an infallible
/// conversion.
impl<T> TryConvert for T
where
    T: Convert,
{
    type Artichoke = <Self as Convert>::Artichoke;
    type From = <Self as Convert>::From;
    type Error = <Self as Convert>::Error;

    unsafe fn try_convert(interp: Self::Artichoke, value: Self::From) -> Result<Self, Self::Error> {
        Ok(Convert::convert(interp, value))
    }
}

/// Error for a failed conversion from `From` to `To`.
#[derive(Clone, Eq, PartialEq)]
pub struct Error<From, To> {
    from: From,
    to: To,
}

impl<From, To> fmt::Display for Error<From, To>
where
    From: fmt::Display,
    To: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to convert from {} to {}", self.from, self.to)
    }
}

impl<From, To> fmt::Debug for Error<From, To>
where
    From: fmt::Display,
    To: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<To, From> error::Error for Error<To, From>
where
    From: fmt::Display,
    To: fmt::Display,
{
    fn description(&self) -> &str {
        "Failed to convert types between ruby and rust"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
