//! Glue between mruby FFI and `ENV` Rust implementation.

use std::fmt::Write as _;

use crate::convert::implicitly_convert_to_int;
use crate::extn::prelude::*;

pub fn acos(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::acos(value)?;
    Ok(result)
}

pub fn acosh(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::acosh(value)?;
    Ok(result)
}

pub fn asin(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::asin(value)?;
    Ok(result)
}

pub fn asinh(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::asinh(value);
    Ok(result)
}

pub fn atan(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::atan(value);
    Ok(result)
}

pub fn atan2(interp: &mut Artichoke, value: Value, other: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let other = interp.coerce_to_float(other)?;
    let result = spinoso_math::atan2(value, other);
    Ok(result)
}

pub fn atanh(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::atanh(value)?;
    Ok(result)
}

pub fn cbrt(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::cbrt(value);
    Ok(result)
}

pub fn cos(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::cos(value);
    Ok(result)
}

pub fn cosh(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::cosh(value);
    Ok(result)
}

pub fn erf(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::erf(value)?;
    Ok(result)
}

pub fn erfc(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::erfc(value)?;
    Ok(result)
}

pub fn exp(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::exp(value);
    Ok(result)
}

pub fn frexp(interp: &mut Artichoke, value: Value) -> Result<(f64, i64), Error> {
    let value = interp.coerce_to_float(value)?;
    let (fraction, exponent) = spinoso_math::frexp(value)?;
    Ok((fraction, exponent.into()))
}

pub fn gamma(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::gamma(value)?;
    Ok(result)
}

pub fn hypot(interp: &mut Artichoke, value: Value, other: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let other = interp.coerce_to_float(other)?;
    let result = spinoso_math::hypot(value, other);
    Ok(result)
}

#[allow(clippy::cast_possible_truncation)]
pub fn ldexp(interp: &mut Artichoke, fraction: Value, exponent: Value) -> Result<f64, Error> {
    let fraction = interp.coerce_to_float(fraction)?;
    let exponent = implicitly_convert_to_int(interp, exponent).map_err(|_| exponent.try_convert_into::<f64>(interp));
    let exponent = match exponent {
        Ok(exp) => exp,
        Err(Ok(exp)) if exp.is_nan() => {
            return Err(RangeError::with_message("float NaN out of range of integer").into());
        }
        Err(Ok(exp)) => {
            // This saturating cast will be rejected by the `i32::try_from`
            // below if `exp` is too large.
            exp as i64
        }
        Err(Err(err)) => return Err(err),
    };
    match i32::try_from(exponent) {
        Ok(exp) => {
            let result = spinoso_math::ldexp(fraction, exp)?;
            Ok(result)
        }
        Err(_) if exponent < 0 => {
            let mut message = String::from("integer ");
            write!(&mut message, "{}", exponent).map_err(WriteError::from)?;
            message.push_str("too small to convert to `int'");
            Err(RangeError::from(message).into())
        }
        Err(_) => {
            let mut message = String::from("integer ");
            write!(&mut message, "{}", exponent).map_err(WriteError::from)?;
            message.push_str("too big to convert to `int'");
            Err(RangeError::from(message).into())
        }
    }
}

pub fn lgamma(interp: &mut Artichoke, value: Value) -> Result<(f64, i64), Error> {
    let value = interp.coerce_to_float(value)?;
    let (result, sign) = spinoso_math::lgamma(value)?;
    Ok((result, sign.into()))
}

pub fn log(interp: &mut Artichoke, value: Value, base: Option<Value>) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let base = if let Some(base) = base {
        let base = interp.coerce_to_float(base)?;
        Some(base)
    } else {
        None
    };
    let result = spinoso_math::log(value, base)?;
    Ok(result)
}

pub fn log10(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::log10(value)?;
    Ok(result)
}

pub fn log2(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::log2(value)?;
    Ok(result)
}

pub fn sin(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = value.sin();
    Ok(result)
}

pub fn sinh(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::sinh(value);
    Ok(result)
}

pub fn sqrt(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::sqrt(value)?;
    Ok(result)
}

pub fn tan(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::tan(value);
    Ok(result)
}

pub fn tanh(interp: &mut Artichoke, value: Value) -> Result<f64, Error> {
    let value = interp.coerce_to_float(value)?;
    let result = spinoso_math::tanh(value);
    Ok(result)
}
