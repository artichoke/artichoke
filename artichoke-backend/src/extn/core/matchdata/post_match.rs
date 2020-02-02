//! [`MatchData#post_match`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-post_match)

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
    let post_match = &borrow.string[borrow.region.end..];
    Ok(interp.convert_mut(post_match))
}
