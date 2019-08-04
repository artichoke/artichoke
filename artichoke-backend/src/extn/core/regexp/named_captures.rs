//! [`Regexp#named_captures`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-named_captures)

use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::regexp::{Backend, Regexp};
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let regex = (*borrow.regex).as_ref().ok_or(Error::Fatal)?;
    let Backend::Onig(regex) = regex;
    // Use a Vec of key-value pairs because insertion order matters for spec
    // compliance.
    let mut map = vec![];
    for (name, index) in regex.capture_names() {
        let mut indexes = vec![];
        for idx in index {
            let idx = Int::try_from(*idx).map_err(|_| Error::Fatal)?;
            indexes.push(idx);
        }
        map.push((name, Value::convert(interp, indexes)));
    }
    Ok(Value::convert(interp, map))
}
