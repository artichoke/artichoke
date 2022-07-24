use std::fmt;

use crate::extn::core::numeric::{self, Coercion, Outcome};
use crate::extn::prelude::*;

pub mod mruby;

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float(f64);

impl fmt::Debug for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ConvertMut<Float, Value> for Artichoke {
    #[inline]
    fn convert_mut(&mut self, from: Float) -> Value {
        self.convert_mut(from.0)
    }
}

impl TryConvert<Value, Float> for Artichoke {
    type Error = Error;

    #[inline]
    fn try_convert(&self, value: Value) -> Result<Float, Self::Error> {
        let num = self.try_convert(value)?;
        Ok(Float(num))
    }
}

impl From<f64> for Float {
    #[inline]
    fn from(flt: f64) -> Self {
        Self(flt)
    }
}

impl From<Float> for f64 {
    #[inline]
    fn from(flt: Float) -> Self {
        flt.as_f64()
    }
}

impl From<Float> for Outcome {
    #[inline]
    fn from(flt: Float) -> Self {
        Self::Float(flt.into())
    }
}

impl From<f64> for Outcome {
    #[inline]
    fn from(flt: f64) -> Self {
        Self::Float(flt)
    }
}

impl Float {
    /// The minimum number of significant decimal digits in a double-precision
    /// floating point.
    ///
    /// Usually defaults to 15.
    pub const DIG: i64 = f64::DIGITS as i64;

    /// The difference between 1 and the smallest double-precision floating
    /// point number greater than 1.
    ///
    /// Usually defaults to 2.2204460492503131e-16.
    pub const EPSILON: f64 = f64::EPSILON;

    /// An expression representing positive infinity.
    pub const INFINITY: f64 = f64::INFINITY;

    /// The minimum number of significant decimal digits in a double-precision
    /// floating point.
    ///
    /// Usually defaults to 15.
    pub const MANT_DIG: i64 = f64::MANTISSA_DIGITS as i64;

    /// The largest possible integer in a double-precision floating point
    /// number.
    ///
    /// Usually defaults to `1.7976931348623157e+308`.
    pub const MAX: f64 = f64::MAX;

    /// The largest positive exponent in a double-precision floating point where
    /// 10 raised to this power minus 1.
    ///
    /// Usually defaults to 308.
    pub const MAX_10_EXP: i64 = f64::MAX_10_EXP as i64;

    /// The largest possible exponent value in a double-precision floating
    /// point.
    ///
    /// Usually defaults to 1024.
    pub const MAX_EXP: i64 = f64::MAX_EXP as i64;

    /// The smallest positive normalized number in a double-precision floating
    /// point.
    ///
    /// Usually defaults to 2.2250738585072014e-308.
    ///
    /// If the platform supports denormalized numbers, there are numbers between
    /// zero and [`Float::MIN`]. `0.0.next_float` returns the smallest positive
    /// floating point number including denormalized numbers.
    pub const MIN: f64 = f64::MIN;

    /// The smallest negative exponent in a double-precision floating point
    /// where 10 raised to this power minus 1.
    ///
    /// Usually defaults to -307.
    pub const MIN_10_EXP: i64 = f64::MIN_10_EXP as i64;

    /// The smallest possible exponent value in a double-precision floating
    /// point.
    ///
    /// Usually defaults to -1021.
    pub const MIN_EXP: i64 = f64::MIN_EXP as i64;

    /// An expression representing a value which is "not a number".
    pub const NAN: f64 = f64::NAN;

    pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;

    /// The base of the floating point, or number of unique digits used to
    /// represent the number.
    ///
    /// Usually defaults to 2 on most systems, which would represent a base-10
    /// decimal.
    pub const RADIX: i64 = f64::RADIX as i64;

    /// Represents the rounding mode for floating point addition.
    ///
    /// Usually defaults to 1, rounding to the nearest number.
    ///
    /// Other modes include:
    ///
    /// | mode                               | value |
    /// |------------------------------------|-------|
    /// | Indeterminable                     | -1    |
    /// | Rounding towards zero              | 0     |
    /// | Rounding to the nearest number     | 1     |
    /// | Rounding towards positive infinity | 2     |
    /// | Rounding towards negative infinity | 3     |
    ///
    /// # Rust Caveats
    ///
    /// Rust does not support setting the rounding mode and the behavior from
    /// LLVM is not documented.  Because of this uncertainty, Artichoke sets its
    /// rounding mode to `-1`, Indeterminable.
    ///
    /// The Rust docs say [`f64::round`][round] rounds "half-way cases away from
    /// 0.0." Stack Overflow has a
    /// [discussion around float rounding semantics][stackoverflow] in Rust and
    /// LLVM.
    ///
    /// [stackoverflow]: https://stackoverflow.com/a/28122536
    /// [round]: https://doc.rust-lang.org/1.42.0/std/primitive.f64.html#method.round
    pub const ROUNDS: i64 = -1;

    /// Construct a new, zero, float.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(0.0)
    }

    /// Construct a new `Float` with a given [`f64`].
    #[inline]
    #[must_use]
    pub const fn with_f64(f: f64) -> Self {
        Self(f)
    }

    /// Convert self to an `i64` with a saturating cast.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_i64(self) -> i64 {
        self.0 as i64
    }

    /// Return the inner [`f64`].
    #[inline]
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn try_into_fixnum(self) -> Option<i64> {
        const FIXABLE_MAX: f64 = 2_i64.pow(f64::MANTISSA_DIGITS) as f64;
        const FIXABLE_MIN: f64 = -(2_i64.pow(f64::MANTISSA_DIGITS)) as f64;

        match self.0 {
            x if !x.is_finite() => None,
            x if x > FIXABLE_MAX => None,
            x if x < FIXABLE_MIN => None,
            x => Some(x as i64),
        }
    }

    /// Compute the remainder of self and other.
    ///
    /// Equivalent to `self.as_f64() % other.as_f64()`.
    #[inline]
    #[must_use]
    pub fn modulo(self, other: Self) -> Self {
        Self(self.0 % other.0)
    }

    #[inline]
    pub fn coerced_modulo(self, interp: &mut Artichoke, other: Value) -> Result<Outcome, Error> {
        if let Ruby::Float = other.ruby_type() {
            let other = other.try_convert_into::<Float>(interp)?;
            return Ok(self.modulo(other).into());
        }
        let x = interp.convert_mut(self);
        let coerced = numeric::coerce(interp, x, other)?;
        match coerced {
            Coercion::Float(x, y) => Ok((x % y).into()),
            Coercion::Integer(x, y) => Ok((x % y).into()),
        }
    }
}
