//! [`MatchData#regexp`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-regexp)

use crate::convert::RustBackedValue;
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::matchdata::MatchData;
use crate::value::Value;
use crate::Artichoke;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Box<dyn RubyException>> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust MatchData from Ruby MatchData receiver",
        )
    })?;
    let borrow = data.borrow();
    let regexp = borrow.regexp.clone();
    let regexp = unsafe { regexp.try_into_ruby(interp, None) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to initialize Ruby Regexp Value from Rust Regexp",
        )
    })?;
    Ok(regexp)
}
