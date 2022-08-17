use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::core::symbol::Symbol;
use crate::extn::core::time::Offset;
use crate::extn::prelude::*;

const MAX_FLOAT_OFFSET: f64 = i32::MAX as f64;
const MIN_FLOAT_OFFSET: f64 = i32::MIN as f64;

impl TryConvertMut<Value, Option<Offset>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, options: Value) -> Result<Option<Offset>, Self::Error> {
        let hash: Vec<(Value, Value)> = self.try_convert_mut(options)?;

        // Short circuit. An empty options hash does not error.
        //
        // Example:
        //
        // ```console
        // [2.6.3]> Time.at(0, {})
        // => 1970-01-01 01:00:00 +0100
        // ```
        if hash.len() == 0 {
            return Ok(None);
        }

        // All other keys are rejected.
        //
        // Example:
        // ```console
        // [2.6.3]> Time.at(0, i: 0)
        // ArgumentError (unknown keyword: i)
        // ```
        for (mut key, _) in hash.iter() {
            let k = unsafe { Symbol::unbox_from_value(&mut key, self)? }.bytes(self);
            if k != b"in" {
                let mut message = b"unknown keyword: ".to_vec();
                message.extend_from_slice(k);
                Err(ArgumentError::from(message))?
            }
        }

        // Based on the above logic, the only option in the hash is `in`.
        // >0 keys, and all other keys are rejected).
        let mut in_value = hash.get(0).expect("Only the `in` parameter should be available").1;

        match in_value.ruby_type() {
            Ruby::String => {
                let offset_str = unsafe { implicitly_convert_to_string(self, &mut in_value) }?;

                let offset = Offset::try_from(offset_str).map_err(|_| {
                    let mut message =
                        b"\"+HH:MM\", \"-HH:MM\", \"UTC\" or \"A\"..\"I\",\"K\"..\"Z\" expected for utc_offset: "
                            .to_vec();
                    message.extend_from_slice(offset_str);
                    ArgumentError::from(message)
                })?;

                Ok(Some(offset))
            }
            Ruby::Float => {
                // This impl differs from MRI. MRI supports any float value and
                // will set an offset with subsec fractions however this is not
                // supported in spinoso_time with the `tzrs` feature.
                let offset_seconds: f64 = self.try_convert(in_value)?;

                if !(MIN_FLOAT_OFFSET..=MAX_FLOAT_OFFSET).contains(&offset_seconds) {
                    Err(ArgumentError::with_message("utc_offset out of range").into())
                } else {
                    Ok(Some(Offset::try_from(offset_seconds as i32)?))
                }
            }
            _ => {
                let offset_seconds = implicitly_convert_to_int(self, in_value).and_then(|seconds| {
                    i32::try_from(seconds).map_err(|_| ArgumentError::with_message("utc_offset out of range").into())
                })?;

                Ok(Some(Offset::try_from(offset_seconds)?))
            }
        }
    }
}
