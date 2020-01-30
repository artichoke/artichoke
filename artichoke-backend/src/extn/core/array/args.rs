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
    } else {
        let name = elem.pretty_name();
        if let Ok(index) = elem.implicitly_convert_to_int() {
            Ok(ElementReference::Index(index))
        } else {
            let rangelen = Int::try_from(ary_len)
                .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
            match unsafe { is_range(interp, &elem, rangelen) } {
                Ok(Some((start, len))) => Ok(ElementReference::StartLen(start, len)),
                Ok(None) => Ok(ElementReference::Empty),
                Err(_) => Err(Exception::from(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", name),
                ))),
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
            let start = usize::try_from(start)
                .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
            if start < len {
                len - start
            } else {
                return Err(Exception::from(IndexError::new(
                    interp,
                    format!("index {} too small for array; minimum: -{}", start, len),
                )));
            }
        };
        let slice_len = second.implicitly_convert_to_int()?;
        if let Ok(slice_len) = usize::try_from(slice_len) {
            Ok((start, Some(slice_len), elem))
        } else {
            Err(Exception::from(IndexError::new(
                interp,
                format!("negative length ({})", slice_len),
            )))
        }
    } else if let Ok(index) = first.implicitly_convert_to_int() {
        if let Ok(index) = usize::try_from(index) {
            Ok((index, None, second))
        } else {
            let index = usize::try_from(-index)
                .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
            if index < len {
                Ok((len - index, None, second))
            } else {
                Err(Exception::from(IndexError::new(
                    interp,
                    format!("index {} too small for array; minimum: -{}", index, len),
                )))
            }
        }
    } else {
        let rangelen = Int::try_from(len)
            .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
        match unsafe { is_range(interp, &first, rangelen) } {
            Ok(Some((start, len))) => {
                if let Ok(start) = usize::try_from(start) {
                    Ok((start, Some(len), second))
                } else {
                    Ok((0, Some(len), second))
                }
            }
            Ok(None) => {
                let start = if let Ok(start) = first.funcall::<Value>("begin", &[], None) {
                    start
                } else {
                    return Err(Exception::from(Fatal::new(
                        interp,
                        "Unable to extract first from Range",
                    )));
                };
                let start = start.implicitly_convert_to_int()?;
                let end = if let Ok(end) = first.funcall::<Value>("last", &[], None) {
                    end
                } else {
                    return Err(Exception::from(Fatal::new(
                        interp,
                        "Unable to extract first from Range",
                    )));
                };
                let end = end.implicitly_convert_to_int()?;
                if start + (end - start) < 0 {
                    return Err(Exception::from(RangeError::new(
                        interp,
                        format!("{}..{} out of range", start, end),
                    )));
                }
                match (usize::try_from(start), usize::try_from(end)) {
                    (Ok(start), Ok(end)) => {
                        if end > start {
                            Ok((start, Some(end - start), second))
                        } else {
                            Ok((start, None, second))
                        }
                    }
                    (Err(_), Ok(end)) => {
                        let start = usize::try_from(start)
                            .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
                        if start < len {
                            let start = len - start;
                            if end > start {
                                Ok((start, Some(end - start), second))
                            } else {
                                Ok((start, None, second))
                            }
                        } else {
                            Err(Exception::from(IndexError::new(
                                interp,
                                format!("index {} too small for array; minimum: -{}", start, len),
                            )))
                        }
                    }
                    (Ok(start), Err(_)) => Ok((start, None, second)),
                    (Err(_), Err(_)) => Err(Exception::from(IndexError::new(
                        interp,
                        format!("index {} too small for array; minimum: -{}", start, len),
                    ))),
                }
            }
            Err(_) => {
                let index_type_name = first.pretty_name();
                Err(Exception::from(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", index_type_name),
                )))
            }
        }
    }
}

unsafe fn is_range(
    interp: &Artichoke,
    range: &Value,
    length: Int,
) -> Result<Option<(Int, usize)>, Exception> {
    let mut start = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mut len = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mrb = interp.0.borrow().mrb;
    // `mrb_range_beg_len` can raise.
    // TODO: Wrap this in a call to `mrb_protect`.
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
        let len = usize::try_from(len)
            .map_err(|_| TypeError::new(interp, "no implicit conversion into Integer"))?;
        Ok(Some((start, len)))
    } else {
        Ok(None)
    }
}
