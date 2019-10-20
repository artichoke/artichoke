//! [`Regexp#eql?`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-eql-3F)
//! and
//! [`Regexp#==`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-3D)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::regexp::Regexp;
use crate::value::Value;
use crate::Artichoke;

pub fn method(
    interp: &Artichoke,
    value: &Value,
    other: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    let other = if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, other) } {
        regexp
    } else {
        return Ok(interp.convert(false));
    };
    let value = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let slf = value.borrow();
    let oth = other.borrow();
    Ok(interp.convert(slf.pattern == oth.pattern && slf.encoding == oth.encoding))
}
