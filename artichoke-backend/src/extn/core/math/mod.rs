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
    if let Ok(value) = value.clone().try_into::<Float>() {
        Ok(value)
    } else if let Ok(value) = value.clone().try_into::<Int>() {
        Ok(value as Float)
    } else if let Ruby::Nil = value.ruby_type() {
        Err(Exception::from(TypeError::new(
            interp,
            "can't convert nil into Float",
        )))
    } else {
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

pub fn erf(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "depend on libm or statsrs",
    )))
}

pub fn erfc(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "depend on libm or statsrs",
    )))
}

pub fn exp(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let result = value.exp();
    Ok(result)
}

pub fn frexp(interp: &mut Artichoke, value: Value) -> Result<(Float, Int), Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "depend on libm or statsrs",
    )))
}

pub fn gamma(interp: &mut Artichoke, value: Value) -> Result<Float, Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "depend on libm or statsrs",
    )))
}

pub fn hypot(interp: &mut Artichoke, value: Value, other: Value) -> Result<Float, Exception> {
    let value = value_to_float(interp, value)?;
    let other = value_to_float(interp, other)?;
    let result = value.hypot(other);
    Ok(result)
}

pub fn ldexp(interp: &mut Artichoke, fraction: Value, exponent: Value) -> Result<Float, Exception> {
    let _ = fraction;
    let _ = exponent;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "depend on libm or statsrs",
    )))
}

pub fn lgamma(interp: &mut Artichoke, value: Value) -> Result<(Float, Int), Exception> {
    let _ = value;
    Err(Exception::from(NotImplementedError::new(
        interp,
        "depend on libm or statsrs",
    )))
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

#[derive(Debug, Clone)]
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
