//! [`Regexp#inspect`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-inspect)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::regexp::Regexp;
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
    let s = format!(
        "/{}/{}{}",
        borrow.literal_pattern.replace("/", r"\/"),
        borrow.literal_options.modifier_string(),
        borrow.encoding.string()
    );
    Ok(interp.convert(s))
}
