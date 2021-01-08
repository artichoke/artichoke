use crate::convert::UnboxRubyError;
use crate::core::{ConvertMut, TryConvert, Value as _};
use crate::error::Error;
use crate::sys;
use crate::types::{Fp, Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

// TODO: when ,mruby is gone, float conversion should not allocate.
impl ConvertMut<Fp, Value> for Artichoke {
    fn convert_mut(&mut self, value: Fp) -> Value {
        let float = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_sys_float_value(mrb, value)) };
        self.protect(Value::from(float.unwrap()))
    }
}

impl TryConvert<Value, Fp> for Artichoke {
    type Error = Error;

    fn try_convert(&self, value: Value) -> Result<Fp, Self::Error> {
        if let Ruby::Float = value.ruby_type() {
            let value = value.inner();
            Ok(unsafe { sys::mrb_sys_float_to_cdouble(value) })
        } else {
            Err(UnboxRubyError::new(&value, Rust::Float).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = interpreter().unwrap();
        // get a Ruby Value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_into::<Fp>(&interp);
        assert!(result.is_err());
    }

    quickcheck! {
        fn convert_to_float(f: Fp) -> bool {
            let mut interp = interpreter().unwrap();
            let value = interp.convert_mut(f);
            value.ruby_type() == Ruby::Float
        }

        fn float_with_value(f: Fp) -> bool {
            let mut interp = interpreter().unwrap();
            let value = interp.convert_mut(f);
            let inner = value.inner();
            let cdouble = unsafe { sys::mrb_sys_float_to_cdouble(inner) };
            if f.is_nan() {
                cdouble.is_nan()
            } else if f.is_infinite() {
                f.is_infinite() && cdouble.signum() == f.signum()
            } else if cdouble >= f {
                let difference = cdouble - f;
                difference < Fp::EPSILON
            } else if f >= cdouble {
                let difference = f - cdouble;
                difference < Fp::EPSILON
            } else {
                false
            }
        }

        fn roundtrip(f: Fp) -> bool {
            let mut interp = interpreter().unwrap();
            let value = interp.convert_mut(f);
            let value = value.try_into::<Fp>(&interp).unwrap();
            if f.is_nan() {
                value.is_nan()
            } else if f.is_infinite() {
                value.is_infinite() && value.signum() == f.signum()
            } else if value >= f {
                let difference = value - f;
                difference < Fp::EPSILON
            } else if f >= value {
                let difference = f - value;
                difference < Fp::EPSILON
            } else {
                false
            }
        }

        fn roundtrip_err(b: bool) -> bool {
            let interp = interpreter().unwrap();
            let value = interp.convert(b);
            let value = value.try_into::<Fp>(&interp);
            value.is_err()
        }
    }
}
