//! [`MatchData#string`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-string)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::matchdata::MatchData;
use crate::value::{Value, ValueLike};
use crate::Artichoke;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Box<dyn RubyException>> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust MatchData from Ruby MatchData receiver",
        )
    })?;
    let mut result = interp.convert(data.borrow().string.as_bytes());
    result
        .freeze()
        .map_err(|_| Fatal::new(interp, "Unable to freeze MatchData#string result"))?;
    Ok(result)
}
