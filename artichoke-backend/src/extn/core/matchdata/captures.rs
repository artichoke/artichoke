//! [`MatchData#captures`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-captures)

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
    let captures = borrow.regexp.inner().captures(interp, haystack)?;
    if let Some(mut captures) = captures {
        captures.remove(0);
        Ok(interp.convert(captures))
    } else {
        Ok(interp.convert(None::<Value>))
    }
}
