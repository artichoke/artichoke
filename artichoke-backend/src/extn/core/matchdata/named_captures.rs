//! [`MatchData#named_captures`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-named_captures)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &mut Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust MatchData from Ruby MatchData receiver",
        )
    })?;
    let borrow = data.borrow();
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let named_captures = borrow
        .regexp
        .inner()
        .named_captures_for_haystack(interp, haystack)?;
    Ok(interp.convert_mut(named_captures))
}
