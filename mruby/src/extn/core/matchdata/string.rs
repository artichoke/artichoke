//! [`MatchData#string`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-string)

use crate::convert::{FromMrb, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Mrb, value: &Value) -> Result<Value, Error> {
    if let Ok(data) = unsafe { MatchData::try_from_ruby(interp, value) } {
        Value::from_mrb(interp, data.borrow().string.as_str())
            .freeze()
            .map_err(|_| Error::Fatal)
    } else {
        Err(Error::Fatal)
    }
}
