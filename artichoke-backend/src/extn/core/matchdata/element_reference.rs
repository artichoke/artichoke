//! [`MatchData#[]`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-5B-5D)

use artichoke_core::value::Value as ValueLike;
use bstr::BStr;
use std::convert::TryFrom;
use std::mem;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, IndexError, RubyException, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub enum Args<'a> {
    Empty,
    Index(Int),
    Name(&'a [u8]),
    StartLen(Int, usize),
}

impl<'a> Args<'a> {
    pub fn num_captures(
        interp: &Artichoke,
        value: &Value,
    ) -> Result<usize, Box<dyn RubyException>> {
        let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| {
            Fatal::new(
                interp,
                "Unable to extract Rust MatchData from Ruby MatchData receiver",
            )
        })?;
        let borrow = data.borrow();
        let regex = (*borrow.regexp.regex)
            .as_ref()
            .ok_or_else(|| Fatal::new(interp, "Uninitalized Regexp"))?;
        match regex {
            Backend::Onig(regex) => Ok(regex.captures_len()),
            Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
        }
    }

    pub fn extract(
        interp: &Artichoke,
        elem: Value,
        len: Option<Value>,
        num_captures: usize,
    ) -> Result<Self, Box<dyn RubyException>> {
        if let Some(len) = len {
            let elem_type_name = elem.pretty_name();
            let start = if let Ok(start) = elem.clone().try_into::<Int>() {
                start
            } else if let Ok(start) = elem.funcall::<Int>("to_int", &[], None) {
                start
            } else {
                return Err(Box::new(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", elem_type_name),
                )));
            };
            let len_type_name = len.pretty_name();
            let len = if let Ok(len) = len.clone().try_into::<Int>() {
                len
            } else if let Ok(len) = len.funcall::<Int>("to_int", &[], None) {
                len
            } else {
                return Err(Box::new(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", len_type_name),
                )));
            };
            if let Ok(len) = usize::try_from(len) {
                Ok(Self::StartLen(start, len))
            } else {
                Ok(Self::Empty)
            }
        } else {
            let name = elem.pretty_name();
            if let Ok(index) = elem.clone().try_into::<Int>() {
                Ok(Self::Index(index))
            } else if let Ok(name) = elem.clone().try_into::<&[u8]>() {
                Ok(Self::Name(name))
            } else if let Ok(name) = elem.funcall::<&[u8]>("to_str", &[], None) {
                Ok(Self::Name(name))
            } else if let Ok(index) = elem.funcall::<Int>("to_int", &[], None) {
                Ok(Self::Index(index))
            } else {
                let rangelen = Int::try_from(num_captures)
                    .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
                match unsafe { Self::is_range(interp, &elem, rangelen) } {
                    Ok(Some(args)) => Ok(args),
                    Ok(None) => Ok(Self::Empty),
                    Err(_) => Err(Box::new(TypeError::new(
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
    ) -> Result<Option<Self>, Box<dyn RubyException>> {
        let mut start = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        let mut len = <mem::MaybeUninit<sys::mrb_int>>::uninit();
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
    match regex {
        Backend::Onig(regex) => {
            let captures = if let Some(captures) = regex.captures(match_against) {
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
                            Some(index) => Ok(interp.convert(captures.at(index))),
                        }
                    } else {
                        let idx = usize::try_from(index).map_err(|_| {
                            Fatal::new(interp, "Expected positive position to convert to usize")
                        })?;
                        Ok(interp.convert(captures.at(idx)))
                    }
                }
                Args::Name(name) => {
                    let mut indexes = None;
                    regex.foreach_name(|group, group_indexes| {
                        if name == group.as_bytes() {
                            indexes = Some(group_indexes.to_vec());
                            false
                        } else {
                            true
                        }
                    });
                    if let Some(indexes) = indexes {
                        let group = indexes
                            .iter()
                            .filter_map(|index| {
                                usize::try_from(*index)
                                    .ok()
                                    .and_then(|index| captures.at(index))
                            })
                            .last();
                        Ok(interp.convert(group))
                    } else {
                        let groupstr = format!("{:?}", <&BStr>::from(name));
                        Err(Box::new(IndexError::new(
                            interp,
                            format!(
                                "undefined group name reference: {}",
                                &groupstr[1..groupstr.len() - 1]
                            ),
                        )))
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
                    let mut matches = Vec::with_capacity(len);
                    for index in start..(start + len) {
                        matches.push(captures.at(index));
                    }
                    Ok(interp.convert(matches))
                }
            }
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
