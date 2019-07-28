//! [`MatchData#captures`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-captures)

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
    let regex = (*borrow.regexp.regex).as_ref().ok_or(Error::Fatal)?;
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = regex.captures(match_against).ok_or(Error::NoMatch)?;
    let mut iter = captures.iter();
    // skip 0 (full match) capture group
    iter.next();
    let vec = iter.collect::<Vec<_>>();
    Ok(Value::from_mrb(&interp, vec))
}
