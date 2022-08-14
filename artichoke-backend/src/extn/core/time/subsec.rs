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
                Ruby::Fixnum => {
                    let subsec: i64 = subsec.try_convert_into(self)?;

                    // The below conversions should be safe. The multiplier is gauranteed to not be
                    // 0, the remainder should never overflow, and is gauranteed to be less than
                    // u32::MAX;
                    let secs = subsec / seconds_base;
                    let nanos = ((subsec % seconds_base) * multiplier_nanos) as u32;
                    Ok(Subsec { secs, nanos })
                },
                Ruby::Float => {
                    // TODO: Safe conversions here are really hard, there may end up being some
                    // loss in accuracy.
                    unreachable!("Not yet implemented")
                },
                _ => {
                    let subsec: i64 = implicitly_convert_to_int(self, subsec)?;

                    // The below conversions should be safe. The multiplier is gauranteed to not be
                    // 0, the remainder should never overflow, and is gauranteed to be less than
                    // u32::MAX;
                    let secs = subsec / seconds_base;
                    let nanos = ((subsec % seconds_base) * multiplier_nanos) as u32;
                    Ok(Subsec { secs, nanos })
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

        let result = subsec(&mut interp, (Some(b"0"), None)).unwrap();
        assert_eq!(result.to_tuple(), (0, 0));

        let result = subsec(&mut interp, (Some(b"999999"), None)).unwrap();
        assert_eq!(result.to_tuple(), (0, 999_999_000));

        let result = subsec(&mut interp, (Some(b"1000000"), None)).unwrap();
        assert_eq!(result.to_tuple(), (1, 0));

        let result = subsec(&mut interp, (Some(b"1000001"), None)).unwrap();
        assert_eq!(result.to_tuple(), (1, 1_000));
    }

    #[test]
    fn subsec_millis() {
        let mut interp = interpreter();

        let result = subsec(&mut interp, (Some(b"0"), Some(b":milliseconds"))).unwrap();
        assert_eq!(result.to_tuple(), (0, 0));

        let result = subsec(&mut interp, (Some(b"999"), Some(b":milliseconds"))).unwrap();
        assert_eq!(result.to_tuple(), (0, 999_000_000));

        let result = subsec(&mut interp, (Some(b"1000"), Some(b":milliseconds"))).unwrap();
        assert_eq!(result.to_tuple(), (1, 0));

        let result = subsec(&mut interp, (Some(b"1001"), Some(b":milliseconds"))).unwrap();
        assert_eq!(result.to_tuple(), (1, 1_000_000));
    }

    #[test]
    fn subsec_micros() {
        let mut interp = interpreter();

        let result = subsec(&mut interp, (Some(b"0"), Some(b":usec"))).unwrap();
        assert_eq!(result.to_tuple(), (0, 0));

        let result = subsec(&mut interp, (Some(b"999999"), Some(b":usec"))).unwrap();
        assert_eq!(result.to_tuple(), (0, 999_999_000));

        let result = subsec(&mut interp, (Some(b"1000000"), Some(b":usec"))).unwrap();
        assert_eq!(result.to_tuple(), (1, 0));

        let result = subsec(&mut interp, (Some(b"1000001"), Some(b":usec"))).unwrap();
        assert_eq!(result.to_tuple(), (1, 1_000));
    }

    #[test]
    fn subsub_nanos() {
        let mut interp = interpreter();

        let result = subsec(&mut interp, (Some(b"0"), Some(b":nsec"))).unwrap();
        assert_eq!(result.to_tuple(), (0, 0));

        let result = subsec(&mut interp, (Some(b"999999999"), Some(b":nsec"))).unwrap();
        assert_eq!(result.to_tuple(), (0, 999_999_999));

        let result = subsec(&mut interp, (Some(b"1000000000"), Some(b":nsec"))).unwrap();
        assert_eq!(result.to_tuple(), (1, 0));

        let result = subsec(&mut interp, (Some(b"1000000001"), Some(b":nsec"))).unwrap();
        assert_eq!(result.to_tuple(), (1, 1));
    }
}
