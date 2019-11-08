//! [`MatchData#begin`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-begin)

use artichoke_core::value::Value as ValueLike;
use std::convert::TryFrom;
use std::str;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, IndexError, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub enum Args<'a> {
    Index(Int),
    Name(&'a str),
}

impl<'a> Args<'a> {
    pub fn extract(interp: &Artichoke, at: Value) -> Result<Self, Box<dyn RubyException>> {
        let name = at.pretty_name();
        if let Ok(index) = at.clone().try_into::<Int>() {
            Ok(Self::Index(index))
        } else if let Ok(name) = at.clone().try_into::<&str>() {
            Ok(Self::Name(name))
        } else if let Ok(index) = at.funcall::<Int>("to_int", &[], None) {
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
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let index = match args {
        Args::Index(index) => {
            let captures_len = borrow.regexp.inner().captures_len(interp, Some(haystack))?;
            if index < 0 {
                // Positive Int must be usize
                let idx = usize::try_from(-index).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                captures_len.checked_sub(idx).ok_or_else(|| {
                    IndexError::new(interp, format!("index {} out of matches", index))
                })?
            } else {
                let idx = usize::try_from(index).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                if idx > captures_len {
                    return Err(Box::new(IndexError::new(
                        interp,
                        format!("index {} out of matches", index),
                    )));
                }
                idx
            }
        }
        Args::Name(name) => {
            let indexes = borrow
                .regexp
                .inner()
                .capture_indexes_for_name(interp, name.as_bytes())?;
            let indexes = if let Some(indexes) = indexes {
                indexes
            } else {
                return Ok(interp.convert(None::<Value>));
            };
            if let Some(Ok(index)) = indexes.last().copied().map(usize::try_from) {
                index
            } else {
                return Ok(interp.convert(None::<Value>));
            }
        }
    };
    if let Some((begin, _)) = borrow.regexp.inner().pos(interp, haystack, index)? {
        let begin = if let Ok(haystack) = str::from_utf8(&haystack[0..begin]) {
            haystack.chars().count()
        } else {
            haystack.len()
        };
        let begin = begin + borrow.region.start;
        let begin = Int::try_from(begin)
            .map_err(|_| Fatal::new(interp, "MatchData begin pos does not fit in Integer"))?;

        Ok(interp.convert(begin))
    } else {
        Ok(interp.convert(None::<Value>))
    }
}
