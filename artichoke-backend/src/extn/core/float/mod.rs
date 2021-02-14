use std::ffi::CStr;

use crate::extn::core::numeric::{self, Coercion, Outcome};
use crate::extn::prelude::*;

const FLOAT_CSTR: &CStr = cstr::cstr!("Float");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Float>() {
        return Ok(());
    }
    let spec = class::Spec::new("Float", FLOAT_CSTR, None, None)?;
    interp.def_class::<Float>(spec)?;
    let _ = interp.eval(&include_bytes!("float.rb")[..])?;

    let dig = interp.convert(Float::DIG);
    interp.define_class_constant::<Float>("DIG", dig)?;
    let epsilon = interp.convert_mut(Float::EPSILON);
    interp.define_class_constant::<Float>("EPSILON", epsilon)?;
    let infinity = interp.convert_mut(Float::INFINITY);
    interp.define_class_constant::<Float>("INFINITY", infinity)?;
    let mant_dig = interp.convert(Float::MANT_DIG);
    interp.define_class_constant::<Float>("MANT_DIG", mant_dig)?;
    let max = interp.convert_mut(Float::MAX);
    interp.define_class_constant::<Float>("MAX", max)?;
    let max_10_exp = interp.convert(Float::MAX_10_EXP);
    interp.define_class_constant::<Float>("MAX_10_EXP", max_10_exp)?;
    let max_exp = interp.convert(Float::MAX_EXP);
    interp.define_class_constant::<Float>("MAX_EXP", max_exp)?;
    let min = interp.convert_mut(Float::MIN);
    interp.define_class_constant::<Float>("MIN", min)?;
    let min_10_exp = interp.convert(Float::MIN_10_EXP);
    interp.define_class_constant::<Float>("MIN_10_EXP", min_10_exp)?;
    let min_exp = interp.convert(Float::MIN_EXP);
    interp.define_class_constant::<Float>("MIN_EXP", min_exp)?;
    let nan = interp.convert_mut(Float::NAN);
    interp.define_class_constant::<Float>("NAN", nan)?;
    let radix = interp.convert(Float::RADIX);
    interp.define_class_constant::<Float>("RADIX", radix)?;
    let rounds = interp.convert(Float::ROUNDS);
    interp.define_class_constant::<Float>("ROUNDS", rounds)?;

    trace!("Patched Float onto interpreter");
    Ok(())
}

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float(Fp);

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

impl From<Fp> for Float {
    #[inline]
    fn from(flt: Fp) -> Self {
        Self(flt)
    }
}

impl From<Float> for Fp {
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

impl From<Fp> for Outcome {
    #[inline]
    fn from(flt: Fp) -> Self {
        Self::Float(flt)
    }
}

impl Float {
    /// The minimum number of significant decimal digits in a double-precision
    /// floating point.
    ///
    /// Usually defaults to 15.
    pub const DIG: Int = Fp::DIGITS as Int;

    /// The difference between 1 and the smallest double-precision floating
    /// point number greater than 1.
    ///
    /// Usually defaults to 2.2204460492503131e-16.
    pub const EPSILON: Fp = Fp::EPSILON;

    /// An expression representing positive infinity.
    pub const INFINITY: Fp = Fp::INFINITY;

    /// The minimum number of significant decimal digits in a double-precision
    /// floating point.
    ///
    /// Usually defaults to 15.
    pub const MANT_DIG: Int = Fp::MANTISSA_DIGITS as Int;

    /// The largest possible integer in a double-precision floating point
    /// number.
    ///
    /// Usually defaults to 1.7976931348623157e+308.
    pub const MAX: Fp = Fp::MAX;

    /// The largest positive exponent in a double-precision floating point where
    /// 10 raised to this power minus 1.
    ///
    /// Usually defaults to 308.
    pub const MAX_10_EXP: Int = Fp::MAX_10_EXP as Int;

    /// The largest possible exponent value in a double-precision floating
    /// point.
    ///
    /// Usually defaults to 1024.
    pub const MAX_EXP: Int = Fp::MAX_EXP as Int;

    /// The smallest positive normalized number in a double-precision floating
    /// point.
    ///
    /// Usually defaults to 2.2250738585072014e-308.
    ///
    /// If the platform supports denormalized numbers, there are numbers between
    /// zero and [`Float::MIN`]. `0.0.next_float` returns the smallest positive
    /// floating point number including denormalized numbers.
    pub const MIN: Fp = Fp::MIN;

    /// The smallest negative exponent in a double-precision floating point
    /// where 10 raised to this power minus 1.
    ///
    /// Usually defaults to -307.
    pub const MIN_10_EXP: Int = Fp::MIN_10_EXP as Int;

    /// The smallest possible exponent value in a double-precision floating
    /// point.
    ///
    /// Usually defaults to -1021.
    pub const MIN_EXP: Int = Fp::MIN_EXP as Int;

    /// An expression representing a value which is "not a number".
    pub const NAN: Fp = Fp::NAN;

    pub const NEG_INFINITY: Fp = Fp::NEG_INFINITY;

    /// The base of the floating point, or number of unique digits used to
    /// represent the number.
    ///
    /// Usually defaults to 2 on most systems, which would represent a base-10
    /// decimal.
    pub const RADIX: Int = Fp::RADIX as Int;

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
    pub const ROUNDS: Int = -1;

    /// Construct a new, zero, float.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(0.0)
    }

    /// Return the inner [`f64`].
    #[inline]
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0
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
            let other = other.try_into::<Float>(interp)?;
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
