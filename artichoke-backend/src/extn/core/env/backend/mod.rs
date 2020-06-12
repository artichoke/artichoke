use bstr::BStr;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error;
use std::fmt;

use crate::extn::prelude::*;

pub mod memory;
pub mod system;

pub trait EnvType {
    /// Return a `dyn Debug` representation of this `Environ`.
    fn as_debug(&self) -> &dyn fmt::Debug;

    fn get<'a>(&'a self, name: &[u8]) -> Result<Option<Cow<'a, [u8]>>, Exception>;

    fn put(&mut self, name: &[u8], value: Option<&[u8]>) -> Result<(), Exception>;

    fn to_map(&self) -> Result<HashMap<Vec<u8>, Vec<u8>>, Exception>;
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnvArgumentError(Cow<'static, BStr>);

impl From<Vec<u8>> for EnvArgumentError {
    fn from(message: Vec<u8>) -> Self {
        Self(Cow::Owned(message.into()))
    }
}

impl From<&'static str> for EnvArgumentError {
    fn from(message: &'static str) -> Self {
        Self(Cow::Borrowed(message.into()))
    }
}

impl From<&'static [u8]> for EnvArgumentError {
    fn from(message: &'static [u8]) -> Self {
        Self(Cow::Borrowed(message.into()))
    }
}

impl EnvArgumentError {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::from("ArgumentError")
    }
}

impl fmt::Display for EnvArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to write message into destination")
    }
}

impl error::Error for EnvArgumentError {}

impl RubyException for EnvArgumentError {
    #[inline]
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self.0.as_ref())
    }

    #[inline]
    fn name(&self) -> Cow<'_, str> {
        "ArgumentError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let value = interp
            .new_instance::<ArgumentError>(&[message])
            .ok()
            .flatten()?;
        Some(value.inner())
    }
}

impl From<EnvArgumentError> for Exception {
    #[inline]
    fn from(exception: EnvArgumentError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<EnvArgumentError>> for Exception {
    #[inline]
    fn from(exception: Box<EnvArgumentError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<EnvArgumentError> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: EnvArgumentError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<EnvArgumentError>> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: Box<EnvArgumentError>) -> Box<dyn RubyException> {
        exception
    }
}
