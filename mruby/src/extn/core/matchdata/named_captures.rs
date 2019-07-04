//! [`MatchData#named_captures`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-named_captures)

use std::collections::HashMap;
use std::convert::TryFrom;

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
    let mut map = HashMap::default();
    for (name, index) in borrow.regexp.regex.capture_names() {
        let index = usize::try_from(index[0]).map_err(|_| Error::Fatal)?;
        map.insert(name, Value::from_mrb(interp, captures.at(index)));
    }
    Ok(Value::from_mrb(interp, map))
}
