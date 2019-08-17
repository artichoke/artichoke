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
    // Use a Vec of key-value pairs because insertion order matters for spec
    // compliance.
    let mut map = vec![];
    let regex = (*borrow.regex).as_ref().ok_or(Error::Fatal)?;
    match regex {
        Backend::Onig(regex) => {
            regex.foreach_name(|group, group_indexes| {
                let mut indexes = vec![];
                for idx in group_indexes {
                    let idx = Int::try_from(*idx).unwrap_or_default();
                    indexes.push(idx);
                }
                map.push((group.to_owned(), Value::convert(interp, indexes)));
                true
            });
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
    Ok(Value::convert(interp, map))
}
