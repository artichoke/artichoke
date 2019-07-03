use std::convert::TryFrom;
use std::mem;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::extn::core::matchdata::MatchData;
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::Value;

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
    Index(i64),
    Name(String),
    StartLen(i64, usize),
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o|o?\0";

    pub unsafe fn extract(interp: &Mrb, num_captures: usize) -> Result<Self, Error> {
        let num_captures = i64::try_from(num_captures).map_err(|_| Error::Fatal)?;
        let first = mem::uninitialized::<sys::mrb_value>();
        let second = mem::uninitialized::<sys::mrb_value>();
        let has_second = mem::uninitialized::<sys::mrb_bool>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &first,
            &second,
            &has_second,
        );
        let has_length = has_second != 0;
        if has_length {
            let start = i64::try_from_mrb(&interp, Value::new(interp, first))
                .map_err(|_| Error::IndexType)?;
            let len = usize::try_from_mrb(&interp, Value::new(interp, second))
                .map_err(|_| Error::LengthType)?;
            Ok(Args::StartLen(start, len))
        } else if let Ok(index) = i64::try_from_mrb(interp, Value::new(interp, first)) {
            Ok(Args::Index(index))
        } else if let Ok(name) = String::try_from_mrb(interp, Value::new(interp, first)) {
            Ok(Args::Name(name))
        } else if let Some(args) = Self::is_range(interp, first, num_captures)? {
            Ok(args)
        } else {
            Err(Error::Fatal)
        }
    }

    unsafe fn is_range(
        interp: &Mrb,
        first: sys::mrb_value,
        num_captures: i64,
    ) -> Result<Option<Self>, Error> {
        let mut start = mem::uninitialized::<sys::mrb_int>();
        let mut len = mem::uninitialized::<sys::mrb_int>();
        let check_range = sys::mrb_range_beg_len(
            interp.borrow().mrb,
            first,
            &mut start,
            &mut len,
            num_captures,
            0_u8,
        );
        if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
            let len =
                usize::try_from_mrb(&interp, interp.fixnum(len)).map_err(|_| Error::LengthType)?;
            Ok(Some(Args::StartLen(start, len)))
        } else {
            Ok(None)
        }
    }
}

pub fn method(interp: &Mrb, args: Args, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = borrow
        .regexp
        .regex
        .captures(match_against)
        .ok_or(Error::NoMatch)?;
    match args {
        Args::Index(index) => {
            if index < 0 {
                // Positive i64 must be usize
                let index = usize::try_from(-index).map_err(|_| Error::Fatal)?;
                match captures.len().checked_sub(index) {
                    Some(index) => Ok(Value::from_mrb(&interp, captures.at(index))),
                    None => Ok(interp.nil()),
                }
            } else {
                // Positive i64 must be usize
                let index = usize::try_from(index).map_err(|_| Error::Fatal)?;
                Ok(Value::from_mrb(&interp, captures.at(index)))
            }
        }
        Args::Name(name) => {
            let index = borrow
                .regexp
                .regex
                .capture_names()
                .find_map(|capture| {
                    if capture.0 == name {
                        Some(capture.1)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| Error::NoGroup(name))?;
            let group = index
                .iter()
                .filter_map(|index| {
                    usize::try_from(*index)
                        .ok()
                        .and_then(|index| captures.at(index))
                })
                .last();
            Ok(Value::from_mrb(&interp, group))
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
            Ok(Value::from_mrb(&interp, matches))
        }
    }
}
