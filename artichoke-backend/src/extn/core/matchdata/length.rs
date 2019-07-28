//! [`MatchData#length`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-length)

use std::convert::TryFrom;

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
    let regex = (*borrow.regexp.regex).as_ref().ok_or(Error::Fatal)?;
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = regex.captures(match_against);
    if let Some(captures) = captures {
        let len = i64::try_from(captures.len()).map_err(|_| Error::Fatal)?;
        Ok(Value::convert(interp, len))
    } else {
        Ok(Value::convert(interp, 0))
    }
}
