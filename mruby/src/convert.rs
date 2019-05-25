use std::error;
use std::fmt;

use crate::interpreter::Mrb;

mod array;
mod boolean;
mod bytes;
mod fixnum;
mod float;
mod hash;
mod nilable;
mod string;

pub use self::array::*;
pub use self::boolean::*;
pub use self::bytes::*;
pub use self::fixnum::*;
pub use self::float::*;
pub use self::hash::*;
pub use self::nilable::*;
pub use self::string::*;

pub trait FromMrb<T> {
    type From;
    type To;

    fn from_mrb(interp: &Mrb, value: T) -> Self;
}

pub trait TryFromMrb<T>
where
    Self: Sized,
{
    type From;
    type To;

    unsafe fn try_from_mrb(interp: &Mrb, value: T) -> Result<Self, Error<Self::From, Self::To>>;
}

/// Provide a falible converter for types that implement an infallible
/// conversion.
// Lint disabled because the suggestion does not compile.
// See: https://github.com/rust-lang/rust-clippy/issues/4140
#[allow(clippy::use_self)]
impl<From, To> TryFromMrb<From> for To
where
    To: FromMrb<From>,
{
    type From = To::From;
    type To = To::To;

    unsafe fn try_from_mrb(interp: &Mrb, value: From) -> Result<Self, Error<Self::From, Self::To>> {
        Ok(FromMrb::from_mrb(interp, value))
    }
}

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
        write!(f, "mruby conversion error: {}", self)
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

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::Error;
    use crate::value::types::*;

    #[test]
    fn ruby_to_rust_error_display() {
        let err = Error {
            from: Ruby::Fixnum,
            to: Rust::Vec,
        };
        assert_eq!(
            format!("{}", err),
            "failed to convert from ruby Fixnum to rust Vec"
        );
    }

    #[test]
    fn ruby_to_rust_error_debug() {
        let err = Error {
            from: Ruby::Fixnum,
            to: Rust::Vec,
        };
        assert_eq!(
            format!("{:?}", err),
            "mruby conversion error: failed to convert from ruby Fixnum to rust Vec"
        );
    }

    #[test]
    fn rust_to_ruby_error_display() {
        let err = Error {
            from: Rust::Bool,
            to: Ruby::String,
        };
        assert_eq!(
            format!("{}", err),
            "failed to convert from rust bool to ruby String"
        );
    }

    #[test]
    fn rust_to_ruby_error_debug() {
        let err = Error {
            from: Rust::Bool,
            to: Ruby::String,
        };
        assert_eq!(
            format!("{:?}", err),
            "mruby conversion error: failed to convert from rust bool to ruby String"
        );
    }
}
