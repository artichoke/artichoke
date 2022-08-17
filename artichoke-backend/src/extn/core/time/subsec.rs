///! Parser of Ruby Time subsecond parameters to help generate `Time`.
///!
///! This module implements the logic to parse two optional parameters in the
///! `Time.at` function call. These parameters (if specified) provide the number
///! of subsecond parts to add, and a scale of those subsecond parts (millis, micros,
///! and nanos).

use crate::extn::prelude::*;
use crate::extn::core::symbol::Symbol;
use crate::convert::implicitly_convert_to_int;

const NANOS_IN_SECOND: i64 = 1_000_000_000;

const MILLIS_IN_NANO: i64 = 1_000_000;
const MICROS_IN_NANO: i64 = 1_000;
const NANOS_IN_NANO: i64 = 1;

enum SubsecMultiplier {
    Millis,
    Micros,
    Nanos,
}

impl SubsecMultiplier {
    const fn as_nanos(self) -> i64 {
        match self {
           Self::Millis => MILLIS_IN_NANO,
           Self::Micros => MICROS_IN_NANO,
           Self::Nanos => NANOS_IN_NANO,
        }
    }
}

impl TryConvertMut<Option<Value>, SubsecMultiplier> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, subsec_type: Option<Value>) -> Result<SubsecMultiplier, Self::Error> {
        if let Some(mut subsec_type) = subsec_type {
            let subsec_symbol = unsafe { Symbol::unbox_from_value(&mut subsec_type, self)? }.bytes(self);
            match subsec_symbol {
                b"milliseconds" => Ok(SubsecMultiplier::Millis),
                b"usec" => Ok(SubsecMultiplier::Micros),
                b"nsec" => Ok(SubsecMultiplier::Nanos),
                _ => Err(ArgumentError::with_message("unexpected unit. expects :milliseconds, :usec, :nsec").into()),
            }
        } else {
            Ok(SubsecMultiplier::Micros)
        }
    }
}



#[derive(Debug, Copy, Clone)]
pub struct Subsec {
    secs: i64,
    nanos: u32,
}

impl Subsec {
    /// Returns a tuple of (seconds, nanoseconds). Subseconds are provided in
    /// various accuracies, and can overflow. e.g. 1001 milliseconds, is 1
    /// second, and 1_000_000 nanoseconds.
    pub fn to_tuple(&self) -> (i64, u32) {
        (self.secs, self.nanos)
    }
}


impl TryConvertMut<(Option<Value>, Option<Value>), Subsec> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, params: (Option<Value>, Option<Value>)) -> Result<Subsec, Self::Error> {
        let (subsec, subsec_type) = params;

        if let Some(subsec) = subsec {
            let multiplier: SubsecMultiplier = self.try_convert_mut(subsec_type)?;
            let multiplier_nanos = multiplier.as_nanos();
            let seconds_base = NANOS_IN_SECOND / multiplier_nanos;

            match subsec.ruby_type() {
                Ruby::Float => {
                    // TODO: Safe conversions here are really hard, there may end up being some
                    // loss in accuracy.
                    unreachable!("Not yet implemented")
                },
                _ => {
                    let subsec: i64 = implicitly_convert_to_int(self, subsec)?;

                    // The below calculations should always be safe. The
                    // multiplier is gauranteed to not be 0, the remainder
                    // should never overflow, and is gauranteed to be less
                    // than u32::MAX.
                    let mut secs = subsec / seconds_base;
                    let mut nanos = (subsec % seconds_base) * multiplier_nanos;

                    if subsec.is_negative() {
                        // Nanos always needs to be a positive u32. If subsec
                        // is negative, we will always need remove one second.
                        // Nanos can then be adjusted since it will always be
                        // the inverse of the total nanos in a second.
                        secs = secs.checked_sub(1)
                            .ok_or(ArgumentError::with_message("Time too small"))?;

                        if nanos.signum() != 0 {
                            nanos += NANOS_IN_SECOND;
                        }
                    }

                    // Cast to u32 is safe since it will always be less than NANOS_IN_SECOND due to modulo and negative adjustments.
                    Ok(Subsec { secs, nanos: nanos as u32 })
                }
            }
        } else {
            Ok(Subsec { secs: 0, nanos: 0 })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    use super::Subsec;
    use bstr::ByteSlice;

    fn subsec(interp: &mut Artichoke, params: (Option<&[u8]>, Option<&[u8]>)) -> Result<Subsec, Error> {
        let (subsec, subsec_type) = params;
        let subsec = subsec.map(|s| interp.eval(s).unwrap());
        let subsec_type = subsec_type.map(|s| interp.eval(s).unwrap());

        interp.try_convert_mut((subsec, subsec_type))
    }

    #[test]
    fn no_subsec_provided() {
        let mut interp = interpreter();

        let result: Subsec = interp.try_convert_mut((None, None)).unwrap();
        let (secs, nanos) = result.to_tuple();
        assert_eq!(secs, 0);
        assert_eq!(nanos, 0);
    }

    #[test]
    fn no_subsec_provided_but_has_unit() {
        let mut interp = interpreter();
        let unit = interp.eval(b":usec").unwrap();

        let result: Subsec = interp.try_convert_mut((None, Some(unit))).unwrap();
        let (secs, nanos) = result.to_tuple();
        assert_eq!(secs, 0);
        assert_eq!(nanos, 0);
    }

    #[test]
    fn no_unit_implies_micros() {
        let mut interp = interpreter();

        let expectations = [
            (b"-1000001".as_slice(), (-2, 999_999_000)),
            (b"-1000000".as_slice(), (-2, 0)),
            (b"-999999".as_slice(), (-1, 1_000)),
            (b"-1".as_slice(), (-1, 999_999_000)),
            (b"0".as_slice(), (0, 0)),
            (b"1".as_slice(), (0, 1_000)),
            (b"999999".as_slice(), (0, 999_999_000)),
            (b"1000000".as_slice(), (1, 0)),
            (b"1000001".as_slice(), (1, 1_000))
        ];

        let subsec_unit = None;

        for (input, expectation) in expectations.iter() {
            let result = subsec(&mut interp, (Some(input), subsec_unit)).unwrap();
            assert_eq!(
                result.to_tuple(),
                *expectation,
                "Expected TryConvertMut<(Some({}), None), Result<Subsec>>, to return {} secs, {} nanos",
                input.as_bstr(),
                expectation.0,
                expectation.1
            );
        }
    }

    #[test]
    fn subsec_millis() {
        let mut interp = interpreter();

        let expectations = [
            (b"-1001".as_slice(), (-2, 999_000_000)),
            (b"-1000".as_slice(), (-2, 0)),
            (b"-999".as_slice(), (-1, 1_000_000)),
            (b"-1".as_slice(), (-1, 999_000_000)),
            (b"0".as_slice(), (0, 0)),
            (b"1".as_slice(), (0, 1_000_000)),
            (b"999".as_slice(), (0, 999_000_000)),
            (b"1000".as_slice(), (1, 0)),
            (b"1001".as_slice(), (1, 1_000_000))
        ];

        let subsec_unit = b":milliseconds";

        for (input, expectation) in expectations.iter() {
            let result = subsec(&mut interp, (Some(input), Some(subsec_unit))).unwrap();
            assert_eq!(
                result.to_tuple(),
                *expectation,
                "Expected TryConvertMut<(Some({}), Some({})), Result<Subsec>>, to return {} secs, {} nanos",
                input.as_bstr(),
                subsec_unit.as_bstr(),
                expectation.0,
                expectation.1
            );
        }
    }

    #[test]
    fn subsec_micros() {
        let mut interp = interpreter();

        //let expectations: [(&[u8], (i64, u32))] = [
        let expectations = [
            (b"-1000001".as_slice(), (-2, 999_999_000)),
            (b"-1000000".as_slice(), (-2, 0)),
            (b"-999999".as_slice(), (-1, 1_000)),
            (b"-1".as_slice(), (-1, 999_999_000)),
            (b"0".as_slice(), (0, 0)),
            (b"1".as_slice(), (0, 1_000)),
            (b"999999".as_slice(), (0, 999_999_000)),
            (b"1000000".as_slice(), (1, 0)),
            (b"1000001".as_slice(), (1, 1_000))
        ];

        let subsec_unit = b":usec";

        for (input, expectation) in expectations.iter() {
            let result = subsec(&mut interp, (Some(input), Some(subsec_unit))).unwrap();
            assert_eq!(
                result.to_tuple(),
                *expectation,
                "Expected TryConvertMut<(Some({}), Some({})), Result<Subsec>>, to return {} secs, {} nanos",
                input.as_bstr(),
                subsec_unit.as_bstr(),
                expectation.0,
                expectation.1
            );
        }
    }

    #[test]
    fn subsec_nanos() {
        let mut interp = interpreter();

        let expectations = [
            (b"-1000000001".as_slice(), (-2, 999_999_999)),
            (b"-1000000000".as_slice(), (-2, 0)),
            (b"-999999999".as_slice(), (-1, 1)),
            (b"-1".as_slice(), (-1, 999_999_999)),
            (b"0".as_slice(), (0, 0)),
            (b"1".as_slice(), (0, 1)),
            (b"999999999".as_slice(), (0, 999_999_999)),
            (b"1000000000".as_slice(), (1, 0)),
            (b"1000000001".as_slice(), (1, 1))
        ];

        let subsec_unit = b":nsec";

        for (input, expectation) in expectations.iter() {
            let result = subsec(&mut interp, (Some(input), Some(subsec_unit))).unwrap();
            assert_eq!(
                result.to_tuple(),
                *expectation,
                "Expected TryConvertMut<(Some({}), Some({})), Result<Subsec>>, to return {} secs, {} nanos",
                input.as_bstr(),
                subsec_unit.as_bstr(),
                expectation.0,
                expectation.1
            );
        }
    }
}
