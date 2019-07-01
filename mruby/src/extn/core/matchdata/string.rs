use crate::convert::RustBackedValue;
use crate::extn::core::matchdata::MatchData;
use crate::interpreter::{Mrb, MrbApi};
use crate::value::Value;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Mrb, value: &Value) -> Result<Value, Error> {
    if let Ok(data) = unsafe { MatchData::try_from_ruby(interp, value) } {
        Ok(interp.string(data.borrow().string.as_str()))
    } else {
        Err(Error::Fatal)
    }
}
