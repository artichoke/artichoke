use std::borrow::Cow;

use artichoke_core::convert::TryConvertMut;
use scolapasta_path::ConvertBytesError;
use spinoso_exception::ArgumentError;

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
        let err: Box<dyn RubyException> = Box::new(exception);
        Self::from(err)
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
