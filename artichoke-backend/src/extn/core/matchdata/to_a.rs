//! [`MatchData#to_a`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-to_a)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &mut Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let borrow = data.borrow();
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    if let Some(captures) = borrow.regexp.inner().captures(interp, haystack)? {
        Ok(interp.convert_mut(captures))
    } else {
        Ok(interp.convert(None::<Value>))
    }
}
