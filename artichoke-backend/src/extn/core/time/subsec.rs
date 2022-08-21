//! Parser for Ruby Time subsecond parameters to help generate `Time`.
//!
//! This module implements the logic to parse two optional parameters in the
//! `Time.at` function call. These parameters (if specified) provide the number
//! of subsecond parts to add, and a scale of those subsecond parts (millis, micros,
//! and nanos).

use crate::convert::implicitly_convert_to_int;
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

const NANOS_IN_SECOND: i64 = 1_000_000_000;

const MILLIS_IN_NANO: i64 = 1_000_000;
const MICROS_IN_NANO: i64 = 1_000;
const NANOS_IN_NANO: i64 = 1;

#[allow(clippy::cast_precision_loss)]
const MIN_FLOAT_SECONDS: f64 = i64::MIN as f64;
#[allow(clippy::cast_precision_loss)]
const MAX_FLOAT_SECONDS: f64 = i64::MAX as f64;
const MIN_FLOAT_NANOS: f64 = 0.0;
#[allow(clippy::cast_precision_loss)]
const MAX_FLOAT_NANOS: f64 = NANOS_IN_SECOND as f64;

enum SubsecMultiplier {
    Millis,
    Micros,
    Nanos,
}

impl SubsecMultiplier {
    #[must_use]
    const fn as_nanos(&self) -> i64 {
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
                _ => {
                    let mut message = b"unexpected unit: ".to_vec();
                    message.extend_from_slice(subsec_symbol);
                    Err(ArgumentError::from(message).into())
                }
            }
        } else {
            Ok(SubsecMultiplier::Micros)
        }
    }
}

/// A struct that represents the adjustment needed to a `Time` based on a
/// the parsing of optional Ruby Values. Seconds can require adjustment as a
/// means for handling overflow of values. e.g. `1_001` millis can be requested
/// which should result in 1 seconds, and `1_000_000` nanoseconds.
///
/// Note: Negative nanoseconds are not supported, thus any negative adjustment
/// will generally result in at least -1 second, and the relevant positive
/// amount of nanoseconds. e.g. `-1_000` microseconds should result in -1
/// second, and `999_999_000` nanoseconds.
#[derive(Debug, Copy, Clone)]
pub struct Subsec {
    secs: i64,
    nanos: u32,
}

impl Subsec {
    /// Returns a tuple of (seconds, nanoseconds). Subseconds are provided in
    /// various accuracies, and can overflow. e.g. 1001 milliseconds, is 1
    /// second, and `1_000_000` nanoseconds.
    #[must_use]
    pub fn to_tuple(&self) -> (i64, u32) {
        (self.secs, self.nanos)
    }
}

impl TryConvertMut<(Option<Value>, Option<Value>), Subsec> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, params: (Option<Value>, Option<Value>)) -> Result<Subsec, Self::Error> {
        let (subsec, subsec_unit) = params;

        if let Some(subsec) = subsec {
            let multiplier: SubsecMultiplier = self.try_convert_mut(subsec_unit)?;
            let multiplier_nanos = multiplier.as_nanos();
            // `subsec` represents the user provided value in `subsec_unit`
            // resolution. The base used to derive the number of seconds is
            // based on the `subsec_unit`. e.g. `1_001` milliseconds is 1
            // second, and `1_000_000` nanoseconds.
            let seconds_base = NANOS_IN_SECOND / multiplier_nanos;

            if subsec.ruby_type() == Ruby::Float {
                // FIXME: The below deviates from the MRI implementation of
                // Time. MRI uses `to_r` for subsec calculation on floats
                // subsec nanos, and this could result in different values.

                let subsec: f64 = self.try_convert(subsec)?;

                if subsec.is_nan() {
                    return Err(FloatDomainError::with_message("NaN").into());
                }
                if subsec.is_infinite() {
                    return Err(FloatDomainError::with_message("Infinity").into());
                }

                // These conversions are luckily not lossy. `seconds_base`
                // and `multiplier_nanos` are gauranteed to be represented
                // without loss in a f64.
                #[allow(clippy::cast_precision_loss)]
                let seconds_base = seconds_base as f64;
                #[allow(clippy::cast_precision_loss)]
                let multiplier_nanos = multiplier_nanos as f64;

                let mut secs = subsec / seconds_base;
                let mut nanos = (subsec % seconds_base) * multiplier_nanos;

                // is_sign_negative() is not enough here, since this logic
                // should also be skilled for negative zero.
                if subsec < -0.0 {
                    // Nanos always needs to be a positive u32. If subsec
                    // is negative, we will always need remove one second.
                    // Nanos can then be adjusted since it will always be
                    // the inverse of the total nanos in a second.
                    secs -= 1.0;

                    #[allow(clippy::cast_precision_loss)]
                    if nanos != 0.0 && nanos != -0.0 {
                        nanos += NANOS_IN_SECOND as f64;
                    }
                }

                if !(MIN_FLOAT_SECONDS..=MAX_FLOAT_SECONDS).contains(&secs)
                    || !(MIN_FLOAT_NANOS..=MAX_FLOAT_NANOS).contains(&nanos)
                {
                    return Err(ArgumentError::with_message("subsec outside of bounds").into());
                }

                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                Ok(Subsec {
                    secs: secs as i64,
                    nanos: nanos as u32,
                })
            } else {
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
                    secs = secs
                        .checked_sub(1)
                        .ok_or(ArgumentError::with_message("Time too small"))?;

                    if nanos.signum() != 0 {
                        nanos += NANOS_IN_SECOND;
                    }
                }

                // Cast to u32 is safe since it will always be less than NANOS_IN_SECOND due to modulo and negative adjustments.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                Ok(Subsec {
                    secs,
                    nanos: nanos as u32,
                })
            }
        } else {
            Ok(Subsec { secs: 0, nanos: 0 })
        }
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::Subsec;
    use crate::test::prelude::*;

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
    fn int_no_unit_implies_micros() {
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
            (b"1000001".as_slice(), (1, 1_000)),
        ];

        let subsec_unit: Option<&[u8]> = None;

        for (input, expectation) in &expectations {
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
    fn int_subsec_millis() {
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
            (b"1001".as_slice(), (1, 1_000_000)),
        ];

        let subsec_unit: Option<&[u8]> = Some(b":milliseconds");

        for (input, expectation) in &expectations {
            let result = subsec(&mut interp, (Some(input), subsec_unit)).unwrap();
            assert_eq!(
                result.to_tuple(),
                *expectation,
                "Expected TryConvertMut<(Some({}), Some({})), Result<Subsec>>, to return {} secs, {} nanos",
                input.as_bstr(),
                subsec_unit.unwrap().as_bstr(),
                expectation.0,
                expectation.1
            );
        }
    }

    #[test]
    fn int_subsec_micros() {
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
            (b"1000001".as_slice(), (1, 1_000)),
        ];

        let subsec_unit: Option<&[u8]> = Some(b":usec");

        for (input, expectation) in &expectations {
            let result = subsec(&mut interp, (Some(input), subsec_unit)).unwrap();
            assert_eq!(
                result.to_tuple(),
                *expectation,
                "Expected TryConvertMut<(Some({}), Some({})), Result<Subsec>>, to return {} secs, {} nanos",
                input.as_bstr(),
                subsec_unit.unwrap().as_bstr(),
                expectation.0,
                expectation.1
            );
        }
    }

    #[test]
    fn int_subsec_nanos() {
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
            (b"1000000001".as_slice(), (1, 1)),
        ];

        let subsec_unit: Option<&[u8]> = Some(b":nsec");

        for (input, expectation) in &expectations {
            let result = subsec(&mut interp, (Some(input), subsec_unit)).unwrap();
            assert_eq!(
                result.to_tuple(),
                *expectation,
                "Expected TryConvertMut<(Some({}), Some({})), Result<Subsec>>, to return {} secs, {} nanos",
                input.as_bstr(),
                subsec_unit.unwrap().as_bstr(),
                expectation.0,
                expectation.1
            );
        }
    }

    #[test]
    fn float_no_unit_implies_micros() {
        let mut interp = interpreter();

        let expectations = [
            // Numbers in and around 0.
            (b"-1000000.5".as_slice(), (-2, 999_999_500)),
            (b"-1000000.0".as_slice(), (-2, 0)),
            (b"-999999.5".as_slice(), (-1, 500)),
            (b"-999999.0".as_slice(), (-1, 1_000)),
            (b"-1000.5".as_slice(), (-1, 998_999_500)),
            (b"-1.5".as_slice(), (-1, 999_998_500)),
            (b"-1.0".as_slice(), (-1, 999_999_000)),
            (b"-0.0".as_slice(), (0, 0)),
            (b"0.0".as_slice(), (0, 0)),
            (b"1.0".as_slice(), (0, 1_000)),
            (b"1.5".as_slice(), (0, 1_500)),
            (b"1000.5".as_slice(), (0, 1_000_500)),
            (b"999999.0".as_slice(), (0, 999_999_000)),
            (b"999999.5".as_slice(), (0, 999_999_500)),
            (b"1000000.0".as_slice(), (1, 0)),
            (b"1000000.5".as_slice(), (1, 500)),
            (b"1000001.0".as_slice(), (1, 1000)),
            // Nanosecond and below (truncates, does not round).
            (b"0.123".as_slice(), (0, 123)),
            (b"0.001".as_slice(), (0, 1)),
            (b"0.0001".as_slice(), (0, 0)),
            (b"0.0009".as_slice(), (0, 0)),
        ];

        let subsec_unit: Option<&[u8]> = None;

        for (input, expectation) in &expectations {
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
    fn float_subsec_millis() {
        let mut interp = interpreter();

        let expectations = [
            // Numbers in and around 0.
            (b"-1000.5".as_slice(), (-2, 999_500_000)),
            (b"-1000.0".as_slice(), (-2, 0)),
            (b"-999.5".as_slice(), (-1, 500_000)),
            (b"-999.0".as_slice(), (-1, 1_000_000)),
            (b"-1.5".as_slice(), (-1, 998_500_000)),
            (b"-1.0".as_slice(), (-1, 999_000_000)),
            (b"-0.0".as_slice(), (0, 0)),
            (b"0.0".as_slice(), (0, 0)),
            (b"1.0".as_slice(), (0, 1_000_000)),
            (b"1.5".as_slice(), (0, 1_500_000)),
            (b"999.0".as_slice(), (0, 999_000_000)),
            (b"999.5".as_slice(), (0, 999_500_000)),
            (b"1000.0".as_slice(), (1, 0)),
            (b"1000.5".as_slice(), (1, 500_000)),
            (b"1001.0".as_slice(), (1, 1_000_000)),
            // Nanosecond and below (truncates, does not round).
            (b"0.123456".as_slice(), (0, 123_456)),
            (b"0.000001".as_slice(), (0, 1)),
            (b"0.0000001".as_slice(), (0, 0)),
            (b"0.0000009".as_slice(), (0, 0)),
        ];

        let subsec_unit: Option<&[u8]> = Some(b":milliseconds");

        for (input, expectation) in &expectations {
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
    fn float_subsec_micros() {
        let mut interp = interpreter();

        let expectations = [
            // Numbers in and around 0.
            (b"-1000000.5".as_slice(), (-2, 999_999_500)),
            (b"-1000000.0".as_slice(), (-2, 0)),
            (b"-999999.5".as_slice(), (-1, 500)),
            (b"-999999.0".as_slice(), (-1, 1_000)),
            (b"-1000.5".as_slice(), (-1, 998_999_500)),
            (b"-1.5".as_slice(), (-1, 999_998_500)),
            (b"-1.0".as_slice(), (-1, 999_999_000)),
            (b"-0.0".as_slice(), (0, 0)),
            (b"0.0".as_slice(), (0, 0)),
            (b"1.0".as_slice(), (0, 1_000)),
            (b"1.5".as_slice(), (0, 1_500)),
            (b"1000.5".as_slice(), (0, 1_000_500)),
            (b"999999.0".as_slice(), (0, 999_999_000)),
            (b"999999.5".as_slice(), (0, 999_999_500)),
            (b"1000000.0".as_slice(), (1, 0)),
            (b"1000000.5".as_slice(), (1, 500)),
            (b"1000001.0".as_slice(), (1, 1000)),
            // Nanosecond and below (truncates, does not round).
            (b"0.123".as_slice(), (0, 123)),
            (b"0.001".as_slice(), (0, 1)),
            (b"0.0001".as_slice(), (0, 0)),
            (b"0.0009".as_slice(), (0, 0)),
        ];

        let subsec_unit: Option<&[u8]> = Some(b":usec");

        for (input, expectation) in &expectations {
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
    fn float_subsec_nanos() {
        let mut interp = interpreter();

        let expectations = [
            // Numbers in and around 0.
            (b"-1000000000.5".as_slice(), (-2, 999_999_999)),
            (b"-1000000000.0".as_slice(), (-2, 0)),
            (b"-999999999.5".as_slice(), (-1, 0)),
            (b"-999999999.0".as_slice(), (-1, 1)),
            (b"-1000.5".as_slice(), (-1, 999_998_999)),
            (b"-1.5".as_slice(), (-1, 999_999_998)),
            (b"-1.0".as_slice(), (-1, 999_999_999)),
            (b"-0.0".as_slice(), (0, 0)),
            (b"0.0".as_slice(), (0, 0)),
            (b"1.0".as_slice(), (0, 1)),
            (b"1.5".as_slice(), (0, 1)),
            (b"1000.5".as_slice(), (0, 1_000)),
            (b"999999999.0".as_slice(), (0, 999_999_999)),
            (b"999999999.5".as_slice(), (0, 999_999_999)),
            (b"1000000000.0".as_slice(), (1, 0)),
            (b"1000000000.5".as_slice(), (1, 0)),
            (b"1000000001.0".as_slice(), (1, 1)),
            // Nanosecond and below (truncates, does not round).
            (b"-0.1".as_slice(), (-1, 999_999_999)),
            (b"0.1".as_slice(), (0, 0)),
        ];

        let subsec_unit: Option<&[u8]> = Some(b":nsec");

        for (input, expectation) in &expectations {
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
    fn float_nan_raises() {
        let mut interp = interpreter();

        let err = subsec(&mut interp, (Some(b"Float::NAN"), None)).unwrap_err();

        assert_eq!(err.name(), "FloatDomainError");
        assert_eq!(err.message(), b"NaN".as_slice());
    }

    #[test]
    fn float_infinite_raises() {
        let mut interp = interpreter();

        let err = subsec(&mut interp, (Some(b"Float::INFINITY"), None)).unwrap_err();

        assert_eq!(err.name(), "FloatDomainError");
        assert_eq!(err.message(), b"Infinity".as_slice());
    }

    #[test]
    fn invalid_subsec_unit() {
        let mut interp = interpreter();

        let err = subsec(&mut interp, (Some(b"1"), Some(b":bad_unit"))).unwrap_err();

        assert_eq!(err.name(), "ArgumentError");
        assert_eq!(err.message(), b"unexpected unit: bad_unit".as_slice());
    }
}
