use std::borrow::Cow;

use artichoke_core::convert::TryConvertMut;
use spinoso_exception::ArgumentError;

use super::ConvertBytesError;
use crate::core::ClassRegistry;
use crate::error::{Error, RubyException};
use crate::sys;
use crate::Artichoke;

impl RubyException for ConvertBytesError {
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"invalid byte sequence")
    }

    fn name(&self) -> Cow<'_, str> {
        "ArgumentError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.message()).ok()?;
        let value = interp.new_instance::<ArgumentError>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<ConvertBytesError> for Error {
    fn from(exception: ConvertBytesError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<ConvertBytesError>> for Error {
    fn from(exception: Box<ConvertBytesError>) -> Self {
        Self::from(*exception)
    }
}

impl From<ConvertBytesError> for Box<dyn RubyException> {
    fn from(exception: ConvertBytesError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<ConvertBytesError>> for Box<dyn RubyException> {
    fn from(exception: Box<ConvertBytesError>) -> Box<dyn RubyException> {
        exception
    }
}

#[cfg(test)]
mod tests {
    use super::ConvertBytesError;
    use crate::error::{Error, RubyException};

    #[test]
    fn box_convert() {
        // Prevents regressing on a stack overflow caused by a mutual recursion.
        let err = Error::from(ConvertBytesError::new());
        assert_eq!(err.name(), "ArgumentError");
    }
}
