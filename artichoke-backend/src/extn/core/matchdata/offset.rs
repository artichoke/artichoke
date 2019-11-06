//! [`MatchData#offset`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-offset)

use artichoke_core::value::Value as ValueLike;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub enum Args<'a> {
    Index(Int),
    Name(&'a str),
}

impl<'a> Args<'a> {
    pub fn extract(interp: &Artichoke, elem: Value) -> Result<Self, Box<dyn RubyException>> {
        let name = elem.pretty_name(interp);
        if let Ok(index) = elem.clone().try_into::<Int>(interp) {
            Ok(Self::Index(index))
        } else if let Ok(name) = elem.funcall::<&str>(interp, "to_str", &[], None) {
            Ok(Self::Name(name))
        } else if let Ok(index) = elem.funcall::<Int>(interp, "to_int", &[], None) {
            Ok(Self::Index(index))
        } else {
            Err(Box::new(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", name),
            )))
        }
    }
}

pub fn method(
    interp: &Artichoke,
    args: Args,
    value: &Value,
) -> Result<Value, Box<dyn RubyException>> {
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
    let (begin, end) = match regex {
        Backend::Onig(regex) => {
            let captures = if let Some(captures) = regex.captures(match_against) {
                captures
            } else {
                return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
            };
            let index = match args {
                Args::Index(index) => {
                    if index < 0 {
                        // Positive Int must be usize
                        let idx = usize::try_from(-index).map_err(|_| {
                            Fatal::new(interp, "Expected positive position to convert to usize")
                        })?;
                        if let Some(idx) = captures.len().checked_sub(idx) {
                            idx
                        } else {
                            return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
                        }
                    } else {
                        let idx = usize::try_from(index).map_err(|_| {
                            Fatal::new(interp, "Expected positive position to convert to usize")
                        })?;
                        if idx > captures.len() {
                            return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
                        }
                        idx
                    }
                }
                Args::Name(name) => {
                    let mut indexes = None;
                    regex.foreach_name(|group, group_indexes| {
                        if name == group {
                            indexes = Some(group_indexes.to_vec());
                            false
                        } else {
                            true
                        }
                    });
                    if let Some(indexes) = indexes {
                        if let Some(last) = indexes.last() {
                            if let Ok(index) = usize::try_from(*last) {
                                index
                            } else {
                                return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
                            }
                        } else {
                            return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
                        }
                    } else {
                        return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
                    }
                }
            };
            if let Some(pos) = captures.pos(index) {
                pos
            } else {
                return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    };
    let begin = match_against[0..begin].chars().count();
    let begin = begin + borrow.region.start;
    let begin = Int::try_from(begin)
        .map_err(|_| Fatal::new(interp, "MatchData begin pos does not fit in Integer"))?;
    let end = match_against[0..end].chars().count();
    let end = end + borrow.region.start;
    let end = Int::try_from(end)
        .map_err(|_| Fatal::new(interp, "MatchData begin pos does not fit in Integer"))?;
    Ok(interp.convert([begin, end].as_ref()))
}
