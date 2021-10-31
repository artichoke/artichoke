//! Parse options parameter to `Regexp#initialize` and `Regexp::compile`.

use super::Options;
use crate::convert::implicitly_convert_to_int;
use crate::extn::prelude::*;

impl ConvertMut<Value, Options> for Artichoke {
    fn convert_mut(&mut self, value: Value) -> Options {
        // If options is an Integer, it should be one or more of the constants
        // `Regexp::EXTENDED`, `Regexp::IGNORECASE`, and `Regexp::MULTILINE`,
        // logically or-ed together. Otherwise, if options is not nil or false,
        // the regexp will be case insensitive.
        if let Ok(options) = implicitly_convert_to_int(self, value) {
            Options::from(options)
        } else if let Ok(options) = value.try_convert_into::<Option<bool>>(self) {
            Options::from(options)
        } else if let Ok(options) = value.try_convert_into_mut::<&[u8]>(self) {
            Options::from(options)
        } else {
            Options::with_ignore_case()
        }
    }
}
