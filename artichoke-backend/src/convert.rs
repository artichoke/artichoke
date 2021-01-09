use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::core::{ClassRegistry, Convert, ConvertMut, TryConvert, TryConvertMut, Value as _};
use crate::error::{Error, RubyException};
use crate::extn::core::exception::TypeError;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

mod array;
mod boolean;
mod boxing;
mod bytes;
mod fixnum;
mod float;
mod hash;
mod implicit;
mod nilable;
mod string;

pub use boxing::{BoxUnboxVmValue, HeapAllocated, HeapAllocatedData, Immediate, UnboxedValueGuard};
pub use implicit::{
    implicitly_convert_to_array, implicitly_convert_to_int, implicitly_convert_to_nilable_string,
    implicitly_convert_to_string,
};

/// Provide a fallible converter for types that implement an infallible
/// conversion.
impl<T, U> TryConvert<T, U> for Artichoke
where
    Artichoke: Convert<T, U>,
{
    // TODO: this should be the never type.
    // https://github.com/rust-lang/rust/issues/35121
    type Error = Error;

    /// Blanket implementation that always succeeds by delegating to
    /// [`Convert::convert`].
    #[inline]
    fn try_convert(&self, value: T) -> Result<U, Self::Error> {
        Ok(Convert::convert(self, value))
    }
}

/// Provide a mutable fallible converter for types that implement an infallible
/// conversion.
impl<T, U> TryConvertMut<T, U> for Artichoke
where
    Artichoke: ConvertMut<T, U>,
{
    // TODO: this should be the never type.
    // https://github.com/rust-lang/rust/issues/35121
    type Error = Error;

    /// Blanket implementation that always succeeds by delegating to
    /// [`Convert::convert`].
    #[inline]
    fn try_convert_mut(&mut self, value: T) -> Result<U, Self::Error> {
        Ok(ConvertMut::convert_mut(self, value))
    }
}

/// Failed to convert from boxed Ruby value to a Rust type.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnboxRubyError {
    pub from: Ruby,
    pub into: Rust,
}

impl UnboxRubyError {
    #[must_use]
    #[inline]
    pub fn new(value: &Value, into: Rust) -> Self {
        Self {
            from: value.ruby_type(),
            into,
        }
    }
}

impl fmt::Display for UnboxRubyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to convert from {} to {}", self.from, self.into)
    }
}

impl error::Error for UnboxRubyError {}

impl RubyException for UnboxRubyError {
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"Failed to convert from Ruby value to Rust type")
    }

    fn name(&self) -> Cow<'_, str> {
        "TypeError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.to_string()).ok()?;
        let value = interp.new_instance::<TypeError>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<UnboxRubyError> for Error {
    fn from(exception: UnboxRubyError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<UnboxRubyError>> for Error {
    fn from(exception: Box<UnboxRubyError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<UnboxRubyError> for Box<dyn RubyException> {
    fn from(exception: UnboxRubyError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<UnboxRubyError>> for Box<dyn RubyException> {
    fn from(exception: Box<UnboxRubyError>) -> Box<dyn RubyException> {
        exception
    }
}

/// Failed to convert from Rust type to a boxed Ruby value.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoxIntoRubyError {
    pub from: Rust,
    pub into: Ruby,
}

impl BoxIntoRubyError {
    #[must_use]
    #[inline]
    pub fn new(from: Rust, into: Ruby) -> Self {
        Self { from, into }
    }
}

impl fmt::Display for BoxIntoRubyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to convert from {} to {}", self.from, self.into)
    }
}

impl error::Error for BoxIntoRubyError {}

impl RubyException for BoxIntoRubyError {
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"Failed to convert from Rust type to Ruby value")
    }

    fn name(&self) -> Cow<'_, str> {
        "TypeError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.to_string()).ok()?;
        let value = interp.new_instance::<TypeError>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<BoxIntoRubyError> for Error {
    fn from(exception: BoxIntoRubyError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<BoxIntoRubyError>> for Error {
    fn from(exception: Box<BoxIntoRubyError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<BoxIntoRubyError> for Box<dyn RubyException> {
    fn from(exception: BoxIntoRubyError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<BoxIntoRubyError>> for Box<dyn RubyException> {
    fn from(exception: Box<BoxIntoRubyError>) -> Box<dyn RubyException> {
        exception
    }
}
