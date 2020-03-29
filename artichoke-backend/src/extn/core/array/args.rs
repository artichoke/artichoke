use std::convert::TryFrom;
use std::mem;

use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ElementReference {
    Empty,
    Index(Int),
    StartLen(Int, usize),
}

pub fn element_reference(
    interp: &Artichoke,
    elem: Value,
    len: Option<Value>,
    ary_len: usize,
) -> Result<ElementReference, Exception> {
    if let Some(len) = len {
        let start = elem.implicitly_convert_to_int()?;
        let len = len.implicitly_convert_to_int()?;
        if let Ok(len) = usize::try_from(len) {
            Ok(ElementReference::StartLen(start, len))
        } else {
            Ok(ElementReference::Empty)
        }
    } else if let Ok(index) = elem.implicitly_convert_to_int() {
        Ok(ElementReference::Index(index))
    } else {
        let rangelen = Int::try_from(ary_len)
            .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
        match unsafe { is_range(interp, &elem, rangelen) } {
            Ok(Some((start, len))) => {
                if let Ok(len) = usize::try_from(len) {
                    Ok(ElementReference::StartLen(start, len))
                } else {
                    Ok(ElementReference::Empty)
                }
            }
            Ok(None) => Ok(ElementReference::Empty),
            Err(_) => {
                let mut message = String::from("no implicit conversion of ");
                message.push_str(elem.pretty_name());
                message.push_str(" into Integer");
                Err(Exception::from(TypeError::new(interp, message)))
            }
        }
    }
}

pub fn element_assignment(
    interp: &Artichoke,
    first: Value,
    second: Value,
    third: Option<Value>,
    len: usize,
) -> Result<(usize, Option<usize>, Value), Exception> {
    if let Some(elem) = third {
        let start = first.implicitly_convert_to_int()?;
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
                string::format_int_into(&mut message, start)?;
                message.push_str(" too small for array; minimum: -");
                string::format_int_into(&mut message, len)?;
                return Err(Exception::from(IndexError::new(interp, message)));
            }
        };
        let slice_len = second.implicitly_convert_to_int()?;
        if let Ok(slice_len) = usize::try_from(slice_len) {
            Ok((start, Some(slice_len), elem))
        } else {
            let mut message = String::from("negative length (");
            string::format_int_into(&mut message, slice_len)?;
            message.push(')');
            Err(Exception::from(IndexError::new(interp, message)))
        }
    } else if let Ok(index) = first.implicitly_convert_to_int() {
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
                string::format_int_into(&mut message, index)?;
                message.push_str(" too small for array; minimum: -");
                string::format_int_into(&mut message, len)?;
                Err(Exception::from(IndexError::new(interp, message)))
            }
        }
    } else {
        let rangelen = Int::try_from(len)
            .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
        match unsafe { is_range(interp, &first, rangelen) } {
            Ok(Some((start, len))) => {
                let start = usize::try_from(start).unwrap_or_else(|_| {
                    unimplemented!("should throw RangeError (-11..1 out of range)")
                });
                let len = usize::try_from(len)
                    .unwrap_or_else(|_| unreachable!("Range can't have negative length"));
                Ok((start, Some(len), second))
            }
            Ok(None) => {
                let start = first.funcall::<Value>("begin", &[], None)?;
                let start = start.implicitly_convert_to_int()?;
                let end = first.funcall::<Value>("last", &[], None)?;
                let end = end.implicitly_convert_to_int()?;
                // TODO: This conditional is probably not doing the right thing
                if start + (end - start) < 0 {
                    let mut message = String::new();
                    string::format_int_into(&mut message, start)?;
                    message.push_str("..");
                    string::format_int_into(&mut message, end)?;
                    message.push_str(" out of range");
                    return Err(Exception::from(RangeError::new(interp, message)));
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
                            string::format_int_into(&mut message, start)?;
                            message.push_str(" too small for array; minimum: -");
                            string::format_int_into(&mut message, len)?;
                            Err(Exception::from(IndexError::new(interp, message)))
                        }
                    }
                    (Ok(start), Err(_)) => Ok((start, None, second)),
                    (Err(_), Err(_)) => {
                        let mut message = String::from("index ");
                        string::format_int_into(&mut message, start)?;
                        message.push_str(" too small for array; minimum: -");
                        string::format_int_into(&mut message, len)?;
                        Err(Exception::from(IndexError::new(interp, message)))
                    }
                }
            }
            Err(_) => {
                let mut message = String::from("no implicit conversion of ");
                message.push_str(first.pretty_name());
                message.push_str(" into Integer");
                Err(Exception::from(TypeError::new(interp, message)))
            }
        }
    }
}

// TODO(GH-308): extract this function into `sys::protect`
unsafe fn is_range(
    interp: &Artichoke,
    range: &Value,
    length: Int,
) -> Result<Option<(Int, Int)>, Exception> {
    let mut start = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mut len = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mrb = interp.0.borrow().mrb;
    // NOTE: `mrb_range_beg_len` can raise.
    // TODO(GH-308): wrap this in a call to `mrb_protect`.
    let check_range = sys::mrb_range_beg_len(
        mrb,
        range.inner(),
        start.as_mut_ptr(),
        len.as_mut_ptr(),
        length,
        0_u8,
    );
    let start = start.assume_init();
    let len = len.assume_init();
    if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
        Ok(Some((start, len)))
    } else {
        Ok(None)
    }
}
