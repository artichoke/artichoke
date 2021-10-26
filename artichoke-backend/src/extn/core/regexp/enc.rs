//! Parse encoding parameter to `Regexp#initialize` and `Regexp::compile`.

use super::{Encoding, InvalidEncodingError};
use crate::extn::prelude::*;

impl TryConvertMut<Value, Encoding> for Artichoke {
    type Error = InvalidEncodingError;

    fn try_convert_mut(&mut self, value: Value) -> Result<Encoding, Self::Error> {
        if let Ok(encoding) = value.try_convert_into::<i64>(self) {
            Encoding::try_from(encoding)
        } else if let Ok(encoding) = value.try_convert_into_mut::<&[u8]>(self) {
            Encoding::try_from(encoding)
        } else {
            Ok(Encoding::new())
        }
    }
}
