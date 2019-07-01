use std::convert::TryFrom;
use std::mem;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::extn::core::matchdata::MatchData;
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::Value;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    IndexType,
    NoGroup,
    NoMatch,
}

#[derive(Debug, Clone)]
pub enum Args {
    Index(i64),
    Name(String),
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o\0";

    pub unsafe fn extract(interp: &Mrb) -> Result<Self, Error> {
        let first = mem::uninitialized::<sys::mrb_value>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &first,
        );
        if let Ok(index) = i64::try_from_mrb(interp, Value::new(interp, first)) {
            Ok(Args::Index(index))
        } else if let Ok(name) = String::try_from_mrb(interp, Value::new(interp, first)) {
            Ok(Args::Name(name))
        } else {
            Err(Error::IndexType)
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
    let index = match args {
        Args::Index(index) => {
            if index < 0 {
                // Positive i64 must be usize
                let index = usize::try_from(-index).map_err(|_| Error::Fatal)?;
                captures.len().checked_sub(index).ok_or(Error::Fatal)?
            } else {
                // Positive i64 must be usize
                usize::try_from(index).map_err(|_| Error::Fatal)?
            }
        }
        Args::Name(name) => {
            let index = borrow
                .regexp
                .regex
                .capture_names()
                .find(|capture| capture.0 == name)
                .ok_or(Error::NoGroup)?;
            usize::try_from(index.1[0]).map_err(|_| Error::Fatal)?
        }
    };
    let begin = captures.pos(index).ok_or(Error::NoMatch)?.0;
    let begin = match_against[0..begin].chars().count();
    let begin = i64::try_from(begin).map_err(|_| Error::Fatal)?;
    Ok(Value::from_mrb(&interp, begin))
}
