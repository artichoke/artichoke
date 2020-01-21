//! [`MatchData#string`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-string)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust MatchData from Ruby MatchData receiver",
        )
    })?;
    let mut result = interp.convert(data.borrow().string.as_slice());
    result
        .freeze()
        .map_err(|_| Fatal::new(interp, "Unable to freeze MatchData#string result"))?;
    Ok(result)
}
