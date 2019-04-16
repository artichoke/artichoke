use mruby_sys::*;
use std::error;
use std::fmt;

use crate::value::{Ruby, Rust};

mod array;
mod boolean;
mod fixnum;
mod nilable;
mod string;

pub use self::array::*;
pub use self::boolean::*;
pub use self::fixnum::*;
pub use self::nilable::*;
pub use self::string::*;

pub type RubyToRustError = Error<Ruby, Rust>;
pub type RustToRubyError = Error<Rust, Ruby>;

pub trait TryRuby<From>
where
    Self: Sized,
{
    type RubyConvertError;

    fn try_ruby_convert(mrb: *mut mrb_state, value: From) -> Result<Self, Self::RubyConvertError>;
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

    #[test]
    fn ruby_to_rust_error_display() {
        let err: RubyToRustError = Error {
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
        let err: RubyToRustError = Error {
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
        let err: RustToRubyError = Error {
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
        let err: RustToRubyError = Error {
            from: Rust::Bool,
            to: Ruby::String,
        };
        assert_eq!(
            format!("{:?}", err),
            "mruby conversion error: failed to convert from rust bool to ruby String"
        );
    }
}
