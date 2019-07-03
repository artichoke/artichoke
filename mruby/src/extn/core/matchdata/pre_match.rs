use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Mrb, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let pre_match = &borrow.string[0..borrow.region.start];
    Ok(Value::from_mrb(&interp, pre_match))
}
