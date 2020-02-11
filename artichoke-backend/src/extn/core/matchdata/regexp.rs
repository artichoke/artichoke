//! [`MatchData#regexp`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-regexp)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let borrow = data.borrow();
    let regexp = borrow.regexp.clone();
    let regexp = regexp.try_into_ruby(interp, None).map_err(|_| {
        Fatal::new(
            interp,
            "Unable to initialize Ruby Regexp Value from Rust Regexp",
        )
    })?;
    Ok(regexp)
}
