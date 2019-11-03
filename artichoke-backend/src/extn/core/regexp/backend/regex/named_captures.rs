//! [`Regexp#named_captures`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-named_captures)

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::regexp::Regexp;
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
    // Use a Vec of key-value pairs because insertion order matters for spec
    // compliance.
    let mut map = HashMap::new();
    let mut captures = vec![];
    for (idx, name) in regex.capture_names().enumerate() {
        if let Some(name) = name {
            if !map.contains_key(name) {
                captures.push(name);
                map.insert(name.to_owned(), vec![]);
            }
            if let Some(indexes) = map.get_mut(name) {
                let idx = Int::try_from(idx).map_err(|_| Error::Fatal)?;
                indexes.push(idx);
            }
        }
    }
    let pairs = captures
        .into_iter()
        .filter_map(|name| {
            if let Some(indexes) = map.remove(name) {
                Some((name.to_owned(), indexes))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(Value::convert(interp, pairs))
}
