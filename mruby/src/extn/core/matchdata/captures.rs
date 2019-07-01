use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::interpreter::Mrb;
use crate::value::Value;

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
    let mut iter = captures.iter();
    // skip 0 (full match) capture group
    iter.next();
    let vec = iter.collect::<Vec<_>>();
    Ok(Value::from_mrb(&interp, vec))
}
