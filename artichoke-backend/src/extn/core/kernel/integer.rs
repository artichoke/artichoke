use core::fmt::Write as _;

use scolapasta_int_parse::Radix;

use crate::extn::prelude::*;

impl TryConvertMut<Option<Value>, Option<Radix>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<Value>) -> Result<Option<Radix>, Self::Error> {
        if let Some(value) = value {
            let num = match value.try_convert_into::<Option<i64>>(self) {
                // nil and non-integer arguments are ignored.
                //
                // ```
                // [3.1.2] > Integer('999', nil)
                // => 999
                // [3.1.2] > Integer('999', Object.new)
                // => 999
                // [3.1.2] > Integer('0x999', nil)
                // => 2457
                // [3.1.2] > Integer('0x999', Object.new)
                // => 2457
                // ```
                Ok(None) | Err(_) => return Ok(None),
                Ok(Some(num)) => num,
            };
            let radix = if let Ok(radix) = u32::try_from(num) {
                radix
            } else {
                let num = num
                    .checked_neg()
                    .ok_or_else(|| ArgumentError::with_message("invalid radix"))?;
                match u32::try_from(num) {
                    // See https://github.com/ruby/ruby/blob/v2_6_3/bignum.c#L4106-L4110
                    Ok(1) => return Ok(None),
                    Ok(radix) => radix,
                    Err(_) => {
                        let mut message = String::new();
                        write!(&mut message, "invalid radix {}", num).map_err(WriteError::from)?;
                        return Err(ArgumentError::from(message).into());
                    }
                }
            };
            match Radix::new(radix) {
                Some(radix) => Ok(Some(radix)),
                // a zero radix means `Integer` should fall back to string parsing
                // of numeric literal prefixes.
                None if radix == 0 => Ok(None),
                None => {
                    let mut message = String::new();
                    write!(&mut message, "invalid radix {}", radix).map_err(WriteError::from)?;
                    Err(ArgumentError::from(message).into())
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl<'a> From<scolapasta_int_parse::ArgumentError<'a>> for Error {
    fn from(err: scolapasta_int_parse::ArgumentError<'a>) -> Self {
        let message = err.to_string();
        ArgumentError::from(message).into()
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::Radix;
    use crate::test::prelude::*;

    #[test]
    fn nil_radix_parses_to_none() {
        let mut interp = interpreter();
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(None);
        let result = result.unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn zero_radix_parses_to_none() {
        let mut interp = interpreter();
        let radix = interp.convert(0);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        let result = result.unwrap();
        assert_eq!(
            result, None,
            "0 radix should parse to None and fallback to literal prefix parsing"
        );
    }

    #[test]
    fn negative_one_radix_parses_to_none() {
        // ```
        // [3.1.2] > Integer('0x123f'.upcase, -1)
        // => 4671
        // [3.1.2] > Integer('0x123f'.upcase, 16)
        // => 4671
        let mut interp = interpreter();
        let radix = interp.convert(-1);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        let result = result.unwrap();
        assert_eq!(
            result, None,
            "-1 radix should parse to None and fallback to literal prefix parsing"
        );
    }

    #[test]
    fn one_radix_has_parse_failure() {
        let mut interp = interpreter();
        let radix = interp.convert(1);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 1".as_bstr()
        );
    }

    #[test]
    fn invalid_radix_has_parse_failure() {
        let mut interp = interpreter();
        let radix = interp.convert(12000);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 12000".as_bstr()
        );
    }

    #[test]
    fn invalid_negative_radix_has_parse_failure() {
        let mut interp = interpreter();
        let radix = interp.convert(-12000);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        // ```ruby
        // irb(main):003:0> Integer("123", -12000)
        // (irb):3:in `Integer': invalid radix 12000 (ArgumentError)
        // from (irb):3:in `<main>'
        // from C:/Ruby30-x64/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
        // from C:/Ruby30-x64/bin/irb.cmd:31:in `load'
        // from C:/Ruby30-x64/bin/irb.cmd:31:in `<main>'
        // ```
        assert_eq!(
            result.unwrap_err().message().as_bstr(),
            // should be:
            b"invalid radix 12000".as_bstr()
        );
    }

    #[test]
    fn positive_radix_in_valid_range_is_parsed() {
        let mut interp = interpreter();
        for r in 2_i32..=36_i32 {
            let radix = interp.convert(r);
            let expected = Radix::new(r.try_into().unwrap()).unwrap();
            let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
            let result = result.unwrap();
            assert_eq!(result, Some(expected), "expected {} to parse to Radix({})", r, r);
        }
    }

    #[test]
    fn negative_radix_in_valid_range_is_parsed() {
        let mut interp = interpreter();
        for r in 2_i32..=36_i32 {
            let radix = interp.convert(-r);
            let expected = Radix::new(r.try_into().unwrap()).unwrap();
            let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
            let result = result.unwrap();
            assert_eq!(result, Some(expected), "expected -{} to parse to Radix({})", r, r);
        }
    }

    #[test]
    fn int_max_min_radix_do_not_panic() {
        let mut interp = interpreter();
        let radix = interp.convert(i64::MAX);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        result.unwrap_err();

        let radix = interp.convert(i64::MIN);
        let result: Result<Option<Radix>, _> = interp.try_convert_mut(Some(radix));
        result.unwrap_err();
    }
}
