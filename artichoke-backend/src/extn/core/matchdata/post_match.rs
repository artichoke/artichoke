//! [`MatchData#post_match`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-post_match)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let post_match = &borrow.string[borrow.region.end..];
    Ok(Value::convert(&interp, post_match))
}
