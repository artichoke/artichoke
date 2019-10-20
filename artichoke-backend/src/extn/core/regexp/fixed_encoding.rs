//! [`Regexp#fixed_encoding?`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-fixed_encoding-3F)

use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::regexp::enc::Encoding;
use crate::extn::core::regexp::Regexp;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Box<dyn RubyException>> {
    let value = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = value.borrow();
    match borrow.encoding {
        Encoding::No => {
            let opts = Int::try_from(borrow.literal_options.flags().bits())
                .map_err(|_| Fatal::new(interp, "Regexp options do not fit in Integer"))?;
            Ok(interp.convert(opts & Regexp::NOENCODING != 0))
        }
        Encoding::Fixed => Ok(interp.convert(true)),
        Encoding::None => Ok(interp.convert(false)),
    }
}
