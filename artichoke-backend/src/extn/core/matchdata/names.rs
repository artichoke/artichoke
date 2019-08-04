//! [`MatchData#names`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-names)

use std::cmp::Ordering;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let mut names = vec![];
    let regex = (*borrow.regexp.regex).as_ref().ok_or(Error::Fatal)?;
    match regex {
        Backend::Onig(regex) => {
            let mut capture_names = regex.capture_names().collect::<Vec<_>>();
            capture_names.sort_by(|a, b| {
                a.1.iter()
                    .fold(u32::max_value(), |a, &b| a.min(b))
                    .partial_cmp(b.1.iter().fold(&u32::max_value(), |a, b| a.min(b)))
                    .unwrap_or(Ordering::Equal)
            });
            for (name, _) in capture_names {
                if !names.contains(&name) {
                    names.push(name);
                }
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    };
    Ok(Value::convert(interp, names))
}
