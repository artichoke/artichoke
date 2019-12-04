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
) -> Result<i64, Box<dyn RubyException>> {
    if let Some(offset) = offset {
        let mut start = if let Ok(start) = offset.clone().try_into::<Int>() {
            start
        } else if let Ok(start) = offset.funcall::<Int>("to_int", &[], None) {
            start
        } else {
            let offset_type_name = offset.pretty_name();
            return Err(Box::new(TypeError::new(
                interp,
                format!("no implicit conversion {} into Integer", offset_type_name),
            )));
        };
        if start < 0 {
            start += string.len() as i64;
        }
        Ok(start)
    } else {
        Ok(0)
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
    } else if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &pattern) } {
        let start = start_index(interp, string, offset)?;
        if start < 0 || start > string.len() as i64 {
            return Ok(interp.convert(None::<Value>));
        }
        let start = usize::try_from(start).unwrap();
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
        let pattern_bytes = match (
            pattern.clone().try_into::<&[u8]>(),
            pattern.funcall::<&[u8]>("to_str", &[], None),
        ) {
            (Ok(a), Ok(_)) | (Ok(a), Err(_)) => a,
            (Err(_), Ok(b)) => b,
            (Err(_), Err(_)) => {
                return Err(Box::new(TypeError::new(
                    interp,
                    format!("type mismatch: {} given", pattern_type_name,),
                )))
            }
        };
        let start = start_index(interp, string, offset)?;
        if start < 0 || start > string.len() as i64 {
            return Ok(interp.convert(None::<Value>));
        }
        let start = usize::try_from(start).unwrap();
        let string = &string[start..];
        if let Some(result) = string.find(pattern_bytes) {
            let result = result + start;
            let result = Int::try_from(result)
                .map_err(|_| Fatal::new(interp, "Index pos does not fit in Integer"))?;
            Ok(interp.convert(result))
        } else {
            Ok(interp.convert(None::<Value>))
        }
    }
}
