//! [`MatchData#[]`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-5B-5D)

use std::convert::TryFrom;
use std::mem;

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Args<'a> {
    Empty,
    Index(Int),
    Name(&'a [u8]),
    StartLen(Int, usize),
}

impl<'a> Args<'a> {
    pub fn num_captures(interp: &Artichoke, value: &Value) -> Result<usize, Exception> {
        let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
        let borrow = data.borrow();
        borrow.regexp.inner().captures_len(interp, None)
    }

    pub fn extract(
        interp: &Artichoke,
        elem: Value,
        len: Option<Value>,
        num_captures: usize,
    ) -> Result<Self, Exception> {
        if let Some(len) = len {
            let start = elem.implicitly_convert_to_int()?;
            let len = len.implicitly_convert_to_int()?;
            if let Ok(len) = usize::try_from(len) {
                Ok(Self::StartLen(start, len))
            } else {
                Ok(Self::Empty)
            }
        } else {
            let name = elem.pretty_name();
            if let Ok(index) = elem.implicitly_convert_to_int() {
                Ok(Self::Index(index))
            } else if let Ok(name) = elem.clone().try_into::<&[u8]>() {
                Ok(Self::Name(name))
            } else if let Ok(name) = elem.funcall::<&[u8]>("to_str", &[], None) {
                Ok(Self::Name(name))
            } else {
                let rangelen = Int::try_from(num_captures)
                    .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
                match unsafe { Self::is_range(interp, &elem, rangelen) } {
                    Ok(Some(args)) => Ok(args),
                    Ok(None) => Ok(Self::Empty),
                    Err(_) => Err(Exception::from(TypeError::new(
                        interp,
                        format!("no implicit conversion of {} into Integer", name),
                    ))),
                }
            }
        }
    }

    unsafe fn is_range(
        interp: &Artichoke,
        first: &Value,
        length: Int,
    ) -> Result<Option<Self>, Exception> {
        let mut start = mem::MaybeUninit::<sys::mrb_int>::uninit();
        let mut len = mem::MaybeUninit::<sys::mrb_int>::uninit();
        let mrb = interp.0.borrow().mrb;
        // `mrb_range_beg_len` can raise.
        // TODO: Wrap this in a call to `mrb_protect`.
        let check_range = sys::mrb_range_beg_len(
            mrb,
            first.inner(),
            start.as_mut_ptr(),
            len.as_mut_ptr(),
            length,
            0_u8,
        );
        let start = start.assume_init();
        let len = len.assume_init();
        if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
            let len = usize::try_from(len)
                .map_err(|_| TypeError::new(interp, "no implicit conversion into Integer"))?;
            Ok(Some(Self::StartLen(start, len)))
        } else {
            Ok(None)
        }
    }
}

pub fn method(interp: &mut Artichoke, args: Args, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let borrow = data.borrow();
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let mut captures = if let Some(captures) = borrow.regexp.inner().captures(interp, haystack)? {
        captures
    } else {
        return Ok(interp.convert(None::<Value>));
    };
    match args {
        Args::Empty => Ok(interp.convert(None::<Value>)),
        Args::Index(index) => {
            if index < 0 {
                // Positive Int must be usize
                let idx = usize::try_from(-index).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                match captures.len().checked_sub(idx) {
                    Some(0) | None => Ok(interp.convert(None::<Value>)),
                    Some(index) => Ok(interp.convert_mut(captures.remove(index))),
                }
            } else {
                let idx = usize::try_from(index).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                if idx < captures.len() {
                    Ok(interp.convert_mut(captures.remove(idx)))
                } else {
                    Ok(interp.convert(None::<Value>))
                }
            }
        }
        Args::Name(name) => {
            let indexes = borrow
                .regexp
                .inner()
                .capture_indexes_for_name(interp, name)?;
            if let Some(indexes) = indexes {
                let group = indexes
                    .iter()
                    .copied()
                    .filter_map(|index| captures.get(index).map(Option::as_deref).flatten())
                    .last();
                Ok(interp.convert_mut(group))
            } else {
                let mut message = String::from("undefined group name reference: \"");
                string::escape_unicode(&mut message, name)?;
                message.push('"');
                Err(Exception::from(IndexError::new(interp, message)))
            }
        }
        Args::StartLen(start, len) => {
            let start = if start < 0 {
                // Positive Int must be usize
                let idx = usize::try_from(-start).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                if let Some(start) = captures.len().checked_sub(idx) {
                    start
                } else {
                    return Ok(interp.convert(None::<Value>));
                }
            } else {
                usize::try_from(start).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?
            };
            // TODO: construct an Array from iterator directly.
            let matches = captures
                .into_iter()
                .skip(start)
                .take(len)
                .collect::<Vec<_>>();
            Ok(interp.convert_mut(matches))
        }
    }
}
