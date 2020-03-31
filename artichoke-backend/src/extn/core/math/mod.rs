use std::borrow::Cow;
use std::error;
use std::f64;
use std::fmt;

use crate::extn::core::numeric::Numeric;
use crate::extn::prelude::*;

pub mod mruby;

pub const E: f64 = f64::consts::E;
pub const PI: f64 = f64::consts::PI;

#[derive(Debug)]
pub struct Math;

fn value_to_float(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    match value.ruby_type() {
        Ruby::Float => value.try_into(),
        Ruby::Fixnum =>
        {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_precision_loss)]
            value.try_into::<Int>().map(|num| num as Float)
        }
        Ruby::Nil => Err(Exception::from(TypeError::new(
            interp,
            "can't convert nil into Float",
        ))),
        _ => {
            // TODO: This should use `numeric::coerce`
            let borrow = interp.0.borrow();
            let numeric = borrow
                .class_spec::<Numeric>()
                .ok_or_else(|| NotDefinedError::class("Numeric"))?;
            let numeric = numeric
                .value(interp)
                .ok_or_else(|| NotDefinedError::class("Numeric"))?;
            drop(borrow);
            if let Ok(true) = value.funcall("is_a?", &[numeric], None) {
                if value.respond_to("to_f")? {
                    let coerced = value.funcall::<Value>("to_f", &[], None)?;
                    if let Ruby::Float = coerced.ruby_type() {
                        coerced.try_into::<Float>()
                    } else {
                        let mut message = String::from("can't convert ");
                        message.push_str(value.pretty_name());
                        message.push_str(" into Float (");
                        message.push_str(value.pretty_name());
                        message.push_str("#to_f gives ");
                        message.push_str(coerced.pretty_name());
                        message.push(')');
                        Err(Exception::from(TypeError::new(interp, message)))
                    }
                } else {
                    let mut message = String::from("can't convert ");
                    message.push_str(value.pretty_name());
                    message.push_str(" into Float");
                    Err(Exception::from(TypeError::new(interp, message)))
                }
            } else {
                let mut message = String::from("can't convert ");
                message.push_str(value.pretty_name());
                message.push_str(" into Float");
                Err(Exception::from(TypeError::new(interp, message)))
            }
        }
    }
}

pub fn acos(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = value.acos();
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "acos""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn acosh(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = value.acosh();
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "acosh""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn asin(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = value.asin();
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "asin""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn asinh(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.asinh();
    Ok(result)
}

pub fn atan(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.atan();
    Ok(result)
}

pub fn atan2(interp: &mut Artichoke, value: Value, other: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let other = value_to_float(interp, other)?;
    let result = value.atan2(other);
    Ok(result)
}

pub fn atanh(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = value.atanh();
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "atanh""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn cbrt(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.cbrt();
    Ok(result)
}

pub fn cos(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.cos();
    Ok(result)
}

pub fn cosh(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.cosh();
    Ok(result)
}

#[cfg(not(feature = "core-math-extra"))]
pub fn erf(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "enable 'core-math-extra' feature when building Artichoke",
    )))
}

#[cfg(feature = "core-math-extra")]
pub fn erf(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = libm::erf(value);
    Ok(result)
}

#[cfg(not(feature = "core-math-extra"))]
pub fn erfc(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "enable 'core-math-extra' feature when building Artichoke",
    )))
}

#[cfg(feature = "core-math-extra")]
pub fn erfc(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = libm::erfc(value);
    Ok(result)
}

pub fn exp(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.exp();
    Ok(result)
}

#[cfg(not(feature = "core-math-extra"))]
pub fn frexp(interp: &mut Artichoke, value: Value) -> Result<(Float, Int), Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "enable 'core-math-extra' feature when building Artichoke",
    )))
}

#[cfg(feature = "core-math-extra")]
pub fn frexp(interp: &mut Artichoke, value: Value) -> Result<(Float, Int), Exception> {
    let value = value_to_float(interp, value)?;
    let (fraction, exponent) = libm::frexp(value);
    Ok((fraction, exponent.into()))
}

#[cfg(not(feature = "core-math-extra"))]
pub fn gamma(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "enable 'core-math-extra' feature when building Artichoke",
    )))
}

#[cfg(feature = "core-math-extra")]
pub fn gamma(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    use crate::extn::core::float;
    use std::convert::TryFrom;
    use std::num::FpCategory;

    let value = value_to_float(interp, value)?;
    // `gamma(n)` is the same as `n!` for integer n > 0. `gamma` returns float
    // and might be an approximation so include a lookup table for as many `n`
    // as can fit in the float manitssa.
    let factorial_table = [
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
    if value.is_infinite() {
        if value.is_sign_negative() {
            Err(Exception::from(DomainError::new(
                r#"Numerical argument is out of domain - "gamma""#,
            )))
        } else {
            Ok(float::Float::INFINITY)
        }
    } else if let FpCategory::Zero = value.classify() {
        if value.is_sign_negative() {
            Ok(float::Float::NEG_INFINITY)
        } else {
            Ok(float::Float::INFINITY)
        }
    } else if (value - value.floor()).abs() < f64::EPSILON {
        if value.is_sign_negative() {
            Err(Exception::from(DomainError::new(
                r#"Numerical argument is out of domain - "gamma""#,
            )))
        } else {
            // TODO: use `approx_unchecked_to` once stabilized instead of `as`
            // cast.
            #[allow(clippy::cast_possible_truncation)]
            let idx = (value as Int).checked_sub(1).map(usize::try_from);
            let result = if let Some(Ok(idx)) = idx {
                factorial_table
                    .get(idx)
                    .copied()
                    .unwrap_or_else(|| libm::tgamma(value))
            } else {
                libm::tgamma(value)
            };
            Ok(result)
        }
    } else {
        let result = libm::tgamma(value);
        Ok(result)
    }
}

pub fn hypot(interp: &mut Artichoke, value: Value, other: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let other = value_to_float(interp, other)?;
    let result = value.hypot(other);
    Ok(result)
}

#[cfg(not(feature = "core-math-extra"))]
pub fn ldexp(interp: &mut Artichoke, fraction: Value, exponent: Value) -> Result<Float, Exception> {
    let _ = fraction;
    let _ = exponent;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "enable 'core-math-extra' feature when building Artichoke",
    )))
}

#[cfg(feature = "core-math-extra")]
pub fn ldexp(interp: &mut Artichoke, fraction: Value, exponent: Value) -> Result<Float, Exception> {
    use std::convert::TryFrom;

    let fraction = value_to_float(interp, fraction)?;
    let exponent = exponent.implicitly_convert_to_int(interp).or_else(|err| {
        if let Ok(exponent) = exponent.try_into::<Float>() {
            if exponent.is_nan() {
                Err(Exception::from(RangeError::new(
                    interp,
                    "float NaN out of range of integer",
                )))
            } else {
                // TODO: use `approx_unchecked_to` once stabilized.
                #[allow(clippy::cast_possible_truncation)]
                Ok(exponent as Int)
            }
        } else {
            Err(Exception::from(err))
        }
    })?;
    if let Ok(exponent) = i32::try_from(exponent) {
        Ok(libm::ldexp(fraction, exponent))
    } else if exponent < 0 {
        let mut message = String::from("integer ");
        string::format_int_into(&mut message, exponent)?;
        message.push_str("too small to convert to `int'");
        Err(Exception::from(RangeError::new(interp, message)))
    } else {
        let mut message = String::from("integer ");
        string::format_int_into(&mut message, exponent)?;
        message.push_str("too big to convert to `int'");
        Err(Exception::from(RangeError::new(interp, message)))
    }
}

#[cfg(not(feature = "core-math-extra"))]
pub fn lgamma(interp: &mut Artichoke, value: Value) -> Result<(Float, Int), Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "enable 'core-math-extra' feature when building Artichoke",
    )))
}

#[cfg(feature = "core-math-extra")]
pub fn lgamma(interp: &mut Artichoke, value: Value) -> Result<(Float, Int), Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_infinite() && value.is_sign_negative() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "lgamma""#,
        )))
    } else {
        let (result, sign) = libm::lgamma_r(value);
        Ok((result, Int::from(sign)))
    }
}

pub fn log(interp: &mut Artichoke, value: Value, base: Option<Value>) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = if let Some(base) = base {
        let base = value_to_float(interp, base)?;
        if base.is_nan() {
            return Ok(base);
        }
        value.log(base)
    } else {
        value.ln()
    };
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "log""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn log10(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = value.log10();
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "log10""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn log2(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = value.log2();
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "log2""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn sin(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.sin();
    Ok(result)
}

pub fn sinh(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.sinh();
    Ok(result)
}

pub fn sqrt(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    if value.is_nan() {
        return Ok(value);
    }
    let result = value.sqrt();
    if result.is_nan() {
        Err(Exception::from(DomainError::new(
            r#"Numerical argument is out of domain - "sqrt""#,
        )))
    } else {
        Ok(result)
    }
}

pub fn tan(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.tan();
    Ok(result)
}

pub fn tanh(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.tanh();
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DomainError(Cow<'static, str>);

impl DomainError {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self(name.into())
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for DomainError {}

impl RubyException for DomainError {
    fn message(&self) -> &[u8] {
        self.0.as_ref().as_bytes()
    }

    fn name(&self) -> String {
        String::from("DomainError")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let borrow = interp.0.borrow();
        let spec = borrow.class_spec::<Self>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<DomainError> for Exception {
    fn from(exception: DomainError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<DomainError>> for Exception {
    fn from(exception: Box<DomainError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<DomainError> for Box<dyn RubyException> {
    fn from(exception: DomainError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<DomainError>> for Box<dyn RubyException> {
    fn from(exception: Box<DomainError>) -> Box<dyn RubyException> {
        exception
    }
}
