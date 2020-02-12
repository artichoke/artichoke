//! [`MatchData#captures`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-captures)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &mut Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let borrow = data.borrow();
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = borrow.regexp.inner().captures(interp, haystack)?;
    if let Some(captures) = captures {
        Ok(interp.convert_mut(&captures[1..]))
    } else {
        Ok(interp.convert(None::<Value>))
    }
}
