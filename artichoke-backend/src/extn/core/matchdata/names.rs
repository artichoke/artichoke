//! [`MatchData#names`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-names)

use std::cmp::Ordering;

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
    let mut names = vec![];
    let regex = (*borrow.regexp.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "Uninitalized Regexp"))?;
    match regex {
        Backend::Onig(regex) => {
            let mut capture_names = vec![];
            regex.foreach_name(|group, group_indexes| {
                capture_names.push((group.to_owned(), group_indexes.to_vec()));
                true
            });
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
    Ok(interp.convert(names))
}
