//! [`MatchData#[]`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-5B-5D)

use std::convert::TryFrom;
use std::mem;

use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Backend;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    IndexType,
    LengthType,
    NoGroup(String),
    NoMatch,
}

#[derive(Debug, Clone)]
pub enum Args {
    Index(Int),
    Name(String),
    StartLen(Int, usize),
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o|o?\0";

    pub unsafe fn extract(interp: &Artichoke, num_captures: usize) -> Result<Self, Error> {
        let num_captures = Int::try_from(num_captures).map_err(|_| Error::Fatal)?;
        let mut first = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut second = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut has_second = <mem::MaybeUninit<sys::mrb_bool>>::uninit();
        sys::mrb_get_args(
            interp.0.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            first.as_mut_ptr(),
            second.as_mut_ptr(),
            has_second.as_mut_ptr(),
        );
        let first = first.assume_init();
        let second = second.assume_init();
        let has_length = has_second.assume_init() != 0;
        if has_length {
            let start = interp
                .try_convert(Value::new(interp, first))
                .map_err(|_| Error::IndexType)?;
            let len = interp
                .try_convert(Value::new(interp, second))
                .map_err(|_| Error::LengthType)?;
            Ok(Args::StartLen(start, len))
        } else if let Ok(index) = interp.try_convert(Value::new(interp, first)) {
            Ok(Args::Index(index))
        } else if let Ok(name) = interp.try_convert(Value::new(interp, first)) {
            Ok(Args::Name(name))
        } else if let Some(args) = Self::is_range(interp, first, num_captures)? {
            Ok(args)
        } else {
            Err(Error::IndexType)
        }
    }

    unsafe fn is_range(
        interp: &Artichoke,
        first: sys::mrb_value,
        num_captures: Int,
    ) -> Result<Option<Self>, Error> {
        let mut start = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        let mut len = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        let check_range = sys::mrb_range_beg_len(
            interp.0.borrow().mrb,
            first,
            start.as_mut_ptr(),
            len.as_mut_ptr(),
            num_captures + 1,
            0_u8,
        );
        let start = start.assume_init();
        let len = len.assume_init();
        if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
            let len = usize::try_from(len).map_err(|_| Error::LengthType)?;
            Ok(Some(Args::StartLen(start, len)))
        } else {
            Ok(None)
        }
    }
}

pub fn method(interp: &Artichoke, args: Args, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let regex = (*borrow.regexp.regex).as_ref().ok_or(Error::Fatal)?;
    match regex {
        Backend::Onig(regex) => {
            let captures = regex.captures(match_against).ok_or(Error::NoMatch)?;
            match args {
                Args::Index(index) => {
                    if index < 0 {
                        // Positive Int must be usize
                        let index = usize::try_from(-index).map_err(|_| Error::Fatal)?;
                        match captures.len().checked_sub(index) {
                            Some(0) | None => Ok(interp.convert(None::<Value>)),
                            Some(index) => Ok(interp.convert(captures.at(index))),
                        }
                    } else {
                        // Positive Int must be usize
                        let index = usize::try_from(index).map_err(|_| Error::Fatal)?;
                        Ok(interp.convert(captures.at(index)))
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
                    let indexes = indexes.ok_or_else(|| Error::NoGroup(name))?;
                    let group = indexes
                        .iter()
                        .filter_map(|index| {
                            usize::try_from(*index)
                                .ok()
                                .and_then(|index| captures.at(index))
                        })
                        .last();
                    Ok(interp.convert(group))
                }
                Args::StartLen(start, len) => {
                    let start = if start < 0 {
                        // Positive i64 must be usize
                        let start = usize::try_from(-start).map_err(|_| Error::Fatal)?;
                        captures.len().checked_sub(start).ok_or(Error::Fatal)?
                    } else {
                        // Positive i64 must be usize
                        usize::try_from(start).map_err(|_| Error::Fatal)?
                    };
                    let mut matches = vec![];
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
