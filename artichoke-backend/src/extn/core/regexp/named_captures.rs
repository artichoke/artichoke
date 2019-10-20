//! [`Regexp#named_captures`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-named_captures)

use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::regexp::{Backend, Regexp};
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Box<dyn RubyException>> {
    let value = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = value.borrow();
    // Use a Vec of key-value pairs because insertion order matters for spec
    // compliance.
    let mut map = vec![];
    let regex = (*borrow.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "Uninitialized Regexp"))?;
    match regex {
        Backend::Onig(regex) => {
            regex.foreach_name(|group, group_indexes| {
                let mut indexes = vec![];
                for idx in group_indexes {
                    let idx = Int::try_from(*idx).unwrap_or_default();
                    indexes.push(idx);
                }
                map.push((group.to_owned(), interp.convert(indexes)));
                true
            });
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
    Ok(interp.convert(map))
}
