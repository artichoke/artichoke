use spinoso_exception::{FloatDomainError, RangeError};

use crate::error::Error;

/// Convert a [`f64`] to an [`i64`] by rounding toward zero.
///
/// # Errors
///
/// This function can return either a [`FloatDomainError`] or a [`RangeError`].
///
/// [`FloatDomainError`] is returned if the input is either [`NaN`] or infinite.
///
/// [`RangeError`] is returned if the input is finite but out of range of
/// `i64::MIN..=i64::MAX`.
///
/// [`NaN`]: f64::NAN
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
pub fn float_to_int(float: f64) -> Result<i64, Error> {
    if float.is_nan() {
        return Err(FloatDomainError::with_message("NaN").into());
    }
    if float.is_sign_negative() {
        if float.is_infinite() {
            return Err(FloatDomainError::with_message("-Infinity").into());
        }
        // ```
        // [3.1.2] > Integer -10.9
        // => -10
        // [3.1.2] > Integer -10.5
        // => -10
        // [3.1.2] > Integer -10.2
        // => -10
        // ```
        let float = float.ceil();
        if float < i64::MIN as f64 {
            return Err(RangeError::with_message("too small for int").into());
        }
        Ok(float as i64)
    } else {
        if float.is_infinite() {
            return Err(FloatDomainError::with_message("Infinity").into());
        }
        // ```
        // [3.1.2] > Integer 10.9
        // => 10
        // [3.1.2] > Integer 10.5
        // => 10
        // [3.1.2] > Integer 10.2
        // => 10
        // ```
        let float = float.floor();
        if float > i64::MAX as f64 {
            return Err(RangeError::with_message("too big for int").into());
        }
        Ok(float as i64)
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::float_to_int;
    use crate::test::prelude::*;

    #[test]
    fn float_to_int_rounds_to_zero() {
        let result = float_to_int(10.0).unwrap();
        assert_eq!(result, 10);
        let result = float_to_int(10.2).unwrap();
        assert_eq!(result, 10);
        let result = float_to_int(10.5).unwrap();
        assert_eq!(result, 10);
        let result = float_to_int(10.9).unwrap();
        assert_eq!(result, 10);

        let result = float_to_int(-10.0).unwrap();
        assert_eq!(result, -10);
        let result = float_to_int(-10.2).unwrap();
        assert_eq!(result, -10);
        let result = float_to_int(-10.5).unwrap();
        assert_eq!(result, -10);
        let result = float_to_int(-10.9).unwrap();
        assert_eq!(result, -10);
    }

    #[test]
    fn float_nan_is_domain_error() {
        let err = float_to_int(f64::NAN).unwrap_err();
        assert_eq!(err.message().as_bstr(), b"NaN".as_bstr());
        assert_eq!(err.name(), "FloatDomainError");
    }

    #[test]
    fn float_infinities_are_domain_error() {
        let err = float_to_int(f64::INFINITY).unwrap_err();
        assert_eq!(err.message().as_bstr(), b"Infinity".as_bstr());
        assert_eq!(err.name(), "FloatDomainError");

        let err = float_to_int(f64::NEG_INFINITY).unwrap_err();
        assert_eq!(err.message().as_bstr(), b"-Infinity".as_bstr());
        assert_eq!(err.name(), "FloatDomainError");
    }

    // FIXME: MRI converts these to `BigNum`s.
    #[test]
    fn float_out_of_i64_range_is_range_error() {
        let err = float_to_int(f64::MAX).unwrap_err();
        assert_eq!(err.name(), "RangeError");

        let err = float_to_int(f64::MIN).unwrap_err();
        assert_eq!(err.name(), "RangeError");
    }
}
