//! [`MatchData#offset`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-offset)

use std::convert::TryFrom;
use std::str;

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &mut Artichoke, value: Value, offset: Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, &value) }?;
    let borrow = data.borrow();
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let index = if let Ok(name) = offset.implicitly_convert_to_string() {
        let indexes = borrow
            .regexp
            .inner()
            .capture_indexes_for_name(interp, name)?;
        let indexes = if let Some(indexes) = indexes {
            indexes
        } else {
            return Ok(interp.convert_mut(&[None::<Value>, None::<Value>][..]));
        };
        if let Some(Ok(index)) = indexes.last().copied().map(usize::try_from) {
            index
        } else {
            return Ok(interp.convert_mut(&[None::<Value>, None::<Value>][..]));
        }
    } else {
        let index = offset.implicitly_convert_to_int()?;
        let captures_len = borrow.regexp.inner().captures_len(interp, Some(haystack))?;
        if index < 0 {
            // Positive Int must be usize
            let idx = usize::try_from(-index).map_err(|_| {
                Fatal::new(interp, "Expected positive position to convert to usize")
            })?;
            if let Some(idx) = captures_len.checked_sub(idx) {
                idx
            } else {
                return Ok(interp.convert_mut(&[None::<Value>, None::<Value>][..]));
            }
        } else {
            let idx = usize::try_from(index).map_err(|_| {
                Fatal::new(interp, "Expected positive position to convert to usize")
            })?;
            if idx > captures_len {
                return Ok(interp.convert_mut(&[None::<Value>, None::<Value>][..]));
            }
            idx
        }
    };
    if let Some((begin, end)) = borrow.regexp.inner().pos(interp, haystack, index)? {
        let begin = if let Ok(haystack) = str::from_utf8(&haystack[0..begin]) {
            haystack.chars().count()
        } else {
            haystack.len()
        };
        let begin = begin + borrow.region.start;
        let begin = Int::try_from(begin)
            .map_err(|_| Fatal::new(interp, "MatchData begin pos does not fit in Integer"))?;

        let end = if let Ok(haystack) = str::from_utf8(&haystack[0..end]) {
            haystack.chars().count()
        } else {
            haystack.len()
        };
        let end = end + borrow.region.start;
        let end = Int::try_from(end)
            .map_err(|_| Fatal::new(interp, "MatchData end pos does not fit in Integer"))?;

        Ok(interp.convert_mut(&[begin, end][..]))
    } else {
        Ok(interp.convert_mut(&[None::<Value>, None::<Value>][..]))
    }
}
