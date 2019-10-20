//! [`MatchData#length`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-length)

use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
use crate::types::Int;
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
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let regex = (*borrow.regexp.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "Uninitalized Regexp"))?;
    match regex {
        Backend::Onig(regex) => {
            let captures = regex.captures(match_against);
            if let Some(captures) = captures {
                let len = Int::try_from(captures.len())
                    .map_err(|_| Fatal::new(interp, "Number of captures exceeds Integer max"))?;
                Ok(interp.convert(len))
            } else {
                Ok(interp.convert(0))
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
