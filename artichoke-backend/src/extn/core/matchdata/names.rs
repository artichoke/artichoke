//! [`MatchData#names`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-names)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &mut Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let borrow = data.borrow();
    let names = borrow.regexp.names(interp);
    Ok(interp.convert_mut(names))
}
