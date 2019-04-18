use mruby_sys::*;
use std::error;
use std::fmt;

#[macro_use]
pub mod macros;

mod array;
mod boolean;
mod bytes;
mod fixnum;
mod nilable;
mod string;

pub use self::array::*;
pub use self::boolean::*;
pub use self::bytes::*;
pub use self::fixnum::*;
pub use self::nilable::*;
pub use self::string::*;

pub trait TryFromMrb<T>
where
    Self: Sized,
{
    type From;
    type To;

    fn try_from_mrb(mrb: *mut mrb_state, value: T) -> Result<Self, Error<Self::From, Self::To>>;
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
    use super::*;
    use crate::value::*;

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
