//! [`MatchData#captures`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-captures)

use crate::convert::{Convert, RustBackedValue};
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
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = borrow.regexp.inner().captures(interp, haystack)?;
    if let Some(mut captures) = captures {
        captures.remove(0);
        Ok(interp.convert(captures))
    } else {
        Ok(interp.convert(None::<Value>))
    }
}
