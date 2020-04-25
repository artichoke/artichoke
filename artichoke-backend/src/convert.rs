use std::error;
use std::fmt;

use crate::class_registry::ClassRegistry;
use crate::core::{Convert, ConvertMut, TryConvert, TryConvertMut};
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception::TypeError;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

mod array;
mod boolean;
mod bytes;
mod fixnum;
mod float;
mod hash;
mod nilable;
mod object;
mod string;

pub use object::RustBackedValue;

/// Provide a fallible converter for types that implement an infallible
/// conversion.
impl<T, U> TryConvert<T, U> for Artichoke
where
    Artichoke: Convert<T, U>,
{
    // TODO: this should be the never type.
    // https://github.com/rust-lang/rust/issues/35121
    type Error = Exception;

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
    type Error = Exception;

    /// Blanket implementation that always succeeds by delegating to
    /// [`Convert::convert`].
    #[inline]
    fn try_convert_mut(&mut self, value: T) -> Result<U, Self::Error> {
        Ok(ConvertMut::convert_mut(self, value))
    }
}

/// Failed to convert from boxed Ruby value to a Rust type.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    fn message(&self) -> &[u8] {
        &b"Failed to convert from Ruby value to Rust type"[..]
    }

    fn name(&self) -> String {
        String::from("TypeError")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.to_string());
        let spec = interp.class_spec::<TypeError>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<UnboxRubyError> for Exception {
    fn from(exception: UnboxRubyError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<UnboxRubyError>> for Exception {
    fn from(exception: Box<UnboxRubyError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<UnboxRubyError> for Box<dyn RubyException> {
    fn from(exception: UnboxRubyError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<UnboxRubyError>> for Box<dyn RubyException> {
    fn from(exception: Box<UnboxRubyError>) -> Box<dyn RubyException> {
        exception
    }
}

/// Failed to convert from Rust type to a boxed Ruby value.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    fn message(&self) -> &[u8] {
        &b"Failed to convert from Rust type to Ruby value"[..]
    }

    fn name(&self) -> String {
        String::from("TypeError")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.to_string());
        let spec = interp.class_spec::<TypeError>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<BoxIntoRubyError> for Exception {
    fn from(exception: BoxIntoRubyError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<BoxIntoRubyError>> for Exception {
    fn from(exception: Box<BoxIntoRubyError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<BoxIntoRubyError> for Box<dyn RubyException> {
    fn from(exception: BoxIntoRubyError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<BoxIntoRubyError>> for Box<dyn RubyException> {
    fn from(exception: Box<BoxIntoRubyError>) -> Box<dyn RubyException> {
        exception
    }
}
