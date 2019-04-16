use mruby_sys::*;
use std::fmt;

use crate::value::{Ruby, Rust};

mod array;
mod boolean;
mod fixnum;
mod nilable;
mod string;

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

impl<To, From> fmt::Debug for Error<To, From>
where
    To: fmt::Display,
    From: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to convert from {} to {}", self.from, self.to)
    }
}

type RubyToRustError = Error<Ruby, Rust>;
type RustToRubyError = Error<Rust, Ruby>;
