use std::convert::TryFrom;

use crate::convert::RustBackedValue;
use crate::extn::core::matchdata::MatchData;
use crate::interpreter::MrbApi;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Mrb, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = borrow.regexp.regex.captures(match_against);
    if let Some(captures) = captures {
        let len = i64::try_from(captures.len()).map_err(|_| Error::Fatal)?;
        Ok(interp.fixnum(len))
    } else {
        Ok(interp.fixnum(0))
    }
}
