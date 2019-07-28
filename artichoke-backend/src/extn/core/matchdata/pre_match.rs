//! [`MatchData#pre_match`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-pre_match)

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
    let pre_match = &borrow.string[0..borrow.region.start];
    Ok(Value::convert(&interp, pre_match))
}
