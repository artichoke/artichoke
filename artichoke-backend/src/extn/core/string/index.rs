use artichoke_core::value::Value as _;
use bstr::ByteSlice;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::regexp::Regexp;
use crate::types::Int;
use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;

fn start_index(
    interp: &Artichoke,
    string: &[u8],
    offset: Option<Value>,
) -> Result<Option<usize>, Box<dyn RubyException>> {
    if let Some(offset) = offset {
        let index = offset.implicitly_convert_to_int()?;
        let index = if index < 0 {
            let idx = usize::try_from(-index).map_err(|_| {
                Fatal::new(interp, "Expected positive position to convert to usize")
            })?; 
            string.len().checked_sub(idx)
        } else {
            let idx = usize::try_from(index).map_err(|_| {
                Fatal::new(interp, "Expected positive position to convert to usize")
            })?;
            Some(idx)
        };
        if let Some(idx) = index  {
            if idx > string.len() {
                return Ok(None);
            }
        }
        Ok(index)
    } else {
        Ok(Some(0_usize))
    }
}

pub fn method(
    interp: &Artichoke,
    value: Value,
    pattern: Value,
    offset: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    let string = value.clone().try_into::<&[u8]>().map_err(|_| {
        Fatal::new(
            interp,
            "Unable to convert Ruby String Receiver to Rust byte slice",
        )
    })?;
    if let Ruby::Symbol = pattern.ruby_type() {
        Err(Box::new(TypeError::new(
            interp,
            format!("type mismatch: {} given", pattern.pretty_name()),
        )))
    } else if let Ok(pattern_bytes) = pattern.clone().try_into::<&[u8]>() {
        let start = if let Some(start) = start_index(interp, string, offset)? {
            start
        } else {
            return Ok(interp.convert(None::<Value>));
        };
        let string = &string[start..];
        if let Some(result) = string.find(pattern_bytes) {
            let result = result + start;
            let result = Int::try_from(result)
                .map_err(|_| Fatal::new(interp, "Index pos does not fit in Integer"))?;
            Ok(interp.convert(result))
        } else {
            Ok(interp.convert(None::<Value>))
        }
    } else if let Ok(pattern_bytes) = pattern.funcall::<&[u8]>("to_str", &[], None) {
       let start = if let Some(start) = start_index(interp, string, offset)? {
            start
        } else {
            return Ok(interp.convert(None::<Value>));
        };
        let string = &string[start..];
        if let Some(result) = string.find(pattern_bytes) {
            let result = result + start;
            let result = Int::try_from(result)
                .map_err(|_| Fatal::new(interp, "Index pos does not fit in Integer"))?;
            Ok(interp.convert(result))
        } else {
            Ok(interp.convert(None::<Value>))
        }
    } else if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
        let start = if let Some(start) = start_index(interp, string, offset)? {
            start
        } else {
            return Ok(interp.convert(None::<Value>));
        };
        if let Some((begin, _)) = regexp.borrow().inner().pos(interp, &string[start..], 0)? {
            let result = begin + start;
            let result = Int::try_from(result)
                .map_err(|_| Fatal::new(interp, "Index pos does not fit in Integer"))?;
            Ok(interp.convert(result))
        } else {
            Ok(interp.convert(None::<Value>))
        }
    } else {
        let pattern_type_name = pattern.pretty_name();
        Err(Box::new(TypeError::new(
            interp,
            format!("type mismatch: {} given", pattern_type_name)
        )))
    }
}
