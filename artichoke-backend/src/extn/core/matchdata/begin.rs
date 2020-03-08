//! [`MatchData#begin`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-begin)

use std::convert::TryFrom;
use std::str;

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Args<'a> {
    Index(Int),
    Name(&'a str),
}

impl<'a> Args<'a> {
    pub fn extract(interp: &Artichoke, at: Value) -> Result<Self, Exception> {
        let _ = interp;
        if let Ok(name) = at.clone().try_into::<&str>() {
            Ok(Self::Name(name))
        } else {
            let index = at.implicitly_convert_to_int()?;
            Ok(Self::Index(index))
        }
    }
}

pub fn method(interp: &Artichoke, args: Args<'_>, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
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
                if let Some(index) = captures_len.checked_sub(idx) {
                    index
                } else {
                    let mut message = String::from("index ");
                    string::format_int_into(&mut message, index)?;
                    message.push_str(" out of matches");
                    return Err(Exception::from(IndexError::new(interp, message)));
                }
            } else {
                let idx = usize::try_from(index).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                if idx > captures_len {
                    let mut message = String::from("index ");
                    string::format_int_into(&mut message, index)?;
                    message.push_str(" out of matches");
                    return Err(Exception::from(IndexError::new(interp, message)));
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
