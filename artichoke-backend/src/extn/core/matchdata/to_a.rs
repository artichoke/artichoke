//! [`MatchData#to_a`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-to_a)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    NoMatch,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let regex = (*borrow.regexp.regex).as_ref().ok_or(Error::Fatal)?;
    match regex {
        Backend::Onig(regex) => {
            let captures = regex.captures(match_against).ok_or(Error::NoMatch)?;
            let vec = captures.iter().collect::<Vec<_>>();
            Ok(interp.convert(vec))
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
