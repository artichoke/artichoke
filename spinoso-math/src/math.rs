#[cfg(feature = "full")]
use core::num::FpCategory;

use crate::{DomainError, NotImplementedError};

/// Computes the arccosine of the given value. Returns results in the range
/// `(0..=PI)`.
///
/// Domain: [-1, 1]
///
/// Codomain: [0, PI]
///
/// # Examples
///
/// ```
/// # use spinoso_math::PI;
/// use spinoso_math as math;
/// assert_eq!(math::acos(0.0), Ok(PI / 2.0));
/// assert!(math::acos(100.0).is_err());
///
/// assert!(matches!(math::acos(f64::NAN), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the result of computing the arccosine is [`NAN`], a domain error is
/// returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn acos(value: f64) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = value.acos();
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "acos""#,
        ))
    } else {
        Ok(result)
    }
}

/// Computes the inverse hyperbolic cosine of the given value.
///
/// Domain: [1, INFINITY)
///
/// Codomain: [0, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::acosh(1.0), Ok(0.0));
/// assert!(math::acosh(0.0).is_err());
///
/// assert!(matches!(math::acosh(f64::NAN), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the result of computing the inverse hyperbolic cosine is [`NAN`], a
/// domain error is returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn acosh(value: f64) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = value.acosh();
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "acosh""#,
        ))
    } else {
        Ok(result)
    }
}

/// Computes the arcsine of the given value. Returns results in the range
/// `(-PI/2..=PI/2)`.
///
/// Domain: [-1, -1]
///
/// Codomain: [-PI/2, PI/2]
///
/// # Examples
///
/// ```
/// # use spinoso_math::PI;
/// use spinoso_math as math;
/// assert_eq!(math::asin(1.0), Ok(PI / 2.0));
/// assert!(math::asin(100.0).is_err());
///
/// assert!(matches!(math::asin(f64::NAN), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the result of computing the arcsine is [`NAN`], a domain error is
/// returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn asin(value: f64) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = value.asin();
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "asin""#,
        ))
    } else {
        Ok(result)
    }
}

/// Computes the inverse hyperbolic sine of the given value.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert!((math::asinh(1.0) - 0.881373587019543).abs() < f64::EPSILON);
/// ```
#[inline]
#[must_use]
pub fn asinh(value: f64) -> f64 {
    value.asinh()
}

/// Computes the arctangent of the given value. Returns results in the range
/// `(-PI/2..=PI/2)`.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-PI/2, PI/2)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::atan(0.0), 0.0);
/// ```
#[inline]
#[must_use]
pub fn atan(value: f64) -> f64 {
    value.atan()
}

/// Computes the four quadrant arctangent of `value` (`y`) and `other` (`x`) in
/// radians.
///
/// Return value is a angle in radians between the positive x-axis of Cartesian
/// plane and the point given by the coordinates (`x`, `y`) on it.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: [-PI, PI]
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert!((math::atan2(-0.0, -1.0) - (-3.141592653589793)).abs() < f64::EPSILON);
/// assert!((math::atan2(-1.0, -1.0) - (-2.356194490192345)).abs() < f64::EPSILON);
/// assert!((math::atan2(-1.0, 0.0) - (-1.5707963267948966)).abs() < f64::EPSILON);
/// assert!((math::atan2(-1.0, 1.0) - (-0.7853981633974483)).abs() < f64::EPSILON);
/// assert!(math::atan2(-0.0, 1.0) == -0.0);
/// assert!(math::atan2(0.0, 1.0) == 0.0);
/// assert!((math::atan2(1.0, 1.0) - 0.7853981633974483).abs() < f64::EPSILON);
/// assert!((math::atan2(1.0, 0.0) - 1.5707963267948966).abs() < f64::EPSILON);
/// assert!((math::atan2(1.0, -1.0) - 2.356194490192345).abs() < f64::EPSILON);
/// assert!((math::atan2(0.0, -1.0) - 3.141592653589793).abs() < f64::EPSILON);
/// assert!((math::atan2(f64::INFINITY, f64::INFINITY) - 0.7853981633974483).abs() < f64::EPSILON);
/// assert!(
///     (math::atan2(f64::INFINITY, f64::NEG_INFINITY) - 2.356194490192345).abs() < f64::EPSILON
/// );
/// assert!(
///     (math::atan2(f64::NEG_INFINITY, f64::INFINITY) - (-0.7853981633974483)).abs()
///         < f64::EPSILON
/// );
/// assert!(
///     (math::atan2(f64::NEG_INFINITY, f64::NEG_INFINITY) - (-2.356194490192345)).abs()
///         < f64::EPSILON
/// );
/// ```
#[inline]
#[must_use]
pub fn atan2(value: f64, other: f64) -> f64 {
    value.atan2(other)
}

/// Computes the inverse hyperbolic tangent of the given value.
///
/// Domain: (-1, 1)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::atanh(1.0), Ok(f64::INFINITY));
/// assert!(math::atanh(100.0).is_err());
///
/// assert!(matches!(math::atanh(f64::NAN), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the result of computing the inverse hyperbolic tangent is [`NAN`]
/// a domain error is returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn atanh(value: f64) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = value.atanh();
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "atanh""#,
        ))
    } else {
        Ok(result)
    }
}

/// Returns the cube root of the given value.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert!((math::cbrt(-9.0) - (-2.080083823051904)).abs() < f64::EPSILON);
/// assert!((math::cbrt(9.0) - 2.080083823051904).abs() < f64::EPSILON);
/// ```
#[inline]
#[must_use]
pub fn cbrt(value: f64) -> f64 {
    value.cbrt()
}

/// Computes the cosine of the given value (expressed in radians). Returns
/// values in the range `-1.0..=1.0`.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: [-1, 1]
///
/// # Examples
///
/// ```
/// # use spinoso_math::PI;
/// use spinoso_math as math;
/// assert_eq!(math::cos(PI), -1.0);
/// ```
#[inline]
#[must_use]
pub fn cos(value: f64) -> f64 {
    value.cos()
}

/// Computes the hyperbolic cosine of the given value (expressed in radians).
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: [1, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::cosh(0.0), 1.0);
/// ```
#[inline]
#[must_use]
pub fn cosh(value: f64) -> f64 {
    value.cosh()
}

/// Calculates the error function of the given value.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-1, 1)
///
/// # Errors
///
/// Because `spinoso-math` was built without the `full` feature, this function
/// will always return a not implemented error.
#[inline]
#[cfg(not(feature = "full"))]
pub fn erf(value: f64) -> Result<f64, NotImplementedError> {
    let _ = value;
    Err(NotImplementedError::with_message(
        "Artichoke was not built with Math::erf support",
    ))
}

/// Calculates the error function of the given value.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-1, 1)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::erf(0.0), Ok(0.0));
/// ```
///
/// # Errors
///
/// Because `spinoso-math` was built with the `full` feature, this function will
/// always succeed and return the error function of the given value.
#[inline]
#[cfg(feature = "full")]
pub fn erf(value: f64) -> Result<f64, NotImplementedError> {
    let result = libm::erf(value);
    Ok(result)
}

/// Calculates the complementary error function of the given value.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (0, 2)
///
/// # Errors
///
/// Because `spinoso-math` was built without the `full` feature, this function
/// will always return a not implemented error.
#[inline]
#[cfg(not(feature = "full"))]
pub fn erfc(value: f64) -> Result<f64, NotImplementedError> {
    let _ = value;
    Err(NotImplementedError::with_message(
        "Artichoke was not built with Math::erfc support",
    ))
}

/// Calculates the complementary error function of the given value.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (0, 2)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::erfc(0.0), Ok(1.0));
/// ```
///
/// # Errors
///
/// Because `spinoso-math` was built with the `full` feature, this function will
/// always succeed and return the complementary error function of the given
/// value.
#[inline]
#[cfg(feature = "full")]
pub fn erfc(value: f64) -> Result<f64, NotImplementedError> {
    let result = libm::erfc(value);
    Ok(result)
}

/// Returns `e**x`.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (0, INFINITY)
///
/// # Examples
///
/// ```
/// # use spinoso_math::E;
/// use spinoso_math as math;
/// assert_eq!(math::exp(0.0), 1.0);
/// assert_eq!(math::exp(1.0), E);
/// assert!((math::exp(1.5) - 4.4816890703380645).abs() < f64::EPSILON);
/// ```
#[inline]
#[must_use]
pub fn exp(value: f64) -> f64 {
    value.exp()
}

/// Returns a tuple array containing the normalized fraction (a Float) and
/// exponent (an Integer) of the given value.
///
/// # Errors
///
/// Because `spinoso-math` was built without the `full` feature, this function
/// will always return a not implemented error.
#[inline]
#[cfg(not(feature = "full"))]
pub const fn frexp(value: f64) -> Result<(f64, i32), NotImplementedError> {
    let _ = value;
    Err(NotImplementedError::with_message(
        "Artichoke was not built with Math::frexp support",
    ))
}

/// Returns a tuple array containing the normalized fraction (a Float) and
/// exponent (an Integer) of the given value.
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// # fn example() -> Result<(), math::NotImplementedError> {
/// let (fraction, exponent) = math::frexp(1234.0)?;
/// let float = math::ldexp(fraction, exponent)?;
/// assert_eq!(float, 1234.0);
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// Because `spinoso-math` was built with the `full` feature, this function will
/// always succeed and return the normalized fraction and exponent of the given
/// value.
#[inline]
#[cfg(feature = "full")]
pub fn frexp(value: f64) -> Result<(f64, i32), NotImplementedError> {
    let result = libm::frexp(value);
    Ok(result)
}

/// Calculates the gamma function of the given value.
///
/// Note that `gamma(n)` is same as `fact(n-1)` for integer `n > 0`. However
/// `gamma(n)` returns float and can be an approximation.
///
/// # Errors
///
/// Because `spinoso-math` was built without the `full` feature, this function
/// will always return a not implemented error.
#[cfg(not(feature = "full"))]
pub const fn gamma(value: f64) -> Result<f64, NotImplementedError> {
    let _ = value;
    Err(NotImplementedError::with_message(
        "Artichoke was not built with Math::gamma support",
    ))
}

/// Calculates the gamma function of the given value.
///
/// Note that `gamma(n)` is same as `fact(n-1)` for integer `n > 0`. However
/// `gamma(n)` returns float and can be an approximation.
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::gamma(1.0), Ok(1.0));
/// assert_eq!(math::gamma(2.0), Ok(1.0));
/// assert_eq!(math::gamma(3.0), Ok(2.0));
/// assert_eq!(math::gamma(4.0), Ok(6.0));
/// assert_eq!(math::gamma(5.0), Ok(24.0));
/// assert_eq!(math::gamma(20.0), Ok(1.21645100408832e+17));
///
/// assert!(math::gamma(-15.0).is_err());
/// assert!(matches!(math::gamma(-15.1), Ok(result) if (result - 5.9086389724319095e-12).abs() < f64::EPSILON));
///
/// assert!(math::gamma(f64::NEG_INFINITY).is_err());
/// assert_eq!(math::gamma(f64::INFINITY), Ok(f64::INFINITY));
/// ```
///
/// # Errors
///
/// If the given value is negative, a domain error is returned.
#[inline]
#[cfg(feature = "full")]
pub fn gamma(value: f64) -> Result<f64, DomainError> {
    // `gamma(n)` is the same as `n!` for integer `n > 0`. `gamma` returns float
    // and might be an approximation so include a lookup table for as many `n`
    // as can fit in the float mantissa.
    const FACTORIAL_TABLE: [f64; 23] = [
        1.0_f64,                         // fact(0)
        1.0,                             // fact(1)
        2.0,                             // fact(2)
        6.0,                             // fact(3)
        24.0,                            // fact(4)
        120.0,                           // fact(5)
        720.0,                           // fact(6)
        5_040.0,                         // fact(7)
        40_320.0,                        // fact(8)
        362_880.0,                       // fact(9)
        3_628_800.0,                     // fact(10)
        39_916_800.0,                    // fact(11)
        479_001_600.0,                   // fact(12)
        6_227_020_800.0,                 // fact(13)
        87_178_291_200.0,                // fact(14)
        1_307_674_368_000.0,             // fact(15)
        20_922_789_888_000.0,            // fact(16)
        355_687_428_096_000.0,           // fact(17)
        6_402_373_705_728_000.0,         // fact(18)
        121_645_100_408_832_000.0,       // fact(19)
        2_432_902_008_176_640_000.0,     // fact(20)
        51_090_942_171_709_440_000.0,    // fact(21)
        1_124_000_727_777_607_680_000.0, // fact(22)
    ];
    match value {
        value if value.is_infinite() && value.is_sign_negative() => Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "gamma""#,
        )),
        value if value.is_infinite() => Ok(f64::INFINITY),
        value if matches!(value.classify(), FpCategory::Zero) && value.is_sign_negative() => Ok(f64::NEG_INFINITY),
        value if matches!(value.classify(), FpCategory::Zero) => Ok(f64::INFINITY),
        value if (value - value.floor()).abs() < f64::EPSILON && value.is_sign_negative() => Err(
            DomainError::with_message(r#"Numerical argument is out of domain - "gamma""#),
        ),
        value if (value - value.floor()).abs() < f64::EPSILON => {
            #[allow(clippy::cast_possible_truncation)]
            let idx = (value as i64).checked_sub(1).map(usize::try_from);
            if let Some(Ok(idx)) = idx {
                if let Some(&result) = FACTORIAL_TABLE.get(idx) {
                    return Ok(result);
                }
            }
            let result = libm::tgamma(value);
            Ok(result)
        }
        value => {
            let result = libm::tgamma(value);
            Ok(result)
        }
    }
}

/// Returns `sqrt(x**2 + y**2)`, the hypotenuse of a right-angled triangle with
/// sides x and y.
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::hypot(3.0, 4.0), 5.0);
/// ```
#[inline]
#[must_use]
pub fn hypot(x: f64, y: f64) -> f64 {
    x.hypot(y)
}

/// Returns the value of `fraction * (2**exponent)`.
///
/// # Errors
///
/// Because `spinoso-math` was built without the `full` feature, this function
/// will always return a not implemented error.
#[cfg(not(feature = "full"))]
pub fn ldexp(fraction: f64, exponent: i32) -> Result<f64, NotImplementedError> {
    let _ = fraction;
    let _ = exponent;
    Err(NotImplementedError::with_message(
        "Artichoke was not built with Math::ldexp support",
    ))
}

/// Returns the value of `fraction * (2**exponent)`.
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// # fn example() -> Result<(), math::NotImplementedError> {
/// let (fraction, exponent) = math::frexp(1234.0)?;
/// let float = math::ldexp(fraction, exponent)?;
/// assert_eq!(float, 1234.0);
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// Because `spinoso-math` was built with the `full` feature, this function will
/// always succeed and return the float determined by the given fraction and
/// exponent.
#[inline]
#[cfg(feature = "full")]
pub fn ldexp(fraction: f64, exponent: i32) -> Result<f64, NotImplementedError> {
    let result = libm::ldexp(fraction, exponent);
    Ok(result)
}

/// Calculates the logarithmic gamma of value and the sign of gamma of value.
///
/// `lgamma` is same as:
///
/// ```ruby
/// [Math.log(Math.gamma(x).abs), Math.gamma(x) < 0 ? -1 : 1]
/// ```
///
/// but avoids overflow of `gamma` for large values.
///
/// # Errors
///
/// Because `spinoso-math` was built without the `full` feature, this function
/// will always return a not implemented error.
#[inline]
#[cfg(not(feature = "full"))]
pub fn lgamma(value: f64) -> Result<(f64, i32), NotImplementedError> {
    let _ = value;
    Err(NotImplementedError::with_message(
        "Artichoke was not built with Math::lgamma support",
    ))
}

/// Calculates the logarithmic gamma of value and the sign of gamma of value.
///
/// `lgamma` is same as:
///
/// ```ruby
/// [Math.log(Math.gamma(x).abs), Math.gamma(x) < 0 ? -1 : 1]
/// ```
///
/// but avoids overflow of `gamma` for large values.
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::lgamma(0.0), Ok((f64::INFINITY, 1)));
///
/// assert!(math::lgamma(f64::NEG_INFINITY).is_err());
/// ```
///
/// # Errors
///
/// If the given value is [negative infinity], an error is returned.
///
/// [negative infinity]: f64::NEG_INFINITY
#[inline]
#[cfg(feature = "full")]
pub fn lgamma(value: f64) -> Result<(f64, i32), DomainError> {
    if value.is_infinite() && value.is_sign_negative() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "lgamma""#,
        ))
    } else {
        let (result, sign) = libm::lgamma_r(value);
        Ok((result, sign))
    }
}

/// Returns the logarithm of the number with respect to an arbitrary base.
///
/// Domain: (0, INFINITY)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// # use spinoso_math::E;
/// use spinoso_math as math;
/// assert_eq!(math::log(1.0, None), Ok(0.0));
/// assert_eq!(math::log(E, None), Ok(1.0));
/// assert_eq!(math::log(64.0, Some(4.0)), Ok(3.0));
///
/// assert_eq!(math::log(0.0, None), Ok(f64::NEG_INFINITY));
/// assert!(math::log(-0.1, None).is_err());
///
/// assert!(matches!(math::log(f64::NAN, None), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the given arbitrary base is [`NAN`], a domain error is returned.
///
/// If the result of computing the logarithm is [`NAN`], a domain error is
/// returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn log(value: f64, base: Option<f64>) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = match base {
        Some(base) if base.is_nan() => return Ok(f64::NAN),
        Some(base) => value.log(base),
        None => value.ln(),
    };
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "log""#,
        ))
    } else {
        Ok(result)
    }
}

/// Returns the base 10 logarithm of the number.
///
/// Domain: (0, INFINITY)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::log10(1.0), Ok(0.0));
/// assert_eq!(math::log10(10.0), Ok(1.0));
/// assert_eq!(math::log10(1e100), Ok(100.0));
///
/// assert_eq!(math::log10(0.0), Ok(f64::NEG_INFINITY));
/// assert!(math::log10(-0.1).is_err());
///
/// assert!(matches!(math::log10(f64::NAN), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the result of computing the logarithm is [`NAN`], a domain error is
/// returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn log10(value: f64) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = value.log10();
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "log10""#,
        ))
    } else {
        Ok(result)
    }
}

/// Returns the base 2 logarithm of the number.
///
/// Domain: (0, INFINITY)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::log2(1.0), Ok(0.0));
/// assert_eq!(math::log2(2.0), Ok(1.0));
/// assert_eq!(math::log2(32768.0), Ok(15.0));
/// assert_eq!(math::log2(65536.0), Ok(16.0));
///
/// assert_eq!(math::log2(0.0), Ok(f64::NEG_INFINITY));
/// assert!(math::log2(-0.1).is_err());
///
/// assert!(matches!(math::log2(f64::NAN), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the result of computing the logarithm is [`NAN`], a domain error is
/// returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn log2(value: f64) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = value.log2();
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "log2""#,
        ))
    } else {
        Ok(result)
    }
}

/// Computes the sine of the given value (expressed in radians). Returns a Float
/// in the range `-1.0..=1.0`.
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: [-1, 1]
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::sin(math::PI / 2.0), 1.0);
/// ```
#[inline]
#[must_use]
pub fn sin(value: f64) -> f64 {
    value.sin()
}

/// Computes the hyperbolic sine of the given value (expressed in radians).
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::sinh(0.0), 0.0);
/// ```
#[inline]
#[must_use]
pub fn sinh(value: f64) -> f64 {
    value.sinh()
}

/// Returns the non-negative square root of the given value.
///
/// Domain: [0, INFINITY)
///
/// Codomain: [0, INFINITY)
///
/// # Examples
///
/// ```
/// # use spinoso_math::DomainError;
/// use spinoso_math as math;
/// assert_eq!(math::sqrt(0.0), Ok(0.0));
/// assert_eq!(math::sqrt(1.0), Ok(1.0));
/// assert_eq!(math::sqrt(9.0), Ok(3.0));
///
/// assert!(math::sqrt(-9.0).is_err());
///
/// assert!(matches!(math::sqrt(f64::NAN), Ok(result) if result.is_nan()));
/// ```
///
/// # Errors
///
/// If the result of computing the square root is [`NAN`], a domain error is
/// returned.
///
/// [`NAN`]: f64::NAN
#[inline]
pub fn sqrt(value: f64) -> Result<f64, DomainError> {
    if value.is_nan() {
        return Ok(f64::NAN);
    }
    let result = value.sqrt();
    if result.is_nan() {
        Err(DomainError::with_message(
            r#"Numerical argument is out of domain - "sqrt""#,
        ))
    } else {
        Ok(result)
    }
}

/// Computes the tangent of the given value (expressed in radians).
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-INFINITY, INFINITY)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::tan(0.0), 0.0);
/// ```
#[inline]
#[must_use]
pub fn tan(value: f64) -> f64 {
    value.tan()
}

/// Computes the hyperbolic tangent of the given value (expressed in radians).
///
/// Domain: (-INFINITY, INFINITY)
///
/// Codomain: (-1, 1)
///
/// # Examples
///
/// ```
/// use spinoso_math as math;
/// assert_eq!(math::tanh(0.0), 0.0);
/// ```
#[inline]
#[must_use]
pub fn tanh(value: f64) -> f64 {
    value.tanh()
}
