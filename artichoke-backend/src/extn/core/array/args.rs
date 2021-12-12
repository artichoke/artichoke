use std::fmt::Write as _;

use crate::convert::implicitly_convert_to_int;
use crate::extn::prelude::*;
use crate::sys::protect;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ElementReference {
    Empty,
    Index(i64),
    StartLen(i64, usize),
}

pub fn element_reference(
    interp: &mut Artichoke,
    elem: Value,
    len: Option<Value>,
    ary_len: usize,
) -> Result<ElementReference, Error> {
    if let Some(len) = len {
        let start = implicitly_convert_to_int(interp, elem)?;
        let len = implicitly_convert_to_int(interp, len)?;
        if let Ok(len) = usize::try_from(len) {
            Ok(ElementReference::StartLen(start, len))
        } else {
            Ok(ElementReference::Empty)
        }
    } else if let Ok(index) = implicitly_convert_to_int(interp, elem) {
        Ok(ElementReference::Index(index))
    } else {
        let rangelen = i64::try_from(ary_len).map_err(|_| Fatal::from("Range length exceeds Integer max"))?;
        if let Some(protect::Range { start, len }) = elem.is_range(interp, rangelen)? {
            if let Ok(len) = usize::try_from(len) {
                Ok(ElementReference::StartLen(start, len))
            } else {
                Ok(ElementReference::Empty)
            }
        } else {
            Ok(ElementReference::Empty)
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn element_assignment(
    interp: &mut Artichoke,
    first: Value,
    second: Value,
    third: Option<Value>,
    len: usize,
) -> Result<(usize, Option<usize>, Value), Error> {
    if let Some(elem) = third {
        let start = implicitly_convert_to_int(interp, first)?;
        let start = if let Ok(start) = usize::try_from(start) {
            start
        } else {
            let pos = start
                .checked_neg()
                .and_then(|start| usize::try_from(start).ok())
                .and_then(|start| len.checked_sub(start));
            if let Some(start) = pos {
                start
            } else {
                let mut message = String::from("index ");
                write!(&mut message, "{}", start).map_err(WriteError::from)?;
                message.push_str(" too small for array; minimum: -");
                write!(&mut message, "{}", len).map_err(WriteError::from)?;
                return Err(IndexError::from(message).into());
            }
        };
        let slice_len = implicitly_convert_to_int(interp, second)?;
        if let Ok(slice_len) = usize::try_from(slice_len) {
            Ok((start, Some(slice_len), elem))
        } else {
            let mut message = String::from("negative length (");
            write!(&mut message, "{}", slice_len).map_err(WriteError::from)?;
            message.push(')');
            Err(IndexError::from(message).into())
        }
    } else if let Ok(index) = implicitly_convert_to_int(interp, first) {
        if let Ok(index) = usize::try_from(index) {
            Ok((index, None, second))
        } else {
            let idx = index
                .checked_neg()
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| len.checked_sub(index));
            if let Some(idx) = idx {
                Ok((idx, None, second))
            } else {
                let mut message = String::from("index ");
                write!(&mut message, "{}", index).map_err(WriteError::from)?;
                message.push_str(" too small for array; minimum: -");
                write!(&mut message, "{}", len).map_err(WriteError::from)?;
                Err(IndexError::from(message).into())
            }
        }
    } else {
        let rangelen = i64::try_from(len).map_err(|_| Fatal::from("Range length exceeds Integer max"))?;
        if let Some(protect::Range { start, len }) = first.is_range(interp, rangelen)? {
            let start = usize::try_from(start)
                .unwrap_or_else(|_| unimplemented!("should throw RangeError (-11..1 out of range)"));
            let len = usize::try_from(len).unwrap_or_else(|_| unreachable!("Range can't have negative length"));
            Ok((start, Some(len), second))
        } else {
            let start = first.funcall(interp, "begin", &[], None)?;
            let start = implicitly_convert_to_int(interp, start)?;
            let end = first.funcall(interp, "last", &[], None)?;
            let end = implicitly_convert_to_int(interp, end)?;
            // TODO: This conditional is probably not doing the right thing
            if start + (end - start) < 0 {
                let mut message = String::new();
                write!(&mut message, "{}", start).map_err(WriteError::from)?;
                message.push_str("..");
                write!(&mut message, "{}", end).map_err(WriteError::from)?;
                message.push_str(" out of range");
                return Err(RangeError::from(message).into());
            }
            match (usize::try_from(start), usize::try_from(end)) {
                (Ok(start), Ok(end)) => Ok((start, end.checked_sub(start), second)),
                (Err(_), Ok(end)) => {
                    let pos = start
                        .checked_neg()
                        .and_then(|start| usize::try_from(start).ok())
                        .and_then(|start| len.checked_sub(start));
                    if let Some(start) = pos {
                        Ok((start, end.checked_sub(start), second))
                    } else {
                        let mut message = String::from("index ");
                        write!(&mut message, "{}", start).map_err(WriteError::from)?;
                        message.push_str(" too small for array; minimum: -");
                        write!(&mut message, "{}", len).map_err(WriteError::from)?;
                        Err(IndexError::from(message).into())
                    }
                }
                (Ok(start), Err(_)) => Ok((start, None, second)),
                (Err(_), Err(_)) => {
                    let mut message = String::from("index ");
                    write!(&mut message, "{}", start).map_err(WriteError::from)?;
                    message.push_str(" too small for array; minimum: -");
                    write!(&mut message, "{}", len).map_err(WriteError::from)?;
                    Err(IndexError::from(message).into())
                }
            }
        }
    }
}
