//! [`MatchData#string`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-string)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::value::{Value, ValueLike};
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    if let Ok(data) = unsafe { MatchData::try_from_ruby(interp, value) } {
        let mut result = interp.convert(data.borrow().string.as_str());
        result.freeze().map_err(|_| Error::Fatal)?;
        Ok(result)
    } else {
        Err(Error::Fatal)
    }
}
