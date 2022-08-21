//! Parser for Ruby Time offset parameter to help generate `Time`.

use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::core::symbol::Symbol;
use crate::extn::core::time::Offset;
use crate::extn::prelude::*;

#[allow(clippy::cast_precision_loss)]
const MAX_FLOAT_OFFSET: f64 = i32::MAX as f64;
#[allow(clippy::cast_precision_loss)]
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
        if hash.is_empty() {
            return Ok(None);
        }

        // All other keys are rejected.
        //
        // Example:
        // ```console
        // [2.6.3]> Time.at(0, i: 0)
        // ArgumentError (unknown keyword: i)
        // ```
        //
        // FIXME: In Ruby 3.1.2, this exception message formats the symbol with
        // `Symbol#inspect`:
        //
        // ```console
        // [3.1.2] > Time.at(0, i: 0)
        // <internal:timev>:270:in `at': unknown keyword: :i (ArgumentError)
        // ```
        for (mut key, _) in &hash {
            let k = unsafe { Symbol::unbox_from_value(&mut key, self)? }.bytes(self);
            if k != b"in" {
                let mut message = b"unknown keyword: ".to_vec();
                message.extend_from_slice(k);
                Err(ArgumentError::from(message))?;
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
                        br#"+HH:MM", "-HH:MM", "UTC" or "A".."I","K".."Z" expected for utc_offset: "#.to_vec();
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

                if (MIN_FLOAT_OFFSET..=MAX_FLOAT_OFFSET).contains(&offset_seconds) {
                    #[allow(clippy::cast_possible_truncation)]
                    Ok(Some(Offset::try_from(offset_seconds as i32)?))
                } else {
                    Err(ArgumentError::with_message("utc_offset out of range").into())
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

#[cfg(test)]
mod tests {
    use crate::extn::core::time::Offset;
    use crate::test::prelude::*;

    #[test]
    fn no_options_does_not_raise() {
        let mut interp = interpreter();

        let options = interp.eval(b"{}").unwrap();

        let offset: Option<Offset> = interp.try_convert_mut(options).unwrap();
        assert_eq!(offset, None);
    }

    #[test]
    fn raises_on_keys_except_in() {
        let mut interp = interpreter();

        let options = interp.eval(b"{ foo: 'bar' }").unwrap();

        let result: Result<Option<Offset>, Error> = interp.try_convert_mut(options);
        let error = result.unwrap_err();

        assert_eq!(error.name(), "ArgumentError");
        assert_eq!(error.message(), b"unknown keyword: foo".as_slice());
    }

    #[test]
    fn raises_on_invalid_timezone_string() {
        let mut interp = interpreter();

        let options = interp.eval(b"{ in: 'J' }").unwrap();

        let result: Result<Option<Offset>, Error> = interp.try_convert_mut(options);
        let error = result.unwrap_err();

        assert_eq!(error.name(), "ArgumentError");
        assert_eq!(
            error.message(),
            br#"+HH:MM", "-HH:MM", "UTC" or "A".."I","K".."Z" expected for utc_offset: J"#.as_slice()
        );
    }

    #[test]
    fn provides_an_int_based_offset() {
        let mut interp = interpreter();

        let options = interp.eval(b"{ in: 3600 }").unwrap();

        let result: Option<Offset> = interp.try_convert_mut(options).unwrap();
        assert_eq!(result.unwrap(), Offset::fixed(3600).unwrap());
    }

    #[test]
    fn provides_a_float_based_offset() {
        let mut interp = interpreter();

        let options = interp.eval(b"{ in: 3600.0 }").unwrap();

        let result: Option<Offset> = interp.try_convert_mut(options).unwrap();
        assert_eq!(result.unwrap(), Offset::fixed(3600).unwrap());
    }

    #[test]
    fn provides_a_string_based_offset() {
        let mut interp = interpreter();

        let options = interp.eval(b"{ in: 'A' }").unwrap();

        let result: Option<Offset> = interp.try_convert_mut(options).unwrap();
        assert_eq!(result.unwrap(), Offset::fixed(3600).unwrap());
    }

    #[test]
    fn raises_on_float_out_of_range() {
        let mut interp = interpreter();

        // this value is i32::MIN - 1.
        let options = interp.eval(b"{ in: -2_147_483_649.00 }").unwrap();

        let result: Result<Option<Offset>, Error> = interp.try_convert_mut(options);
        let error = result.unwrap_err();

        assert_eq!(error.message(), b"utc_offset out of range".as_slice());
        assert_eq!(error.name(), "ArgumentError");

        // this value is i32::MAX + 1.
        let options = interp.eval(b"{ in: 2_147_483_648.00 }").unwrap();

        let result: Result<Option<Offset>, Error> = interp.try_convert_mut(options);
        let error = result.unwrap_err();

        assert_eq!(error.message(), b"utc_offset out of range".as_slice());
        assert_eq!(error.name(), "ArgumentError");
    }

    #[test]
    fn raises_on_int_out_of_range() {
        let mut interp = interpreter();

        // this value is i32::MIN - 1.
        let options = interp.eval(b"{ in: -2_147_483_649 }").unwrap();

        let result: Result<Option<Offset>, Error> = interp.try_convert_mut(options);
        let error = result.unwrap_err();

        assert_eq!(error.message(), b"utc_offset out of range".as_slice());
        assert_eq!(error.name(), "ArgumentError");

        // this value is i32::MAX + 1.
        let options = interp.eval(b"{ in: 2_147_483_648 }").unwrap();

        let result: Result<Option<Offset>, Error> = interp.try_convert_mut(options);
        let error = result.unwrap_err();

        assert_eq!(error.message(), b"utc_offset out of range".as_slice());
        assert_eq!(error.name(), "ArgumentError");
    }
}
