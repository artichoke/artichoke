use std::fmt::Write as _;

use crate::convert::implicitly_convert_to_int;
use crate::extn::prelude::*;
use crate::fmt::WriteError;
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
        return if let Ok(len) = usize::try_from(len) {
            Ok(ElementReference::StartLen(start, len))
        } else {
            Ok(ElementReference::Empty)
        };
    }
    let rangelen = i64::try_from(ary_len).map_err(|_| Fatal::from("Range length exceeds Integer max"))?;
    match elem.is_range(interp, rangelen)? {
        None => {
            let index = implicitly_convert_to_int(interp, elem)?;
            Ok(ElementReference::Index(index))
        }
        // ```
        // [3.1.2] > a = []
        // => []
        // [3.1.2] > a[-1..-1]
        // => nil
        // ```
        Some(protect::Range::Out) => Ok(ElementReference::Empty),
        Some(protect::Range::Valid { start, len }) => {
            if let Ok(len) = usize::try_from(len) {
                Ok(ElementReference::StartLen(start, len))
            } else {
                Ok(ElementReference::Empty)
            }
        }
    }
}

pub fn element_assignment(
    interp: &mut Artichoke,
    first: Value,
    second: Value,
    third: Option<Value>,
    len: usize,
) -> Result<(usize, Option<usize>, Value), Error> {
    if let Some(elem) = third {
        let start = implicitly_convert_to_int(interp, first)?;
        let start = if let Some(start) = aref::offset_to_index(start, len) {
            start
        } else {
            let mut message = String::new();
            write!(&mut message, "index {} too small for array; minimum: -{}", start, len)
                .map_err(WriteError::from)?;
            return Err(IndexError::from(message).into());
        };
        let slice_len = implicitly_convert_to_int(interp, second)?;
        if let Ok(slice_len) = usize::try_from(slice_len) {
            Ok((start, Some(slice_len), elem))
        } else {
            let mut message = String::new();
            write!(&mut message, "negative length ({})", slice_len).map_err(WriteError::from)?;
            Err(IndexError::from(message).into())
        }
    } else {
        let rangelen = i64::try_from(len).map_err(|_| Fatal::from("Range length exceeds Integer max"))?;
        match first.is_range(interp, rangelen)? {
            None => {
                let index = implicitly_convert_to_int(interp, first)?;
                if let Some(index) = aref::offset_to_index(index, len) {
                    Ok((index, None, second))
                } else {
                    let mut message = String::new();
                    write!(&mut message, "index {} too small for array; minimum: -{}", index, len)
                        .map_err(WriteError::from)?;
                    Err(IndexError::from(message).into())
                }
            }
            // ```
            // [3.1.2] > a = []
            // => []
            // [3.1.2] > a[-1..-1] = 'x'
            // (irb):13:in `[]=': -1..-1 out of range (RangeError)
            //         from (irb):13:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            Some(protect::Range::Out) => Err(RangeError::with_message("out of range").into()),
            Some(protect::Range::Valid { start, len }) => {
                let start = usize::try_from(start)
                    .unwrap_or_else(|_| unimplemented!("should throw RangeError (-11..1 out of range)"));
                let len = usize::try_from(len).unwrap_or_else(|_| unreachable!("Range can't have negative length"));
                Ok((start, Some(len), second))
            }
        }
    }
}
