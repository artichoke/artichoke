//! [`MatchData#to_s`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-to_s)

use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    NoMatch,
}

pub fn method(interp: &Mrb, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = borrow
        .regexp
        .regex
        .captures(match_against)
        .ok_or(Error::NoMatch)?;
    Ok(Value::from_mrb(&interp, captures.at(0).unwrap_or_default()))
}
