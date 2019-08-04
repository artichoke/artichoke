//! [`MatchData#length`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-length)

use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let regex = (*borrow.regexp.regex).as_ref().ok_or(Error::Fatal)?;
    match regex {
        Backend::Onig(regex) => {
            let captures = regex.captures(match_against);
            if let Some(captures) = captures {
                let len = Int::try_from(captures.len()).map_err(|_| Error::Fatal)?;
                Ok(Value::convert(interp, len))
            } else {
                Ok(Value::convert(interp, 0))
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
