//! [`MatchData#to_s`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-to_s)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
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
            if let Some(captures) = regex.captures(match_against) {
                Ok(interp.convert(captures.at(0).unwrap_or_default()))
            } else {
                Ok(interp.convert(&[] as &[Value]))
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
