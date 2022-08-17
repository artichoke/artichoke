use crate::extn::core::time::Offset;
use crate::extn::prelude::*;
use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::core::symbol::Symbol;

const MAX_FLOAT_OFFSET: f64 = i32::MAX as f64;
const MIN_FLOAT_OFFSET: f64 = i32::MIN as f64;

impl TryConvertMut<Value, Option<Offset>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, options: Value) -> Result<Option<Offset>, Self::Error> {
        let hash: Vec<(Value, Value)> = self.try_convert_mut(options)?;

        // Short circuit the offset parameter fetching.
        if hash.len() == 0 {
            return Ok(None)
        }

        // Extract the value from the options hash with the key `:in`,
        // rejecting any other values as ArgumentError.
        let mut in_param: Option<Value> = None;

        for (mut key, value) in hash.iter() {
            let k = unsafe { Symbol::unbox_from_value(&mut key, self)? }.bytes(self);
            if k == b"in" {
                in_param = Some(*value)
            } else {
                let mut message = b"unknown keyword: ".to_vec();
                message.extend(k.to_vec());
                Err(ArgumentError::from(message))?
            }
        }

        if let Some(mut in_param) = in_param {
            match in_param.ruby_type() {
                Ruby::String => {
                    let offset_str = unsafe { implicitly_convert_to_string(self, &mut in_param) }?;

                    let offset = Offset::try_from(offset_str)
                        .map_err(|_| {
                            let mut message = b"\"+HH:MM\", \"-HH:MM\", \"UTC\" or \"A\"..\"I\",\"K\"..\"Z\" expected for utc_offset: ".to_vec();
                            message.extend(offset_str.to_vec());
                            ArgumentError::from(message)
                        })?;

                    Ok(Some(offset))
                },
                Ruby::Float => {
                    // This impl differs from MRI. MRI supports any float
                    // value and will set an offset with subsec fractions
                    // however this is not supported in spinoso_time with
                    // the `tzrs` feature.
                    let offset_seconds: f64 = self.try_convert(in_param)?;

                    if !(MIN_FLOAT_OFFSET..=MAX_FLOAT_OFFSET).contains(&offset_seconds) {
                        Err(ArgumentError::with_message("utc_offset out of range").into())
                    } else {
                        Ok(Some(Offset::try_from(offset_seconds as i32)?))
                    }
                },
                _ => {
                    let offset_seconds = implicitly_convert_to_int(self, in_param)
                        .and_then(|seconds| {
                            i32::try_from(seconds)
                                .map_err(|_| ArgumentError::with_message("utc_offset out of range").into())
                        })?;

                    Ok(Some(Offset::try_from(offset_seconds)?))
                }
            }
        } else {
            // The parameter parsing loop will rejected all params except `in`,
            // so the side affect is that this branch is never reachable.
            unreachable!("should not have attempted to parse an empty `in` option");
        }
    }
}
